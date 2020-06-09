use crate::data::default::combine_with_default_files;
use crate::data::exam::ToNumbas;
use std::env;
use std::path::Path;
mod data;

const CACHE_FOLDER: &'static str = ".rumbas";
const OUTPUT_FOLDER: &'static str = "_output";

fn main() {
    let numbas_path = env::var("NUMBAS_FOLDER").expect("NUMBAS_FOLDER to be set");
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
                    combine_with_default_files(path, &mut exam);
                    println!("{:#?}", exam);
                    for locale_item in exam.locales.clone().unwrap().iter() {
                        let locale = locale_item.clone().name.unwrap(); //TODO?
                        let numbas = exam.to_numbas(&locale);
                        match numbas {
                            Ok(res) => {
                                let numbas_exam_name = path.with_extension("exam");
                                let numbas_exam_path = Path::new(CACHE_FOLDER)
                                    .join(&locale) //TODO, in filename?
                                    .join(&numbas_exam_name);
                                std::fs::create_dir_all(numbas_exam_path.parent().unwrap()); //TODO
                                let numbas_output_path = Path::new(OUTPUT_FOLDER)
                                    .join(&locale)
                                    .join(path.with_extension(""));
                                println!("{}", numbas_output_path.display());
                                let err = std::fs::create_dir_all(&numbas_output_path);
                                eprintln!("{:?}", err);

                                res.write(&numbas_exam_path.to_str().unwrap());

                                let numbas_settings = exam.numbas_settings.clone().unwrap();

                                let output = std::process::Command::new("python")
                                    .current_dir(numbas_path.clone())
                                    .arg("bin/numbas.py")
                                    .arg("-l")
                                    //TODO.arg(&numbas_settings.locale.unwrap().to_str())
                                    .arg(locale_item.numbas_locale.clone().unwrap().to_str())
                                    .arg("-t")
                                    .arg(numbas_settings.theme.unwrap())
                                    .arg("-o")
                                    .arg(numbas_output_path.canonicalize().unwrap())
                                    .arg(numbas_exam_path.canonicalize().unwrap()) //TODO?
                                    .output()
                                    .expect("failed to execute process");
                                if output.stdout.len() > 0 {
                                    println!("{}", std::str::from_utf8(&output.stdout).unwrap());
                                }
                                if output.stderr.len() > 0 {
                                    eprintln!("{}", std::str::from_utf8(&output.stderr).unwrap());
                                }
                            }
                            Err(missing_fields) => {
                                println!("Missing fields:\n{}", missing_fields.join("\n"));
                            }
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
