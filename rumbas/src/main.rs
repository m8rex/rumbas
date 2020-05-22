use std::env;
mod data;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => println!("Please provide an argument"),
        2 => {
            let path = &args[1];

            let exam = data::exam::Exam::from_file(path);
            match exam {
                Ok(v) => {
                    println!("{:#?}", v);
                    let numbas = v.to_numbas();
                    match numbas {
                        Ok(res) => println!("{:#?}", res),
                        Err(missing_fields) => {
                            println!("Missing fields:\n{}", missing_fields.join("\n"))
                        }
                    }
                }
                Err(e) => {
                    println!(
                        "Error in the json on column {} of line {}. The type of the error is {:?}",
                        e.column(),
                        e.line(),
                        e.classify() // Better explanation: Eof -> end of file, Data: wrong datatype or missing field, Syntax: syntax error
                    );
                }
            };
        }
        _ => println!("Too many arguments"),
    }
}
