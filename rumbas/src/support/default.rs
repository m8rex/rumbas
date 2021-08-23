use crate::exam::diagnostic::DiagnosticExam;
use crate::exam::navigation::{
    DiagnosticNavigation, MenuNavigation, NormalNavigation, SequentialNavigation,
};
use crate::exam::normal::NormalExam;
use crate::exam::numbas_settings::NumbasSettings;
use crate::exam::timing::Timing;
use crate::exam::Exam;
use crate::question::feedback::Feedback;
use crate::question::part::extension::QuestionPartExtension;
use crate::question::part::gapfill::QuestionPartGapFill;
use crate::question::part::information::QuestionPartInformation;
use crate::question::part::jme::QuestionPartJME;
use crate::question::part::multiple_choice::choose_multiple::QuestionPartChooseMultiple;
use crate::question::part::multiple_choice::choose_one::QuestionPartChooseOne;
use crate::question::part::multiple_choice::match_answers::QuestionPartMatchAnswersWithItems;
use crate::question::part::number_entry::QuestionPartNumberEntry;
use crate::question::part::pattern_match::QuestionPartPatternMatch;
use crate::question::part::question_part::{QuestionPart, QuestionPartBuiltin};
use crate::question::Question;
use crate::support::optional_overwrite::*;
use crate::support::template::{Value, ValueType};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

//TODO Tests
//Questionnavigation?? -> in question?
//

/// Combine an exam with all data from the default files
pub fn combine_with_default_files(path: &Path, exam: &mut Exam) {
    let default_files = default_files(path);
    if let Exam::Normal(ref mut e) = exam {
        handle!(
            default_files,
            e,
            |n: &SequentialNavigation, e: &mut NormalExam| e
                .navigation
                .overwrite(&Value::Normal(NormalNavigation::Sequential(n.clone()))),
            |n: &MenuNavigation, e: &mut NormalExam| e
                .navigation
                .overwrite(&Value::Normal(NormalNavigation::Menu(n.clone()))),
            |_n: &DiagnosticNavigation, _e: &mut NormalExam| ()
        );
    } else if let Exam::Diagnostic(ref mut e) = exam {
        handle!(
            default_files,
            e,
            |_n: &SequentialNavigation, _e: &mut DiagnosticExam| (),
            |_n: &MenuNavigation, _e: &mut DiagnosticExam| (),
            |n: &DiagnosticNavigation, e: &mut DiagnosticExam| e
                .navigation
                .overwrite(&Value::Normal(n.clone()))
        );
    }
}

/// Returns a vector with all DefaultFiles that are found for the given path
fn default_files(path: &Path) -> Vec<DefaultFile> {
    let paths = default_file_paths(path);
    let usefull_paths = paths
        .into_iter()
        .map(|p| DefaultFile::from(&p))
        .filter(|p| p.is_some());
    usefull_paths.map(|p| p.unwrap()).collect()
}

/// Returns a vector of paths to default files for the given path
fn default_file_paths(path: &Path) -> Vec<PathBuf> {
    let mut result = HashSet::new(); //Use set to remove duplicates (only happens for the 'defaults' folder in root
                                     //TODO: write tests and maybe use .take(count()-1) instead of hashset
    let ancestors = path.ancestors();
    for a in ancestors {
        let defaults_path = a.with_file_name(crate::DEFAULTS_FOLDER);
        if defaults_path.is_dir() {
            for entry in defaults_path
                .read_dir()
                .expect("read_dir call failed")
                .flatten()
            // We only care about the ones that are 'Ok'
            {
                result.insert(entry.path()); //TODO: order files from the folder
                                             //println!("{:?}", entry.path());
            }
        }
    }

    result.into_iter().collect::<Vec<PathBuf>>()
}

