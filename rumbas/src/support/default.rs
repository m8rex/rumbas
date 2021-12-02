use crate::exam::diagnostic::DiagnosticExamInput;
use crate::exam::feedback::FeedbackInput;
use crate::exam::navigation::{
    DiagnosticNavigationInput, MenuNavigationInput, MenuNavigationInputEnum, NormalNavigationInput,
    SequentialNavigationInput, SequentialNavigationInputEnum,
};
use crate::exam::normal::NormalExamInput;
use crate::exam::numbas_settings::NumbasSettingsInput;
use crate::exam::timing::TimingInput;
use crate::exam::ExamInput;
use crate::question::part::extension::QuestionPartExtensionInput;
use crate::question::part::extension::QuestionPartExtensionInputEnum;
use crate::question::part::gapfill::QuestionPartGapFillInput;
use crate::question::part::gapfill::QuestionPartGapFillInputEnum;
use crate::question::part::information::QuestionPartInformationInput;
use crate::question::part::information::QuestionPartInformationInputEnum;
use crate::question::part::jme::QuestionPartJMEInput;
use crate::question::part::jme::QuestionPartJMEInputEnum;
use crate::question::part::multiple_choice::choose_multiple::QuestionPartChooseMultipleInput;
use crate::question::part::multiple_choice::choose_multiple::QuestionPartChooseMultipleInputEnum;
use crate::question::part::multiple_choice::choose_one::QuestionPartChooseOneInput;
use crate::question::part::multiple_choice::choose_one::QuestionPartChooseOneInputEnum;
use crate::question::part::multiple_choice::match_answers::QuestionPartMatchAnswersWithItemsInput;
use crate::question::part::multiple_choice::match_answers::QuestionPartMatchAnswersWithItemsInputEnum;
use crate::question::part::number_entry::QuestionPartNumberEntryInput;
use crate::question::part::number_entry::QuestionPartNumberEntryInputEnum;
use crate::question::part::pattern_match::QuestionPartPatternMatchInput;
use crate::question::part::pattern_match::QuestionPartPatternMatchInputEnum;
use crate::question::part::question_part::{QuestionPartBuiltinInput, QuestionPartInput};
use crate::question::QuestionInput;
use rumbas_support::preamble::*;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

//TODO Tests
//Questionnavigation?? -> in question?
//

