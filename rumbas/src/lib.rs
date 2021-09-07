#[macro_use]
extern crate enum_display_derive;

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

/// The name of the local folder used as cache
/// It caches the .exam files that are given to Numbas.
pub const CACHE_FOLDER: &str = ".rumbas";

/// The name of the local folder used for the output.
pub const OUTPUT_FOLDER: &str = "_output";
