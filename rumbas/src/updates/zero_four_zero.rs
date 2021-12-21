use crate::support::file_manager::CACHE;
use yaml_rust::{yaml::Yaml, YamlEmitter, YamlLoader};

macro_rules! update_hash {
    ($question: expr => $($vec: expr, $method: ident):*) => {
        if let Some(question) = $question.as_hash() {
            Yaml::Hash(question
                .to_owned()
                .into_iter()
                .map(|(k, v)| {
                        $(
                            if $vec
                                .into_iter()
                                .map(|a| Yaml::String(a.to_string()))
                                .collect::<Vec<_>>()
                                .contains(&k)
                            {
                                (k, $method(v))
                            } else
                        )*
                        {
                            (k, v)
                        }
                })
                .collect())
        } else {
            $question
        }
    };
}

/// Update from version 0.4.0 to 0.5.0
pub fn update() -> String {
    // TODO: extract code to read questions and exams
    let question_files: Vec<_> = CACHE
        .read_all_questions()
        .into_iter()
        .filter_map(|lf| match lf {
            rumbas_support::input::LoadedFile::Normal(n) => Some(n),
            _ => None,
        })
        .collect();
    let exam_files: Vec<_> = CACHE
        .read_all_exams()
        .into_iter()
        .filter_map(|lf| match lf {
            rumbas_support::input::LoadedFile::Normal(n) => Some(n),
            _ => None,
        })
        .collect();

    let mut questions: Vec<_> = question_files
        .into_iter()
        .filter_map(|lf| {
            YamlLoader::load_from_str(&lf.content[..])
                .ok()
                .map(|a| (lf, a[0].clone()))
        })
        .collect();

    let exams: Vec<_> = exam_files
        .into_iter()
        .filter_map(|lf| {
            YamlLoader::load_from_str(&lf.content[..])
                .map(|a| (lf, a[0].clone()))
                .ok()
        })
        .collect();

    for question_idx in 0..questions.len() {
        let question = &questions[question_idx];
        log::info!("Updating {}", question.0.file_path.display());
        let new_question = update_hash!(question.1.clone() =>
            vec!["advice", "statement"], update_translatable_string:
            vec!["diagnostic_topic_names"], update_translatable_string_vector
        );
        questions[question_idx].1 = new_question;
    }

    for (file, question) in questions.into_iter() {
        let mut out_str = String::new();
        {
            let mut emitter = YamlEmitter::new(&mut out_str);
            emitter.dump(&question).unwrap(); // dump the YAML object to a String
        }
        std::fs::write(file.file_path, out_str).expect("Failed writing file");
    }

    "0.5.0".to_string()
}

fn update_translatable_string(yaml: Yaml) -> Yaml {
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
                        update_translatable_string_placeholder(v.clone()),
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
            new_yaml
        }
        _ => yaml,
    }
}

fn update_translatable_string_vector(yaml: Yaml) -> Yaml {
    match yaml {
        Yaml::Array(v) => Yaml::Array(v.into_iter().map(update_translatable_string).collect()),
        _ => yaml,
    }
}

fn update_translatable_string_placeholder(yaml: Yaml) -> Yaml {
    match yaml {
        Yaml::Hash(_) => update_translatable_string(yaml),
        Yaml::String(_) => Yaml::Hash(
            vec![
                (Yaml::String("content".to_string()), yaml),
                (Yaml::String("placeholders".to_string()), Yaml::Null),
            ]
            .into_iter()
            .collect(),
        ),
        _ => yaml,
    }
}

fn remove_first_and_last(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str()
}
