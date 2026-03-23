use crate::model::prelude::*;
use chrono::NaiveDateTime;

pub use crate::domain::args::a_role::*;
pub use crate::domain::entity::{
    sys_menu::{self, Model as SysMenuModel},
    sys_role::{self, ActiveModel, Model as SysRoleModel},
    sys_role_menu::{self, Model as SysRoleMenuModel},
    sys_user_role,
    sys_role_dept,
};
use super::m_role_dept::SysRoleDeptModel;

impl SysRoleModel {
    pub async fn list(arg: PageParams, search: RoleSearch) -> Result<ListData<RoleResp>> {
        let page_num = arg.page_num.unwrap_or(1);
        let page_per_size = arg.page_size.unwrap_or(10);
        let db = DB().await;
        let mut rmodel = sys_role::Entity::find();
        rmodel = rmodel.filter(sys_role::Column::DeletedAt.is_null());

        if let Some(role_name) = search.role_name {
            rmodel = rmodel.filter(sys_role::Column::RoleName.contains(role_name));
        }
        if let Some(role_key) = search.role_key {
            rmodel = rmodel.filter(sys_role::Column::RoleKey.contains(role_key));
        }
        if let Some(status) = search.status {
            rmodel = rmodel.filter(sys_role::Column::Status.eq(status));
        }
        if let Some(start_time) = search.start_time {
            if let Ok(dt) = NaiveDateTime::parse_from_str(&start_time, "%Y-%m-%d %H:%M:%S") {
                rmodel = rmodel.filter(sys_role::Column::CreatedAt.gte(dt));
            }
        }
        if let Some(end_time) = search.end_time {
            if let Ok(dt) = NaiveDateTime::parse_from_str(&end_time, "%Y-%m-%d %H:%M:%S") {
                rmodel = rmodel.filter(sys_role::Column::CreatedAt.lte(dt));
            }
        }

        let total = rmodel.clone().count(db).await?;
        let paginator = rmodel
            .order_by_asc(sys_role::Column::Order)
            .into_model::<RoleResp>()
            .paginate(db, page_per_size);
        let total_pages = paginator.num_pages().await?;
        let mut list = paginator.fetch_page(page_num - 1).await?;

        for role in &mut list {
            role.admin = Some(role.role_key == "admin");
            // 查询角色的权限菜单
            role.menus = Self::get_role_menu_infos(role.role_id).await.unwrap_or_default();
        }

        let res = ListData {
            list,
            total,
            total_pages,
            page_num,
        };
        Ok(res)
    }
    
    // 获取角色的菜单信息（用于列表展示）
    async fn get_role_menu_infos(role_id: i64) -> Result<Vec<RoleMenuInfo>> {
        let db = DB().await;
        let menu_ids: Vec<i64> = sys_role_menu::Entity::find()
            .filter(sys_role_menu::Column::RoleId.eq(role_id))
            .select_only()
            .column(sys_role_menu::Column::MenuId)
            .into_tuple::<i64>()
            .all(db)
            .await?;
        
        if menu_ids.is_empty() {
            return Ok(vec![]);
        }
        
        let menus = sys_menu::Entity::find()
            .filter(sys_menu::Column::Id.is_in(menu_ids))
            .select_only()
            .column(sys_menu::Column::Id)
            .column(sys_menu::Column::Title)
            .into_tuple::<(i64, String)>()
            .all(db)
            .await?;
        
        Ok(menus.into_iter().map(|(menu_id, title)| RoleMenuInfo { menu_id, title }).collect())
    }

    pub async fn tree() -> Result<Vec<RoleResp>> {
        let db = DB().await;
        let mut rmodel = sys_role::Entity::find();
        rmodel = rmodel.filter(sys_role::Column::DeletedAt.is_null());
        let mut list = rmodel.into_model::<RoleResp>().all(db).await?;
        for role in &mut list {
            role.admin = Some(role.role_key == "admin");
        }
        Ok(list)
    }
    pub async fn menu() -> Result<Vec<RoleMenuResp>> {
        let db = DB().await;

        let mut rmodel = sys_role::Entity::find();

        rmodel = rmodel.columns([
            sys_role::Column::RoleId,
            sys_role::Column::RoleName,
            sys_role::Column::RoleKey,
            sys_role::Column::Order,
            sys_role::Column::DataScope,
            sys_role::Column::Status,
        ]);

        let mut mode = rmodel.into_model::<RoleMenuResp>().all(db).await.unwrap();

        for m in mode.iter_mut() {
            let ms1: Vec<RoleMResp> = SysRoleMenuModel::listmenus::<RoleMResp>(m.role_id)
                .await
                .unwrap();
            m.mens = ms1;
        }
        Ok(mode)
    }

