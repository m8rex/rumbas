trait OptionalOverwrite {
    type Item;

    fn empty_fields(&self) -> Vec<String>;
    fn overwrite(&mut self, other: &Self::Item);
}

impl OptionalOverwrite for String {
    type Item = String;
    fn empty_fields(&self) -> Vec<String> {
        Vec::new()
    }
    fn overwrite(&mut self, _other: &String) {}
}

macro_rules! optional_overwrite {
    // This macro creates a struct with all optional fields
    // It also adds a method to overwrite all fields with None value with the values of another object of the same type
    // It also adds a method to list the fields that are None
    ($struct: ident, $($field: ident: $type: ty), *) => {
        #[derive(Debug, Clone)]
        pub struct $struct {
            $(
                pub $field: Option<$type>
            ),*
        }
        impl OptionalOverwrite for $struct {
            type Item = $struct;
            fn empty_fields(&self) -> Vec<String> {
                let mut empty = Vec::new();
                $(

                    if let Some(val) = &self.$field {
                        let extra_empty = val.empty_fields();
                        for extra in extra_empty.iter() {
                            empty.push(format!("{}.{}", stringify!($field), extra));
                        }
                    } else{
                        empty.push(stringify!($field).to_string());
                    }
                )*
                empty
            }
            fn overwrite(&mut self, other: &$struct) {
                $(
                    if let Some(ref mut val) = self.$field {
                        if let Some(other_val) = &other.$field {
                            val.overwrite(&other_val);
                        }
                    } else {
                        self.$field = other.$field.clone();
                    }
                )*
            }

        }
    }
}

optional_overwrite! {
    Temp,
    name: String,
    test: String
}

optional_overwrite! {
    Temp2,
    other: String,
    t: Temp
}

fn main() {
    println!("Hello, world!");
    let mut t = Temp {
        name: Some("test".to_string()),
        test: None,
    };
    let t2 = Temp {
        name: Some("test2".to_string()),
        test: Some("name".to_string()),
    };
    println!("{:?}", t);
    println!("{:?}", t.empty_fields());
    t.overwrite(&t2);
    println!("{:?}", t);
    let t3 = Temp2 {
        other: None,
        t: Some(Temp {
            name: None,
            test: Some("name".to_string()),
        }),
    };
    println!("{:?}", t3.empty_fields());
    let mut t4 = Temp2 {
        other: None,
        t: None,
    };
    println!("{:?}", t4.empty_fields());
    t4.overwrite(&t3);
    println!("{:?}", t4);
    let t5 = Temp2 {
        other: None,
        t: Some(Temp {
            name: Some("test".to_string()),
            test: Some("name".to_string()),
        }),
    };
    t4.overwrite(&t5);
    println!("{:?}", t4);
    println!("{:?}", t5);
}
