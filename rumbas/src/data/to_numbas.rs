use crate::data::optional_overwrite::{Noneable, OptionalOverwrite};
use crate::data::template::{Value, ValueType};

pub type NumbasResult<T> = Result<T, Vec<String>>;

pub trait ToNumbas: Clone {
    type NumbasType;
    fn to_numbas(&self, locale: &String) -> NumbasResult<Self::NumbasType>;
    fn to_numbas_with_name(
        &self,
        locale: &String,
        _name: String,
    ) -> NumbasResult<Self::NumbasType> {
        self.to_numbas(&locale)
    }
}

impl<T: ToNumbas + OptionalOverwrite> ToNumbas for Value<T> {
    type NumbasType = <T as ToNumbas>::NumbasType;
    fn to_numbas(&self, locale: &String) -> NumbasResult<Self::NumbasType> {
        match &self.0 {
            Some(ValueType::Normal(val)) => {
                let empty_fields = val.empty_fields();
                if empty_fields.is_empty() {
                    Ok(val.to_numbas(&locale).unwrap())
                } else {
                    Err(empty_fields)
                }
            }
            Some(ValueType::Template(ts)) => Err(vec![ts.yaml()]),
            None => Err(vec!["".to_string()]),
        }
    }
}

impl<T: ToNumbas + OptionalOverwrite> ToNumbas for Noneable<T> {
    type NumbasType = Option<<T as ToNumbas>::NumbasType>;
    fn to_numbas(&self, locale: &String) -> NumbasResult<Self::NumbasType> {
        match self {
            Noneable::NotNone(val) => {
                let empty_fields = val.empty_fields();
                if empty_fields.is_empty() {
                    Ok(Some(val.clone().to_numbas(&locale).unwrap()))
                } else {
                    Err(empty_fields)
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
            fn to_numbas(&self, _locale: &String) -> NumbasResult<Self::NumbasType> {
                Ok(self.clone())
            }
        }
        )*
    };
}

impl_to_numbas!(String, bool, f64, usize, [f64; 2]);

impl<O: ToNumbas> ToNumbas for Vec<O> {
    type NumbasType = Vec<O::NumbasType>;
    fn to_numbas(&self, locale: &String) -> NumbasResult<Self::NumbasType> {
        let mut v = Vec::new();
        for item in self.into_iter() {
            v.push(item.to_numbas(&locale)?);
        }
        Ok(v)
    }
}
