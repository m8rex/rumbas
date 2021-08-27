use crate::support::noneable::*;
use crate::support::optional_overwrite::*;
use crate::support::template::Value;
use crate::support::translatable::TranslatableStrings;
use crate::support::translatable::TranslatableStringsInput;
use crate::support::variable_valued::VariableValued;
use numbas::support::primitive::Primitive;

macro_rules! create_input_alias {
    ($name: ident, $type: ty) => {
        paste::paste! {
            pub type [<$name Input>] = $type;
        }
    };
}

macro_rules! create_rumbas_type {
    ($name: ident, $type: ty) => {
        pub type $name = $type;
        impl_optional_overwrite!($name);
    };
}

create_rumbas_type!(RumbasBool, bool);
create_rumbas_type!(RumbasString, String);
pub type NoneableString = Noneable<RumbasString>;
create_input_alias!(NoneableString, NoneableString);
pub type RumbasStrings = Vec<RumbasString>;
pub type RumbasStringsInput = Vec<Value<RumbasStringInput>>;
create_rumbas_type!(RumbasFloat, f64);
pub type NoneableFloat = Noneable<RumbasFloat>;
create_input_alias!(NoneableFloat, NoneableFloat);
pub type RumbasFloatsInput = Vec<Value<RumbasFloatInput>>;
pub type RumbasFloats = Vec<RumbasFloat>;
create_rumbas_type!(RumbasFloats2, [f64; 2]);
create_rumbas_type!(RumbasNatural, usize);
pub type NoneableNatural = Noneable<RumbasNatural>;
create_input_alias!(NoneableNatural, NoneableNatural);

create_input_alias!(Primitive, Primitive);

pub type VariableValuedNatural = VariableValued<usize>;
create_input_alias!(VariableValuedNatural, VariableValuedNatural);
pub type NoneableVariableValuedNatural = Noneable<VariableValuedNatural>;
create_input_alias!(NoneableVariableValuedNatural, NoneableVariableValuedNatural);

pub type VariableValuedTranslatableStringsInput = VariableValued<TranslatableStringsInput>;
pub type VariableValuedTranslatableStrings = VariableValued<TranslatableStrings>;

pub type VariableValuedPrimitivesInput =
    VariableValued<Vec<Value<numbas::support::primitive::Primitive>>>;
pub type VariableValuedPrimitives = VariableValued<Vec<numbas::support::primitive::Primitive>>;

pub type VariableValuedPrimitivessInput =
    VariableValued<Vec<Value<Vec<Value<numbas::support::primitive::Primitive>>>>>;
pub type VariableValuedPrimitivess =
    VariableValued<Vec<Vec<numbas::support::primitive::Primitive>>>;
pub type NoneableTranslatableStrings = Noneable<TranslatableStrings>;
pub type NoneableTranslatableStringsInput = Noneable<TranslatableStringsInput>;

pub(crate) use create_input_alias;