    //根据角色ID获取角色菜单
    pub async fn get_role_menus(role_id: i64) -> Result<Vec<RoleMResp>> {
        SysRoleMenuModel::listmenus::<RoleMResp>(role_id).await
    }

    pub async fn get_role_depts(role_id: i64) -> Result<Vec<RoleDeptResp>> {
        SysRoleDeptModel::listdepts::<RoleDeptResp>(role_id).await
    }

    pub async fn edit(arg: RoleEditReq) -> Result<String> {
        let db = DB().await;
        let rmodel = sys_role::Entity::find_by_id(arg.role_id).one(db).await?;

        let mut amodel: sys_role::ActiveModel = if let Some(rmodel) = rmodel {
            rmodel.into()
        } else {
            return Err("role not found".into());
        };
        amodel.role_name = Set(arg.role_name);
        amodel.role_key = Set(arg.role_key);
        amodel.order = Set(arg.order);
        if let Some(data_scope) = arg.data_scope {
            amodel.data_scope = Set(data_scope);
        }
        amodel.status = Set(arg.status);
        amodel.remark = Set(arg.remark);
        let umodel = amodel.update(db).await;
        if umodel.is_err() {
            return Err("update failed".into());
        }
        
        // 只有当明确传递了 menu 字段时才更新权限，避免误清空
        if let Some(menu_ids) = arg.menu {
            let _ = SysRoleMenuModel::update_menus(arg.role_id, menu_ids).await;
        }

        if let Some(data_depts) = arg.data_depts {
            let _ = SysRoleDeptModel::update_depts(arg.role_id, data_depts).await;
        }
        Ok("Success".to_string())
    }

    pub async fn assign_menu(role_id: i64, menu_ids: Vec<i64>) -> Result<String> {
        let _ = SysRoleMenuModel::update_menus(role_id, menu_ids).await;
        Ok("Success".to_string())
    }

    pub async fn add(arg: RoleAddReq) -> Result<String> {
        let db = DB().await;
        let id = GID().await;
        let amode = sys_role::ActiveModel {
            role_id: Set(id),
            role_name: Set(arg.role_name),
            role_key: Set(arg.role_key),
            order: Set(arg.order),
            data_scope: Set(arg.data_scope.unwrap_or_default()),
            status: Set(arg.status),
            remark: Set(arg.remark),
            ..Default::default()
        };
        amode.insert(db).await?;
        Ok("Success".to_string())
    }

    pub async fn delete(arg: RoleReq) -> Result<String> {
        let db = DB().await;

        // 检查是否有用户关联到此角色
        let user_count = sys_user_role::Entity::find()
            .filter(sys_user_role::Column::RoleId.eq(arg.role_id))
            .count(db)
            .await?;
        if user_count > 0 {
            return Err("该角色已关联用户，无法删除".into());
        }

        // 检查是否有部门关联到此角色
        let dept_count = sys_role_dept::Entity::find()
            .filter(sys_role_dept::Column::RoleId.eq(arg.role_id))
            .count(db)
            .await?;
        if dept_count > 0 {
            return Err("该角色已关联部门，无法删除".into());
        }

        // 删除角色菜单关联
        let _ = SysRoleMenuModel::delete(arg.role_id).await;

        // 删除角色
        let dmodel = sys_role::Entity::delete_many()
            .filter(sys_role::Column::RoleId.eq(arg.role_id))
            .exec(db)
            .await?;

        if dmodel.rows_affected > 0 {
            Ok("Success".to_string())
        } else {
            Err("delete failed".into())
        }
    }
}
