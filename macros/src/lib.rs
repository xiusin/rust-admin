use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn handler(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input: syn::ItemFn = syn::parse_macro_input!(item as syn::ItemFn);
    let fn_name = &input.sig.ident;
    let fn_sig = &input.sig;
    let fn_body = &input.block;

    let attr_input = attr.to_string();

    let mut method = String::new();
    let mut path = String::new();
    let mut tag = String::new();
    let mut summary = String::new();
    let mut auth_required = true;
    let mut auth_roles: Vec<String> = Vec::new();
    let mut auth_permissions: Vec<String> = Vec::new();
    let mut log_operation: Option<String> = None;
    let mut rate_limit_requests: Option<u32> = None;
    let mut rate_limit_period: Option<String> = None;

    for part in attr_input.split(',') {
        let part = part.trim();
        if let Some(pos) = part.find('=') {
            let key = part[..pos].trim().to_string();
            let value = part[pos + 1..].trim().to_string();

            match key.as_str() {
                "method" => method = value.trim_matches('"').to_string(),
                "path" => path = value.trim_matches('"').to_string(),
                "tag" => tag = value.trim_matches('"').to_string(),
                "summary" => summary = value.trim_matches('"').to_string(),
                "required" => auth_required = value == "true",
                "roles" => auth_roles = parse_array(&value),
                "permissions" => auth_permissions = parse_array(&value),
                "operation" => log_operation = Some(value.trim_matches('"').to_string()),
                "requests" => rate_limit_requests = value.parse().ok(),
                "period" => rate_limit_period = Some(value.trim_matches('"').to_string()),
                _ => {}
            }
        }
    }

    let fn_name_str = fn_name.to_string();
    let auth_required_val = auth_required;
    let auth_roles_val = auth_roles;
    let auth_permissions_val = auth_permissions;
    let log_op_val = log_operation.clone().unwrap_or_default();
    let has_log = log_operation.is_some();
    let has_rate_limit = rate_limit_requests.is_some();
    let rl_requests = rate_limit_requests.unwrap_or(100);
    let rl_period = rate_limit_period.unwrap_or_else(|| "1m".to_string());

    let registration_fn_name = syn::Ident::new(&format!("__register_route_{}", fn_name), fn_name.span());
    let registration_static_name = syn::Ident::new(&format!("__REG_INIT_{}", fn_name), fn_name.span());

    let registration_fn = if has_rate_limit {
        quote! {
            fn #registration_fn_name() {
                use crate::core::{RouteInfo, HttpMethod, AuthConfig, LogConfig, RateLimitConfig, AnnotatedHandler};

                let route_info = RouteInfo::new(
                    #path.to_string(),
                    HttpMethod::from_str(#method).unwrap(),
                    #fn_name_str,
                )
                .with_tag(#tag.to_string())
                .with_summary(#summary.to_string());

                let mut handler = AnnotatedHandler::new(route_info);

                handler = handler.with_auth(AuthConfig {
                    required: #auth_required_val,
                    roles: vec![#(#auth_roles_val.to_string()),*],
                    permissions: vec![#(#auth_permissions_val.to_string()),*],
                });

                handler = handler.with_log(LogConfig {
                    operation: #log_op_val.to_string(),
                });

                handler = handler.with_rate_limit(RateLimitConfig {
                    requests: #rl_requests,
                    period: #rl_period.to_string(),
                });

                crate::core::ROUTE_REGISTRY.register(stringify!(#fn_name), handler);
            }
        }
    } else if has_log {
        quote! {
            fn #registration_fn_name() {
                use crate::core::{RouteInfo, HttpMethod, AuthConfig, LogConfig, AnnotatedHandler};

                let route_info = RouteInfo::new(
                    #path.to_string(),
                    HttpMethod::from_str(#method).unwrap(),
                    #fn_name_str,
                )
                .with_tag(#tag.to_string())
                .with_summary(#summary.to_string());

                let mut handler = AnnotatedHandler::new(route_info);

                handler = handler.with_auth(AuthConfig {
                    required: #auth_required_val,
                    roles: vec![#(#auth_roles_val.to_string()),*],
                    permissions: vec![#(#auth_permissions_val.to_string()),*],
                });

                handler = handler.with_log(LogConfig {
                    operation: #log_op_val.to_string(),
                });

                crate::core::ROUTE_REGISTRY.register(stringify!(#fn_name), handler);
            }
        }
    } else {
        quote! {
            fn #registration_fn_name() {
                use crate::core::{RouteInfo, HttpMethod, AuthConfig, AnnotatedHandler};

                let route_info = RouteInfo::new(
                    #path.to_string(),
                    HttpMethod::from_str(#method).unwrap(),
                    #fn_name_str,
                )
                .with_tag(#tag.to_string())
                .with_summary(#summary.to_string());

                let mut handler = AnnotatedHandler::new(route_info);

                handler = handler.with_auth(AuthConfig {
                    required: #auth_required_val,
                    roles: vec![#(#auth_roles_val.to_string()),*],
                    permissions: vec![#(#auth_permissions_val.to_string()),*],
                });

                crate::core::ROUTE_REGISTRY.register(stringify!(#fn_name), handler);
            }
        }
    };

    let expanded = quote! {
        #registration_fn

        #fn_sig #fn_body

        static #registration_static_name: std::sync::LazyLock<()> = std::sync::LazyLock::new(#registration_fn_name);
    };

    TokenStream::from(expanded)
}

fn parse_array(value: &str) -> Vec<String> {
    let value = value.trim();
    if value.starts_with('[') && value.ends_with(']') {
        let inner = &value[1..value.len()-1];
        inner.split(',')
            .map(|s| s.trim().trim_matches('"').trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    } else {
        vec![value.trim_matches('"').to_string()]
    }
}

#[proc_macro_attribute]
pub fn route(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn auth(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn log(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn rate_limit(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}