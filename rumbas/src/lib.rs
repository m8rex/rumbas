#[macro_use]
extern crate enum_display_derive;
#[macro_use]
extern crate rumbas_support;
#[macro_use]
extern crate lazy_static;

pub mod exam;
pub mod question;
pub mod support;

pub const QUESTIONS_FOLDER: &str = "questions";
pub const EXAMS_FOLDER: &str = "exams";
pub const QUESTION_TEMPLATES_FOLDER: &str = "question_templates";
pub const EXAM_TEMPLATES_FOLDER: &str = "exam_templates";
pub const RESOURCES_FOLDER: &str = "resources";
pub const DEFAULTS_FOLDER: &str = "defaults";
pub const THEMES_FOLDER: &str = "themes";
pub const CUSTOM_PART_TYPES_FOLDER: &str = "custom_part_types";

pub const QUESTION_PREVIEW_TEMPLATE_NAME: &str = "question_preview";
pub const NUMBAS_FOLDER_ENV: &str = "NUMBAS_FOLDER";
