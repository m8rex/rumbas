pub fn init() {
    let repo_path = std::path::Path::new(".");
    let repo_path = rumbas::support::rc::within_repo(&repo_path);
    if let Some(repo_path) = repo_path {
        log::error!("Aborting, it seems we already are within a rumbas project",);
        std::process::exit(1);
    } else {
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
        let paths = folders.iter().map(std::path::Path::new).collect::<Vec<_>>();
        let file_paths = vec![std::path::Path::new(rumbas::RC_FILE_NAME)];
        let existing_paths = paths
            .iter()
            .chain(file_paths.iter())
            .filter(|p| p.exists())
            .collect::<Vec<_>>();

        if !existing_paths.is_empty() {
            log::error!(
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
            let rc = rumbas::support::rc::RC::default();
            let rumbas_version = crate_version!();
            let rumbas_version = semver::Version::parse(rumbas_version).unwrap();

            let rc = rc.with_version(rumbas_version);
            rc.write().expect("writing of rc file to work.");
        }
    }
}
