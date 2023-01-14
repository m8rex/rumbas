use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::*;
use comparable::Comparable;
use rumbas_support::preamble::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use structdoc::StructDoc;

macro_rules! extensions {
    (
        $(
            $(#[$inner:meta])*
            $name: ident: $path: literal
        ),+
    ) => {
            #[derive(Input, Overwrite, RumbasCheck, Examples, StructDoc)]
            #[input(name = "ExtensionsInput")]
            #[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq, Eq)]
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
            type ToNumbasHelper = ();
            fn to_numbas(&self, locale: &str, _data: &Self::ToNumbasHelper) -> Vec<String> {
                let mut extensions = Vec::new();
                $(
                    if self.$name.to_numbas(locale, &()) {
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
    /// The Eukleides extension provides functions to embed diagrams created using the
    /// Eukleides language.
    /// https://numbas-editor.readthedocs.io/en/latest/extensions/first-party.html#eukleides
    eukleides: "eukleides",
    /// The GeoGebra extension provides functions to embed GeoGebra worksheets in a question.
    /// https://numbas-editor.readthedocs.io/en/latest/extensions/first-party.html#geogebra
    geogebra: "geogebra",
    /// This extension provides some functions for working with and drawing graphs (networks of vertices joined by edges) in Numbas.
    /// https://numbas-editor.readthedocs.io/en/latest/extensions/first-party.html#graph-theory
    graphs: "graphs",
    /// The JSXGraph extension provides functions to create and manipulate interactive diagrams with the JSXGraph library.
    /// https://numbas-editor.readthedocs.io/en/latest/extensions/first-party.html#eukleides
    jsx_graph: "jsxgraph",
    linear_algebra: "linear-algebra",
    /// This extension provides a new data type and some functions to deal with linear codes.
    /// https://numbas-editor.readthedocs.io/en/latest/extensions/first-party.html#linear-codes
    linear_codes: "codewords",
    optimisation: "optimisation",
    permutations: "permutations",
    /// This extension provides a new data type and some functions to deal with rational polynomials.
    /// https://numbas-editor.readthedocs.io/en/latest/extensions/first-party.html#polynomials
    polynomials: "polynomials",
    /// This extension wraps the js-quantities library to provide a “quantity with units” data type to Numbas.
    /// https://numbas-editor.readthedocs.io/en/latest/extensions/first-party.html#quantities
    quantities: "quantities",
    /// The “random person” extension provides a collection of functions to generate random people, for use in word problems.
    /// https://numbas-editor.readthedocs.io/en/latest/extensions/first-party.html#random-person
    random_person: "random_person",
    /// The statistical functions extension provides many new functions for generating samples from random distributions, and calculating statistics.
    /// https://numbas-editor.readthedocs.io/en/latest/extensions/first-party.html#statistical-functions
    stats: "stats",
    sqlite: "sqlite",
    text: "text",
    written_number: "written-number"

    // TODO: programming extension
}

impl ToRumbas<Extensions> for Vec<String> {
    fn to_rumbas(&self) -> Extensions {
        Extensions::from(self)
    }
}
