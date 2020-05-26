use crate::data::default::{default_files, DefaultData};
use crate::data::optional_overwrite::OptionalOverwrite;
use std::env;
use std::path::Path;
mod data;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => println!("Please provide an argument"),
        2 => {
            let path = Path::new(&args[1]);
            if path.is_absolute() {
                println!("Absolute path's are not supported");
                return;
            }
            let exam_result = data::exam::Exam::from_file(path);
            match exam_result {
                Ok(mut exam) => {
                    println!("{:#?}", exam);
                    let default_files = default_files(path);
                    println!("Found {} default files.", default_files.len());
                    for default_file in default_files.iter() {
                        if !exam.empty_fields().is_empty() {
                            println!("Reading {}", default_file.get_path().display());
                            let default_data = default_file.read_as_data().unwrap(); //TODO
                            match default_data {
                                DefaultData::Navigation(n) => exam.navigation.overwrite(&Some(n)),
                                DefaultData::Timing(t) => exam.timing.overwrite(&Some(t)),
                                DefaultData::Feedback(f) => exam.feedback.overwrite(&Some(f)),
                                DefaultData::Question(q) => {
                                    if let Some(ref mut groups) = exam.question_groups {
                                        groups.iter_mut().for_each(|qg| {
                                            if let Some(ref mut questions) = &mut qg.questions {
                                                questions.iter_mut().for_each(|question| {
                                                    question
                                                        .question_data
                                                        .overwrite(&Some(q.clone()))
                                                })
                                            }
                                        })
                                    }
                                }
                            }
                        }
                    }
                    let numbas = exam.to_numbas();
                    match numbas {
                        Ok(res) => (), // println!("{:#?}", res),
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
