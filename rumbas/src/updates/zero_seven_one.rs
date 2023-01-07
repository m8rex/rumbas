use crate::support::rc::within_repo;
use std::path::Path;
use yaml_subset::{parse_yaml_file, AliasedYaml, Yaml, YamlInsert, YamlPath};

pub fn update() -> semver::Version {
    if let Some(root) = within_repo(Path::new(".")) {
        let default_files = super::find_default_files(&root);

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
    }

    log::warn!("");
    log::warn!("Question parts:");
    log::warn!("Remove `minimum_marks` field");
    log::warn!("Remove `enable_minimum_marks` field");
    log::warn!("Remove `use_custom_name` field");
    log::warn!("Rename `custom_name` -> `part_name` (this fields can now be set to 'none')");
    log::warn!("Move following fields (and rename them) to a new field named `adaptive_marking` (which can be set to 'none')");
    log::warn!("variable_replacement_strategy -> adaptive_marking.variable_replacement_strategy");
    log::warn!("adaptive_marking_penalty -> adaptive_marking.penalty");
    log::warn!("Also add a new field adaptive_marking.variable_replacements");

    log::warn!("Move following fields (and rename them) to a new field named `custom_marking` (which can be set to 'none')");
    log::warn!("custom_marking_algorithm -> custom_marking.algorithm_notes");
    log::warn!("extend_base_marking_algorithm -> custom_marking.extend_base_marking_algorithm");

    log::warn!("");
    log::warn!("JME Question parts:");
    log::warn!("Move following fields (and rename them) to a new field named `accuracy`:");
    log::warn!("answer_check -> accuracy.checking_type");
    log::warn!("failure_rate -> accuracy.max_failures");
    log::warn!("vset_range -> accuracy.checking_range");
    log::warn!("vset_range_point -> accuracy.points_to_check");

    log::warn!("");
    log::warn!("NumberEntry Question parts:");
    log::warn!("hint_fraction -> fractions_must_be_reduced_hint");

    log::warn!("");
    log::warn!("Pattern Match Question parts:");
    log::warn!("partial_credit -> wrong_case_partial_credit");

    semver::Version::new(0, 8, 0)
}
