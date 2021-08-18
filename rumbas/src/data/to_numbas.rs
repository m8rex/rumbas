use crate::data::optional_overwrite::*;
use crate::data::template::{Value, ValueType};

pub type NumbasResult<T> = Result<T, RumbasCheckResult>;

pub trait ToNumbas<NumbasType>: Clone + RumbasCheck {
    /// Method that safely converts a type to another (probably numbas) type
    fn to_numbas_safe(&self, locale: &str) -> NumbasResult<NumbasType> {
        let check = self.check(locale);
        if check.is_empty() {
            Ok(self.to_numbas(locale))
        } else {
            Err(check)
        }
    }
    /// Method that converts a type to another type
    /// This method assumes that it is called by a function that is initially called from `to_numbas_safe`
    fn to_numbas(&self, locale: &str) -> NumbasType;
    fn to_numbas_with_name(&self, locale: &str, _name: String) -> NumbasType {
        self.to_numbas(locale)
    }
}

impl<T: RumbasCheck> RumbasCheck for Value<T> {
    fn check(&self, locale: &str) -> RumbasCheckResult {
        match &self.0 {
            Some(ValueType::Normal(val)) => val.check(locale),
            Some(ValueType::Template(ts)) => RumbasCheckResult::from_missing(Some(ts.yaml())),
            Some(ValueType::Invalid(v)) => RumbasCheckResult::from_invalid(v),
            None => RumbasCheckResult::from_missing(None),
        }
    }
}

impl<S, T: ToNumbas<S> + RumbasCheck> ToNumbas<S> for Value<T> {
    fn to_numbas(&self, locale: &str) -> S {
        match &self.0 {
            Some(ValueType::Normal(val)) => val.to_numbas(locale),
            Some(ValueType::Template(_ts)) => unreachable!(),
            Some(ValueType::Invalid(_v)) => unreachable!(),
            None => unreachable!(),
        }
    }
}

impl<T: RumbasCheck> RumbasCheck for Noneable<T> {
    fn check(&self, locale: &str) -> RumbasCheckResult {
        match self {
            Noneable::NotNone(val) => val.check(locale),
            _ => RumbasCheckResult::empty(),
        }
    }
}

impl<S, T: ToNumbas<S> + RumbasCheck> ToNumbas<Option<S>> for Noneable<T> {
    fn to_numbas(&self, locale: &str) -> Option<S> {
        match self {
            Noneable::NotNone(val) => Some(val.clone().to_numbas(locale)),
            _ => None,
        }
    }
}

macro_rules! impl_to_numbas {
    ($($type: ty), *) => {
        $(
        impl ToNumbas<$type> for $type {
            fn to_numbas(&self, _locale: &str) -> $type {
                self.clone()
            }
        }
        )*
    };
}

impl_to_numbas!(String, bool, f64, usize, [f64; 2]);

impl<S, O: ToNumbas<S>> ToNumbas<Vec<S>> for Vec<O> {
    fn to_numbas(&self, locale: &str) -> Vec<S> {
        let mut v = Vec::new();
        for item in self.iter() {
            v.push(item.to_numbas(locale));
        }
        v
    }
}
