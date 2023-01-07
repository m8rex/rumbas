use crate::support::default::{DefaultFile, DefaultQuestionFileType};
use crate::support::file_manager::CACHE;
use crate::support::rc::within_repo;
use rumbas_support::preamble::{FileToLoad, LoadedFile};
use std::path::Path;
use yaml_subset::{parse_yaml_file, AliasedYaml, Yaml, YamlInsert, YamlPath};

/// Update from version 0.5.* to 0.6.0
pub fn update() -> semver::Version {
    if let Some(root) = within_repo(Path::new(".")) {
        // Add rulesets field in default question file
        let default_files = super::find_default_files(&root);

        let mut default_questions = super::read_default_question_files(&default_files);
        for default_question in &mut default_questions {
            log::info!("Updating {}", default_question.0.file_path.display());
            if default_question.0.file_path.in_main_folder("./defaults") {
                log::info!(
                    "Updating main default file {}",
                    default_question.0.file_path.display()
                );
                let path = YamlPath::Key("rulesets".to_string(), Vec::new(), None);
                let value = AliasedYaml {
                    alias: None,
                    value: Yaml::EmptyInlineHash,
                };
                default_question.1.insert_into_hash(&path, &value, false);
            }
        }

        for (file, default_question) in default_questions.into_iter() {
            let out_str = default_question.format().unwrap();
            std::fs::write(file.file_path, out_str).expect("Failed writing file");
        }

        semver::Version::new(0, 6, 3)
    } else {
        log::error!("Are you in a rumbas repo?");
        panic!("Can't find the rumbas repo");
    }
}
