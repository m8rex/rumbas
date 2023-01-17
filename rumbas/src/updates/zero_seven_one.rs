use crate::{question::variable::UNGROUPED_GROUP, support::rc::within_repo};
use std::{path::Path, process::exit};
use yaml_subset::yaml::{parse_yaml_file, AliasedYaml, Yaml, YamlInsert};
use yaml_subset::YamlPath;

#[derive(Debug, Clone)]
pub enum YamlChangeAction {
    ToObject(YamlPath, String),
    MoveToSubfield(YamlPath, String, Vec<String>),
    RemoveFromHash(YamlPath),
    RenameField(YamlPath, String),
    InsertIntoHash(YamlPath, AliasedYaml, bool),
    MoveToMapWithFieldAsKey(YamlPath, String, String, String, Vec<String>),
}

impl YamlChangeAction {
    fn exec<T: YamlInsert>(&self, object: &mut T) -> usize {
        match self {
            Self::ToObject(a, b) => object.to_object(&a, b.clone()),
            Self::MoveToSubfield(a, b, c) => object.move_to_subfield(&a, b.clone(), c.clone()),
            Self::RemoveFromHash(a) => object.remove_from_hash(&a),
            Self::RenameField(a, b) => object.rename_field(&a, b.clone()),
            Self::InsertIntoHash(a, b, c) => object.insert_into_hash(&a, &b, *c),
            Self::MoveToMapWithFieldAsKey(a, b, c, d, e) => {
                object.move_to_map_with_field_as_key(&a, b.clone(), c.clone(), d.clone(), e.clone())
            }
        }
    }
    fn within(self, mut begin: YamlPath) -> Self {
        match self {
            Self::ToObject(a, b) => {
                begin.insert(a);
                Self::ToObject(begin, b)
            }
            Self::MoveToSubfield(a, b, c) => {
                begin.insert(a);
                Self::MoveToSubfield(begin, b, c)
            }
            Self::RemoveFromHash(a) => {
                begin.insert(a);
                Self::RemoveFromHash(begin)
            }
            Self::RenameField(a, b) => {
                begin.insert(a);
                Self::RenameField(begin, b)
            }
            Self::InsertIntoHash(a, b, c) => {
                begin.insert(a);
                Self::InsertIntoHash(begin, b, c)
            }
            Self::MoveToMapWithFieldAsKey(a, b, c, d, e) => {
                begin.insert(a);
                Self::MoveToMapWithFieldAsKey(begin, b, c, d, e)
            }
        }
    }
}

macro_rules! update_default_part {
    ($default_files: expr, $failures: expr, $method: ident, $all: ident, $vec: ident) => {{
        let mut defaults = super::$method(&$default_files);
        for question in &mut defaults {
            log::info!("Fixing {}", question.0.file_path.display());
            match question.1.as_mut() {
                Ok(q) => {
                    for action in $all.iter() {
                        action.exec(q);
                    }
                    for action in $vec.iter() {
                        action.exec(q);
                    }
                }
                Err(e) => {
                    log::error!("Error parsing {}: {}", question.0.file_path.display(), e);
                    *$failures += 1;
                }
            }
        }
        defaults
    }};
}