// Create the needed enum by specifying which files contain which data
create_default_file_type_enums!(
    SequentialNavigation with type SequentialNavigation: in "navigation";
    MenuNavigation with type MenuNavigation: in "navigation.menu";
    DiagnosticNavigation with type DiagnosticNavigation: in "navigation.diagnostic";
    Timing with type Timing: in "timing";
    Feedback with type Feedback: in "feedback";
    NumbasSettings with type NumbasSettings: in "numbas_settings";
    Question with type Question: in "question";
    QuestionPartJME with type QuestionPartJME: in "questionpart.jme";
    QuestionPartGapFill with type QuestionPartGapFill: in "questionpart.gapfill";
    QuestionPartChooseOne with type QuestionPartChooseOne: in "questionpart.choose_one";
    QuestionPartChooseMultiple with type QuestionPartChooseMultiple: in "questionpart.choose_multiple";
    QuestionPartMatchAnswersWithItems with type QuestionPartMatchAnswersWithItems: in "questionpart.match_answers";
    QuestionPartNumberEntry with type QuestionPartNumberEntry: in "questionpart.number_entry";
    QuestionPartPatternMatch with type QuestionPartPatternMatch: in "questionpart.pattern_match";
    QuestionPartInformation with type QuestionPartInformation: in "questionpart.information";
    QuestionPartExtension with type QuestionPartExtension: in "questionpart.extension";
    QuestionPartGapFillGapJME with type QuestionPartJME: in "questionpart.gapfill.gap.jme";
    QuestionPartGapFillGapChooseOne with type QuestionPartChooseOne: in "questionpart.gapfill.gap.choose_one";
    QuestionPartGapFillGapChooseMultiple with type QuestionPartChooseMultiple: in "questionpart.gapfill.gap.choose_multiple";
    QuestionPartGapFillGapMatchAnswersWithItems with type QuestionPartMatchAnswersWithItems: in "questionpart.gapfill.gap.match_answers";
    QuestionPartGapFillGapNumberEntry with type QuestionPartNumberEntry: in "questionpart.gapfill.gap.number_entry";
    QuestionPartGapFillGapPatternMatch with type QuestionPartPatternMatch: in "questionpart.gapfill.gap.pattern_match";
    QuestionPartGapFillGapInformation with type QuestionPartInformation: in "questionpart.gapfill.gap.information";
    QuestionPartGapFillGapExtension with type QuestionPartExtension: in "questionpart.gapfill.gap.extension"
);

#[derive(Debug)]
/// Struct used to overwrite values with defaults
struct DefaultFile {
    r#type: DefaultFileType,
    path: PathBuf,
}

impl DefaultFile {
    /// Create a DefaultFile from the file_name of the given path, returns None if invalid path
    fn from(path: &Path) -> Option<DefaultFile> {
        let default_type: Option<DefaultFileType> = DefaultFileType::from(path);
        if let Some(t) = default_type {
            return Some(DefaultFile {
                r#type: t,
                path: path.to_path_buf(),
            });
        }
        None
    }

    /// Read the given path as the data needed for this DefaultFile
    fn read_as_data(&self) -> serde_yaml::Result<DefaultData> {
        self.r#type.read_as_data(&self.path)
    }

    /// Get the path of this DefaultFile
    fn get_path(&self) -> PathBuf {
        self.path.clone()
    }
}

/// Create the DefaultFileType and DefaultData enums and their methods to read data
macro_rules! create_default_file_type_enums {
    ( $($file_type:ident with type $data_type: ty: in $file_name: literal);* ) => {
        #[derive(Debug)]
        pub enum DefaultFileType {
            $(
                $file_type
            ),*
        }

        pub enum DefaultData {
            $(
                $file_type($data_type)
            ),*
        }

        impl DefaultFileType {
            /// Creates a DefaultFileType based on the filename, returns None if unknown
            fn from(path: &Path) -> Option<DefaultFileType> {
                let file_name = path.file_stem();
                match file_name {
                    Some(f) => match f.to_str() {
                        $(
                            Some($file_name) => Some(DefaultFileType::$file_type),
                        )*
                        _ => None
                    }
                    _ => None
                }
            }

            /// Read the given path as the data needed for this DefaultFileType
            fn read_as_data(&self, path: &Path) -> serde_yaml::Result<DefaultData> {
                let yaml = fs::read_to_string(path).unwrap();
                match self {
                    $(
                    DefaultFileType::$file_type => {
                        let n: $data_type = serde_yaml::from_str(&yaml)?;
                        Ok(DefaultData::$file_type(n))
                    }
                    )*
                }
            }
        }
    }
}

