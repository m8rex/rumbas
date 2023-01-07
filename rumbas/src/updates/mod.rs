use crate::support::default::DefaultExamFileType;
use crate::support::default::DefaultFile;
use crate::support::default::DefaultQuestionFileType;
use crate::support::file_manager::{RumbasRepoFileData, CACHE};
use crate::support::rc::RC;
use rumbas_support::preamble::*;
use semver::{Version, VersionReq};
use yaml_subset::{parse_yaml_file, Document};

mod zero_five;
mod zero_seven_one;

pub fn update(current_rc: RC) -> Option<RC> {
    let current_version = current_rc.version();
    let new_version = if current_version == Version::new(0, 4, 0) {
        log::error!(
            "This rumbas repo is to old to update with this version. Please use rumbas 0.6.3"
        );
        None
    } else if VersionReq::parse("0.5.*")
        .expect("this to be a valid version requirements")
        .matches(&current_version)
    {
        Some(zero_five::update())
    } else if current_version == Version::new(0, 6, 0)
        || current_version == Version::new(0, 6, 1)
        || current_version == Version::new(0, 6, 2)
    {
        Some(Version::new(0, 6, 3))
    } else if current_version == Version::new(0, 6, 3) {
        log::warn!("You will need to manually update you rumbas repo to 0.7.0");
        log::warn!("Please move all templates files to the regular folders and update all paths.");
        Some(Version::new(0, 7, 0))
    } else if current_version == Version::new(0, 7, 0) {
        Some(Version::new(0, 7, 1))
    } else if current_version == Version::new(0, 7, 1) {
        Some(zero_seven_one::update())
    } else {
        None
    };

    new_version.map(|n| current_rc.with_version(n))
}

fn read_files(v: Vec<LoadedNormalFile>) -> Vec<(LoadedNormalFile, Document)> {
    v.into_iter()
        .filter_map(|lf| {
            parse_yaml_file(&lf.content[..])
                .ok()
                .map(|a| (lf.clone(), a))
        })
        .collect()
}

fn read_all_questions(root: &RumbasPath) -> Vec<(LoadedNormalFile, Document)> {
    let question_files = CACHE
        .read_all_questions(root)
        .into_iter()
        .filter_map(|lf| match lf {
            rumbas_support::input::LoadedFile::Normal(n) => Some(n),
            _ => None,
        })
        .collect();
    read_files(question_files)
}

fn read_all_exams(root: &RumbasPath) -> Vec<(LoadedNormalFile, Document)> {
    let exam_files = CACHE
        .read_all_exams(root)
        .into_iter()
        .filter_map(|lf| match lf {
            rumbas_support::input::LoadedFile::Normal(n) => Some(n),
            _ => None,
        })
        .collect();
    read_files(exam_files)
}

fn find_default_files(root: &RumbasPath) -> Vec<RumbasRepoFileData> {
    crate::support::file_manager::CACHE
        .find_default_folders(&root)
        .into_iter()
        .flat_map(|folder| crate::support::file_manager::CACHE.read_folder(&folder.path()))
        .filter_map(|entry| match entry {
            crate::support::file_manager::RumbasRepoEntry::Folder(_) => None,
            crate::support::file_manager::RumbasRepoEntry::File(f) => Some(f),
        })
        .collect::<Vec<_>>()
}

macro_rules! add_read_default_question_file {
    ($name: ident, $( $enum_item: ident ),* ) => {
        fn $name(default_files: &Vec<RumbasRepoFileData>) -> Vec<(LoadedNormalFile, Document)> {
            let default_question_files: Vec<_> = default_files
                .iter()
                .filter_map(|file| <DefaultFile<DefaultQuestionFileType>>::from_path(&file.path()))
                .collect();
            default_question_files
                .iter()
                .filter_map(|d| match d.get_type() {
                    $(
                    DefaultQuestionFileType::$enum_item => {
                        let lf_opt = CACHE.read_file(FileToLoad {
                            file_path: d.get_path(),
                            locale_dependant: false,
                        });
                        lf_opt
                            .and_then(|lf| match lf {
                                LoadedFile::Normal(n) => Some(n),
                                _ => None,
                            })
                            .and_then(|lf| {
                                parse_yaml_file(&lf.content[..])
                                    .ok()
                                    .map(|a| (lf.clone(), a))
                            })
                    },
                    )*
                    _ => None,
                })
                .collect()
        }
    };
}

add_read_default_question_file!(read_default_question_files, Question);
add_read_default_question_file!(
    read_default_jme_files,
    QuestionPartJME,
    QuestionPartGapFillGapJME
);
add_read_default_question_file!(read_default_gapfill_files, QuestionPartGapFill);
add_read_default_question_file!(
    read_default_choose_one_files,
    QuestionPartChooseOne,
    QuestionPartGapFillGapChooseOne
);
add_read_default_question_file!(
    read_default_choose_multiple_files,
    QuestionPartChooseMultiple,
    QuestionPartGapFillGapChooseMultiple
);
add_read_default_question_file!(
    read_default_match_answers_files,
    QuestionPartMatchAnswersWithItems,
    QuestionPartGapFillGapMatchAnswersWithItems
);
add_read_default_question_file!(
    read_default_number_entry_files,
    QuestionPartNumberEntry,
    QuestionPartGapFillGapNumberEntry
);
add_read_default_question_file!(
    read_default_pattern_match_files,
    QuestionPartPatternMatch,
    QuestionPartGapFillGapPatternMatch
);
add_read_default_question_file!(
    read_default_information_files,
    QuestionPartInformation,
    QuestionPartGapFillGapInformation
);
add_read_default_question_file!(
    read_default_extension_files,
    QuestionPartExtension,
    QuestionPartGapFillGapExtension
);

macro_rules! add_read_default_exam_file {
    ($name: ident, $( $enum_item: ident ),* ) => {
        fn $name(default_files: &Vec<RumbasRepoFileData>) -> Vec<(LoadedNormalFile, Document)> {
            let default_question_files: Vec<_> = default_files
                .iter()
                .filter_map(|file| <DefaultFile<DefaultExamFileType>>::from_path(&file.path()))
                .collect();
            default_question_files
                .iter()
                .filter_map(|d| match d.get_type() {
                    $(
                    DefaultExamFileType::$enum_item => {
                        let lf_opt = CACHE.read_file(FileToLoad {
                            file_path: d.get_path(),
                            locale_dependant: false,
                        });
                        lf_opt
                            .and_then(|lf| match lf {
                                LoadedFile::Normal(n) => Some(n),
                                _ => None,
                            })
                            .and_then(|lf| {
                                parse_yaml_file(&lf.content[..])
                                    .ok()
                                    .map(|a| (lf.clone(), a))
                            })
                    },
                    )*
                    _ => None,
                })
                .collect()
        }
    };
}

add_read_default_exam_file!(
    read_default_sequential_navigation_files,
    SequentialNavigation
);
add_read_default_exam_file!(read_default_menu_navigation_files, MenuNavigation);
add_read_default_exam_file!(
    read_default_diagnostic_navigation_files,
    DiagnosticNavigation
);
add_read_default_exam_file!(read_default_timing_files, Timing);
add_read_default_exam_file!(read_default_feedback_files, Feedback);
add_read_default_exam_file!(read_default_numbas_settings_files, NumbasSettings);

fn write_files(files: Vec<(LoadedNormalFile, Document)>) {
    for (file, default_question) in files.into_iter() {
        let out_str = default_question.format().unwrap();
        std::fs::write(file.file_path, out_str).expect("Failed writing file");
    }
}
