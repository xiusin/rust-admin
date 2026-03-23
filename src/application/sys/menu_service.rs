use crate::domain::model::m_menu::{
    ListMeta, MenuAdd, MenuDel, MenuEdit, MenuResp, MenuSearch, RoleKeyReq, RouteMeta, SysMenuListTree, SysMenuModel,
    SysMenuRouterTree as SysMenuRouteTree, UserMenu,
};
use crate::domain::entity::{sys_menu, sys_role};
use crate::service::prelude::*;

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RouterMeta {
    #[serde(rename = "type")]
    pub menu_type: u8,
    pub title: String,
    pub icon: Option<String>,
    pub svg_icon: Option<String>,
    pub hide: bool,
    pub disable: bool,
    #[serde(rename = "keepAlive")]
    pub keep_alive: bool,
    pub affix: bool,
    pub link: Option<String>,
    pub iframe: bool,
    pub is_full: bool,
    pub roles: Option<Vec<String>>,
    pub sort: i32,
    #[serde(rename = "noCache")]
    pub no_cache: bool,
    pub href: Option<String>,
    pub active_menu: Option<String>,
    pub breadcrumb: bool,
    pub always_show: bool,
    pub can_to: bool,
    pub no_tags_view: bool,
    pub i18nkey: Option<String>,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RouterItem {
    pub id: i64,
    pub pid: i64,
    pub path: String,
    pub name: Option<String>,
    pub component: Option<String>,
    pub redirect: Option<String>,
    pub order: i32,
    pub meta: RouterMeta,
    pub children: Option<Vec<RouterItem>>,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MenuListItem {
    pub id: i64,
    pub pid: i64,
    pub path: Option<String>,
    pub name: Option<String>,
    pub component: Option<String>,
    pub redirect: Option<String>,
    pub order: i32,
    pub menu_type: String,
    pub status: String,
    pub meta: RouterMeta,
    pub children: Option<Vec<MenuListItem>>,
}

fn menu_resp_to_router_meta(menu: &MenuResp) -> RouterMeta {
    let menu_type_val = match menu.menu_type.as_str() {
        "M" => 1,
        "C" => 2,
        "F" => 3,
        _ => 2,
    };
    RouterMeta {
        menu_type: menu_type_val,
        title: menu.title.clone(),
        icon: menu.icon.clone(),
        svg_icon: None,
        hide: menu.hidden == "1",
        disable: menu.status != "0",
        keep_alive: menu.no_cache == "1",
        affix: menu.affix == "1",
        link: menu.href.clone(),
        iframe: menu.href.is_some() && !menu.href.clone().unwrap_or_default().is_empty(),
        is_full: false,
        roles: None,
        sort: menu.order,
        no_cache: menu.no_cache == "1",
        href: menu.href.clone(),
        active_menu: if menu.active_menu.is_empty() { None } else { Some(menu.active_menu.clone()) },
        breadcrumb: menu.breadcrumb == "1",
        always_show: menu.always_show == "1",
        can_to: menu.can_to == "1",
        no_tags_view: menu.no_tags_view == "1",
        i18nkey: menu.i18nkey.clone(),
    }
}

fn build_router_tree(menus: &[MenuResp], pid: i64) -> Vec<RouterItem> {
    menus
        .iter()
        .filter(|m| m.pid == pid)
        .map(|m| RouterItem {
            id: m.id,
            pid: m.pid,
            path: m.path.clone().unwrap_or_default(),
            name: m.name.clone(),
            component: m.component.clone(),
            redirect: m.redirect.clone(),
            order: m.order,
            meta: menu_resp_to_router_meta(m),
            children: Some(build_router_tree(menus, m.id)),
        })
        .collect()
}

fn build_menu_list_tree(menus: &[MenuResp], pid: i64) -> Vec<MenuListItem> {
    menus
        .iter()
        .filter(|m| m.pid == pid)
        .map(|m| {
            let children = build_menu_list_tree(menus, m.id);
            MenuListItem {
                id: m.id,
                pid: m.pid,
                path: m.path.clone(),
                name: m.name.clone(),
                component: m.component.clone(),
                redirect: m.redirect.clone(),
                order: m.order,
                menu_type: m.menu_type.clone(),
                status: m.status.clone(),
                meta: menu_resp_to_router_meta(m),
                children: if children.is_empty() { None } else { Some(children) },
            }
        })
        .collect()
}

pub async fn list(uinfo: UserInfo) -> impl IntoResponse {
    let menus: Vec<MenuResp> = SysMenuModel::get_menus(false, false, true, uinfo.rid)
        .await
        .unwrap();
    let menu_data = self::get_menu_list_data(menus);
    let menu_tree = self::get_menu_list_tree(menu_data, 0);
    ApiResponse::ok(menu_tree)
}

pub async fn edit(VJson(arg): VJson<MenuEdit>) -> impl IntoResponse {
    let umodel = SysMenuModel::edit(arg).await;
    ApiResponse::from_result(umodel)
}
pub async fn add(VJson(arg): VJson<MenuAdd>) -> impl IntoResponse {
    let amodel = SysMenuModel::add(arg).await;
    ApiResponse::from_result(amodel)
}
pub async fn delete(VQuery(arg): VQuery<MenuDel>) -> impl IntoResponse {
    let dmodel = SysMenuModel::del(arg).await;
    ApiResponse::from_result(dmodel)
}
pub async fn tree(uinfo: UserInfo) -> impl IntoResponse {
    let menus = SysMenuModel::get_menus(true, false, true, uinfo.rid)
        .await
        .unwrap();
    let menu_data = self::get_menu_list_data(menus);
    let menu_tree = self::get_menu_list_tree(menu_data, 0);
    ApiResponse::ok(menu_tree)
}

pub async fn all_router(uinfo: UserInfo) -> impl IntoResponse {
    let menus: Vec<MenuResp> = SysMenuModel::get_menus(true, false, true, uinfo.rid)
        .await
        .unwrap();
    info!("{:?}", menus.len());
    let menu_data = self::get_menu_route_data(menus);

    let menu_tree = self::get_menu_route_tree(menu_data, 0);

    ApiResponse::ok(menu_tree)
}

fn get_menu_list_tree(user_menus: Vec<SysMenuListTree>, pid: i64) -> Vec<SysMenuListTree> {
    let mut menu_tree: Vec<SysMenuListTree> = Vec::new();
    for mut user_menu in user_menus.clone() {
        if user_menu.pid == pid {
            let children = get_menu_list_tree(user_menus.clone(), user_menu.id);
            user_menu.children = if children.is_empty() { None } else { Some(children) };
            menu_tree.push(user_menu.clone());
        }
    }
    menu_tree
}
fn get_menu_list_data(menus: Vec<MenuResp>) -> Vec<SysMenuListTree> {
    let mut menu_res: Vec<SysMenuListTree> = Vec::new();
    for menu in menus {
        let meta = ListMeta {
            icon: menu.icon.clone(),
            title: menu.title.clone(),
            no_cache: menu.no_cache,
            hidden: menu.hidden,
            active_menu: menu.active_menu,
            breadcrumb: menu.breadcrumb,
            always_show: menu.always_show,
            affix: menu.affix,
            can_to: menu.can_to,
            no_tags_view: menu.no_tags_view,
            i18nkey: menu.i18nkey,
        };
        let menu_tree = SysMenuListTree {
            meta,
            id: menu.id,
            pid: menu.pid,
            path: menu.path,
            order: menu.order,
            redirect: menu.redirect,
            menu_type: menu.menu_type,
            status: menu.status,
            component: menu.component,
            name: menu.name,
            ..Default::default()
        };
        menu_res.push(menu_tree);
    }
    menu_res
}

fn get_menu_route_data(menus: Vec<MenuResp>) -> Vec<SysMenuRouteTree> {
    let mut menu_res: Vec<SysMenuRouteTree> = Vec::new();
    for menu in menus {
        let meta = RouteMeta {
            icon: menu.icon.clone(),
            title: menu.title.clone(),
            no_cache: menu.no_cache,
            href: menu.href,
            hidden: menu.hidden,
            active_menu: menu.active_menu,
            breadcrumb: menu.breadcrumb,
            always_show: menu.always_show,
            affix: menu.affix,
            can_to: menu.can_to,
            no_tags_view: menu.no_tags_view,
            i18nkey: menu.i18nkey,
        };
        let user_menu = UserMenu {
            meta,
            id: menu.id,
            pid: menu.pid,
            path: menu.path.clone(),
            title: menu.title.clone(),
            order: menu.order,
            redirect: menu.redirect,
            menu_type: menu.menu_type,
            status: menu.status,
            component: menu.component.clone(),
            name: menu.name,
        };
        let menu_tree = SysMenuRouteTree {
            user_menu,
            ..Default::default()
        };
        menu_res.push(menu_tree);
    }
    menu_res
}

fn get_menu_route_tree(user_menus: Vec<SysMenuRouteTree>, pid: i64) -> Vec<SysMenuRouteTree> {
    let mut menu_tree: Vec<SysMenuRouteTree> = Vec::new();
    for mut user_menu in user_menus.clone() {
        if user_menu.user_menu.pid == pid {
            let children = get_menu_route_tree(
                user_menus.clone(),
                user_menu.user_menu.id,
            );
            user_menu.children = if children.is_empty() { None } else { Some(children) };
            menu_tree.push(user_menu.clone());
        }
    }
    menu_tree
}

pub async fn get_routers() -> impl IntoResponse {
    let db = DB().await;
    let menus = crate::domain::entity::sys_menu::Entity::find()
        .filter(crate::domain::entity::sys_menu::Column::DeletedAt.is_null())
        .into_model::<MenuResp>()
        .all(db)
        .await
        .unwrap_or_default();

    let tree = build_router_tree(&menus, 0);
    ApiResponse::ok(tree)
}

pub async fn get_menu_list(VQuery(search): VQuery<MenuSearch>) -> impl IntoResponse {
    let db = DB().await;
    let mut query = sys_menu::Entity::find();
    
    // 应用筛选条件
    if let Some(title) = search.title {
        query = query.filter(sys_menu::Column::Title.contains(title));
    }
    if let Some(hidden) = search.hidden {
        query = query.filter(sys_menu::Column::Hidden.eq(hidden));
    }
    if let Some(status) = search.status {
        query = query.filter(sys_menu::Column::Status.eq(status));
    }
    
    let menus = query
        .into_model::<MenuResp>()
        .all(db)
        .await
        .unwrap_or_default();
    
    let tree = build_menu_list_tree(&menus, 0);
    ApiResponse::ok(tree)
}

pub async fn get_user_permission(VQuery(arg): VQuery<RoleKeyReq>) -> impl IntoResponse {
    let db = DB().await;
    
    // 根据 role_key 查找 role_id
    let role = sys_role::Entity::find()
        .filter(sys_role::Column::RoleKey.eq(arg.role))
        .filter(sys_role::Column::DeletedAt.is_null())
        .one(db)
        .await;
    
    let role_id = match role {
        Ok(Some(r)) => r.role_id,
        _ => return ApiResponse::ok(vec![] as Vec<i64>),
    };
    
    let menus = SysMenuModel::get_menus(false, false, true, role_id)
        .await
        .unwrap_or_default();
    let ids: Vec<i64> = menus.into_iter().map(|m| m.id).collect();
    ApiResponse::ok(ids)
}
