use crate::data::optional_overwrite::*;
use crate::data::question_part::{QuestionPart, VariableReplacementStrategy};
use crate::data::template::{Value, ValueType};
use crate::data::to_numbas::{NumbasResult, ToNumbas};
use crate::data::to_rumbas::*;
use crate::data::translatable::TranslatableString;
use serde::{Deserialize, Serialize};

//TODO: add other extensions
optional_overwrite! {
    /// Specify which extensions should be enabled
    pub struct Extensions {
        /// Whether the jsx_graph extension is enabled
        jsx_graph: bool,
        /// Whether the stats extension is enabled
        stats: bool,
        /// Whether the eukleides extension is enabled
        eukleides: bool,
        /// Whether the geogebra extension is enabled
        geogebra: bool
    }
}

impl ToNumbas for Extensions {
    // TODO: create macro
    type NumbasType = Vec<String>;
    fn to_numbas(&self, _locale: &str) -> NumbasResult<Vec<String>> {
        let check = self.check();
        if check.is_empty() {
            let mut extensions = Vec::new();
            if self.jsx_graph.unwrap() {
                extensions.push("jsx_graph".to_string());
            }
            if self.stats.unwrap() {
                extensions.push("stats".to_string());
            }
            if self.eukleides.unwrap() {
                extensions.push("eukleides".to_string());
            }
            if self.geogebra.unwrap() {
                extensions.push("geogebra".to_string());
            }
            Ok(extensions)
        } else {
            Err(check)
        }
    }
}

impl Extensions {
    pub fn from(v: &[String]) -> Self {
        Extensions {
            jsx_graph: Value::Normal(v.contains(&"jsx_graph".to_string())),
            stats: Value::Normal(v.contains(&"stats".to_string())),
            eukleides: Value::Normal(v.contains(&"eukleides".to_string())),
            geogebra: Value::Normal(v.contains(&"geogebra".to_string())),
        }
    }
}

impl Default for Extensions {
    fn default() -> Extensions {
        Extensions {
            jsx_graph: Value::Normal(false),
            stats: Value::Normal(false),
            eukleides: Value::Normal(false),
            geogebra: Value::Normal(false),
        }
    }
}
impl Extensions {
    pub fn combine(e: Extensions, f: Extensions) -> Extensions {
        Extensions {
            jsx_graph: Value::Normal(e.jsx_graph.unwrap() || f.jsx_graph.unwrap()),
            stats: Value::Normal(e.stats.unwrap() || f.stats.unwrap()),
            eukleides: Value::Normal(e.eukleides.unwrap() || f.eukleides.unwrap()),
            geogebra: Value::Normal(e.geogebra.unwrap() || f.geogebra.unwrap()),
        }
    }

    pub fn to_paths(&self) -> Vec<String> {
        let numbas_path = std::env::var(crate::NUMBAS_FOLDER_ENV)
            .expect(&format!("{} to be set", crate::NUMBAS_FOLDER_ENV)[..]);
        let mut paths = Vec::new();
        if self.jsx_graph.unwrap() {
            paths.push("jsxgraph"); // TODO: add a toString?
        }
        if self.stats.unwrap() {
            paths.push("stats");
        }
        if self.eukleides.unwrap() {
            paths.push("eukleides");
        }
        if self.geogebra.unwrap() {
            paths.push("geogebra");
        }
        paths
            .into_iter()
            .map(|s| format!("{}/extensions/{}", numbas_path, s))
            .collect()
    }
}

question_part_type! {
    pub struct QuestionPartExtension {}
}

impl ToNumbas for QuestionPartExtension {
    type NumbasType = numbas::exam::ExamQuestionPartExtension;
    fn to_numbas(&self, locale: &str) -> NumbasResult<Self::NumbasType> {
        let check = self.check();
        if check.is_empty() {
            Ok(Self::NumbasType {
                part_data: self.to_numbas_shared_data(locale),
            })
        } else {
            Err(check)
        }
    }
}

impl ToRumbas<QuestionPartExtension> for numbas::exam::ExamQuestionPartExtension {
    fn to_rumbas(&self) -> QuestionPartExtension {
        QuestionPartExtension {
            marks: Value::Normal(extract_part_common_marks(&self.part_data)),
            prompt: Value::Normal(TranslatableString::s(&extract_part_common_prompt(
                &self.part_data,
            ))),
            use_custom_name: Value::Normal(extract_part_common_use_custom_name(&self.part_data)),
            custom_name: Value::Normal(extract_part_common_custom_name(&self.part_data)),
            steps_penalty: Value::Normal(extract_part_common_steps_penalty(&self.part_data)),
            enable_minimum_marks: Value::Normal(extract_part_common_enable_minimum_marks(
                &self.part_data,
            )),
            minimum_marks: Value::Normal(extract_part_common_minimum_marks(&self.part_data)),
            show_correct_answer: Value::Normal(extract_part_common_show_correct_answer(
                &self.part_data,
            )),
            show_feedback_icon: Value::Normal(extract_part_common_show_feedback_icon(
                &self.part_data,
            )),
            variable_replacement_strategy: Value::Normal(
                self.part_data.variable_replacement_strategy.to_rumbas(),
            ),
            adaptive_marking_penalty: Value::Normal(extract_part_common_adaptive_marking_penalty(
                &self.part_data,
            )),
            custom_marking_algorithm: Value::Normal(extract_part_common_custom_marking_algorithm(
                &self.part_data,
            )),
            extend_base_marking_algorithm: Value::Normal(
                extract_part_common_extend_base_marking_algorithm(&self.part_data),
            ),
            steps: Value::Normal(extract_part_common_steps(&self.part_data)),
        }
    }
}