/// Combine an exam with all data from the default files
pub fn combine_with_default_files(path: &Path, exam: &mut ExamInput) {
    let default_files = default_files(path);
    if let ExamInput::Normal(ref mut e) = exam {
        handle!(
            default_files,
            e,
            |n: &SequentialNavigationInput, e: &mut NormalExamInput| e.navigation.overwrite(
                &Value::Normal(NormalNavigationInput::Sequential(
                    SequentialNavigationInputEnum(n.clone())
                ))
            ),
            |n: &MenuNavigationInput, e: &mut NormalExamInput| e.navigation.overwrite(
                &Value::Normal(NormalNavigationInput::Menu(MenuNavigationInputEnum(
                    n.clone()
                )))
            ),
            |_n: &DiagnosticNavigationInput, _e: &mut NormalExamInput| ()
        );
    } else if let ExamInput::Diagnostic(ref mut e) = exam {
        handle!(
            default_files,
            e,
            |_n: &SequentialNavigationInput, _e: &mut DiagnosticExamInput| (),
            |_n: &MenuNavigationInput, _e: &mut DiagnosticExamInput| (),
            |n: &DiagnosticNavigationInput, e: &mut DiagnosticExamInput| e
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
    SequentialNavigation with type SequentialNavigationInput: in "navigation";
    MenuNavigation with type MenuNavigationInput: in "navigation.menu";
    DiagnosticNavigation with type DiagnosticNavigationInput: in "navigation.diagnostic";
    Timing with type TimingInput: in "timing";
    Feedback with type FeedbackInput: in "feedback";
    NumbasSettings with type NumbasSettingsInput: in "numbas_settings";
    Question with type QuestionInput: in "question";
    QuestionPartJME with type QuestionPartJMEInput: in "questionpart.jme";
    QuestionPartGapFill with type QuestionPartGapFillInput: in "questionpart.gapfill";
    QuestionPartChooseOne with type QuestionPartChooseOneInput: in "questionpart.choose_one";
    QuestionPartChooseMultiple with type QuestionPartChooseMultipleInput: in "questionpart.choose_multiple";
    QuestionPartMatchAnswersWithItems with type QuestionPartMatchAnswersWithItemsInput: in "questionpart.match_answers";
    QuestionPartNumberEntry with type QuestionPartNumberEntryInput: in "questionpart.number_entry";
    QuestionPartPatternMatch with type QuestionPartPatternMatchInput: in "questionpart.pattern_match";
    QuestionPartInformation with type QuestionPartInformationInput: in "questionpart.information";
    QuestionPartExtension with type QuestionPartExtensionInput: in "questionpart.extension";
    QuestionPartGapFillGapJME with type QuestionPartJMEInput: in "questionpart.gapfill.gap.jme";
    QuestionPartGapFillGapChooseOne with type QuestionPartChooseOneInput: in "questionpart.gapfill.gap.choose_one";
    QuestionPartGapFillGapChooseMultiple with type QuestionPartChooseMultipleInput: in "questionpart.gapfill.gap.choose_multiple";
    QuestionPartGapFillGapMatchAnswersWithItems with type QuestionPartMatchAnswersWithItemsInput: in "questionpart.gapfill.gap.match_answers";
    QuestionPartGapFillGapNumberEntry with type QuestionPartNumberEntryInput: in "questionpart.gapfill.gap.number_entry";
    QuestionPartGapFillGapPatternMatch with type QuestionPartPatternMatchInput: in "questionpart.gapfill.gap.pattern_match";
    QuestionPartGapFillGapInformation with type QuestionPartInformationInput: in "questionpart.gapfill.gap.information";
    QuestionPartGapFillGapExtension with type QuestionPartExtensionInput: in "questionpart.gapfill.gap.extension"
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
                        Ok(DefaultData::$file_type( n ))
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
    let exam = &mut $exam.0;
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
                    if let Value(Some(ValueType::Normal(ref mut groups))) = exam.question_groups {
                        groups.iter_mut().for_each(|qg_value| {
                            if let ValueType::Normal(ref mut qg) = qg_value {
                                if let Some(ValueType::Normal(ref mut questions)) =
                                    &mut qg.questions.0
                                {
                                    questions.iter_mut().for_each(|question_value| {
                                        if let ValueType::Normal(ref mut question) =
                                            question_value
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
                DefaultData::QuestionPartJME(p) => handle_question_parts!(exam, QuestionPartJMEInputEnum(p.clone()), JME),
                DefaultData::QuestionPartGapFillGapJME(p) => handle_question_parts!(gap exam, QuestionPartJMEInputEnum(p.clone()), JME),
                DefaultData::QuestionPartGapFill(p) => handle_question_parts!(exam, QuestionPartGapFillInputEnum(p.clone()), GapFill),
                DefaultData::QuestionPartChooseOne(p) => handle_question_parts!(exam, QuestionPartChooseOneInputEnum(p.clone()), ChooseOne),
                DefaultData::QuestionPartGapFillGapChooseOne(p) => handle_question_parts!(gap exam, QuestionPartChooseOneInputEnum(p.clone()), ChooseOne),
                DefaultData::QuestionPartChooseMultiple(p) => handle_question_parts!(exam, QuestionPartChooseMultipleInputEnum(p.clone()), ChooseMultiple),
                DefaultData::QuestionPartGapFillGapChooseMultiple(p) => handle_question_parts!(gap exam, QuestionPartChooseMultipleInputEnum(p.clone()), ChooseMultiple),
                DefaultData::QuestionPartMatchAnswersWithItems(p) => handle_question_parts!(exam, QuestionPartMatchAnswersWithItemsInputEnum(p.clone()), MatchAnswersWithItems),
                DefaultData::QuestionPartGapFillGapMatchAnswersWithItems(p) => handle_question_parts!(gap exam, QuestionPartMatchAnswersWithItemsInputEnum(p.clone()), MatchAnswersWithItems),
                DefaultData::QuestionPartNumberEntry(p) => handle_question_parts!(exam, QuestionPartNumberEntryInputEnum(p.clone()), NumberEntry),
                DefaultData::QuestionPartGapFillGapNumberEntry(p) => handle_question_parts!(gap exam, QuestionPartNumberEntryInputEnum(p.clone()), NumberEntry),
                DefaultData::QuestionPartPatternMatch(p) => handle_question_parts!(exam, QuestionPartPatternMatchInputEnum(p.clone()), PatternMatch),
                DefaultData::QuestionPartGapFillGapPatternMatch(p) => handle_question_parts!(gap exam, QuestionPartPatternMatchInputEnum(p.clone()), PatternMatch),
                DefaultData::QuestionPartInformation(p) => handle_question_parts!(exam, QuestionPartInformationInputEnum(p.clone()), Information),
                DefaultData::QuestionPartGapFillGapInformation(p) => handle_question_parts!(gap exam, QuestionPartInformationInputEnum(p.clone()), Information),
                DefaultData::QuestionPartExtension(p) => handle_question_parts!(exam, QuestionPartExtensionInputEnum(p.clone()), Extension),
                DefaultData::QuestionPartGapFillGapExtension(p) => handle_question_parts!(gap exam, QuestionPartExtensionInputEnum(p.clone()), Extension),

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
                if let ValueType::Normal(ref mut qg) = qg_value {
                    if let Value(Some(ValueType::Normal(ref mut questions))) = &mut qg.questions {
                        questions.iter_mut().for_each(|question_value| {
                            if let ValueType::Normal(ref mut question) = question_value {
                                if let Value(Some(ValueType::Normal(ref mut parts))) =
                                    question.question_data.parts
                                {
                                    parts.iter_mut().for_each(|part_value| {
                                        if let ValueType::Normal(QuestionPartInput::Builtin(
                                            ref mut part,
                                        )) = part_value
                                        {
                                            if let QuestionPartBuiltinInput::$type(_) = &part {
                                                part.overwrite(&QuestionPartBuiltinInput::$type(
                                                    $p.clone(),
                                                ))
                                            }
                                            if let Value(Some(ValueType::Normal(ref mut steps))) =
                                                &mut part.get_steps()
                                            {
                                                steps.iter_mut().for_each(|part| {
                                                    if let ValueType::Normal(
                                                        QuestionPartInput::Builtin(
                                                            QuestionPartBuiltinInput::$type(_),
                                                        ),
                                                    ) = &part
                                                    {
                                                        part.overwrite(&ValueType::Normal(
                                                            QuestionPartInput::Builtin(
                                                                QuestionPartBuiltinInput::$type(
                                                                    $p.clone(),
                                                                ),
                                                            ),
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
                if let ValueType::Normal(ref mut qg) = qg_value {
                    if let Value(Some(ValueType::Normal(ref mut questions))) = &mut qg.questions {
                        questions.iter_mut().for_each(|question_value| {
                            if let ValueType::Normal(ref mut question) = question_value {
                                if let Value(Some(ValueType::Normal(ref mut parts))) =
                                    question.question_data.parts
                                {
                                    parts.iter_mut().for_each(|part_value| {
                                        if let ValueType::Normal(QuestionPartInput::Builtin(
                                            QuestionPartBuiltinInput::GapFill(ref mut gap_fill),
                                        )) = part_value
                                        {
                                            if let Value(Some(ValueType::Normal(ref mut gaps))) =
                                                gap_fill.0.gaps
                                            {
                                                gaps.iter_mut().for_each(|gap| {
                                                    if let ValueType::Normal(
                                                        QuestionPartInput::Builtin(
                                                            QuestionPartBuiltinInput::$type(_),
                                                        ),
                                                    ) = &gap
                                                    {
                                                        gap.overwrite(&ValueType::Normal(
                                                            QuestionPartInput::Builtin(
                                                                QuestionPartBuiltinInput::$type(
                                                                    $p.clone(),
                                                                ),
                                                            ),
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
