use crate::domain::entity::{cms_field, cms_form_config, cms_model};
use crate::service::prelude::*;
use sea_orm::QueryOrder;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FormSchema {
    pub form_type: String,
    pub layout: FormLayout,
    pub groups: Vec<FormGroupSchema>,
    pub actions: Vec<FormActionSchema>,
    pub rules: Vec<FormRuleSchema>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FormLayout {
    pub label_width: u32,
    pub label_position: String,
    pub label_colon: bool,
    pub inline: bool,
    pub size: String,
}

impl Default for FormLayout {
    fn default() -> Self {
        Self {
            label_width: 100,
            label_position: "right".to_string(),
            label_colon: true,
            inline: false,
            size: "default".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FormGroupSchema {
    pub id: String,
    pub name: String,
    pub code: String,
    pub icon: Option<String>,
    pub fields: Vec<FieldSchema>,
    pub collapsed: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FieldSchema {
    pub field: String,
    pub label: String,
    pub field_type: String,
    pub component: String,
    pub placeholder: Option<String>,
    pub help_text: Option<String>,
    pub default_value: Option<JsonValue>,
    pub required: bool,
    pub disabled: bool,
    pub visible: bool,
    pub rules: Vec<FieldRule>,
    pub props: JsonValue,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FieldRule {
    pub rule_type: String,
    pub message: Option<String>,
    pub trigger: String,
    pub value: Option<JsonValue>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FormActionSchema {
    pub action_type: String,
    pub label: String,
    pub code: String,
    pub icon: Option<String>,
    pub props: Option<JsonValue>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FormRuleSchema {
    pub rule_type: String,
    pub field: Option<String>,
    pub condition: Option<JsonValue>,
    pub action: Option<JsonValue>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FieldComponent {
    pub code: String,
    pub name: String,
    pub category: String,
    pub icon: Option<String>,
    pub props: JsonValue,
    pub default_value: Option<JsonValue>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ValidationRule {
    pub code: String,
    pub name: String,
    pub rule_type: String,
    pub default_message: String,
    pub params: Option<JsonValue>,
}

pub async fn render_schema(form_id: i64) -> Result<FormSchema> {
    let db = DB().await;
    
    let form_config = cms_form_config::Entity::find_by_id(form_id)
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("表单配置不存在"))?;
    
    let fields = cms_field::Entity::find()
        .filter(cms_field::Column::ModelId.eq(form_config.model_id))
        .filter(cms_field::Column::IsFormShow.eq("1"))
        .order_by_asc(cms_field::Column::Sort)
        .all(db)
        .await?;
    
    let layout = parse_layout(&form_config.layout)?;
    let groups = parse_groups(&form_config.groups, &fields)?;
    let actions = parse_actions(&form_config.actions)?;
    let rules = parse_form_rules(&form_config.rules)?;
    
    Ok(FormSchema {
        form_type: form_config.form_type,
        layout,
        groups,
        actions,
        rules,
    })
}

pub async fn render_by_model(model_id: i64, form_type: String) -> Result<FormSchema> {
    let db = DB().await;
    
    let default_config = cms_form_config::Entity::find()
        .filter(cms_form_config::Column::ModelId.eq(model_id))
        .filter(cms_form_config::Column::FormType.eq(&form_type))
        .filter(cms_form_config::Column::IsDefault.eq("1"))
        .filter(cms_form_config::Column::Status.eq("0"))
        .one(db)
        .await?;
    
    if let Some(config) = default_config {
        return render_schema(config.id).await;
    }
    
    let _model = cms_model::Entity::find_by_id(model_id)
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("模型不存在"))?;
    
    let fields = cms_field::Entity::find()
        .filter(cms_field::Column::ModelId.eq(model_id))
        .filter(cms_field::Column::IsFormShow.eq("1"))
        .order_by_asc(cms_field::Column::Sort)
        .all(db)
        .await?;
    
    let field_schemas: Vec<FieldSchema> = fields
        .iter()
        .map(|f| build_field_schema(f))
        .collect();
    
    let group = FormGroupSchema {
        id: "default".to_string(),
        name: "基本信息".to_string(),
        code: "default".to_string(),
        icon: None,
        fields: field_schemas,
        collapsed: false,
    };
    
    let default_action = FormActionSchema {
        action_type: "submit".to_string(),
        label: "提交".to_string(),
        code: "submit".to_string(),
        icon: None,
        props: None,
    };
    
    Ok(FormSchema {
        form_type,
        layout: FormLayout::default(),
        groups: vec![group],
        actions: vec![default_action],
        rules: vec![],
    })
}

pub async fn get_field_components() -> Result<Vec<FieldComponent>> {
    let components = vec![
        FieldComponent {
            code: "input".to_string(),
            name: "输入框".to_string(),
            category: "basic".to_string(),
            icon: Some("edit".to_string()),
            props: json!({"maxlength": 200, "showWordLimit": true}),
            default_value: Some(json!("")),
        },
        FieldComponent {
            code: "textarea".to_string(),
            name: "文本域".to_string(),
            category: "basic".to_string(),
            icon: Some("document".to_string()),
            props: json!({"rows": 4, "maxlength": 500}),
            default_value: Some(json!("")),
        },
        FieldComponent {
            code: "number".to_string(),
            name: "数字输入".to_string(),
            category: "basic".to_string(),
            icon: Some("calculator".to_string()),
            props: json!({"min": 0, "precision": 2}),
            default_value: Some(json!(0)),
        },
        FieldComponent {
            code: "select".to_string(),
            name: "选择器".to_string(),
            category: "basic".to_string(),
            icon: Some("arrow-down".to_string()),
            props: json!({"clearable": true, "filterable": true}),
            default_value: None,
        },
        FieldComponent {
            code: "radio".to_string(),
            name: "单选框".to_string(),
            category: "basic".to_string(),
            icon: Some("select".to_string()),
            props: json!({}),
            default_value: None,
        },
        FieldComponent {
            code: "checkbox".to_string(),
            name: "复选框".to_string(),
            category: "basic".to_string(),
            icon: Some("finished".to_string()),
            props: json!({}),
            default_value: Some(json!([])),
        },
        FieldComponent {
            code: "switch".to_string(),
            name: "开关".to_string(),
            category: "basic".to_string(),
            icon: Some("open".to_string()),
            props: json!({"activeValue": "1", "inactiveValue": "0"}),
            default_value: Some(json!("0")),
        },
        FieldComponent {
            code: "date".to_string(),
            name: "日期选择".to_string(),
            category: "date".to_string(),
            icon: Some("calendar".to_string()),
            props: json!({"format": "YYYY-MM-DD", "valueFormat": "YYYY-MM-DD"}),
            default_value: None,
        },
        FieldComponent {
            code: "datetime".to_string(),
            name: "日期时间选择".to_string(),
            category: "date".to_string(),
            icon: Some("timer".to_string()),
            props: json!({"format": "YYYY-MM-DD HH:mm:ss", "valueFormat": "YYYY-MM-DD HH:mm:ss"}),
            default_value: None,
        },
        FieldComponent {
            code: "time".to_string(),
            name: "时间选择".to_string(),
            category: "date".to_string(),
            icon: Some("clock".to_string()),
            props: json!({"format": "HH:mm:ss", "valueFormat": "HH:mm:ss"}),
            default_value: None,
        },
        FieldComponent {
            code: "upload".to_string(),
            name: "文件上传".to_string(),
            category: "media".to_string(),
            icon: Some("upload".to_string()),
            props: json!({"limit": 1, "accept": "*/*"}),
            default_value: Some(json!([])),
        },
        FieldComponent {
            code: "rich-text".to_string(),
            name: "富文本编辑器".to_string(),
            category: "media".to_string(),
            icon: Some("edit-outline".to_string()),
            props: json!({"height": 300}),
            default_value: Some(json!("")),
        },
        FieldComponent {
            code: "cascader".to_string(),
            name: "级联选择".to_string(),
            category: "advanced".to_string(),
            icon: Some("share".to_string()),
            props: json!({"clearable": true, "filterable": true}),
            default_value: None,
        },
        FieldComponent {
            code: "tree-select".to_string(),
            name: "树选择".to_string(),
            category: "advanced".to_string(),
            icon: Some("tree".to_string()),
            props: json!({"clearable": true, "checkStrictly": true}),
            default_value: None,
        },
    ];
    
    Ok(components)
}

pub async fn get_validation_rules() -> Result<Vec<ValidationRule>> {
    let rules = vec![
        ValidationRule {
            code: "required".to_string(),
            name: "必填".to_string(),
            rule_type: "required".to_string(),
            default_message: "该字段为必填项".to_string(),
            params: None,
        },
        ValidationRule {
            code: "email".to_string(),
            name: "邮箱格式".to_string(),
            rule_type: "pattern".to_string(),
            default_message: "请输入正确的邮箱地址".to_string(),
            params: Some(json!({"pattern": "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$"})),
        },
        ValidationRule {
            code: "phone".to_string(),
            name: "手机号格式".to_string(),
            rule_type: "pattern".to_string(),
            default_message: "请输入正确的手机号".to_string(),
            params: Some(json!({"pattern": "^1[3-9]\\d{9}$"})),
        },
        ValidationRule {
            code: "url".to_string(),
            name: "URL格式".to_string(),
            rule_type: "pattern".to_string(),
            default_message: "请输入正确的URL地址".to_string(),
            params: Some(json!({"pattern": "^https?://[\\w\\-]+(\\.[\\w\\-]+)+[/#?]?.*$"})),
        },
        ValidationRule {
            code: "min_length".to_string(),
            name: "最小长度".to_string(),
            rule_type: "min".to_string(),
            default_message: "长度不能少于{min}个字符".to_string(),
            params: Some(json!({"min": 1})),
        },
        ValidationRule {
            code: "max_length".to_string(),
            name: "最大长度".to_string(),
            rule_type: "max".to_string(),
            default_message: "长度不能超过{max}个字符".to_string(),
            params: Some(json!({"max": 200})),
        },
        ValidationRule {
            code: "range".to_string(),
            name: "数值范围".to_string(),
            rule_type: "range".to_string(),
            default_message: "数值必须在{min}到{max}之间".to_string(),
            params: Some(json!({"min": 0, "max": 100})),
        },
        ValidationRule {
            code: "integer".to_string(),
            name: "整数".to_string(),
            rule_type: "pattern".to_string(),
            default_message: "请输入整数".to_string(),
            params: Some(json!({"pattern": "^-?\\d+$"})),
        },
    ];
    
    Ok(rules)
}

fn parse_layout(layout_str: &Option<String>) -> Result<FormLayout> {
    match layout_str {
        Some(s) if !s.is_empty() => {
            serde_json::from_str(s).map_err(|e| Error::bad_request(format!("布局配置解析失败: {}", e)))
        }
        _ => Ok(FormLayout::default()),
    }
}

fn parse_groups(groups_str: &Option<String>, fields: &[cms_field::Model]) -> Result<Vec<FormGroupSchema>> {
    match groups_str {
        Some(s) if !s.is_empty() => {
            let mut groups: Vec<FormGroupSchema> = serde_json::from_str(s)
                .map_err(|e| Error::bad_request(format!("分组配置解析失败: {}", e)))?;
            
            for group in &mut groups {
                let group_field_codes: std::collections::HashSet<String> = group
                    .fields
                    .iter()
                    .map(|f| f.field.clone())
                    .collect();
                
                for field in fields {
                    if !group_field_codes.contains(&field.code) {
                        group.fields.push(build_field_schema(field));
                    }
                }
            }
            
            if groups.is_empty() {
                groups.push(build_default_group(fields));
            }
            
            Ok(groups)
        }
        _ => Ok(vec![build_default_group(fields)]),
    }
}

fn build_default_group(fields: &[cms_field::Model]) -> FormGroupSchema {
    FormGroupSchema {
        id: "default".to_string(),
        name: "基本信息".to_string(),
        code: "default".to_string(),
        icon: None,
        fields: fields.iter().map(|f| build_field_schema(f)).collect(),
        collapsed: false,
    }
}

fn parse_actions(actions_str: &Option<String>) -> Result<Vec<FormActionSchema>> {
    match actions_str {
        Some(s) if !s.is_empty() => {
            serde_json::from_str(s).map_err(|e| Error::bad_request(format!("动作配置解析失败: {}", e)))
        }
        _ => Ok(vec![
            FormActionSchema {
                action_type: "submit".to_string(),
                label: "提交".to_string(),
                code: "submit".to_string(),
                icon: None,
                props: None,
            },
            FormActionSchema {
                action_type: "reset".to_string(),
                label: "重置".to_string(),
                code: "reset".to_string(),
                icon: None,
                props: None,
            },
        ]),
    }
}

fn parse_form_rules(rules_str: &Option<String>) -> Result<Vec<FormRuleSchema>> {
    match rules_str {
        Some(s) if !s.is_empty() => {
            serde_json::from_str(s).map_err(|e| Error::bad_request(format!("规则配置解析失败: {}", e)))
        }
        _ => Ok(vec![]),
    }
}

fn build_field_schema(field: &cms_field::Model) -> FieldSchema {
    let component = get_component_by_field_type(&field.field_type);
    let rules = build_field_rules(field);
    let props = parse_field_props(&field.form_config);
    
    FieldSchema {
        field: field.code.clone(),
        label: field.name.clone(),
        field_type: field.field_type.clone(),
        component,
        placeholder: Some(format!("请输入{}", field.name)),
        help_text: None,
        default_value: field.default_value.clone().and_then(|v| serde_json::from_str(&v).ok()),
        required: field.is_required == "1",
        disabled: false,
        visible: true,
        rules,
        props,
    }
}

fn get_component_by_field_type(field_type: &str) -> String {
    match field_type {
        "string" | "text" => "input".to_string(),
        "textarea" | "longtext" => "textarea".to_string(),
        "integer" | "decimal" | "float" | "number" => "number".to_string(),
        "boolean" | "switch" => "switch".to_string(),
        "date" => "date".to_string(),
        "datetime" => "datetime".to_string(),
        "time" => "time".to_string(),
        "select" | "enum" => "select".to_string(),
        "radio" => "radio".to_string(),
        "checkbox" | "multiselect" => "checkbox".to_string(),
        "file" | "image" | "upload" => "upload".to_string(),
        "richtext" | "editor" => "rich-text".to_string(),
        "cascader" => "cascader".to_string(),
        "tree" => "tree-select".to_string(),
        _ => "input".to_string(),
    }
}

fn build_field_rules(field: &cms_field::Model) -> Vec<FieldRule> {
    let mut rules = Vec::new();
    
    if field.is_required == "1" {
        rules.push(FieldRule {
            rule_type: "required".to_string(),
            message: Some(format!("{}为必填项", field.name)),
            trigger: "blur".to_string(),
            value: None,
        });
    }
    
    if let Some(validation_str) = &field.validation {
        if let Ok(validation) = serde_json::from_str::<JsonValue>(validation_str) {
            if let Some(validation_rules) = validation.as_array() {
                for rule in validation_rules {
                    if let (Some(rule_type), Some(message)) = (
                        rule.get("rule_type").and_then(|v| v.as_str()),
                        rule.get("message").and_then(|v| v.as_str()),
                    ) {
                        rules.push(FieldRule {
                            rule_type: rule_type.to_string(),
                            message: Some(message.to_string()),
                            trigger: rule.get("trigger").and_then(|v| v.as_str()).unwrap_or("blur").to_string(),
                            value: rule.get("value").cloned(),
                        });
                    }
                }
            }
        }
    }
    
    rules
}

fn parse_field_props(form_config: &Option<String>) -> JsonValue {
    match form_config {
        Some(s) if !s.is_empty() => {
            serde_json::from_str(s).unwrap_or(json!({}))
        }
        _ => json!({}),
    }
}
