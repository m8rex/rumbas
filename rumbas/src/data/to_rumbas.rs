pub trait ToRumbas: Clone {
    type RumbasType;
    fn to_rumbas(&self) -> Self::RumbasType;
}

macro_rules! impl_to_rumbas {
    ($($type: ty), *) => {
        $(
        impl ToRumbas for $type {
            type RumbasType = $type;
            fn to_rumbas(&self) -> Self::RumbasType {
                self.clone()
            }
        }
        )*
    };
}

impl_to_rumbas!(String, bool, f64, usize, [f64; 2]);
impl_to_rumbas!(numbas::exam::Primitive);

impl<O: ToRumbas> ToRumbas for Vec<O> {
    type RumbasType = Vec<O::RumbasType>;
    fn to_rumbas(&self) -> Self::RumbasType {
        self.into_iter().map(|item| item.to_rumbas()).collect()
    }
}
