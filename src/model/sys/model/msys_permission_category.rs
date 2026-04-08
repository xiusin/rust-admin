pub use super::args::asys_permission_category::*;
pub use super::entity::sys_permission_category::{self, ActiveModel, Model as SysPermissionCategoryModel};
use crate::model::prelude::*;

impl SysPermissionCategoryModel {
    pub async fn list(
        arg: PageParams,
        search: PermissionCategorySearch,
    ) -> Result<ListData<PermissionCategoryRes>> {
        let page_num = arg.page_num.unwrap_or(1);
        let page_per_size = arg.page_size.unwrap_or(10);
        let db = DB().await;
        let mut rmodel = sys_permission_category::Entity::find();

        if let Some(name) = search.name {
            rmodel = rmodel.filter(sys_permission_category::Column::Name.contains(name));
        }

        if let Some(code) = search.code {
            rmodel = rmodel.filter(sys_permission_category::Column::Code.contains(code));
        }
        
        let total = rmodel.clone().count(db).await?;
        let paginator = rmodel
            .order_by_asc(sys_permission_category::Column::Sort)
            .into_model::<PermissionCategoryRes>()
            .paginate(db, page_per_size);
        let total_pages = paginator.num_pages().await?;
        let list = paginator.fetch_page(page_num - 1).await?;
        let res = ListData {
            list,
            total,
            total_pages,
            page_num,
        };
        Ok(res)
    }

    pub async fn add(arg: PermissionCategoryAdd) -> Result<PermissionCategoryRes> {
        let db = DB().await;
        let id = GID().await;
        let now = Local::now().naive_local();
        let amode = sys_permission_category::ActiveModel {
            id: Set(id),
            name: Set(arg.name),
            code: Set(arg.code),
            sort: Set(arg.sort),
            remark: Set(arg.remark),
            created_at: Set(now),
        };
        let model = sys_permission_category::Entity::insert(amode)
            .exec_with_returning(db)
            .await?;
        Ok(PermissionCategoryRes {
            id: model.id,
            name: model.name,
            code: model.code,
            sort: model.sort,
            remark: model.remark,
            created_at: model.created_at,
        })
    }

    pub async fn edit(arg: PermissionCategoryEdit) -> Result<PermissionCategoryRes> {
        let db = DB().await;
        let rmodel = sys_permission_category::Entity::find_by_id(arg.id)
            .one(db)
            .await?;

        let mut amodel: sys_permission_category::ActiveModel = if let Some(r) = rmodel {
            r.into()
        } else {
            return Err("permission category not found".into());
        };
        
        amodel.name = Set(arg.name);
        amodel.code = Set(arg.code);
        amodel.sort = Set(arg.sort);
        amodel.remark = Set(arg.remark);
        
        let model = amodel.update(db).await?;
        Ok(PermissionCategoryRes {
            id: model.id,
            name: model.name,
            code: model.code,
            sort: model.sort,
            remark: model.remark,
            created_at: model.created_at,
        })
    }

    pub async fn delete(id: i64) -> Result<String> {
        let db = DB().await;
        sys_permission_category::Entity::delete_by_id(id).exec(db).await?;
        Ok("Success".to_string())
    }

    pub async fn find_by_id(id: i64) -> Result<Option<SysPermissionCategoryModel>> {
        let db = DB().await;
        sys_permission_category::Entity::find_by_id(id)
            .one(db)
            .await
            .map_err(Into::into)
    }
}
