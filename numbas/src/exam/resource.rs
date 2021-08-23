use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
//TODO: remove Exam from front of all types?
//TODO: check what is optional etc
//TODO: advicethreshold?

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct Resource(pub [String; 2]);
