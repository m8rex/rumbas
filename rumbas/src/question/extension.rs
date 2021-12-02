use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::*;
use rumbas_support::preamble::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

macro_rules! extensions {
    (
        $(
            $(#[$inner:meta])*
            $name: ident: $path: literal
        ),+
    ) => {
            #[derive(Input, Overwrite, RumbasCheck, Examples)]
            #[input(name = "ExtensionsInput")]
            #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, PartialEq)]
            /// Specify which extensions should be enabled
            pub struct Extensions {
                $(
                    $(
                        #[$inner]
                    )*
                    pub $name: bool
                ),*
            }

        impl ToNumbas<Vec<String>> for Extensions {
            fn to_numbas(&self, locale: &str) -> Vec<String> {
                let mut extensions = Vec::new();
                $(
                    if self.$name.to_numbas(locale) {
                        extensions.push($path.to_string()); //TODO: Enum in numbas crate?
                    }
                )*
                extensions
            }
        }

        impl Extensions {
            pub fn from(v: &[String]) -> Self {
                Extensions {
                    $(
                        $name: v.contains(&$path.to_string()).to_rumbas()
                    ),*
                }
            }

            pub fn combine(e: Extensions, f: Extensions) -> Extensions {
                Extensions {
                    $(
                    $name: e.$name || f.$name
                    ),*
                }
            }

            pub fn to_paths(&self) -> Vec<String> {
                let numbas_path = std::env::var(crate::NUMBAS_FOLDER_ENV)
                    .expect(&format!("{} to be set", crate::NUMBAS_FOLDER_ENV)[..]);
                let mut paths = Vec::new();
                $(
                    if self.$name {
                        paths.push($path);
                    }
                )*
                paths
                    .into_iter()
                    .map(|s| format!("{}/extensions/{}", numbas_path, s))
                    .collect()
            }
        }

        impl Default for Extensions {
            fn default() -> Extensions {
                Extensions {
                    $(
                        $name: false
                    ),*
                }
            }
        }

    }
}

//fixme vis.js extension? (where to find?)
extensions! {
        chemistry: "chemistry",
        download_text_file: "download-text-file",
        eukleides: "eukleides",
        geogebra: "geogebra",
        graphs: "graphs",
        jsx_graph: "jsxgraph",
        linear_algebra: "linear-algebra",
        linear_codes: "codewords",
        optimisation: "optimisation",
        permutations: "permutations",
        polynomials: "polynomials",
        quantities: "quantities",
        random_person: "random_person",
        stats: "stats",
        sqlite: "sqlite",
        written_number: "written-number"
}

impl ToRumbas<Extensions> for Vec<String> {
    fn to_rumbas(&self) -> Extensions {
        Extensions::from(self)
    }
}

// TODO: remove or with attribute
#[cfg(test)]
mod example_test {
    use super::*;
    use rumbas_support::example::Examples;
    #[test]
    fn compile_examples() {
        for example in ExtensionsInput::examples().into_iter() {
            println!("{:?}", example);
            let item = serde_yaml::to_string(&example);
            assert!(item.is_ok());
            let item = item.unwrap();
            insta::with_settings!({sort_maps => true}, {
                insta::assert_yaml_snapshot!(&example);
            });
            let parsed: Result<ExtensionsInput, _> = serde_yaml::from_str(&item[..]);
            if let Err(ref e) = parsed {
                if "No field is set to a not-none value." == &e.to_string()[..] {
                    continue;
                }
                println!("Input {:?}", item);
                println!("Error: {:?}", e);
            }
            assert!(parsed.is_ok());
            assert_eq!(example, parsed.unwrap())
        }
    }
}
