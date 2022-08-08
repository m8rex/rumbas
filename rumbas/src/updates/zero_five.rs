use crate::support::default::{DefaultExamFileType, DefaultFile, DefaultQuestionFileType};
use crate::support::file_manager::CACHE;
use crate::support::noneable::Noneable;
use crate::support::to_rumbas::ToRumbas;
use rumbas_support::preamble::{FileToLoad, LoadedFile, LoadedNormalFile};
use std::convert::TryFrom;
use yaml_rust::{yaml::Yaml, YamlEmitter, YamlLoader};

/// Update from version 0.5.* to 0.6.0
pub fn update() -> semver::Version {
    // Add rulesets field in default question file
    let default_files = crate::support::file_manager::CACHE
        .find_default_folders()
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
                    .map(|lf| match lf {
                        LoadedFile::Normal(n) => Some(n),
                        _ => None,
                    })
                    .flatten()
                    .map(|lf| {
                        YamlLoader::load_from_str(&lf.content[..])
                            .ok()
                            .map(|a| (lf.clone(), a[0].clone()))
                    })
                    .flatten()
            }
            _ => None,
        })
        .collect();

    for default_question in &mut default_questions {
        log::info!("Updating {}", default_question.0.file_path.display());
        if default_question.0.file_path.starts_with("./defaults") {
            log::info!(
                "Updating main default file {}",
                default_question.0.file_path.display()
            );
            if let Yaml::Hash(h) = default_question.1.clone() {
                default_question.1 = Yaml::Hash(
                    h.into_iter()
                        .chain(
                            vec![(
                                Yaml::String("rulesets".to_string()),
                                Yaml::Hash(Vec::new().into_iter().collect()),
                            )]
                            .into_iter(),
                        )
                        .collect(),
                )
            };
        }
    }

    for (file, default_question) in default_questions.into_iter() {
        let mut out_str = String::new();
        {
            let mut emitter = YamlEmitter::new(&mut out_str);
            emitter.multiline_strings(true);
            emitter.dump(&default_question).unwrap(); // dump the YAML object to a String
        }
        std::fs::write(file.file_path, out_str).expect("Failed writing file");
    }

    semver::Version::new(0, 6, 1)
}
