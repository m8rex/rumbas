use crate::support::default::{DefaultFile, DefaultQuestionFileType};
use crate::support::file_manager::CACHE;
use crate::support::rc::within_repo;
use rumbas_support::preamble::{FileToLoad, LoadedFile};
use std::path::Path;
use yaml_rust_formatter::{yaml::YamlInput, YamlEmitter, YamlLoader};

/// Update from version 0.5.* to 0.6.0
pub fn update() -> semver::Version {
    if let Some(root) = within_repo(Path::new(".")) {
        // Add rulesets field in default question file
        let default_files = crate::support::file_manager::CACHE
            .find_default_folders(&root)
            .into_iter()
            .flat_map(|folder| crate::support::file_manager::CACHE.read_folder(&folder.path()))
            .filter_map(|entry| match entry {
                crate::support::file_manager::RumbasRepoEntry::Folder(_) => None,
                crate::support::file_manager::RumbasRepoEntry::File(f) => Some(f),
            })
            .collect::<Vec<_>>();

        let default_question_files: Vec<_> = default_files
            .iter()
            .filter_map(|file| <DefaultFile<DefaultQuestionFileType>>::from_path(&file.path()))
            .collect();
        let mut default_questions: Vec<_> = default_question_files
            .iter()
            .filter_map(|d| match d.get_type() {
                DefaultQuestionFileType::Question => {
                    let lf_opt = CACHE.read_file(FileToLoad {
                        file_path: d.get_path(),
                        locale_dependant: false,
                    });
                    lf_opt
                        .and_then(|lf| match lf {
                            LoadedFile::Normal(n) => Some(n),
                            _ => None,
                        })
                        .and_then(|lf| {
                            YamlLoader::load_from_str(&lf.content[..])
                                .ok()
                                .map(|a| (lf.clone(), a[0].clone()))
                        })
                }
                _ => None,
            })
            .collect();

        for default_question in &mut default_questions {
            log::info!("Updating {}", default_question.0.file_path.display());
            if default_question.0.file_path.in_main_folder("./defaults") {
                log::info!(
                    "Updating main default file {}",
                    default_question.0.file_path.display()
                );
                if let YamlInput::Hash((cs, h)) = default_question.1.clone() {
                    default_question.1 = YamlInput::Hash((
                        cs,
                        h.into_iter()
                            .chain(
                                vec![(
                                    YamlInput::String("rulesets".to_string()),
                                    (
                                        YamlInput::Hash((
                                            Vec::new(),
                                            Vec::new().into_iter().collect(),
                                        )),
                                        Vec::new(),
                                    ),
                                )]
                                .into_iter(),
                            )
                            .collect(),
                    ))
                };
            }
        }

        for (file, default_question) in default_questions.into_iter() {
            let mut out_str = String::new();
            {
                let mut emitter = YamlEmitter::new(&mut out_str);
                emitter.multiline_strings(true);
                emitter.dump(&(default_question.to_owned().into())).unwrap(); // dump the YAML object to a String
            }
            std::fs::write(file.file_path, out_str).expect("Failed writing file");
        }

        semver::Version::new(0, 6, 3)
    } else {
        log::error!("Are you in a rumbas repo?");
        panic!("Can't find the rumbas repo");
    }
}
