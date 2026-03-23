use crate::common::error::Error;
use crate::domain::model::m_finance::*;
use crate::domain::entity::prelude::{CFinanceAccount, CFinanceTransaction};
use crate::domain::entity::{c_finance_account, c_finance_transaction};
use crate::model::prelude::*;
use sea_orm::*;
use rust_decimal::Decimal;
use chrono::Local;

fn generate_id() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64
}

fn generate_transaction_no(prefix: &str) -> String {
    format!("{}{}{}", prefix, chrono::Utc::now().format("%Y%m%d%H%M%S"), generate_id() % 10000)
}

pub async fn get_account(consumer_id: i64) -> Result<AccountInfoResp> {
    let db = DB_WRITE().await;

    let account = CFinanceAccount::find()
        .filter(c_finance_account::Column::ConsumerId.eq(consumer_id))
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?
        .ok_or_else(|| Error::not_found("账户不存在"))?;

    Ok(AccountInfoResp {
        id: account.id,
        consumer_id: account.consumer_id,
        balance: account.balance,
        frozen_balance: account.frozen_balance,
        available_balance: account.balance - account.frozen_balance,
        created_at: account.created_at,
        updated_at: account.updated_at,
    })
}

pub async fn recharge(params: RechargeParams) -> Result<TransactionModel> {
    let db = DB_WRITE().await;

    let account = CFinanceAccount::find()
        .filter(c_finance_account::Column::ConsumerId.eq(params.consumer_id))
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?
        .ok_or_else(|| Error::not_found("账户不存在"))?;

    if params.amount <= Decimal::ZERO {
        return Err(Error::bad_request("充值金额必须大于0"));
    }

    let now = Local::now().naive_local();
    let transaction_id = generate_id();
    let transaction_no = generate_transaction_no("R");

    let original_balance = account.balance;
    let new_balance = account.balance + params.amount;

    let transaction = c_finance_transaction::ActiveModel {
        id: Set(transaction_id),
        consumer_id: Set(params.consumer_id),
        transaction_no: Set(transaction_no.clone()),
        transaction_type: Set("recharge".to_string()),
        amount: Set(params.amount),
        balance_before: Set(original_balance),
        balance_after: Set(new_balance),
        description: Set(Some(format!("账户充值"))),
        related_order_no: Set(params.related_order_no.clone()),
        operator_id: Set(None),
        created_at: Set(Some(now)),
        ..Default::default()
    };

    let mut active_model: c_finance_account::ActiveModel = account.into();
    active_model.balance = Set(new_balance);
    active_model.updated_at = Set(Some(now));

    CFinanceTransaction::insert(transaction)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Insert error: {}", e)))?;

    CFinanceAccount::update(active_model)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Update error: {}", e)))?;

    Ok(TransactionModel {
        id: transaction_id,
        consumer_id: params.consumer_id,
        transaction_no,
        transaction_type: "recharge".to_string(),
        amount: params.amount,
        balance_before: original_balance,
        balance_after: new_balance,
        description: Some("账户充值".to_string()),
        related_order_no: params.related_order_no,
        operator_id: None,
        created_at: Some(now),
    })
}

pub async fn withdraw(params: WithdrawParams) -> Result<TransactionModel> {
    let db = DB_WRITE().await;

    if params.amount <= Decimal::ZERO {
        return Err(Error::bad_request("提现金额必须大于0"));
    }

    let account = CFinanceAccount::find()
        .filter(c_finance_account::Column::ConsumerId.eq(params.consumer_id))
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?
        .ok_or_else(|| Error::not_found("账户不存在"))?;

    let available = account.balance - account.frozen_balance;
    if available < params.amount {
        return Err(Error::bad_request(format!("可用余额不足，当前可用余额: {}", available)));
    }

    let now = Local::now().naive_local();
    let transaction_id = generate_id();
    let transaction_no = generate_transaction_no("W");

    let original_balance = account.balance;
    let new_frozen = account.frozen_balance + params.amount;

    let transaction = c_finance_transaction::ActiveModel {
        id: Set(transaction_id),
        consumer_id: Set(params.consumer_id),
        transaction_no: Set(transaction_no.clone()),
        transaction_type: Set("withdraw".to_string()),
        amount: Set(params.amount),
        balance_before: Set(original_balance),
        balance_after: Set(original_balance),
        description: Set(Some(format!("提现申请，冻结金额: {}", params.amount))),
        related_order_no: Set(None),
        operator_id: Set(None),
        created_at: Set(Some(now)),
        ..Default::default()
    };

    let mut active_model: c_finance_account::ActiveModel = account.into();
    active_model.frozen_balance = Set(new_frozen);
    active_model.updated_at = Set(Some(now));

    CFinanceTransaction::insert(transaction)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Insert error: {}", e)))?;

    CFinanceAccount::update(active_model)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Update error: {}", e)))?;

    Ok(TransactionModel {
        id: transaction_id,
        consumer_id: params.consumer_id,
        transaction_no,
        transaction_type: "withdraw".to_string(),
        amount: params.amount,
        balance_before: original_balance,
        balance_after: original_balance,
        description: Some("提现申请".to_string()),
        related_order_no: None,
        operator_id: None,
        created_at: Some(now),
    })
}

