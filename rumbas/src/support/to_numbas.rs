use rumbas_support::rumbas_check::{RumbasCheck, RumbasCheckResult};

pub type NumbasResult<T> = Result<T, RumbasCheckResult>;

pub trait ToNumbas<NumbasType>: Clone + RumbasCheck {
    /// Method that safely converts a type to another (probably numbas) type
    fn to_numbas_safe(&self, locale: &str, data: &Self::ToNumbasHelper) -> NumbasResult<NumbasType> {
        let check = self.check(locale);
        if check.is_empty() {
            Ok(self.to_numbas(locale, data))
        } else {
            Err(check)
        }
    }

    type ToNumbasHelper;
    /// Method that converts a type to another type
    /// This method assumes that it is called by a function that is initially called from `to_numbas_safe`
    fn to_numbas(&self, locale: &str, data: &Self::ToNumbasHelper) -> NumbasType;
}

impl_to_numbas!(String, bool, f64, usize);
impl_to_numbas!(numbas::jme::JMEString);
impl_to_numbas!(numbas::jme::EmbracedJMEString);
impl_to_numbas!(numbas::jme::ContentAreaString);
impl_to_numbas!(numbas::support::primitive::Number);

impl<S, O: ToNumbas<S>> ToNumbas<Vec<S>> for Vec<O> {
    type ToNumbasHelper= O::ToNumbasHelper;
    fn to_numbas(&self, locale: &str, data: &Self::ToNumbasHelper) -> Vec<S> {
        let mut v = Vec::new();
        for item in self.iter() {
            v.push(item.to_numbas(locale, &data));
        }
        v
    }
}

impl<K: Clone + std::hash::Hash + std::cmp::Eq, S, O: ToNumbas<S>>
    ToNumbas<std::collections::HashMap<K, S>> for std::collections::HashMap<K, O>
where
    std::collections::HashMap<K, O>: RumbasCheck,
{
    type ToNumbasHelper= O::ToNumbasHelper;
    fn to_numbas(&self, locale: &str, data: &Self::ToNumbasHelper) -> std::collections::HashMap<K, S> {
        self.iter()
            .map(|(k, v)| (k.to_owned(), v.to_numbas(locale, &data)))
            .collect()
    }
}

impl<K: Clone + std::cmp::Ord, S, O: ToNumbas<S>> ToNumbas<std::collections::BTreeMap<K, S>>
    for std::collections::BTreeMap<K, O>
where
    std::collections::BTreeMap<K, O>: RumbasCheck,
{
    type ToNumbasHelper= O::ToNumbasHelper;
    fn to_numbas(&self, locale: &str, data: &Self::ToNumbasHelper) -> std::collections::BTreeMap<K, S> {
        self.iter()
            .map(|(k, v)| (k.to_owned(), v.to_numbas(locale, &data)))
            .collect()
    }
}

impl<AA, A: ToNumbas<AA>, BB, B: ToNumbas<BB>> ToNumbas<(AA, BB)> for (A, B)
where
    (A, B): RumbasCheck,
{
    type ToNumbasHelper= (A::ToNumbasHelper, B::ToNumbasHelper);
    fn to_numbas(&self, locale: &str, data: &Self::ToNumbasHelper) -> (AA, BB) {
        (self.0.to_numbas(locale, &data.0), self.1.to_numbas(locale, &data.1))
    }
}

impl<AA, A: ToNumbas<AA>> ToNumbas<[AA; 2]> for [A; 2]
where
    [A; 2]: RumbasCheck,
{
    type ToNumbasHelper= A::ToNumbasHelper;
    fn to_numbas(&self, locale: &str, data: &Self::ToNumbasHelper) -> [AA; 2] {
        [self[0].to_numbas(locale, data), self[1].to_numbas(locale, data)]
    }
}

impl ToNumbas<numbas::support::primitive::SafeFloat> for f64 {
    type ToNumbasHelper= ();
    fn to_numbas(&self, _locale: &str, _: &Self::ToNumbasHelper) -> numbas::support::primitive::SafeFloat {
        (*self).into()
    }
}

impl ToNumbas<numbas::support::primitive::SafeNatural> for usize {
    type ToNumbasHelper= ();
    fn to_numbas(&self, _locale: &str, _: &Self::ToNumbasHelper) -> numbas::support::primitive::SafeNatural {
        (*self).into()
    }
}

macro_rules! impl_to_numbas {
    ($($type: ty), *) => {
        $(
        impl ToNumbas<$type> for $type {
            type ToNumbasHelper= ();
            fn to_numbas(&self, _locale: &str, _: &Self::ToNumbasHelper) -> $type {
                self.clone()
            }
        }
        )*
    };
}

pub(crate) use impl_to_numbas;