/// Apply all defaults files to the given exam
macro_rules! handle {
    ($default_files:expr, $exam: expr, $handle_seq: expr, $handle_menu: expr, $handle_diag: expr) => {
{
    let exam = $exam;
    // TODO: diagnostic
    log::info!("Found {} default files.", $default_files.len());
    for default_file in $default_files.iter() {
            log::info!("Reading {}", default_file.get_path().display());
            let default_data = default_file.read_as_data().unwrap(); //TODO
            match default_data {
                DefaultData::SequentialNavigation(n) => {
                    $handle_seq(&n, exam)
                }
                DefaultData::MenuNavigation(n) => {
                    $handle_menu(&n, exam)
                }
                DefaultData::DiagnosticNavigation(n) => {
                    $handle_diag(&n, exam)
                }
                DefaultData::Timing(t) => exam.timing.overwrite(&Value::Normal(t)),
                DefaultData::Feedback(f) => exam.feedback.overwrite(&Value::Normal(f)),
                DefaultData::NumbasSettings(f) => exam.numbas_settings.overwrite(&Value::Normal(f)),
                DefaultData::Question(q) => {
                    if let Some(ValueType::Normal(ref mut groups)) = exam.question_groups.0 {
                        groups.iter_mut().for_each(|qg_value| {
                            if let Some(ValueType::Normal(ref mut qg)) = &mut qg_value.0 {
                                if let Some(ValueType::Normal(ref mut questions)) =
                                    &mut qg.questions.0
                                {
                                    questions.iter_mut().for_each(|question_value| {
                                        if let Some(ValueType::Normal(ref mut question)) =
                                            question_value.0
                                        {
                                            question
                                                .question_data
                                                .overwrite(&q);
                                        }
                                    })
                                }
                            }
                        });
                    }
                }
                DefaultData::QuestionPartJME(p) => handle_question_parts!(exam, p, JME),
                DefaultData::QuestionPartGapFillGapJME(p) => handle_question_parts!(gap exam, p, JME),
                DefaultData::QuestionPartGapFill(p) => handle_question_parts!(exam, p, GapFill),
                DefaultData::QuestionPartChooseOne(p) => handle_question_parts!(exam, p, ChooseOne),
                DefaultData::QuestionPartGapFillGapChooseOne(p) => handle_question_parts!(gap exam, p, ChooseOne),
                DefaultData::QuestionPartChooseMultiple(p) => handle_question_parts!(exam, p, ChooseMultiple),
                DefaultData::QuestionPartGapFillGapChooseMultiple(p) => handle_question_parts!(gap exam, p, ChooseMultiple),
                DefaultData::QuestionPartMatchAnswersWithItems(p) => handle_question_parts!(exam, p, MatchAnswersWithItems),
                DefaultData::QuestionPartGapFillGapMatchAnswersWithItems(p) => handle_question_parts!(gap exam, p, MatchAnswersWithItems),
                DefaultData::QuestionPartNumberEntry(p) => handle_question_parts!(exam, p, NumberEntry),
                DefaultData::QuestionPartGapFillGapNumberEntry(p) => handle_question_parts!(gap exam, p, NumberEntry),
                DefaultData::QuestionPartPatternMatch(p) => handle_question_parts!(exam, p, PatternMatch),
                DefaultData::QuestionPartGapFillGapPatternMatch(p) => handle_question_parts!(gap exam, p, PatternMatch),
                DefaultData::QuestionPartInformation(p) => handle_question_parts!(exam, p, Information),
                DefaultData::QuestionPartGapFillGapInformation(p) => handle_question_parts!(gap exam, p, Information),
                DefaultData::QuestionPartExtension(p) => handle_question_parts!(exam, p, Extension),
                DefaultData::QuestionPartGapFillGapExtension(p) => handle_question_parts!(gap exam, p, Extension),

            }

    }
}
}
}

