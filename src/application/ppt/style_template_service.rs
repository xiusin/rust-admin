use crate::service::prelude::*;
use crate::domain::entity::style_template;
use crate::domain::model::m_ppt::*;
use crate::domain::args::a_ppt::*;

pub async fn list(
    VQuery(page): VQuery<PageParams>,
    VQuery(search): VQuery<StyleTemplateListArgs>,
) -> impl IntoResponse {
    let db = DB().await;
    
    let page_num = page.page_num.unwrap_or(1);
    let page_size = page.page_size.unwrap_or(10);
    
    let mut conditions = Condition::all();
    
    if let Some(ref name) = search.name {
        conditions = conditions.add(style_template::Column::Name.contains(name));
    }
    if let Some(ref industry) = search.industry {
        conditions = conditions.add(style_template::Column::Industry.contains(industry));
    }
    
    let total = style_template::Entity::find()
        .filter(conditions.clone())
        .count(db)
        .await
        .unwrap_or(0);
    
    let items = style_template::Entity::find()
        .filter(conditions)
        .order_by_desc(style_template::Column::UsageCount)
        .offset((page_num - 1) * page_size)
        .limit(page_size)
        .all(db)
        .await;
    
    match items {
        Ok(items) => {
            let list: Vec<StyleTemplateListItem> = items.into_iter().map(|t| StyleTemplateListItem {
                id: t.id,
                name: t.name,
                industry: t.industry,
                color_scheme: t.color_scheme,
                font_scheme: t.font_scheme,
                preview_url: t.preview_url,
                usage_count: t.usage_count,
                created_at: t.created_at,
            }).collect();
            
            ApiResponse::ok(ListData {
                list,
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
    
    let template = match style_template::Entity::find_by_id(id)
        .one(db)
        .await
    {
        Ok(Some(t)) => t,
        Ok(None) => return ApiResponse::not_found("样式模板不存在"),
        Err(e) => return ApiResponse::bad_request(e.to_string()),
    };
    
    ApiResponse::ok(StyleTemplateDetail {
        id: template.id,
        name: template.name,
        industry: template.industry,
        color_scheme: template.color_scheme,
        font_scheme: template.font_scheme,
        layout_rules: template.layout_rules,
        component_styles: template.component_styles,
        preview_url: template.preview_url,
        usage_count: template.usage_count,
        created_at: template.created_at,
    })
}

pub async fn add(VJson(arg): VJson<StyleTemplateAddArgs>) -> impl IntoResponse {
    let db = DB().await;
    
    let id = GID().await;
    let now = Local::now().naive_local();
    
    let template = style_template::ActiveModel {
        id: Set(id),
        name: Set(arg.name),
        industry: Set(arg.industry),
        color_scheme: Set(arg.color_scheme),
        font_scheme: Set(arg.font_scheme),
        layout_rules: Set(arg.layout_rules),
        component_styles: Set(arg.component_styles),
        preview_url: Set(arg.preview_url),
        usage_count: Set(0),
        created_at: Set(Some(now)),
    };
    
    match template.insert(db).await {
        Ok(_) => ApiResponse::ok(id),
        Err(e) => ApiResponse::bad_request(e.to_string()),
    }
}

pub async fn edit(VJson(arg): VJson<StyleTemplateEditArgs>) -> impl IntoResponse {
    let db = DB().await;
    
    let template = match style_template::Entity::find_by_id(arg.id)
        .one(db)
        .await
    {
        Ok(Some(t)) => t,
        Ok(None) => return ApiResponse::not_found("样式模板不存在"),
        Err(e) => return ApiResponse::bad_request(e.to_string()),
    };
    
    let mut template: style_template::ActiveModel = template.into();
    if let Some(name) = arg.name {
        template.name = Set(name);
    }
    template.industry = Set(arg.industry);
    template.color_scheme = Set(arg.color_scheme);
    template.font_scheme = Set(arg.font_scheme);
    template.layout_rules = Set(arg.layout_rules);
    template.component_styles = Set(arg.component_styles);
    template.preview_url = Set(arg.preview_url);
    
    match template.update(db).await {
        Ok(_) => ApiResponse::ok("更新成功"),
        Err(e) => ApiResponse::bad_request(e.to_string()),
    }
}

pub async fn delete(VJson(arg): VJson<StyleTemplateDeleteArgs>) -> impl IntoResponse {
    let db = DB().await;
    
    for id in arg.ids {
        let _ = style_template::Entity::delete_by_id(id)
            .exec(db)
            .await;
    }
    
    ApiResponse::ok("删除成功")
}

pub async fn by_industry(Path(industry): Path<String>) -> impl IntoResponse {
    let db = DB().await;
    
    let templates = style_template::Entity::find()
        .filter(style_template::Column::Industry.contains(&industry))
        .order_by_desc(style_template::Column::UsageCount)
        .all(db)
        .await;
    
    match templates {
        Ok(items) => {
            let list: Vec<StyleTemplateListItem> = items.into_iter().map(|t| StyleTemplateListItem {
                id: t.id,
                name: t.name,
                industry: t.industry,
                color_scheme: t.color_scheme,
                font_scheme: t.font_scheme,
                preview_url: t.preview_url,
                usage_count: t.usage_count,
                created_at: t.created_at,
            }).collect();
            
            ApiResponse::ok(list)
        }
        Err(e) => ApiResponse::bad_request(e.to_string()),
    }
}
