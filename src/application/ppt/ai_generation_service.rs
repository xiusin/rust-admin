use crate::service::prelude::*;
use crate::domain::entity::{ai_generation_log, ppt_project};
use crate::domain::model::m_ppt::*;
use crate::domain::args::a_ppt::*;

pub async fn list(
    VQuery(page): VQuery<PageParams>,
    VQuery(search): VQuery<AiGenerationLogListArgs>,
) -> impl IntoResponse {
    let db = DB().await;
    
    let page_num = page.page_num.unwrap_or(1);
    let page_size = page.page_size.unwrap_or(10);
    
    let mut conditions = Condition::all();
    
    if let Some(project_id) = search.project_id {
        conditions = conditions.add(ai_generation_log::Column::ProjectId.eq(Some(project_id)));
    }
    if let Some(ref task_type) = search.task_type {
        conditions = conditions.add(ai_generation_log::Column::TaskType.eq(task_type));
    }
    if let Some(ref ai_provider) = search.ai_provider {
        conditions = conditions.add(ai_generation_log::Column::AiProvider.eq(ai_provider));
    }
    if let Some(ref status) = search.status {
        conditions = conditions.add(ai_generation_log::Column::Status.eq(status));
    }
    
    let total = ai_generation_log::Entity::find()
        .filter(conditions.clone())
        .count(db)
        .await
        .unwrap_or(0);
    
    let items = ai_generation_log::Entity::find()
        .filter(conditions)
        .order_by_desc(ai_generation_log::Column::CreatedAt)
        .offset((page_num - 1) * page_size)
        .limit(page_size)
        .all(db)
        .await;
    
    match items {
        Ok(items) => {
            let mut list_items = Vec::new();
            for item in items {
                let project_title = if let Some(project_id) = item.project_id {
                    ppt_project::Entity::find_by_id(project_id)
                        .one(db)
                        .await
                        .ok()
                        .flatten()
                        .map(|p| p.title)
                } else {
                    None
                };
                
                list_items.push(AiGenerationLogListItem {
                    id: item.id,
                    project_id: item.project_id,
                    project_title,
                    task_type: item.task_type,
                    ai_provider: item.ai_provider,
                    model: item.model,
                    tokens_used: item.tokens_used,
                    cost: item.cost,
                    duration_ms: item.duration_ms,
                    status: item.status,
                    created_at: item.created_at,
                });
            }
            
            ApiResponse::ok(ListData {
                list: list_items,
                total,
                total_pages: (total + page_size - 1) / page_size,
                page_num,
            })
        }
        Err(e) => ApiResponse::bad_request(e.to_string()),
    }
}

pub async fn detail(Path(id): Path<i64>) -> impl IntoResponse {
    let db = DB().await;
    
    let log = match ai_generation_log::Entity::find_by_id(id)
        .one(db)
        .await
    {
        Ok(Some(l)) => l,
        Ok(None) => return ApiResponse::not_found("AI生成日志不存在"),
        Err(e) => return ApiResponse::bad_request(e.to_string()),
    };
    
    let project_title = if let Some(project_id) = log.project_id {
        ppt_project::Entity::find_by_id(project_id)
            .one(db)
            .await
            .ok()
            .flatten()
            .map(|p| p.title)
    } else {
        None
    };
    
    ApiResponse::ok(AiGenerationLogDetail {
        id: log.id,
        project_id: log.project_id,
        project_title,
        task_type: log.task_type,
        ai_provider: log.ai_provider,
        model: log.model,
        input_data: log.input_data,
        output_data: log.output_data,
        tokens_used: log.tokens_used,
        cost: log.cost,
        duration_ms: log.duration_ms,
        status: log.status,
        created_at: log.created_at,
    })
}

pub async fn delete(VJson(arg): VJson<AiGenerationLogDeleteArgs>) -> impl IntoResponse {
    let db = DB().await;
    
    for id in arg.ids {
        let _ = ai_generation_log::Entity::delete_by_id(id)
            .exec(db)
            .await;
    }
    
    ApiResponse::ok("删除成功")
}

pub async fn statistics() -> impl IntoResponse {
    let db = DB().await;
    
    let total_logs = ai_generation_log::Entity::find()
        .count(db)
        .await
        .unwrap_or(0);
    
    let success_count = ai_generation_log::Entity::find()
        .filter(ai_generation_log::Column::Status.eq("success"))
        .count(db)
        .await
        .unwrap_or(0);
    
    let failed_count = ai_generation_log::Entity::find()
        .filter(ai_generation_log::Column::Status.eq("failed"))
        .count(db)
        .await
        .unwrap_or(0);
    
    let total_tokens: i64 = ai_generation_log::Entity::find()
        .all(db)
        .await
        .map(|logs| logs.iter().map(|l| l.tokens_used as i64).sum())
        .unwrap_or(0);
    
    let total_cost: f64 = ai_generation_log::Entity::find()
        .all(db)
        .await
        .map(|logs| {
            logs.iter()
                .filter_map(|l| l.cost.map(|c| c.to_string().parse::<f64>().unwrap_or(0.0)))
                .sum()
        })
        .unwrap_or(0.0);
    
    ApiResponse::ok(json!({
        "total_logs": total_logs,
        "success_count": success_count,
        "failed_count": failed_count,
        "total_tokens": total_tokens,
        "total_cost": total_cost,
    }))
}