pub async fn approve_withdraw(params: WithdrawApproveParams) -> Result<TransactionModel> {
    let db = DB_WRITE().await;

    let transaction = CFinanceTransaction::find_by_id(params.transaction_id)
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?
        .ok_or_else(|| Error::not_found("交易记录不存在"))?;

    if transaction.transaction_type != "withdraw" {
        return Err(Error::bad_request("该交易不是提现申请"));
    }

    let account = CFinanceAccount::find()
        .filter(c_finance_account::Column::ConsumerId.eq(transaction.consumer_id))
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?
        .ok_or_else(|| Error::not_found("账户不存在"))?;

    let now = Local::now().naive_local();
    let amount = transaction.amount;

    let (new_balance, new_frozen, description) = if params.approve {
        (account.balance - amount, account.frozen_balance - amount, "提现成功".to_string())
    } else {
        (account.balance, account.frozen_balance - amount, format!("提现拒绝: {}", params.reason.unwrap_or_default()))
    };

    let mut tx_active: c_finance_transaction::ActiveModel = transaction.clone().into();
    tx_active.description = Set(Some(description.clone()));
    tx_active.operator_id = Set(Some(params.operator_id));

    let mut acc_active: c_finance_account::ActiveModel = account.into();
    acc_active.balance = Set(new_balance);
    acc_active.frozen_balance = Set(new_frozen);
    acc_active.updated_at = Set(Some(now));

    CFinanceTransaction::update(tx_active)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Update error: {}", e)))?;

    CFinanceAccount::update(acc_active)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Update error: {}", e)))?;

    Ok(TransactionModel {
        id: transaction.id,
        consumer_id: transaction.consumer_id,
        transaction_no: transaction.transaction_no,
        transaction_type: transaction.transaction_type,
        amount: transaction.amount,
        balance_before: transaction.balance_before,
        balance_after: new_balance,
        description: Some(description),
        related_order_no: transaction.related_order_no,
        operator_id: Some(params.operator_id),
        created_at: transaction.created_at,
    })
}

pub async fn consume(params: ConsumeParams) -> Result<TransactionModel> {
    let db = DB_WRITE().await;

    if params.amount <= Decimal::ZERO {
        return Err(Error::bad_request("消费金额必须大于0"));
    }

    let account = CFinanceAccount::find()
        .filter(c_finance_account::Column::ConsumerId.eq(params.consumer_id))
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?
        .ok_or_else(|| Error::not_found("账户不存在"))?;

    let available = account.balance - account.frozen_balance;
    if available < params.amount {
        return Err(Error::bad_request(format!("可用余额不足，当前可用余额: {}", available)));
    }

    let now = Local::now().naive_local();
    let transaction_id = generate_id();
    let transaction_no = generate_transaction_no("C");

    let original_balance = account.balance;
    let new_balance = account.balance - params.amount;

    let transaction = c_finance_transaction::ActiveModel {
        id: Set(transaction_id),
        consumer_id: Set(params.consumer_id),
        transaction_no: Set(transaction_no.clone()),
        transaction_type: Set("consume".to_string()),
        amount: Set(params.amount),
        balance_before: Set(original_balance),
        balance_after: Set(new_balance),
        description: Set(params.description.clone()),
        related_order_no: Set(params.related_order_no.clone()),
        operator_id: Set(None),
        created_at: Set(Some(now)),
        ..Default::default()
    };

    let mut active_model: c_finance_account::ActiveModel = account.into();
    active_model.balance = Set(new_balance);
    active_model.updated_at = Set(Some(now));

    CFinanceTransaction::insert(transaction)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Insert error: {}", e)))?;

    CFinanceAccount::update(active_model)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Update error: {}", e)))?;

    Ok(TransactionModel {
        id: transaction_id,
        consumer_id: params.consumer_id,
        transaction_no,
        transaction_type: "consume".to_string(),
        amount: params.amount,
        balance_before: original_balance,
        balance_after: new_balance,
        description: params.description,
        related_order_no: params.related_order_no,
        operator_id: None,
        created_at: Some(now),
    })
}

