use crate::support::file_manager::CACHE;
use yaml_rust::{yaml::Yaml, YamlEmitter, YamlLoader};

/// Update from version 0.4.0 to 0.5.0
pub fn update() -> String {
    // TODO: extract code to read questions and exams
    let question_files = CACHE
        .read_all_questions()
        .into_iter()
        .filter_map(|lf| match lf {
            rumbas_support::input::LoadedFile::Normal(n) => Some(n),
            _ => None,
        });
    let exam_files = CACHE
        .read_all_exams()
        .into_iter()
        .filter_map(|lf| match lf {
            rumbas_support::input::LoadedFile::Normal(n) => Some(n),
            _ => None,
        });

    let questions: Vec<_> = question_files
        .into_iter()
        .filter_map(|lf| YamlLoader::load_from_str(&lf.content[..]).ok())
        .map(|ys| ys[0].clone())
        .collect();

    let exams: Vec<_> = exam_files
        .into_iter()
        .filter_map(|lf| YamlLoader::load_from_str(&lf.content[..]).ok())
        .map(|ys| ys[0].clone())
        .collect();

    "0.5.0".to_string()
}

fn update_translatable_string(yaml: &mut Yaml) {
    match yaml {
        Yaml::Hash(h) => {
            let values: Vec<_> = h
                .into_iter()
                .filter_map(|(k, v)| match k {
                    Yaml::String(s) => Some((s.to_string(), v)),
                    _ => None,
                })
                .collect();
            let (placeholders, other): (Vec<_>, Vec<_>) = values
                .into_iter()
                .partition(|(s, _)| s.starts_with("{") && s.ends_with("}"));
            let (content, locales): (Vec<_>, Vec<_>) = other
                .into_iter()
                .partition(|(s, _)| s == &"content".to_string());
            let content = if locales.len() > 0 {
                (
                    Yaml::String("content".to_string()),
                    Yaml::Hash(
                        locales
                            .into_iter()
                            .map(|(s, v)| (Yaml::String(s.to_string()), v.clone()))
                            .collect(),
                    ),
                )
            } else {
                (Yaml::String("content".to_string()), content[0].1.clone())
            };
            let placeholders = placeholders
                .into_iter()
                .map(|(p, v)| {
                    (
                        Yaml::String(remove_first_and_last(&p[..]).to_string()),
                        v.clone(),
                    )
                })
                .collect();

            let new_yaml = Yaml::Hash(
                vec![
                    content,
                    (
                        Yaml::String("placeholders".to_string()),
                        Yaml::Hash(placeholders),
                    ),
                ]
                .into_iter()
                .collect(),
            );
            *yaml = new_yaml;
        }
        _ => (),
    }
}

fn remove_first_and_last(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str()
}
