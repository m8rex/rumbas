use crate::data::optional_overwrite::*;
use crate::data::template::{Value, ValueType};

pub type NumbasResult<T> = Result<T, RumbasCheckResult>;

pub trait ToNumbas: Clone {
    type NumbasType;
    fn to_numbas(&self, locale: &str) -> NumbasResult<Self::NumbasType>;
    fn to_numbas_with_name(&self, locale: &str, _name: String) -> NumbasResult<Self::NumbasType> {
        self.to_numbas(locale)
    }
}

impl<T: ToNumbas + RumbasCheck> ToNumbas for Value<T> {
    type NumbasType = <T as ToNumbas>::NumbasType;
    fn to_numbas(&self, locale: &str) -> NumbasResult<Self::NumbasType> {
        match &self.0 {
            Some(ValueType::Normal(val)) => {
                let check = val.check();
                if check.is_empty() {
                    Ok(val.to_numbas(locale).unwrap())
                } else {
                    Err(check)
                }
            }
            Some(ValueType::Template(ts)) => Err(RumbasCheckResult::from_missing(Some(ts.yaml()))),
            Some(ValueType::Invalid(v)) => Err(RumbasCheckResult::from_invalid(v)),
            None => Err(RumbasCheckResult::from_missing(None)),
        }
    }
}

impl<T: ToNumbas + RumbasCheck> ToNumbas for Noneable<T> {
    type NumbasType = Option<<T as ToNumbas>::NumbasType>;
    fn to_numbas(&self, locale: &str) -> NumbasResult<Self::NumbasType> {
        match self {
            Noneable::NotNone(val) => {
                let check = val.check();
                if check.is_empty() {
                    Ok(Some(val.clone().to_numbas(locale).unwrap()))
                } else {
                    Err(check)
                }
            }
            _ => Ok(None),
        }
    }
}

macro_rules! impl_to_numbas {
    ($($type: ty), *) => {
        $(
        impl ToNumbas for $type {
            type NumbasType = $type;
            fn to_numbas(&self, _locale: &str) -> NumbasResult<Self::NumbasType> {
                Ok(self.clone())
            }
        }
        )*
    };
}

impl_to_numbas!(String, bool, f64, usize, [f64; 2]);

impl<O: ToNumbas> ToNumbas for Vec<O> {
    type NumbasType = Vec<O::NumbasType>;
    fn to_numbas(&self, locale: &str) -> NumbasResult<Self::NumbasType> {
        let mut v = Vec::new();
        for item in self.iter() {
            v.push(item.to_numbas(locale)?);
        }
        Ok(v)
    }
}
