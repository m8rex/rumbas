use crate::data::optional_overwrite::{Noneable, OptionalOverwrite};

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

impl<T: ToNumbas + OptionalOverwrite> ToNumbas for Option<T> {
    type NumbasType = <T as ToNumbas>::NumbasType;
    fn to_numbas(&self, locale: &String) -> NumbasResult<Self::NumbasType> {
        match self {
            Some(val) => {
                let empty_fields = val.empty_fields();
                if empty_fields.is_empty() {
                    Ok(val.to_numbas(&locale).unwrap())
                } else {
                    Err(empty_fields)
                }
            }
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
