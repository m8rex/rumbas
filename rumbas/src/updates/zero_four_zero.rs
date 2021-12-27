use crate::support::default::{DefaultExamFileType, DefaultFile, DefaultQuestionFileType};
use crate::support::file_manager::CACHE;
use rumbas_support::preamble::{FileToLoad, LoadedFile, LoadedNormalFile};
use yaml_rust::{yaml::Yaml, YamlEmitter, YamlLoader};

macro_rules! update_hash {
    ($question: expr => $($vec: expr, $method: expr => $([$extra_check: expr])? $(rename [$rename: expr])?):*) => {
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
                                .contains(&k) $(&& $extra_check(&$question))?
                            {
                                let name = k;
                                $(let name = $rename(&name);)?
                                (
                                        name,
                                        $method(v)
                                )
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
        .chain(CACHE.read_all_question_templates().into_iter())
        .filter_map(|lf| match lf {
            rumbas_support::input::LoadedFile::Normal(n) => Some(n),
            _ => None,
        })
        .collect();
    let exam_files: Vec<_> = CACHE
        .read_all_exams()
        .into_iter()
        .chain(CACHE.read_all_exam_templates().into_iter())
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

    let mut exams: Vec<_> = exam_files
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
            vec!["advice", "statement"], update_translatable_string => :
            vec!["diagnostic_topic_names"], update_translatable_string_vector => :
            vec!["functions"], update_functions => :
            vec!["variables"], update_translatable_string_vector => :
            vec!["parts"], update_parts =>
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

    for exam_idx in 0..exams.len() {
        let exam = &exams[exam_idx];
        log::info!("Updating {}", exam.0.file_path.display());
        let new_exam = update_hash!(exam.1.clone() =>
            vec!["name"], update_translatable_string => :
            vec!["timing"], update_timing => :
            vec!["feedback"], update_feedback => :
            vec!["question_groups"], update_question_groups =>
        );
        let new_exam = if new_exam["type"] == Yaml::String("diagnostic".to_string()) {
            update_hash!(new_exam =>
                vec!["diagnostic"], update_diagnostic =>
            )
        } else {
            new_exam
        };
        exams[exam_idx].1 = new_exam;
    }

    for (file, exam) in exams.into_iter() {
        let mut out_str = String::new();
        {
            let mut emitter = YamlEmitter::new(&mut out_str);
            emitter.dump(&exam).unwrap(); // dump the YAML object to a String
        }
        std::fs::write(file.file_path, out_str).expect("Failed writing file");
    }

    let default_files = crate::support::file_manager::CACHE
        .find_default_folders()
        .into_iter()
        .flat_map(|folder| crate::support::file_manager::CACHE.read_folder(&folder.path()))
        .filter_map(|entry| match entry {
            crate::support::file_manager::RumbasRepoEntry::Folder(_) => None,
            crate::support::file_manager::RumbasRepoEntry::File(f) => Some(f),
        })
        .collect::<Vec<_>>();

    let mut default_questions: Vec<_> = default_files
        .iter()
        .filter_map(|file| <DefaultFile<DefaultQuestionFileType>>::from_path(&file.path()))
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

    for default_question_idx in 0..default_questions.len() {
        let default_question = &default_questions[default_question_idx];

        log::info!("Updating {}", default_question.0.file_path.display());
        let new_question = update_hash!(default_question.1.clone() =>
            vec!["advice", "statement"], update_translatable_string => :
            vec!["diagnostic_topic_names"], update_translatable_string_vector => :
            vec!["functions"], update_functions => :
            vec!["variables"], update_translatable_string_vector => :
            vec!["parts"], update_parts =>
        );
        let new_question = if default_question.0.file_path.starts_with("./defaults") {
            // TODO
            log::info!(
                "Updating main default file {}",
                default_question.0.file_path.display()
            );
            update_hash!(new_question =>
            vec!["extensions"], update_extensions =>
            )
        } else {
            new_question
        };
        default_questions[default_question_idx].1 = new_question;
    }

    for (file, default_question) in default_questions.into_iter() {
        let mut out_str = String::new();
        {
            let mut emitter = YamlEmitter::new(&mut out_str);
            emitter.dump(&default_question).unwrap(); // dump the YAML object to a String
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
            let placeholders = placeholders
                .into_iter()
                .map(|(p, v)| {
                    (
                        Yaml::String(remove_first_and_last(&p[..]).to_string()),
                        update_translatable_string_placeholder(v.clone()),
                    )
                })
                .collect();

            let new_yaml = if locales.len() > 0 {
                Yaml::Hash(
                    vec![
                        (
                            Yaml::String("content".to_string()),
                            Yaml::Hash(
                                locales
                                    .into_iter()
                                    .map(|(s, v)| (Yaml::String(s.to_string()), v.clone()))
                                    .collect(),
                            ),
                        ),
                        (
                            Yaml::String("placeholders".to_string()),
                            Yaml::Hash(placeholders),
                        ),
                    ]
                    .into_iter()
                    .collect(),
                )
            } else {
                match content[0].1.clone() {
                    Yaml::Hash(_) => update_translatable_string(content[0].1.clone()),
                    _ => Yaml::Hash(
                        vec![
                            (Yaml::String("content".to_string()), content[0].1.clone()),
                            (
                                Yaml::String("placeholders".to_string()),
                                Yaml::Hash(placeholders),
                            ),
                        ]
                        .into_iter()
                        .collect(),
                    ),
                }
            };
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
                (
                    Yaml::String("placeholders".to_string()),
                    Yaml::Hash(Vec::new().into_iter().collect()),
                ),
            ]
            .into_iter()
            .collect(),
        ),
        _ => yaml,
    }
}

fn update_functions(yaml: Yaml) -> Yaml {
    match yaml {
        Yaml::Array(v) => Yaml::Array(
            v.into_iter()
                .map(|f| {
                    update_hash!(f.clone() =>
                        vec!["definition"], update_translatable_string =>
                    )
                })
                .collect(),
        ),
        _ => yaml,
    }
}

fn update_parts(yaml: Yaml) -> Yaml {
    match yaml {
        Yaml::Array(v) => Yaml::Array(
            v.into_iter()
                .map(|f| {
                    remove_fields(update_hash!(f.clone() =>
                        vec!["prompt"], update_translatable_string =>:
                        vec!["custom_marking_algorithm"], update_jme_notes => rename [|_name: &Yaml| Yaml::String("custom_marking_algorithm_notes".to_string())]: // TODO: parse the string
                        vec!["gaps"], update_parts =>:
                        vec!["steps"], update_parts => :
                        vec!["pattern", "display_answer"], update_translatable_string => [|hash: &Yaml| hash["type"] == Yaml::String("pattern_match".to_string())]:
                        vec!["answer"], update_translatable_string => [|hash: &Yaml| hash["type"] == Yaml::String("jme".to_string())]:
                        vec!["max_length", "min_length", "must_have", "may_not_have", "must_match_pattern"], update_jme_restriction => [|hash: &Yaml| hash["type"] == Yaml::String("jme".to_string())]:
                        vec!["answers", "answer_data"], update_choose_answer_data => [|hash: &Yaml| hash["type"] == Yaml::String("choose_one".to_string()) || hash["type"] == Yaml::String("choose_multiple".to_string())] rename [|_name: &Yaml| Yaml::String("answer_data".to_string())]:
                        vec!["display"], |yaml: Yaml| update_choose_one_display(f["columns"].clone(), yaml) => [|hash: &Yaml| hash["type"] == Yaml::String("choose_one".to_string())]:
                        vec!["answers", "answer_data"], update_match_answer_data => [|hash: &Yaml| hash["type"] == Yaml::String("match_answers".to_string())] rename [|_name: &Yaml| Yaml::String("answer_data".to_string())]
                    ), vec!["columns"])
                })
                .collect(),
        ),
        _ => yaml,
    }
}

fn update_jme_notes(yaml: Yaml) -> Yaml {
    match yaml {
        Yaml::Array(v) => Yaml::Array(
            v.into_iter()
                .map(|f| {
                    update_hash!(f.clone() =>
                        vec!["expression"], update_translatable_string =>
                    )
                })
                .collect(),
        ),
        _ => yaml,
    }
}

fn update_jme_restriction(yaml: Yaml) -> Yaml {
    update_hash!(yaml =>
        vec!["message"], update_translatable_string =>:
        vec!["strings"], update_translatable_string_vector =>
    )
}

fn update_choose_answer_data(yaml: Yaml) -> Yaml {
    match yaml {
        // Array means item based
        Yaml::Array(v) => Yaml::Array(
            v.into_iter()
                .map(|f| {
                    update_hash!(f.clone() =>
                        vec!["statement", "feedback", "marks"], update_translatable_string =>
                    )
                })
                .collect(),
        ),
        Yaml::Hash(h) => update_hash!(Yaml::Hash(h.clone()) =>
            vec!["answers", "feedback", "marks"], update_translatable_string_vector =>
        ),
        _ => yaml,
    }
}

fn update_choose_one_display(columns: Yaml, yaml: Yaml) -> Yaml {
    match yaml {
        Yaml::String(v) => Yaml::Hash(
            vec![
                (Yaml::String("type".to_string()), Yaml::String(v)),
                (Yaml::String("columns".to_string()), columns),
            ]
            .into_iter()
            .collect(),
        ),
        _ => yaml,
    }
}

fn update_match_answer_data(yaml: Yaml) -> Yaml {
    match yaml {
        Yaml::Hash(h) => update_hash!(Yaml::Hash(h.clone()) =>
            vec!["answers", "choices"], update_translatable_string_vector => [|hash: &Yaml| hash["type"] == Yaml::String("numbas_like".to_string())]:
            vec!["answers"], update_translatable_string_vector => [|hash: &Yaml| hash["type"] == Yaml::String("item_based".to_string())]:
            vec!["items"], update_match_items => [|hash: &Yaml| hash["type"] == Yaml::String("item_based".to_string())]
        ),
        _ => yaml,
    }
}

fn update_match_items(yaml: Yaml) -> Yaml {
    match yaml {
        Yaml::Array(v) => Yaml::Array(
            v.into_iter()
                .map(|f| {
                    update_hash!(f.clone() =>
                        vec!["statement"], update_translatable_string =>:
                        vec!["answer_marks"], update_match_item_marks =>
                    )
                })
                .collect(),
        ),
        _ => yaml,
    }
}

fn update_match_item_marks(yaml: Yaml) -> Yaml {
    match yaml {
        Yaml::Array(v) => Yaml::Array(
            v.into_iter()
                .map(|f| {
                    update_hash!(f.clone() =>
                        vec!["answer"], update_translatable_string =>
                    )
                })
                .collect(),
        ),
        _ => yaml,
    }
}

fn update_timing(yaml: Yaml) -> Yaml {
    update_hash!(yaml =>
        vec!["on_timeout", "timeout_waring"], update_timeout_action =>
    )
}

fn update_timeout_action(yaml: Yaml) -> Yaml {
    update_hash!(yaml =>
        vec!["message"], update_translatable_string => [|hash: &Yaml| hash["action"] == Yaml::String("warn".to_string())]
    )
}

fn update_diagnostic(yaml: Yaml) -> Yaml {
    update_hash!(yaml =>
        vec!["script"], update_diagnostic_script =>:
        vec!["objectives"], update_diagnostic_objectives =>:
        vec!["topics"], update_diagnostic_topics =>
    )
}

fn update_diagnostic_script(yaml: Yaml) -> Yaml {
    yaml // TODO
}

fn update_diagnostic_objectives(yaml: Yaml) -> Yaml {
    match yaml {
        Yaml::Array(v) => Yaml::Array(
            v.into_iter()
                .map(|f| {
                    update_hash!(f.clone() =>
                        vec!["name", "description"], update_translatable_string =>
                    )
                })
                .collect(),
        ),
        _ => yaml,
    }
}

fn update_diagnostic_topics(yaml: Yaml) -> Yaml {
    match yaml {
        Yaml::Array(v) => Yaml::Array(
            v.into_iter()
                .map(|f| {
                    update_hash!(f.clone() =>
                        vec!["name", "description"], update_translatable_string =>:
                        vec!["objectives", "depends_on"], update_translatable_string_vector =>
                    )
                })
                .collect(),
        ),
        _ => yaml,
    }
}

fn update_feedback(yaml: Yaml) -> Yaml {
    update_hash!(yaml =>
        vec!["advice", "intro"], update_translatable_string =>
    )
}

fn update_question_groups(yaml: Yaml) -> Yaml {
    match yaml {
        Yaml::Array(v) => Yaml::Array(
            v.into_iter()
                .map(|f| {
                    update_hash!(f.clone() =>
                        vec!["name"], update_translatable_string =>
                    )
                })
                .collect(),
        ),
        _ => yaml,
    }
}

fn update_extensions(yaml: Yaml) -> Yaml {
    match yaml {
        Yaml::Hash(h) => Yaml::Hash(
            h.clone()
                .into_iter()
                .chain(vec![(Yaml::String("sqlite".to_string()), Yaml::Boolean(false))].into_iter())
                .collect(),
        ),
        _ => yaml,
    }
}
fn remove_fields(yaml: Yaml, fields: Vec<&str>) -> Yaml {
    let fields = fields
        .into_iter()
        .map(|a| Yaml::String(a.to_string()))
        .collect::<Vec<_>>();
    if let Some(hash) = yaml.as_hash() {
        Yaml::Hash(
            hash.to_owned()
                .into_iter()
                .filter(|(k, _)| !fields.contains(k))
                .collect(),
        )
    } else {
        yaml
    }
}

fn remove_first_and_last(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str()
}
