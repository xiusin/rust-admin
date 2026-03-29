use crate::service::prelude::*;
use crate::domain::entity::{ppt_project, ppt_page, page_element, style_template};
use crate::domain::model::m_ppt::*;
use crate::domain::args::a_ppt::*;

pub async fn list(
    VQuery(page): VQuery<PageParams>,
    VQuery(search): VQuery<PptProjectListArgs>,
) -> impl IntoResponse {
    let db = DB().await;
    
    let page_num = page.page_num.unwrap_or(1);
    let page_size = page.page_size.unwrap_or(10);
    
    let mut conditions = Condition::all();
    
    if let Some(user_id) = search.user_id {
        conditions = conditions.add(ppt_project::Column::UserId.eq(user_id));
    }
    if let Some(ref title) = search.title {
        conditions = conditions.add(ppt_project::Column::Title.contains(title));
    }
    if let Some(ref source_type) = search.source_type {
        conditions = conditions.add(ppt_project::Column::SourceType.eq(source_type));
    }
    if let Some(ref status) = search.status {
        conditions = conditions.add(ppt_project::Column::Status.eq(status));
    }
    if let Some(ref industry) = search.industry {
        conditions = conditions.add(ppt_project::Column::Industry.contains(industry));
    }
    
    let total = ppt_project::Entity::find()
        .filter(conditions.clone())
        .count(db)
        .await
        .unwrap_or(0);
    
    let items = ppt_project::Entity::find()
        .filter(conditions)
        .order_by_desc(ppt_project::Column::CreatedAt)
        .offset((page_num - 1) * page_size)
        .limit(page_size)
        .all(db)
        .await;
    
    match items {
        Ok(items) => {
            let mut list_items = Vec::new();
            for item in items {
                let template_name = if let Some(template_id) = item.style_template_id {
                    style_template::Entity::find_by_id(template_id)
                        .one(db)
                        .await
                        .ok()
                        .flatten()
                        .map(|t| t.name)
                } else {
                    None
                };
                
                let page_count = ppt_page::Entity::find()
                    .filter(ppt_page::Column::ProjectId.eq(item.id))
                    .count(db)
                    .await
                    .unwrap_or(0) as i32;
                
                list_items.push(PptProjectListItem {
                    id: item.id,
                    user_id: item.user_id,
                    title: item.title,
                    description: item.description,
                    source_type: item.source_type,
                    source_file: item.source_file,
                    style_template_id: item.style_template_id,
                    style_template_name: template_name,
                    industry: item.industry,
                    industry_confidence: item.industry_confidence,
                    status: item.status,
                    page_count,
                    created_at: item.created_at,
                    updated_at: item.updated_at,
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
    
    let project = match ppt_project::Entity::find_by_id(id)
        .one(db)
        .await
    {
        Ok(Some(p)) => p,
        Ok(None) => return ApiResponse::not_found("PPT项目不存在"),
        Err(e) => return ApiResponse::bad_request(e.to_string()),
    };
    
    let template_name = if let Some(template_id) = project.style_template_id {
        style_template::Entity::find_by_id(template_id)
            .one(db)
            .await
            .ok()
            .flatten()
            .map(|t| t.name)
    } else {
        None
    };
    
    let pages = match ppt_page::Entity::find()
        .filter(ppt_page::Column::ProjectId.eq(id))
        .order_by_asc(ppt_page::Column::PageOrder)
        .all(db)
        .await
    {
        Ok(p) => p,
        Err(e) => return ApiResponse::bad_request(e.to_string()),
    };
    
    let mut pages_with_elements = Vec::new();
    for page in pages {
        let elements = match page_element::Entity::find()
            .filter(page_element::Column::PageId.eq(page.id))
            .order_by_asc(page_element::Column::ZIndex)
            .all(db)
            .await
        {
            Ok(e) => e,
            Err(_) => vec![],
        };
        
        pages_with_elements.push(PptPageWithElements {
            id: page.id,
            project_id: page.project_id,
            page_order: page.page_order,
            page_type: page.page_type,
            title: page.title,
            layout_config: page.layout_config,
            style_config: page.style_config,
            elements: elements.into_iter().map(|e| PageElementModel {
                id: e.id,
                page_id: e.page_id,
                element_type: e.element_type,
                content: e.content,
                style: e.style,
                position: e.position,
                size: e.size,
                rotation: e.rotation,
                z_index: e.z_index,
                locked: e.locked,
            }).collect(),
        });
    }
    
    ApiResponse::ok(PptProjectDetail {
        id: project.id,
        user_id: project.user_id,
        title: project.title,
        description: project.description,
        source_type: project.source_type,
        source_file: project.source_file,
        style_template_id: project.style_template_id,
        style_template_name: template_name,
        industry: project.industry,
        industry_confidence: project.industry_confidence,
        status: project.status,
        metadata: project.metadata,
        created_at: project.created_at,
        updated_at: project.updated_at,
        pages: pages_with_elements,
    })
}

pub async fn add(VJson(arg): VJson<PptProjectAddArgs>) -> impl IntoResponse {
    let db = DB().await;
    
    let id = GID().await;
    let now = Local::now().naive_local();
    
    let project = ppt_project::ActiveModel {
        id: Set(id),
        user_id: Set(arg.user_id),
        title: Set(arg.title),
        description: Set(arg.description),
        source_type: Set(arg.source_type),
        source_file: Set(arg.source_file),
        style_template_id: Set(arg.style_template_id),
        industry: Set(arg.industry),
        industry_confidence: Set(None),
        status: Set("draft".to_string()),
        metadata: Set(arg.metadata),
        created_at: Set(Some(now)),
        updated_at: Set(Some(now)),
    };
    
    match project.insert(db).await {
        Ok(_) => ApiResponse::ok(id),
        Err(e) => ApiResponse::bad_request(e.to_string()),
    }
}

pub async fn edit(VJson(arg): VJson<PptProjectEditArgs>) -> impl IntoResponse {
    let db = DB().await;
    
    let project = match ppt_project::Entity::find_by_id(arg.id)
        .one(db)
        .await
    {
        Ok(Some(p)) => p,
        Ok(None) => return ApiResponse::not_found("PPT项目不存在"),
        Err(e) => return ApiResponse::bad_request(e.to_string()),
    };
    
    let now = Local::now().naive_local();
    
    let mut project: ppt_project::ActiveModel = project.into();
    project.title = Set(arg.title);
    project.description = Set(arg.description);
    project.source_type = Set(arg.source_type.unwrap_or(project.source_type.unwrap()));
    project.source_file = Set(arg.source_file);
    project.style_template_id = Set(arg.style_template_id);
    project.industry = Set(arg.industry);
    project.status = Set(arg.status.unwrap_or(project.status.unwrap()));
    project.metadata = Set(arg.metadata);
    project.updated_at = Set(Some(now));
    
    match project.update(db).await {
        Ok(_) => ApiResponse::ok("更新成功"),
        Err(e) => ApiResponse::bad_request(e.to_string()),
    }
}

pub async fn delete(VJson(arg): VJson<PptProjectDeleteArgs>) -> impl IntoResponse {
    let db = DB().await;
    
    for id in arg.ids {
        let _ = ppt_project::Entity::delete_by_id(id)
            .exec(db)
            .await;
        let _ = ppt_page::Entity::delete_many()
            .filter(ppt_page::Column::ProjectId.eq(id))
            .exec(db)
            .await;
    }
    
    ApiResponse::ok("删除成功")
}

pub async fn delete_by_id(Path(id): Path<i64>) -> impl IntoResponse {
    let db = DB().await;
    
    let _ = ppt_project::Entity::delete_by_id(id)
        .exec(db)
        .await;
    let _ = ppt_page::Entity::delete_many()
        .filter(ppt_page::Column::ProjectId.eq(id))
        .exec(db)
        .await;
    
    ApiResponse::ok("删除成功")
}

pub async fn copy(Path(id): Path<i64>) -> impl IntoResponse {
    let db = DB().await;
    
    let project = match ppt_project::Entity::find_by_id(id)
        .one(db)
        .await
    {
        Ok(Some(p)) => p,
        Ok(None) => return ApiResponse::not_found("PPT项目不存在"),
        Err(e) => return ApiResponse::bad_request(e.to_string()),
    };
    
    let new_id = GID().await;
    let now = Local::now().naive_local();
    
    let new_project = ppt_project::ActiveModel {
        id: Set(new_id),
        user_id: Set(project.user_id),
        title: Set(format!("{} (副本)", project.title)),
        description: Set(project.description),
        source_type: Set(project.source_type),
        source_file: Set(project.source_file),
        style_template_id: Set(project.style_template_id),
        industry: Set(project.industry),
        industry_confidence: Set(project.industry_confidence),
        status: Set("draft".to_string()),
        metadata: Set(project.metadata),
        created_at: Set(Some(now)),
        updated_at: Set(Some(now)),
    };
    
    match new_project.insert(db).await {
        Ok(_) => ApiResponse::ok(new_id),
        Err(e) => ApiResponse::bad_request(e.to_string()),
    }
}

pub async fn update_status(VJson(arg): VJson<PptProjectStatusArgs>) -> impl IntoResponse {
    let db = DB().await;
    
    let now = Local::now().naive_local();
    
    for id in arg.ids {
        if let Some(project) = ppt_project::Entity::find_by_id(id)
            .one(db)
            .await
            .ok()
            .flatten()
        {
            let mut project: ppt_project::ActiveModel = project.into();
            project.status = Set(arg.status.clone());
            project.updated_at = Set(Some(now));
            let _ = project.update(db).await;
        }
    }
    
    ApiResponse::ok("状态更新成功")
}

pub async fn statistics() -> impl IntoResponse {
    let db = DB().await;
    
    let total = ppt_project::Entity::find()
        .count(db)
        .await
        .unwrap_or(0);
    
    let draft_count = ppt_project::Entity::find()
        .filter(ppt_project::Column::Status.eq("draft"))
        .count(db)
        .await
        .unwrap_or(0);
    
    let generating_count = ppt_project::Entity::find()
        .filter(ppt_project::Column::Status.eq("generating"))
        .count(db)
        .await
        .unwrap_or(0);
    
    let completed_count = ppt_project::Entity::find()
        .filter(ppt_project::Column::Status.eq("completed"))
        .count(db)
        .await
        .unwrap_or(0);
    
    ApiResponse::ok(PptProjectStatistics {
        total_projects: total as i64,
        draft_count: draft_count as i64,
        generating_count: generating_count as i64,
        completed_count: completed_count as i64,
    })
}