pub async fn refund(params: RefundParams) -> Result<TransactionModel> {
    let db = DB_WRITE().await;

    if params.amount <= Decimal::ZERO {
        return Err(Error::bad_request("退款金额必须大于0"));
    }

    let account = CFinanceAccount::find()
        .filter(c_finance_account::Column::ConsumerId.eq(params.consumer_id))
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?
        .ok_or_else(|| Error::not_found("账户不存在"))?;

    let now = Local::now().naive_local();
    let transaction_id = generate_id();
    let transaction_no = generate_transaction_no("RF");

    let original_balance = account.balance;
    let new_balance = account.balance + params.amount;

    let transaction = c_finance_transaction::ActiveModel {
        id: Set(transaction_id),
        consumer_id: Set(params.consumer_id),
        transaction_no: Set(transaction_no.clone()),
        transaction_type: Set("refund".to_string()),
        amount: Set(params.amount),
        balance_before: Set(original_balance),
        balance_after: Set(new_balance),
        description: Set(params.description.clone()),
        related_order_no: Set(params.related_order_no.clone()),
        operator_id: Set(None),
        created_at: Set(Some(now)),
        ..Default::default()
    };

    let mut active_model: c_finance_account::ActiveModel = account.into();
    active_model.balance = Set(new_balance);
    active_model.updated_at = Set(Some(now));

    CFinanceTransaction::insert(transaction)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Insert error: {}", e)))?;

    CFinanceAccount::update(active_model)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Update error: {}", e)))?;

    Ok(TransactionModel {
        id: transaction_id,
        consumer_id: params.consumer_id,
        transaction_no,
        transaction_type: "refund".to_string(),
        amount: params.amount,
        balance_before: original_balance,
        balance_after: new_balance,
        description: params.description,
        related_order_no: params.related_order_no,
        operator_id: None,
        created_at: Some(now),
    })
}

pub async fn freeze_amount(consumer_id: i64, amount: Decimal) -> Result<()> {
    let db = DB_WRITE().await;

    let account = CFinanceAccount::find()
        .filter(c_finance_account::Column::ConsumerId.eq(consumer_id))
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?
        .ok_or_else(|| Error::not_found("账户不存在"))?;

    let available = account.balance - account.frozen_balance;
    if available < amount {
        return Err(Error::bad_request("可用余额不足"));
    }

    let new_frozen = account.frozen_balance + amount;
    let now = Local::now().naive_local();
    let mut active_model: c_finance_account::ActiveModel = account.into();
    active_model.frozen_balance = Set(new_frozen);
    active_model.updated_at = Set(Some(now));

    CFinanceAccount::update(active_model)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Update error: {}", e)))?;

    Ok(())
}

pub async fn unfreeze_amount(consumer_id: i64, amount: Decimal) -> Result<()> {
    let db = DB_WRITE().await;

    let account = CFinanceAccount::find()
        .filter(c_finance_account::Column::ConsumerId.eq(consumer_id))
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?
        .ok_or_else(|| Error::not_found("账户不存在"))?;

    if account.frozen_balance < amount {
        return Err(Error::bad_request("冻结余额不足"));
    }

    let new_frozen = account.frozen_balance - amount;
    let now = Local::now().naive_local();
    let mut active_model: c_finance_account::ActiveModel = account.into();
    active_model.frozen_balance = Set(new_frozen);
    active_model.updated_at = Set(Some(now));

    CFinanceAccount::update(active_model)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Update error: {}", e)))?;

    Ok(())
}

