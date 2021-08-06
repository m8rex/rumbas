use pest::iterators::Pairs;
use pest::Parser;

#[derive(Parser)]
#[grammar = "jme.pest"]
struct JMEParser;

fn parse(s: &str) -> Result<Pairs<'_, Rule>, pest::error::Error<Rule>> {
    JMEParser::parse(Rule::jme, s)
}

#[cfg(test)]
mod test {
    use super::*;

    const VALID_NAMES: [&str; 6] = ["x", "x_1", "time_between_trials", "var1", "row1val2", "y''"];
    const VALID_ANNOTATIONS: [&str; 11] = [
        "verb", "op", "v", "vector", "unit", "dot", "m", "matrix", "diff", "degrees", "vec",
    ];
    const VALID_LITERALS: [&str; 5] = ["true", "false", "1", "4.3", "\"Numbas\""]; // unicode pi and infinity
    const BUILTIN_CONSTANTS: [&str; 6] = ["pi", "e", "i", "infinity", "infty", "nan"]; // unicode pi and infinity

    #[test]
    fn variable_names() {
        for valid_name in VALID_NAMES {
            assert!(parse(valid_name).is_ok());
        }
    }

    #[test]
    fn annotated_variables() {
        for valid_name in VALID_NAMES {
            for valid_annotation in VALID_ANNOTATIONS {
                let annotated = format!("{}:{}", valid_annotation, valid_name);
                println!("{}", annotated);
                assert!(parse(&annotated[..]).is_ok());
            }
        }
        assert!(parse("v:dot:x").is_ok()); // multiple annotations
    }

    #[test]
    fn literals() {
        for valid_literal in VALID_LITERALS {
            assert!(parse(valid_literal).is_ok());
        }
    }

    #[test]
    fn builtin_constants() {
        for builtin_constant in BUILTIN_CONSTANTS {
            assert!(parse(builtin_constant).is_ok());
        }
    }

    #[test]
    fn grouped_terms_simple() {
        for valid_name in VALID_NAMES {
            let grouped = format!("({})", valid_name);
            assert!(parse(&grouped[..]).is_ok());
        }
    }

    #[test]
    fn function_application() {
        assert!(parse("f(a)").is_ok());
        assert!(parse("g(a,b)").is_ok());
    }

    // TODO test operators

    #[test]
    fn collections() {
        assert!(parse("[a: 1, \"first name\": \"Owen\"]").is_ok());
        assert!(parse("[1, 2, 3]").is_ok());
        assert!(parse("[a]").is_ok());
        assert!(parse("[]").is_ok());
    }

    #[test]
    fn indices() {
        assert!(parse("[1, 2, 3][0]").is_ok());
        assert!(parse("x[3..7]").is_ok());
        assert!(parse("id(4)[1]").is_ok());
        assert!(parse("info[\"name\"]").is_ok());
        assert!(parse("\"Numbas\"[0]").is_ok());
    }

    #[test]
    fn implicit_multiplication() {
        // TODO: see warning in docs about settings https://numbas-editor.readthedocs.io/en/latest/jme-reference.html#implicit-multiplication
        assert!(parse("(a+2)(a+1)").is_ok());
        assert!(parse("(a+1)2").is_ok());
        assert!(parse("(x+y)z").is_ok());
        assert!(parse("2x").is_ok());
        assert!(parse("x y").is_ok());
    }
}
