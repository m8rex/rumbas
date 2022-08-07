pub fn bool_true() -> bool {
    true
}

pub fn float_one() -> f64 {
    1.0
}

pub fn safe_natural_one() -> crate::support::primitive::SafeNatural {
    1.into()
}

pub fn variable_safe_natural_three(
) -> crate::support::primitive::VariableValued<crate::support::primitive::SafeNatural> {
    let s: crate::support::primitive::SafeNatural = 3.into();
    crate::support::primitive::VariableValued::Value(s)
}
