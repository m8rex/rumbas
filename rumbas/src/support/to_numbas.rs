use crate::support::optional_overwrite::*;

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

impl_to_numbas!(String, bool, f64, usize);
impl_to_numbas!(numbas::jme::JMEString);
impl_to_numbas!(numbas::jme::EmbracedJMEString);
impl_to_numbas!(numbas::jme::ContentAreaString);

impl<S, O: ToNumbas<S>> ToNumbas<Vec<S>> for Vec<O> {
    fn to_numbas(&self, locale: &str) -> Vec<S> {
        let mut v = Vec::new();
        for item in self.iter() {
            v.push(item.to_numbas(locale));
        }
        v
    }
}

impl<K: Clone + std::hash::Hash + std::cmp::Eq, S, O: ToNumbas<S>>
    ToNumbas<std::collections::HashMap<K, S>> for std::collections::HashMap<K, O>
where
    std::collections::HashMap<K, O>: RumbasCheck,
{
    fn to_numbas(&self, locale: &str) -> std::collections::HashMap<K, S> {
        self.iter()
            .map(|(k, v)| (k.to_owned(), v.to_numbas(locale)))
            .collect()
    }
}

impl<AA, A: ToNumbas<AA>, BB, B: ToNumbas<BB>> ToNumbas<(AA, BB)> for (A, B)
where
    (A, B): RumbasCheck,
{
    fn to_numbas(&self, locale: &str) -> (AA, BB) {
        (self.0.to_numbas(locale), self.1.to_numbas(locale))
    }
}

impl<AA, A: ToNumbas<AA>> ToNumbas<[AA; 2]> for [A; 2]
where
    [A; 2]: RumbasCheck,
{
    fn to_numbas(&self, locale: &str) -> [AA; 2] {
        [self[0].to_numbas(locale), self[1].to_numbas(locale)]
    }
}

impl ToNumbas<numbas::exam::SafeFloat> for f64 {
    fn to_numbas(&self, _locale: &str) -> numbas::exam::SafeFloat {
        (*self).into()
    }
}

impl ToNumbas<numbas::exam::SafeNatural> for usize {
    fn to_numbas(&self, _locale: &str) -> numbas::exam::SafeNatural {
        (*self).into()
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

pub(crate) use impl_to_numbas;
