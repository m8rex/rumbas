use crate::data::default::combine_with_default_files;
use crate::data::to_numbas::ToNumbas;
use std::env;
use std::path::Path;
#[macro_use]
extern crate clap;
use clap::App;
mod data;

const CACHE_FOLDER: &'static str = ".rumbas";
const OUTPUT_FOLDER: &'static str = "_output";

fn main() {
    let numbas_path = env::var("NUMBAS_FOLDER").expect("NUMBAS_FOLDER to be set");
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let path = Path::new(matches.value_of("EXAM_OR_QUESTION").unwrap());
    println!("{:?}", path.display());
    if path.is_absolute() {
        println!("Absolute path's are not supported");
        return;
    }

    let mut extra_args: Vec<&str> = Vec::new();
    if matches.is_present("scorm") {
        extra_args.push("-s");
    }
    let output_extension = if matches.is_present("zip") {
        extra_args.push("-z");
        "zip"
    } else {
        ""
    };

    let log_level = match matches.occurrences_of("v") {
        0 => 0,
        1 => 1,
        2 => 2,
        _ => 3,
    };

    let exam_result = data::exam::Exam::from_file(path);
    match exam_result {
        Ok(mut exam) => {
            combine_with_default_files(path, &mut exam);
            //println!("{:#?}", exam);
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
                        let numbas_output_path = if output_extension == "" {
                            let absolute_path = Path::new(OUTPUT_FOLDER)
                                .join(&locale)
                                .canonicalize()
                                .unwrap()
                                .join(path.with_extension(output_extension));
                            std::fs::remove_dir_all(&absolute_path).unwrap_or(()); //If error, don't mind
                            let err = std::fs::create_dir_all(&absolute_path);
                            eprintln!("{:?}", err);
                            absolute_path
                        } else {
                            let absolute_path = Path::new(OUTPUT_FOLDER)
                                .join(&locale)
                                .canonicalize()
                                .unwrap();
                            let err = std::fs::create_dir_all(&absolute_path);
                            eprintln!("{:?}", err);
                            absolute_path.join(path.with_extension(output_extension))
                        };
                        println!("{}", numbas_output_path.display());

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
                            .args(&extra_args)
                            .arg("-o")
                            .arg(numbas_output_path)
                            .arg(numbas_exam_path.canonicalize().unwrap()) //TODO?
                            .output()
                            .expect("failed to execute numbas process");
                        if output.stdout.len() > 0 {
                            println!("{}", std::str::from_utf8(&output.stdout).unwrap());
                        }
                        if output.stderr.len() > 0 {
                            eprintln!("{}", std::str::from_utf8(&output.stderr).unwrap());
                        }
                    }
                    Err(missing_fields) => {
                        println!(
                            "Missing fields ({}):\n{}",
                            locale,
                            missing_fields.join("\n")
                        );
                    }
                }
            }
        }
        Err(e) => {
            println!("{}", e);
        }
    };
}