pub fn update() -> semver::Version {
    if let Some(root) = within_repo(Path::new(".")) {
        let mut failed_files = 0;
        let mut files_to_write = Vec::new();

        let exam_actions = vec![
            // Change picking_strategy to an object.
            // Place the old picking_strategy value in the type field of the new struct.
            YamlChangeAction::ToObject(
                "question_groups[*].picking_strategy".parse().unwrap(),
                "type".to_string(),
            ),
            // Move `pick_questions` to the new struct if it is there.
            YamlChangeAction::MoveToSubfield(
                "question_groups[*]".parse().unwrap(),
                "picking_strategy".to_string(),
                vec!["pick_questions".to_string()],
            ),
        ];

        let question_part_actions = vec![
            // Remove `minimum_marks` field
            YamlChangeAction::RemoveFromHash("minimum_marks".parse().unwrap()),
            // Remove `enable_minimum_marks` field
            YamlChangeAction::RemoveFromHash("enable_minimum_marks".parse().unwrap()),
            // Remove `use_custom_name`
            YamlChangeAction::RemoveFromHash("use_custom_name".parse().unwrap()),
            // Rename `custom_name` -> `part_name` (this fields can now be set to 'none')
            YamlChangeAction::RenameField("custom_name".parse().unwrap(), "part_name".to_string()),
            // Rename `custom_name` -> `part_name` (this fields can now be set to 'none')
            YamlChangeAction::RenameField("custom_name".parse().unwrap(), "part_name".to_string()),
            // Move following fields (and rename them) to a new field named `custom_marking` (which can be set to 'none')
            // custom_marking_algorithm_notes -> custom_marking.algorithm_notes
            // extend_base_marking_algorithm -> custom_marking.extend_base_marking_algorithm
            YamlChangeAction::MoveToSubfield(
                "".parse().unwrap(),
                "custom_marking".to_string(),
                vec![
                    "custom_marking_algorithm_notes".to_string(),
                    "extend_base_marking_algorithm".to_string(),
                ],
            ),
            YamlChangeAction::RenameField(
                "custom_marking.custom_marking_algorithm_notes"
                    .parse()
                    .unwrap(),
                "algorithm_notes".to_string(),
            ),
            // Move following fields (and rename them) to a new field named `adaptive_marking` (which can be set to 'none')
            // variable_replacement_strategy -> adaptive_marking.variable_replacement_strategy
            // adaptive_marking_penalty -> adaptive_marking.penalty
            // Also add a new field adaptive_marking.variable_replacements
            YamlChangeAction::MoveToSubfield(
                "".parse().unwrap(),
                "adaptive_marking".to_string(),
                vec![
                    "variable_replacement_strategy".to_string(),
                    "adaptive_marking_penalty".to_string(),
                ],
            ),
            YamlChangeAction::RenameField(
                "adaptive_marking.adaptive_marking_penalty".parse().unwrap(),
                "penalty".to_string(),
            ),
            YamlChangeAction::InsertIntoHash(
                "adaptive_marking.variable_replacements".parse().unwrap(),
                AliasedYaml {
                    alias: None,
                    value: Yaml::InlineArray(Vec::new()),
                },
                false,
            ),
        ];

        let question_part_actions_jme = vec![
            // JME question parts
            // Move following fields (and rename them) to a new field named `accuracy`:
            // answer_check -> accuracy.checking_type
            // failure_rate -> accuracy.max_failures
            // vset_range -> accuracy.checking_range
            // vset_range_point -> accuracy.points_to_check
            YamlChangeAction::MoveToSubfield(
                "".parse().unwrap(),
                "accuracy".to_string(),
                vec![
                    "answer_check".to_string(),
                    "failure_rate".to_string(),
                    "vset_range".to_string(),
                    "vset_range_points".to_string(),
                ],
            ),
            // answer_check -> accuracy.checking_type
            YamlChangeAction::RenameField(
                "accuracy.answer_check".parse().unwrap(),
                "checking_type".to_string(),
            ),
            // failure_rate -> accuracy.max_failures
            YamlChangeAction::RenameField(
                "accuracy.failure_rate".parse().unwrap(),
                "max_failures".to_string(),
            ),
            // vset_range -> accuracy.checking_range
            YamlChangeAction::RenameField(
                "accuracy.vset_range".parse().unwrap(),
                "checking_range".to_string(),
            ),
            // vset_range_point -> accuracy.points_to_check
            YamlChangeAction::RenameField(
                "accuracy.vset_range_points".parse().unwrap(),
                "points_to_check".to_string(),
            ),
        ];

        let question_part_actions_gapfill: Vec<YamlChangeAction> = vec![];
        let question_part_actions_choose_one: Vec<YamlChangeAction> = vec![];
        let question_part_actions_choose_multiple: Vec<YamlChangeAction> = vec![];
        let question_part_actions_match_answers: Vec<YamlChangeAction> = vec![];
        let question_part_actions_matrix: Vec<YamlChangeAction> = vec![];
        let question_part_actions_number_entry: Vec<YamlChangeAction> = vec![
            // NumberEntry question parts
            // Rename hint_fraction to fractions_must_be_reduced_hint"
            YamlChangeAction::RenameField(
                "hint_fraction".parse().unwrap(),
                "fractions_must_be_reduced_hint".to_string(),
            ),
        ];
        let question_part_actions_pattern_match = vec![
            // Pattern match question parts
            // rename partial_credit to wrong_case_partial_credit
            YamlChangeAction::RenameField(
                "partial_credit".parse().unwrap(),
                "wrong_case_partial_credit".to_string(),
            ),
        ];
        let question_part_actions_information: Vec<YamlChangeAction> = vec![];
        let question_part_actions_extension: Vec<YamlChangeAction> = vec![];

        let mut question_actions = vec![YamlChangeAction::MoveToMapWithFieldAsKey(
            "".parse().unwrap(),
            "variables".to_string(),
            "group".to_string(),
            "grouped_variables".to_string(),
            vec!["".to_string(), UNGROUPED_GROUP.to_string()],
        )];
        for action in question_part_actions.clone().into_iter() {
            question_actions.push(action.clone().within("parts[*]".parse().unwrap()));
            question_actions.push(
                action
                    .clone()
                    .within("parts[*]|type=gapfill.gaps[*]".parse().unwrap()),
            );
            question_actions.push(action.within("parts[*].steps[*]".parse().unwrap()));
        }
        for action in question_part_actions_jme.clone().into_iter() {
            question_actions.push(action.clone().within("parts[*]|type=jme".parse().unwrap()));
            question_actions.push(
                action
                    .clone()
                    .within("parts[*]|type=gapfill.gaps[*]|type=jme".parse().unwrap()),
            );
            question_actions.push(action.within("parts[*].steps[*]|type=jme".parse().unwrap()));
        }
        for action in question_part_actions_gapfill.clone().into_iter() {
            question_actions.push(action.within("parts[*]|type=gapfill".parse().unwrap()));
        }
        for action in question_part_actions_choose_one.clone().into_iter() {
            question_actions.push(
                action
                    .clone()
                    .within("parts[*]|type=choose_one".parse().unwrap()),
            );
            question_actions.push(
                action.clone().within(
                    "parts[*]|type=gapfill.gaps[*]|type=choose_one"
                        .parse()
                        .unwrap(),
                ),
            );
            question_actions
                .push(action.within("parts[*].steps[*]|type=choose_one".parse().unwrap()));
        }
        for action in question_part_actions_choose_multiple.clone().into_iter() {
            question_actions.push(
                action
                    .clone()
                    .within("parts[*]|type=choose_multiple".parse().unwrap()),
            );
            question_actions.push(
                action.clone().within(
                    "parts[*]|type=gapfill.gaps[*]|type=choose_multiple"
                        .parse()
                        .unwrap(),
                ),
            );
            question_actions
                .push(action.within("parts[*].steps[*]|type=choose_multiple".parse().unwrap()));
        }
        for action in question_part_actions_match_answers.clone().into_iter() {
            question_actions.push(
                action
                    .clone()
                    .within("parts[*]|type=match_answers".parse().unwrap()),
            );
            question_actions.push(
                action.clone().within(
                    "parts[*]|type=gapfill.gaps[*]|type=match_answers"
                        .parse()
                        .unwrap(),
                ),
            );
            question_actions
                .push(action.within("parts[*].steps[*]|type=match_answers".parse().unwrap()));
        }
        for action in question_part_actions_matrix.clone().into_iter() {
            question_actions.push(
                action
                    .clone()
                    .within("parts[*]|type=matrix".parse().unwrap()),
            );
            question_actions.push(
                action
                    .clone()
                    .within("parts[*]|type=gapfill.gaps[*]|type=matrix".parse().unwrap()),
            );
            question_actions.push(action.within("parts[*].steps[*]|type=matrix".parse().unwrap()));
        }
        for action in question_part_actions_number_entry.clone().into_iter() {
            question_actions.push(
                action
                    .clone()
                    .within("parts[*]|type=number_entry".parse().unwrap()),
            );
            question_actions.push(
                action.clone().within(
                    "parts[*]|type=gapfill.gaps[*]|type=number_entry"
                        .parse()
                        .unwrap(),
                ),
            );
            question_actions
                .push(action.within("parts[*].steps[*]|type=number_entry".parse().unwrap()));
        }
        for action in question_part_actions_pattern_match.clone().into_iter() {
            question_actions.push(
                action
                    .clone()
                    .within("parts[*]|type=pattern_match".parse().unwrap()),
            );
            question_actions.push(
                action.clone().within(
                    "parts[*]|type=gapfill.gaps[*]|type=pattern_match"
                        .parse()
                        .unwrap(),
                ),
            );
            question_actions
                .push(action.within("parts[*].steps[*]|type=pattern_match".parse().unwrap()));
        }
        for action in question_part_actions_information.clone().into_iter() {
            question_actions.push(
                action
                    .clone()
                    .within("parts[*]|type=information".parse().unwrap()),
            );
            question_actions.push(
                action.clone().within(
                    "parts[*]|type=gapfill.gaps[*]|type=information"
                        .parse()
                        .unwrap(),
                ),
            );
            question_actions
                .push(action.within("parts[*].steps[*]|type=information".parse().unwrap()));
        }
        for action in question_part_actions_extension.clone().into_iter() {
            question_actions.push(
                action
                    .clone()
                    .within("parts[*]|type=extension".parse().unwrap()),
            );
            question_actions.push(
                action.clone().within(
                    "parts[*]|type=gapfill.gaps[*]|type=extension"
                        .parse()
                        .unwrap(),
                ),
            );
            question_actions
                .push(action.within("parts[*].steps[*]|type=extension".parse().unwrap()));
        }

        // Update exam files
        let mut exams = super::read_all_exams(&root);
        for exam in &mut exams {
            log::info!("Fixing pick_questions in {}", exam.0.file_path.display());

            match exam.1.as_mut() {
                Ok(q) => {
                    for action in exam_actions.iter() {
                        action.exec(q);
                    }
                }
                Err(e) => {
                    log::error!("Failed parsing in {}: {}", exam.0.file_path.display(), e);
                    failed_files += 1;
                }
            };
        }
        files_to_write.extend(exams.into_iter());

        // Update question files
        let default_files = super::find_default_files(&root);

        files_to_write.extend(update_default_part!(
            default_files,
            &mut failed_files,
            read_default_jme_files,
            question_part_actions,
            question_part_actions_jme
        ));
        files_to_write.extend(update_default_part!(
            default_files,
            &mut failed_files,
            read_default_gapfill_files,
            question_part_actions,
            question_part_actions_gapfill
        ));
        files_to_write.extend(update_default_part!(
            default_files,
            &mut failed_files,
            read_default_choose_one_files,
            question_part_actions,
            question_part_actions_choose_one
        ));
        files_to_write.extend(update_default_part!(
            default_files,
            &mut failed_files,
            read_default_choose_multiple_files,
            question_part_actions,
            question_part_actions_choose_multiple
        ));
        files_to_write.extend(update_default_part!(
            default_files,
            &mut failed_files,
            read_default_match_answers_files,
            question_part_actions,
            question_part_actions_match_answers
        ));
        files_to_write.extend(update_default_part!(
            default_files,
            &mut failed_files,
            read_default_matrix_files,
            question_part_actions,
            question_part_actions_matrix
        ));
        files_to_write.extend(update_default_part!(
            default_files,
            &mut failed_files,
            read_default_number_entry_files,
            question_part_actions,
            question_part_actions_number_entry
        ));
        files_to_write.extend(update_default_part!(
            default_files,
            &mut failed_files,
            read_default_pattern_match_files,
            question_part_actions,
            question_part_actions_pattern_match
        ));
        files_to_write.extend(update_default_part!(
            default_files,
            &mut failed_files,
            read_default_information_files,
            question_part_actions,
            question_part_actions_information
        ));
        files_to_write.extend(update_default_part!(
            default_files,
            &mut failed_files,
            read_default_extension_files,
            question_part_actions,
            question_part_actions_extension
        ));

        let mut default_questions = super::read_default_question_files(&default_files);
        for question in &mut default_questions {
            log::info!("Fixing {}", question.0.file_path.display());
            match question.1.as_mut() {
                Ok(q) => {
                    if question.0.file_path.in_main_folder("defaults") {
                        log::info!(
                            "Updating main default file {}",
                            question.0.file_path.display()
                        );
                        q.insert_into_hash(
                            &"grouped_variables".parse().unwrap(),
                            &AliasedYaml {
                                alias: None,
                                value: Yaml::UnquotedString("none".to_string()),
                            },
                            false,
                        );
                    }
                    for action in question_actions.iter() {
                        action.exec(q);
                    }
                }
                Err(e) => {
                    log::error!("Error parsing {}: {}", question.0.file_path.display(), e);
                    failed_files += 1;
                }
            }
        }
        files_to_write.extend(default_questions.into_iter());

        let mut questions = super::read_all_questions(&root);
        for question in &mut questions {
            log::info!("Fixing {}", question.0.file_path.display());

            match question.1.as_mut() {
                Ok(q) => {
                    for action in question_actions.iter() {
                        action.exec(q);
                    }
                }
                Err(e) => {
                    log::error!("Error parsing {}: {}", question.0.file_path.display(), e);
                    failed_files += 1;
                }
            }
        }
        files_to_write.extend(questions.into_iter());

        if failed_files > 0 {
            log::error!("Failed to parse {} files", failed_files);
            exit(1);
        } else {
            super::write_files(files_to_write);
            return semver::Version::new(0, 8, 0);
        }
    }
    exit(1);
}
