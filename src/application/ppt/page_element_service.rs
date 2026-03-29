use crate::service::prelude::*;
use crate::domain::entity::page_element;
use crate::domain::model::m_ppt::*;
use crate::domain::args::a_ppt::*;

pub async fn list(VQuery(search): VQuery<PageElementListArgs>) -> impl IntoResponse {
    let db = DB().await;
    
    let mut conditions = Condition::all();
    
    if let Some(page_id) = search.page_id {
        conditions = conditions.add(page_element::Column::PageId.eq(page_id));
    }
    
    let elements = page_element::Entity::find()
        .filter(conditions)
        .order_by_asc(page_element::Column::ZIndex)
        .all(db)
        .await;
    
    match elements {
        Ok(items) => {
            let list: Vec<PageElementModel> = items.into_iter().map(|e| PageElementModel {
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
            }).collect();
            
            ApiResponse::ok(list)
        }
        Err(e) => ApiResponse::bad_request(e.to_string()),
    }
}

pub async fn detail(Path(id): Path<i64>) -> impl IntoResponse {
    let db = DB().await;
    
    let element = match page_element::Entity::find_by_id(id)
        .one(db)
        .await
    {
        Ok(Some(e)) => e,
        Ok(None) => return ApiResponse::not_found("页面元素不存在"),
        Err(e) => return ApiResponse::bad_request(e.to_string()),
    };
    
    ApiResponse::ok(PageElementModel {
        id: element.id,
        page_id: element.page_id,
        element_type: element.element_type,
        content: element.content,
        style: element.style,
        position: element.position,
        size: element.size,
        rotation: element.rotation,
        z_index: element.z_index,
        locked: element.locked,
    })
}

pub async fn add(VJson(arg): VJson<PageElementAddArgs>) -> impl IntoResponse {
    let db = DB().await;
    
    let id = GID().await;
    let now = Local::now().naive_local();
    
    let element = page_element::ActiveModel {
        id: Set(id),
        page_id: Set(arg.page_id),
        element_type: Set(arg.element_type),
        content: Set(arg.content),
        style: Set(arg.style),
        position: Set(arg.position),
        size: Set(arg.size),
        rotation: Set(arg.rotation),
        z_index: Set(arg.z_index.unwrap_or(0)),
        locked: Set(arg.locked.unwrap_or(0)),
        created_at: Set(Some(now)),
        updated_at: Set(Some(now)),
    };
    
    match element.insert(db).await {
        Ok(_) => ApiResponse::ok(id),
        Err(e) => ApiResponse::bad_request(e.to_string()),
    }
}

pub async fn edit(VJson(arg): VJson<PageElementEditArgs>) -> impl IntoResponse {
    let db = DB().await;
    
    let element = match page_element::Entity::find_by_id(arg.id)
        .one(db)
        .await
    {
        Ok(Some(e)) => e,
        Ok(None) => return ApiResponse::not_found("页面元素不存在"),
        Err(e) => return ApiResponse::bad_request(e.to_string()),
    };
    
    let now = Local::now().naive_local();
    
    let mut element: page_element::ActiveModel = element.into();
    if let Some(element_type) = arg.element_type {
        element.element_type = Set(element_type);
    }
    element.content = Set(arg.content);
    element.style = Set(arg.style);
    element.position = Set(arg.position);
    element.size = Set(arg.size);
    element.rotation = Set(arg.rotation);
    if let Some(z_index) = arg.z_index {
        element.z_index = Set(z_index);
    }
    if let Some(locked) = arg.locked {
        element.locked = Set(locked);
    }
    element.updated_at = Set(Some(now));
    
    match element.update(db).await {
        Ok(_) => ApiResponse::ok("更新成功"),
        Err(e) => ApiResponse::bad_request(e.to_string()),
    }
}

pub async fn delete(VJson(arg): VJson<PageElementDeleteArgs>) -> impl IntoResponse {
    let db = DB().await;
    
    for id in arg.ids {
        let _ = page_element::Entity::delete_by_id(id)
            .exec(db)
            .await;
    }
    
    ApiResponse::ok("删除成功")
}

pub async fn batch_add(VJson(args): VJson<Vec<PageElementAddArgs>>) -> impl IntoResponse {
    let db = DB().await;
    
    let now = Local::now().naive_local();
    let mut ids = Vec::new();
    
    for arg in args {
        let id = GID().await;
        
        let element = page_element::ActiveModel {
            id: Set(id),
            page_id: Set(arg.page_id),
            element_type: Set(arg.element_type),
            content: Set(arg.content),
            style: Set(arg.style),
            position: Set(arg.position),
            size: Set(arg.size),
            rotation: Set(arg.rotation),
            z_index: Set(arg.z_index.unwrap_or(0)),
            locked: Set(arg.locked.unwrap_or(0)),
            created_at: Set(Some(now)),
            updated_at: Set(Some(now)),
        };
        
        match element.insert(db).await {
            Ok(_) => ids.push(id),
            Err(_) => continue,
        }
    }
    
    ApiResponse::ok(ids)
}
