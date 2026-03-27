pub mod date_field;
pub mod editor_field;
pub mod image_field;
pub mod number_field;
pub mod select_field;
pub mod text_field;

use std::sync::Arc;

use crate::domain::cms_plugin::{plugin_manager, FieldTypePlugin, CmsPlugin};

pub fn register_builtin_plugins() {
    let manager = plugin_manager();

    let text: Arc<dyn FieldTypePlugin> = text_field::register();
    let number: Arc<dyn FieldTypePlugin> = number_field::register();
    let select: Arc<dyn FieldTypePlugin> = select_field::register();
    let date: Arc<dyn FieldTypePlugin> = date_field::register();
    let image: Arc<dyn FieldTypePlugin> = image_field::register();
    let editor: Arc<dyn FieldTypePlugin> = editor_field::register();

    manager.register(text).ok();
    manager.register(number).ok();
    manager.register(select).ok();
    manager.register(date).ok();
    manager.register(image).ok();
    manager.register(editor).ok();
}
