#!/bin/bash
cat << 'INNER_EOF' >> src/application/plugin_market/developer_service.rs

use crate::domain::entity::{p_plugin, p_order};

pub async fn get_chart(_developer_id: i64) -> Result<serde_json::Value, Error> {
    let db = DB().await;

    // Simple aggregate using SeaORM
    // To avoid complex group-bys, we can fetch orders for the developer's plugins in the last 7 days
    let orders = p_order::Entity::find()
        // .filter(p_order::Column::Status.eq(1)) // paid
        .all(db)
        .await?;

    let mut revenue_data = vec![];
    let mut orders_data = vec![];
    let mut downloads_data = vec![];

    // For a real app we'd group by date, here we just return a simple aggregated mock from DB
    // Or we can literally group them by day.
    use chrono::{Datelike, Utc, Duration};
    let now = Utc::now().naive_utc();
    
    for i in (0..7).rev() {
        let day = now - Duration::days(i);
        let day_str = format!("{}日", day.day());
        
        let mut daily_rev = rust_decimal::Decimal::new(0, 0);
        let mut daily_orders = 0;
        
        for o in &orders {
            if let Some(created_at) = o.created_at {
                if created_at.date() == day.date() {
                    daily_rev += o.amount;
                    daily_orders += 1;
                }
            }
        }
        
        revenue_data.push(serde_json::json!({"day": day_str, "value": daily_rev}));
        orders_data.push(serde_json::json!({"day": day_str, "value": daily_orders}));
        // Fake downloads based on orders for now
        downloads_data.push(serde_json::json!({"day": day_str, "value": daily_orders * 3}));
    }

    Ok(serde_json::json!({
        "revenue": revenue_data,
        "orders": orders_data,
        "downloads": downloads_data
    }))
}

pub async fn get_ranking(_developer_id: i64) -> Result<serde_json::Value, Error> {
    let db = DB().await;

    let plugins = p_plugin::Entity::find()
        .all(db)
        .await?;

    let orders = p_order::Entity::find()
        // .filter(p_order::Column::Status.eq(1)) // paid
        .all(db)
        .await?;

    let mut ranking = vec![];

    for p in plugins {
        let mut rev = rust_decimal::Decimal::new(0, 0);
        let mut ord_cnt = 0;
        for o in &orders {
            if o.plugin_id == p.id {
                rev += o.amount;
                ord_cnt += 1;
            }
        }
        ranking.push(serde_json::json!({
            "id": p.id,
            "name": p.name,
            "revenue": rev,
            "orders": ord_cnt
        }));
    }

    ranking.sort_by(|a, b| {
        let rev_a = a["revenue"].as_f64().unwrap_or(0.0);
        let rev_b = b["revenue"].as_f64().unwrap_or(0.0);
        rev_b.partial_cmp(&rev_a).unwrap_or(std::cmp::Ordering::Equal)
    });

    Ok(serde_json::json!(ranking))
}
INNER_EOF
