use crate::support::rc::within_repo;
use std::path::Path;
use yaml_subset::{parse_yaml_file, AliasedYaml, Yaml, YamlInsert, YamlPath};

pub fn update() -> semver::Version {
    if let Some(root) = within_repo(Path::new(".")) {
        // Update exam files
        let mut exams = super::read_all_exams(&root);
        for exam in &mut exams {
            log::info!("Fixing pick_questions in {}", exam.0.file_path.display());
            // Change picking_strategy to an object.
            // Place the old picking_strategy value in the type field of the new struct.
            exam.1.to_object(
                &"question_groups[*].picking_strategy".parse().unwrap(),
                "type".to_string(),
            );
            // Move `pick_questions` to the new struct if it is there.
            exam.1.move_to_subfield(
                &"question_groups[*]".parse().unwrap(),
                "picking_strategy".to_string(),
                vec!["pick_questions".to_string()],
            );
        }
        super::write_files(exams);

        // Update question files
        let default_files = super::find_default_files(&root);
        // TODO: default_files

        let mut questions = super::read_all_questions(&root);
        for question in &mut questions {
            log::info!("Fixing {}", question.0.file_path.display());
            // Remove `minimum_marks` field
            question
                .1
                .remove_from_hash(&"parts[*].minimum_marks".parse().unwrap());
            // Remove `enable_minimum_marks` field
            question
                .1
                .remove_from_hash(&"parts[*].enable_minimum_marks".parse().unwrap());
            // Remove `use_custom_name`
            question
                .1
                .remove_from_hash(&"parts[*].use_custom_name".parse().unwrap());
            // Rename `custom_name` -> `part_name` (this fields can now be set to 'none')
            question.1.rename_field(
                &"parts[*].custom_name".parse().unwrap(),
                "part_name".to_string(),
            );

            // Move following fields (and rename them) to a new field named `custom_marking` (which can be set to 'none')
            // custom_marking_algorithm -> custom_marking.algorithm_notes
            // extend_base_marking_algorithm -> custom_marking.extend_base_marking_algorithm
            question.1.move_to_subfield(
                &"parts[*]".parse().unwrap(),
                "custom_marking".to_string(),
                vec![
                    "custom_marking_algorithm".to_string(),
                    "extend_base_marking_algorithm".to_string(),
                ],
            );
            question.1.rename_field(
                &"parts[*].custom_marking.custom_marking_algorithm"
                    .parse()
                    .unwrap(),
                "algorithm_notes".to_string(),
            );

            // Move following fields (and rename them) to a new field named `adaptive_marking` (which can be set to 'none')
            // variable_replacement_strategy -> adaptive_marking.variable_replacement_strategy
            // adaptive_marking_penalty -> adaptive_marking.penalty
            // Also add a new field adaptive_marking.variable_replacements
            question.1.move_to_subfield(
                &"parts[*]".parse().unwrap(),
                "adaptive_marking".to_string(),
                vec![
                    "variable_replacement_strategy".to_string(),
                    "adaptive_marking_penalty".to_string(),
                ],
            );
            question.1.rename_field(
                &"parts[*].adaptive_marking.adaptive_marking_penalty"
                    .parse()
                    .unwrap(),
                "penalty".to_string(),
            );
            question.1.insert_into_hash(
                &"parts[*].adaptive_marking.variable_replacements"
                    .parse()
                    .unwrap(),
                &AliasedYaml {
                    alias: None,
                    value: Yaml::InlineArray(Vec::new()),
                },
                false,
            );

            // NumberEntry question parts
            // Rename hint_fraction to fractions_must_be_reduced_hint"
            question.1.rename_field(
                &"parts[*].hint_fraction|type=number_entry".parse().unwrap(),
                "fractions_must_be_reduced_hint".to_string(),
            );

            // Pattern match question parts
            // rename partial_credit to wrong_case_partial_credit
            question.1.rename_field(
                &"parts[*].partial_credit|type=pattern_match"
                    .parse()
                    .unwrap(),
                "wrong_case_partial_credit".to_string(),
            );
        }
        super::write_files(questions);
    }

    log::warn!("");
    log::warn!("JME Question parts:");
    log::warn!("Move following fields (and rename them) to a new field named `accuracy`:");
    log::warn!("answer_check -> accuracy.checking_type");
    log::warn!("failure_rate -> accuracy.max_failures");
    log::warn!("vset_range -> accuracy.checking_range");
    log::warn!("vset_range_point -> accuracy.points_to_check");

    semver::Version::new(0, 8, 0)
}
