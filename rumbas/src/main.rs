use crate::data::default::{default_files, DefaultData};
use crate::data::exam;
use crate::data::exam::ToNumbas;
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
            println!("{:?}", path.display());
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
                                DefaultData::QuestionPart(p) => {
                                    if let Some(ref mut groups) = exam.question_groups {
                                        groups.iter_mut().for_each(|qg| {
                                            if let Some(ref mut questions) = &mut qg.questions {
                                                questions.iter_mut().for_each(|question| {
                                                    if let Some(ref mut question_data) =
                                                        question.question_data
                                                    {
                                                        if let Some(ref mut parts) =
                                                            question_data.parts
                                                        {
                                                            parts.iter_mut().for_each(|part| {
                                                                if let (
                                                                    exam::QuestionPart::GapFill(_),
                                                                    exam::QuestionPart::GapFill(_),
                                                                ) = (&p, &part)
                                                                {
                                                                    part.overwrite(&p.clone())
                                                                }
                                                            })
                                                        }
                                                    }
                                                })
                                            }
                                        })
                                    }
                                } //TODO: cleanup...
                                DefaultData::QuestionPartGapFillGap(p) => {
                                    if let Some(ref mut groups) = exam.question_groups {
                                        groups.iter_mut().for_each(|qg| {
                                            if let Some(ref mut questions) = &mut qg.questions {
                                                questions.iter_mut().for_each(|question| {
                                                    if let Some(ref mut question_data) =
                                                        question.question_data
                                                    {
                                                        if let Some(ref mut parts) =
                                                            question_data.parts
                                                        {
                                                            parts.iter_mut().for_each(|part| {
                                                                if let exam::QuestionPart::GapFill(
                                                                    gap_fill,
                                                                ) = part
                                                                {
                                                                    if let Some(ref mut gaps) =
                                                                        gap_fill.gaps
                                                                    {
                                                                        gaps.iter_mut().for_each(
                                                                            |gap| {
                                                                                if let (
                                                                    exam::QuestionPart::JME(_),
                                                                    exam::QuestionPart::JME(_),
                                                                ) = (&p, &gap)
                                                                {
                                                                    gap.overwrite(&p.clone())
                                                                }
                                                                            },
                                                                        )
                                                                    }
                                                                }
                                                            })
                                                        }
                                                    }
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
                        //TODO: move this part to numbas crate?
                        Ok(res) => res.write("output.json"),
                        Err(missing_fields) => {
                            println!("Missing fields:\n{}", missing_fields.join("\n"))
                        }
                    }
                }
                Err(e) => {
                    println!("{}", e);
                }
            };
        }
        _ => println!("Too many arguments"),
    }
}
