use crate::service::prelude::*;
use crate::domain::entity::{ppt_page, page_element};
use crate::domain::model::m_ppt::*;
use crate::domain::args::a_ppt::*;

pub async fn list(
    VQuery(page): VQuery<PageParams>,
    VQuery(search): VQuery<PptPageListArgs>,
) -> impl IntoResponse {
    let db = DB().await;
    
    let page_num = page.page_num.unwrap_or(1);
    let page_size = page.page_size.unwrap_or(10);
    
    let mut conditions = Condition::all();
    
    if let Some(project_id) = search.project_id {
        conditions = conditions.add(ppt_page::Column::ProjectId.eq(project_id));
    }
    if let Some(ref page_type) = search.page_type {
        conditions = conditions.add(ppt_page::Column::PageType.eq(page_type));
    }
    
    let total = ppt_page::Entity::find()
        .filter(conditions.clone())
        .count(db)
        .await
        .unwrap_or(0);
    
    let items = ppt_page::Entity::find()
        .filter(conditions)
        .order_by_asc(ppt_page::Column::PageOrder)
        .offset((page_num - 1) * page_size)
        .limit(page_size)
        .all(db)
        .await;
    
    match items {
        Ok(items) => {
            let mut list_items = Vec::new();
            for item in items {
                let element_count = page_element::Entity::find()
                    .filter(page_element::Column::PageId.eq(item.id))
                    .count(db)
                    .await
                    .unwrap_or(0) as i32;
                
                list_items.push(PptPageListItem {
                    id: item.id,
                    project_id: item.project_id,
                    page_order: item.page_order,
                    page_type: item.page_type,
                    title: item.title,
                    element_count,
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
    
    let page = match ppt_page::Entity::find_by_id(id)
        .one(db)
        .await
    {
        Ok(Some(p)) => p,
        Ok(None) => return ApiResponse::not_found("PPT页面不存在"),
        Err(e) => return ApiResponse::bad_request(e.to_string()),
    };
    
    let elements = match page_element::Entity::find()
        .filter(page_element::Column::PageId.eq(id))
        .order_by_asc(page_element::Column::ZIndex)
        .all(db)
        .await
    {
        Ok(e) => e,
        Err(e) => return ApiResponse::bad_request(e.to_string()),
    };
    
    ApiResponse::ok(PptPageWithElements {
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
    })
}

pub async fn add(VJson(arg): VJson<PptPageAddArgs>) -> impl IntoResponse {
    let db = DB().await;
    
    let id = GID().await;
    let now = Local::now().naive_local();
    
    let page = ppt_page::ActiveModel {
        id: Set(id),
        project_id: Set(arg.project_id),
        page_order: Set(arg.page_order),
        page_type: Set(arg.page_type),
        title: Set(arg.title),
        layout_config: Set(arg.layout_config),
        style_config: Set(arg.style_config),
        created_at: Set(Some(now)),
        updated_at: Set(Some(now)),
    };
    
    match page.insert(db).await {
        Ok(_) => ApiResponse::ok(id),
        Err(e) => ApiResponse::bad_request(e.to_string()),
    }
}

pub async fn edit(VJson(arg): VJson<PptPageEditArgs>) -> impl IntoResponse {
    let db = DB().await;
    
    let page = match ppt_page::Entity::find_by_id(arg.id)
        .one(db)
        .await
    {
        Ok(Some(p)) => p,
        Ok(None) => return ApiResponse::not_found("PPT页面不存在"),
        Err(e) => return ApiResponse::bad_request(e.to_string()),
    };
    
    let now = Local::now().naive_local();
    
    let mut page: ppt_page::ActiveModel = page.into();
    if let Some(page_order) = arg.page_order {
        page.page_order = Set(page_order);
    }
    if let Some(page_type) = arg.page_type {
        page.page_type = Set(page_type);
    }
    page.title = Set(arg.title);
    page.layout_config = Set(arg.layout_config);
    page.style_config = Set(arg.style_config);
    page.updated_at = Set(Some(now));
    
    match page.update(db).await {
        Ok(_) => ApiResponse::ok("更新成功"),
        Err(e) => ApiResponse::bad_request(e.to_string()),
    }
}

pub async fn delete(VJson(arg): VJson<PptPageDeleteArgs>) -> impl IntoResponse {
    let db = DB().await;
    
    for id in arg.ids {
        let _ = ppt_page::Entity::delete_by_id(id)
            .exec(db)
            .await;
        let _ = page_element::Entity::delete_many()
            .filter(page_element::Column::PageId.eq(id))
            .exec(db)
            .await;
    }
    
    ApiResponse::ok("删除成功")
}

pub async fn reorder(VJson(arg): VJson<serde_json::Value>) -> impl IntoResponse {
    let db = DB().await;
    
    let orders: Vec<(i64, i32)> = serde_json::from_value(arg).unwrap_or_default();
    
    for (id, new_order) in orders {
        if let Some(page) = ppt_page::Entity::find_by_id(id)
            .one(db)
            .await
            .ok()
            .flatten()
        {
            let mut page: ppt_page::ActiveModel = page.into();
            page.page_order = Set(new_order);
            let _ = page.update(db).await;
        }
    }
    
    ApiResponse::ok("排序更新成功")
}
