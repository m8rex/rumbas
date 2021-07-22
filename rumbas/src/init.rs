pub fn init(_matches: &clap::ArgMatches) {
    let folders = [
        rumbas::QUESTIONS_FOLDER,
        rumbas::EXAMS_FOLDER,
        rumbas::QUESTION_TEMPLATES_FOLDER,
        rumbas::EXAM_TEMPLATES_FOLDER,
        rumbas::RESOURCES_FOLDER,
        rumbas::DEFAULTS_FOLDER,
        rumbas::THEMES_FOLDER,
        rumbas::CUSTOM_PART_TYPES_FOLDER,
    ];
    let paths = folders
        .iter()
        .map(|f| std::path::Path::new(f))
        .collect::<Vec<_>>();
    let existing_paths = paths.iter().filter(|p| p.exists()).collect::<Vec<_>>();

    if existing_paths.len() > 0 {
        eprintln!(
            "Aborting, some folder do already exists: {}",
            existing_paths
                .iter()
                .map(|p| p.display().to_string())
                .collect::<Vec<_>>()
                .join(", ")
        );
        std::process::exit(1)
    } else {
        for path in paths.iter() {
            std::fs::create_dir(path).expect("Failed creating folder");
        }
    }
}
