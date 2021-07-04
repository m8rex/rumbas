use rumbas::data::default::combine_with_default_files;
use rumbas::data::to_numbas::ToNumbas;
use std::env;
use std::path::Path;
#[macro_use]
extern crate clap;
use clap::{crate_version, App};

/// The name of the local folder used as cache
/// It caches the .exam files that are given to Numbas.
const CACHE_FOLDER: &'static str = ".rumbas";

/// The name of the local folder used for the output.
const OUTPUT_FOLDER: &'static str = "_output";

/// The main cli function
/// # Requirements
/// Make sure `NUMBAS_FOLDER` is set
/// # Usage
/// `rumbas compile <path_to_exam_or_question>`
/// will compile a rumbas exam or question to HTML.
///
/// The `_output` folder will contain the generated HTML files.
/// Host these with a webserver.
fn main() {
    let numbas_path = env::var("NUMBAS_FOLDER").expect("NUMBAS_FOLDER to be set");
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).version(crate_version!()).get_matches();

    let path = Path::new(matches.value_of("EXAM_OR_QUESTION_PATH").unwrap());
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

    //TODO: logging
    /*let log_level = match matches.occurrences_of("v") {
        0 => 0,
        1 => 1,
        2 => 2,
        _ => 3,
    };*/

    let exam_result = rumbas::data::exam::Exam::from_file(path);
    match exam_result {
        Ok(mut exam) => {
            combine_with_default_files(path, &mut exam);
            if exam.locales().0.is_none() {
                eprintln!("Locales not set!");
            } else {
                for locale_item in exam.locales().clone().unwrap().iter() {
                    let locale = locale_item.clone().unwrap().name.unwrap();
                    let numbas = exam.to_numbas(&locale);
                    match numbas {
                        Ok(res) => {
                            let numbas_exam_name = path.with_extension("exam");
                            let numbas_exam_path = Path::new(CACHE_FOLDER)
                                .join(&locale) //TODO, in filename?
                                .join(&numbas_exam_name);
                            std::fs::create_dir_all(numbas_exam_path.parent().unwrap())
                                .expect("Failed to create cache folders");
                            let locale_folder_path = Path::new(OUTPUT_FOLDER).join(&locale);
                            std::fs::create_dir_all(&locale_folder_path)
                                .expect("Failed to create output locale folder fath");
                            let absolute_path = locale_folder_path
                                .canonicalize()
                                .unwrap()
                                .join(path.with_extension(output_extension));
                            let numbas_output_path = if output_extension == "" {
                                // Remove current folder
                                std::fs::remove_dir_all(&absolute_path).unwrap_or(()); //If error, don't mind
                                                                                       // Create folder
                                std::fs::create_dir_all(&absolute_path)
                                    .expect("Failed creating folder for output");
                                absolute_path
                            } else {
                                std::fs::create_dir_all(&absolute_path.parent().unwrap())
                                    .expect("Failed creating folder for output");
                                absolute_path
                            };
                            //println!("{}", numbas_output_path.display());

                            res.write(&numbas_exam_path.to_str().unwrap());

                            let numbas_settings = exam.numbas_settings().clone().unwrap();

                            let output = std::process::Command::new("python3")
                                .current_dir(numbas_path.clone())
                                .arg("bin/numbas.py")
                                .arg("-l")
                                //TODO.arg(&numbas_settings.locale.unwrap().to_str())
                                .arg(locale_item.unwrap().numbas_locale.clone().unwrap().to_str())
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
        }
        Err(e) => {
            println!("{}", e);
        }
    };
}
