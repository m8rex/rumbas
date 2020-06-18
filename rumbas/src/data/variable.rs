use crate::data::file_reference::FileString;
use crate::data::optional_overwrite::{Noneable, OptionalOverwrite};
use crate::data::to_numbas::{NumbasResult, ToNumbas};
use serde::{Deserialize, Serialize};

optional_overwrite! {
    Variable,
    definition: FileString,//TODO: definition dependant of template type, for random_range: start, end and step instead
    description: String,
    template_type: VariableTemplateType,
    group: String //TODO "Ungrouped variables" -> real optional? if not -> ungrouped?
}

impl ToNumbas for Variable {
    type NumbasType = numbas::exam::ExamVariable;
    fn to_numbas_with_name(&self, locale: &String, name: String) -> NumbasResult<Self::NumbasType> {
        let empty_fields = self.empty_fields();
        if empty_fields.is_empty() {
            Ok(numbas::exam::ExamVariable::new(
                name,
                self.definition.clone().unwrap().get_content(),
                self.description.clone().unwrap(),
                self.template_type
                    .clone()
                    .unwrap()
                    .to_numbas(&locale)
                    .unwrap(),
                self.group.clone().unwrap(),
            ))
        } else {
            Err(empty_fields)
        }
    }
    fn to_numbas(&self, _locale: &String) -> NumbasResult<Self::NumbasType> {
        //TODO?
        Err(vec![
            "Should not happen, don't call this method Missing name".to_string(),
        ])
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum VariableTemplateType {
    Anything,
    RandomRange,
}

impl ToNumbas for VariableTemplateType {
    type NumbasType = numbas::exam::ExamVariableTemplateType;
    fn to_numbas(&self, _locale: &String) -> NumbasResult<Self::NumbasType> {
        Ok(match self {
            VariableTemplateType::Anything => numbas::exam::ExamVariableTemplateType::Anything,
            VariableTemplateType::RandomRange => {
                numbas::exam::ExamVariableTemplateType::RandomRange
            }
        })
    }
}
impl_optional_overwrite!(VariableTemplateType);
