use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item2: proc_macro2::TokenStream = item.clone().into();

    let input: syn::ItemFn = syn::parse_macro_input!(item as syn::ItemFn);
    let fn_name = &input.sig.ident;

    let attr_input = attr.to_string();
    let mut method = String::new();
    let mut path = String::new();
    let mut tag = String::new();
    let mut summary = String::new();

    for part in attr_input.split(',') {
        let part = part.trim();
        if let Some(pos) = part.find('=') {
            let key = part[..pos].trim().to_string();
            let value = part[pos + 1..].trim().trim_matches('"').to_string();
            match key.as_str() {
                "method" => method = value,
                "path" => path = value,
                "tag" => tag = value,
                "summary" => summary = value,
                _ => {}
            }
        }
    }

    let helper_name = syn::Ident::new(&format!("__route_meta_{}", fn_name), fn_name.span());
    let fn_name_str = fn_name.to_string();

    let expanded = quote! {
        #item2

        pub fn #helper_name() -> crate::core::route::RouteInfo {
            crate::core::route::RouteInfo::new(
                #path,
                crate::core::route::HttpMethod::from_str(#method)
                    .expect(concat!("Invalid HTTP method: ", #method)),
                #fn_name_str,
            )
            .with_tag(#tag)
            .with_summary(#summary)
        }
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
pub fn auth(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item2: proc_macro2::TokenStream = item.clone().into();

    let input: syn::ItemFn = syn::parse_macro_input!(item as syn::ItemFn);
    let fn_name = &input.sig.ident;

    let attr_input = attr.to_string();
    let mut required = true;
    let mut roles: Vec<String> = Vec::new();
    let mut permissions: Vec<String> = Vec::new();

    for part in attr_input.split(',') {
        let part = part.trim();
        if let Some(pos) = part.find('=') {
            let key = part[..pos].trim().to_string();
            let value = part[pos + 1..].trim().to_string();
            match key.as_str() {
                "required" => required = value == "true",
                "roles" => roles = parse_array(&value),
                "permissions" => permissions = parse_array(&value),
                _ => {}
            }
        }
    }

    let helper_name = syn::Ident::new(&format!("__auth_meta_{}", fn_name), fn_name.span());

    let expanded = quote! {
        #item2

        pub fn #helper_name() -> crate::core::route::AuthConfig {
            crate::core::route::AuthConfig {
                required: #required,
                roles: vec![#(#roles.to_string()),*],
                permissions: vec![#(#permissions.to_string()),*],
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn log(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item2: proc_macro2::TokenStream = item.clone().into();

    let input: syn::ItemFn = syn::parse_macro_input!(item as syn::ItemFn);
    let fn_name = &input.sig.ident;

    let attr_input = attr.to_string();
    let mut operation = String::new();

    for part in attr_input.split(',') {
        let part = part.trim();
        if let Some(pos) = part.find('=') {
            let key = part[..pos].trim().to_string();
            let value = part[pos + 1..].trim().trim_matches('"').to_string();
            if key == "operation" {
                operation = value;
            }
        }
    }

    let helper_name = syn::Ident::new(&format!("__log_meta_{}", fn_name), fn_name.span());

    let expanded = quote! {
        #item2

        pub fn #helper_name() -> crate::core::route::LogConfig {
            crate::core::route::LogConfig {
                operation: #operation.to_string(),
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn rate_limit(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item2: proc_macro2::TokenStream = item.clone().into();

    let input: syn::ItemFn = syn::parse_macro_input!(item as syn::ItemFn);
    let fn_name = &input.sig.ident;

    let attr_input = attr.to_string();
    let mut requests: Option<u32> = None;
    let mut period: Option<String> = None;

    for part in attr_input.split(',') {
        let part = part.trim();
        if let Some(pos) = part.find('=') {
            let key = part[..pos].trim().to_string();
            let value = part[pos + 1..].trim().trim_matches('"').to_string();
            match key.as_str() {
                "requests" => requests = Some(value.parse().expect("Invalid requests value: must be a number")),
                "period" => period = Some(value),
                _ => {}
            }
        }
    }

    let requests_val = requests.expect("requests field is required");
    let period_val = period.unwrap_or_else(|| "1m".to_string());

    let helper_name = syn::Ident::new(&format!("__rate_limit_meta_{}", fn_name), fn_name.span());

    let expanded = quote! {
        #item2

        pub fn #helper_name() -> crate::core::route::RateLimitConfig {
            crate::core::route::RateLimitConfig {
                requests: #requests_val,
                period: #period_val.to_string(),
            }
        }
    };

    TokenStream::from(expanded)
}