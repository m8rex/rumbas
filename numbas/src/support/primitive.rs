use crate::jme::JMEString;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
//TODO: remove Exam from front of all types?
//TODO: check what is optional etc
//TODO: advicethreshold?

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Copy)]
#[serde(try_from = "Primitive")]
/// A natural number (unsigned int) that can be parsed from primitive
pub struct SafeNatural(pub usize);

impl std::convert::TryFrom<Primitive> for SafeNatural {
    type Error = String;
    fn try_from(p: Primitive) -> Result<Self, Self::Error> {
        match p {
            Primitive::Natural(n) => Ok(SafeNatural(n)),
            Primitive::Float(_n) => Err("Please use an unsigned integer.".to_string()),
            Primitive::String(n) => n.parse().map(SafeNatural).map_err(|e| e.to_string()),
        }
    }
}

impl std::convert::From<usize> for SafeNatural {
    fn from(u: usize) -> Self {
        SafeNatural(u)
    }
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Copy)]
#[serde(try_from = "Primitive")]
/// A decimal number (float) that can be parsed from primitive
pub struct SafeFloat(pub f64);

impl std::convert::TryFrom<Primitive> for SafeFloat {
    type Error = String;
    fn try_from(p: Primitive) -> Result<Self, Self::Error> {
        match p {
            Primitive::Natural(n) => Ok(SafeFloat(n as f64)),
            Primitive::Float(n) => Ok(SafeFloat(n)),
            Primitive::String(n) => n.parse().map(SafeFloat).map_err(|e| e.to_string()),
        }
    }
}

impl std::convert::From<f64> for SafeFloat {
    fn from(v: f64) -> Self {
        SafeFloat(v)
    }
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(try_from = "BooledPrimitive")]
/// A boolean that can be parsed from (booled) primitive
pub struct SafeBool(pub bool);

impl std::convert::TryFrom<BooledPrimitive> for SafeBool {
    type Error = String;
    fn try_from(p: BooledPrimitive) -> Result<Self, Self::Error> {
        match p {
            BooledPrimitive::Natural(_n) => Err("Please use a boolean value.".to_string()),
            BooledPrimitive::Float(_n) => Err("Please use a boolean value.".to_string()),
            BooledPrimitive::String(n) => n.parse().map(SafeBool).map_err(|e| e.to_string()),
            BooledPrimitive::Bool(b) => Ok(SafeBool(b)),
        }
    }
}

impl std::convert::From<bool> for SafeBool {
    fn from(b: bool) -> Self {
        SafeBool(b)
    }
}

impl std::fmt::Display for SafeBool {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum VariableValued<T> {
    Variable(JMEString),
    Value(T),
}

impl<T> VariableValued<T> {
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> VariableValued<U> {
        match self {
            VariableValued::Variable(x) => VariableValued::Variable(x),
            VariableValued::Value(x) => VariableValued::Value(f(x)),
        }
    }
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum Primitive {
    String(String),
    Natural(usize),
    Float(f64),
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum BooledPrimitive {
    String(String),
    Natural(usize),
    Float(f64),
    Bool(bool),
}

impl std::convert::From<usize> for Primitive {
    fn from(u: usize) -> Self {
        Primitive::Natural(u)
    }
}

impl std::convert::From<f64> for Primitive {
    fn from(f: f64) -> Self {
        Primitive::Float(f)
    }
}

impl std::convert::From<String> for Primitive {
    fn from(s: String) -> Self {
        Primitive::String(s)
    }
}

impl std::fmt::Display for Primitive {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Primitive::String(s) => write!(f, "{}", s),
            Primitive::Natural(n) => write!(f, "{}", n),
            Primitive::Float(fl) => write!(f, "{}", fl),
        }
    }
}