pub async fn get_statistics(consumer_id: i64) -> Result<FinanceStatistics> {
    let db = DB_WRITE().await;

    let transactions = CFinanceTransaction::find()
        .filter(c_finance_transaction::Column::ConsumerId.eq(consumer_id))
        .all(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    let mut total_recharge = Decimal::ZERO;
    let mut total_withdraw = Decimal::ZERO;
    let mut total_consume = Decimal::ZERO;
    let mut total_refund = Decimal::ZERO;

    for t in transactions {
        match t.transaction_type.as_str() {
            "recharge" => total_recharge += t.amount,
            "withdraw" => total_withdraw += t.amount,
            "consume" => total_consume += t.amount,
            "refund" => total_refund += t.amount,
            _ => {}
        }
    }

    let account = CFinanceAccount::find()
        .filter(c_finance_account::Column::ConsumerId.eq(consumer_id))
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    let (balance, frozen_balance, available_balance) = match account {
        Some(a) => (a.balance, a.frozen_balance, a.balance - a.frozen_balance),
        None => (Decimal::ZERO, Decimal::ZERO, Decimal::ZERO),
    };

    Ok(FinanceStatistics {
        balance,
        frozen_balance,
        available_balance,
        total_recharge,
        total_withdraw,
        total_consume,
        total_refund,
    })
}

pub async fn list_transactions(
    params: TransactionListParams,
) -> Result<(Vec<TransactionModel>, u64)> {
    let db = DB_WRITE().await;
    let page_num = params.page_num.unwrap_or(1) as u64;
    let page_size = params.page_size.unwrap_or(10) as u64;

    let mut conditions = Condition::all();

    if let Some(consumer_id) = params.consumer_id {
        conditions = conditions.add(c_finance_transaction::Column::ConsumerId.eq(consumer_id));
    }
    if let Some(transaction_type) = &params.transaction_type {
        conditions = conditions.add(c_finance_transaction::Column::TransactionType.eq(transaction_type.clone()));
    }
    if let Some(start_time) = params.start_time {
        let naive = chrono::DateTime::from_timestamp(start_time, 0)
            .map(|dt| dt.naive_local())
            .unwrap_or_else(|| Local::now().naive_local());
        conditions = conditions.add(c_finance_transaction::Column::CreatedAt.gte(naive));
    }
    if let Some(end_time) = params.end_time {
        let naive = chrono::DateTime::from_timestamp(end_time, 0)
            .map(|dt| dt.naive_local())
            .unwrap_or_else(|| Local::now().naive_local());
        conditions = conditions.add(c_finance_transaction::Column::CreatedAt.lte(naive));
    }

    let paginator = CFinanceTransaction::find()
        .filter(conditions)
        .order_by_desc(c_finance_transaction::Column::CreatedAt)
        .paginate(db, page_size);

    let total = paginator
        .num_items()
        .await
        .map_err(|e| Error::internal_error(format!("Pagination error: {}", e)))?;

    let transactions = paginator
        .fetch_page(page_num - 1)
        .await
        .map_err(|e| Error::internal_error(format!("Fetch error: {}", e)))?;

    let items: Vec<TransactionModel> = transactions
        .into_iter()
        .map(|t| TransactionModel {
            id: t.id,
            consumer_id: t.consumer_id,
            transaction_no: t.transaction_no,
            transaction_type: t.transaction_type,
            amount: t.amount,
            balance_before: t.balance_before,
            balance_after: t.balance_after,
            description: t.description,
            related_order_no: t.related_order_no,
            operator_id: t.operator_id,
            created_at: t.created_at,
        })
        .collect();

    Ok((items, total))
}

pub async fn ensure_account_exists(consumer_id: i64) -> Result<()> {
    let db = DB_WRITE().await;

    let exists = CFinanceAccount::find()
        .filter(c_finance_account::Column::ConsumerId.eq(consumer_id))
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    if exists.is_none() {
        let now = Local::now().naive_local();
        CFinanceAccount::insert(c_finance_account::ActiveModel {
            id: Set(generate_id()),
            consumer_id: Set(consumer_id),
            balance: Set(Decimal::ZERO),
            frozen_balance: Set(Decimal::ZERO),
            created_at: Set(Some(now)),
            updated_at: Set(Some(now)),
            ..Default::default()
        })
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Insert error: {}", e)))?;
    }

    Ok(())
}