/// Apply all defaults files to the question parts (or gaps) of the given exam
macro_rules! handle_question_parts {
    ($exam: expr, $p: expr, $type: ident) => {
        if let Value(Some(ValueType::Normal(ref mut groups))) = $exam.question_groups {
            groups.iter_mut().for_each(|qg_value| {
                if let Some(ValueType::Normal(ref mut qg)) = &mut qg_value.0 {
                    if let Some(ValueType::Normal(ref mut questions)) = &mut qg.questions.0 {
                        questions.iter_mut().for_each(|question_value| {
                            if let Some(ValueType::Normal(ref mut question)) = &mut question_value.0
                            {
                                if let Some(ValueType::Normal(ref mut parts)) =
                                    question.question_data.parts.0
                                {
                                    parts.iter_mut().for_each(|part_value| {
                                        if let Some(ValueType::Normal(QuestionPart::Builtin(
                                            ref mut part,
                                        ))) = &mut part_value.0
                                        {
                                            if let QuestionPartBuiltin::$type(_) = &part {
                                                part.overwrite(&QuestionPartBuiltin::$type(
                                                    $p.clone(),
                                                ))
                                            }
                                            if let Value(Some(ValueType::Normal(ref mut steps))) =
                                                &mut part.get_steps()
                                            {
                                                steps.iter_mut().for_each(|part| {
                                                    if let QuestionPart::Builtin(
                                                        QuestionPartBuiltin::$type(_),
                                                    ) = &part
                                                    {
                                                        part.overwrite(&QuestionPart::Builtin(
                                                            QuestionPartBuiltin::$type($p.clone()),
                                                        ))
                                                    }
                                                })
                                            }
                                        }
                                    });
                                }
                            }
                        })
                    }
                }
            })
        }
    };
    (gap $exam: expr, $p: expr, $type: ident) => {
        if let Value(Some(ValueType::Normal(ref mut groups))) = $exam.question_groups {
            groups.iter_mut().for_each(|qg_value| {
                if let Some(ValueType::Normal(ref mut qg)) = &mut qg_value.0 {
                    if let Some(ValueType::Normal(ref mut questions)) = &mut qg.questions.0 {
                        questions.iter_mut().for_each(|question_value| {
                            if let Some(ValueType::Normal(ref mut question)) = &mut question_value.0
                            {
                                if let Some(ValueType::Normal(ref mut parts)) =
                                    question.question_data.parts.0
                                {
                                    parts.iter_mut().for_each(|part_value| {
                                        if let Some(ValueType::Normal(QuestionPart::Builtin(
                                            QuestionPartBuiltin::GapFill(ref mut gap_fill),
                                        ))) = &mut part_value.0
                                        {
                                            if let Some(ValueType::Normal(ref mut gaps)) =
                                                gap_fill.gaps.0
                                            {
                                                gaps.iter_mut().for_each(|gap| {
                                                    if let QuestionPart::Builtin(
                                                        QuestionPartBuiltin::$type(_),
                                                    ) = &gap
                                                    {
                                                        gap.overwrite(&QuestionPart::Builtin(
                                                            QuestionPartBuiltin::$type($p.clone()),
                                                        ))
                                                    }
                                                })
                                            }
                                        }
                                    })
                                }
                            }
                        })
                    }
                }
            })
        }
    };
}

use create_default_file_type_enums;
use handle;
use handle_question_parts;
