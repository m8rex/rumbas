use crate::exam::diagnostic::DiagnosticExamInput;
use crate::exam::feedback::FeedbackInput;
use crate::exam::locale::LocaleInput;
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
use crate::question::part::matrix::QuestionPartMatrixInput;
use crate::question::part::matrix::QuestionPartMatrixInputEnum;
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
use crate::support::file_manager::RumbasRepoEntry;
use crate::support::yaml::{parse_yaml, YamlResult};
use rumbas_support::input::{FileToLoad, LoadedFile};
use rumbas_support::path::RumbasPath;
use rumbas_support::preamble::*;
use std::collections::HashSet;

//TODO Tests
//Questionnavigation?? -> in question?
//

/// Combine an exam with all data from the default files
pub fn combine_exam_with_default_files(path: RumbasPath, exam: &mut ExamInput) {
    let default_files = <DefaultFile<DefaultExamFileType>>::files(path);
    if let ExamInput::Normal(ref mut e) = exam {
        handle_exam!(
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
        handle_exam!(
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

/// Combine a question with all data from the default files
pub fn combine_question_with_default_files(path: RumbasPath, question: &mut QuestionInput) {
    let default_files = <DefaultFile<DefaultQuestionFileType>>::files(path);
    handle_question!(default_files, question);
}

/// Returns a vector of paths to default files for the given path
fn default_file_paths(path: RumbasPath) -> Vec<RumbasPath> {
    let mut used = HashSet::new(); //Use set to remove duplicates (only happens for the 'defaults' folder in root
                                   //TODO: write tests and maybe use .take(count()-1) instead of hashset
    let mut result = Vec::new();
    let ancestors = path.project().ancestors();
    for a in ancestors {
        let defaults_path = a.with_file_name(crate::DEFAULTS_FOLDER);
        let defaults_path = path.keep_root(defaults_path.as_path());
        for entry in crate::support::file_manager::CACHE
            .read_folder(&defaults_path)
            .into_iter()
            .filter_map(|e| match e {
                RumbasRepoEntry::File(f) => Some(f.path()),
                _ => None,
            })
        {
            let absolute = entry.absolute().to_path_buf();
            if !used.contains(&absolute) {
                used.insert(absolute);
                result.push(entry.clone());
            }
        }
    }

    result
}

// Create the needed enum for exams by specifying which files contain which data
create_default_file_type_enums!(
    DefaultExamFileType: DefaultExamData,
    SequentialNavigation with type SequentialNavigationInput: in "navigation";
    MenuNavigation with type MenuNavigationInput: in "navigation.menu";
    DiagnosticNavigation with type DiagnosticNavigationInput: in "navigation.diagnostic";
    Locales with type Vec<ValueType<LocaleInput>>: in "locales";
    Timing with type TimingInput: in "timing";
    Feedback with type FeedbackInput: in "feedback";
    NumbasSettings with type NumbasSettingsInput: in "numbas_settings"
);

// Create the needed enum for questions by specifying which files contain which data
create_default_file_type_enums!(
    DefaultQuestionFileType: DefaultQuestionData,
    Question with type QuestionInput: in "question";
    QuestionPartJME with type QuestionPartJMEInput: in "questionpart.jme";
    QuestionPartGapFill with type QuestionPartGapFillInput: in "questionpart.gapfill";
    QuestionPartChooseOne with type QuestionPartChooseOneInput: in "questionpart.choose_one";
    QuestionPartChooseMultiple with type QuestionPartChooseMultipleInput: in "questionpart.choose_multiple";
    QuestionPartMatchAnswersWithItems with type QuestionPartMatchAnswersWithItemsInput: in "questionpart.match_answers";
    QuestionPartMatrix with type QuestionPartMatrixInput: in "questionpart.matrix";
    QuestionPartNumberEntry with type QuestionPartNumberEntryInput: in "questionpart.number_entry";
    QuestionPartPatternMatch with type QuestionPartPatternMatchInput: in "questionpart.pattern_match";
    QuestionPartInformation with type QuestionPartInformationInput: in "questionpart.information";
    QuestionPartExtension with type QuestionPartExtensionInput: in "questionpart.extension";
    QuestionPartGapFillGapJME with type QuestionPartJMEInput: in "questionpart.gapfill.gap.jme";
    QuestionPartGapFillGapChooseOne with type QuestionPartChooseOneInput: in "questionpart.gapfill.gap.choose_one";
    QuestionPartGapFillGapChooseMultiple with type QuestionPartChooseMultipleInput: in "questionpart.gapfill.gap.choose_multiple";
    QuestionPartGapFillGapMatchAnswersWithItems with type QuestionPartMatchAnswersWithItemsInput: in "questionpart.gapfill.gap.match_answers";
    QuestionPartGapFillGapMatrix with type QuestionPartMatrixInput: in "questionpart.gapfill.gap.matrix";
    QuestionPartGapFillGapNumberEntry with type QuestionPartNumberEntryInput: in "questionpart.gapfill.gap.number_entry";
    QuestionPartGapFillGapPatternMatch with type QuestionPartPatternMatchInput: in "questionpart.gapfill.gap.pattern_match";
    QuestionPartGapFillGapInformation with type QuestionPartInformationInput: in "questionpart.gapfill.gap.information";
    QuestionPartGapFillGapExtension with type QuestionPartExtensionInput: in "questionpart.gapfill.gap.extension"
);

pub trait DefaultFileTypeMethods: Sized {
    type Data;
    fn from_path(path: &RumbasPath) -> Option<Self>;
    fn read_as_data(&self, path: &RumbasPath) -> YamlResult<Self::Data>;
}

#[derive(Debug, Clone)]
/// Struct used to overwrite values with defaults
pub struct DefaultFile<T: Clone> {
    r#type: T,
    path: RumbasPath,
}

impl<T: DefaultFileTypeMethods + Clone> DefaultFile<T> {
    /// Create a DefaultFile from the file_name of the given path, returns None if invalid path
    pub fn from_path(path: &RumbasPath) -> Option<Self> {
        let default_type: Option<T> = T::from_path(path);
        if let Some(t) = default_type {
            return Some(DefaultFile {
                r#type: t,
                path: path.clone(),
            });
        }
        None
    }

    /// Read the given path as the data needed for this DefaultFile
    pub fn read_as_data(&self) -> YamlResult<T::Data> {
        self.r#type.read_as_data(&self.path)
    }

    /// Returns a vector with all DefaultExamFiles that are found for the given path
    fn files(path: RumbasPath) -> Vec<Self> {
        let paths = default_file_paths(path);
        let usefull_paths = paths
            .into_iter()
            .map(|p| Self::from_path(&p))
            .filter(|p| p.is_some());
        usefull_paths.map(|p| p.unwrap()).collect()
    }
}

impl<T: Clone> DefaultFile<T> {
    /// Get the path of this DefaultFile
    pub fn get_path(&self) -> RumbasPath {
        self.path.clone()
    }

    /// Get the type of this DefaultFile
    pub fn get_type(&self) -> T {
        self.r#type.clone()
    }
}

#[derive(Debug, Clone)]
pub struct DefaultFileData<T> {
    data: T,
    path: RumbasPath,
}

/// Create the DefaultFileType and DefaultQuestionData enums and their methods to read data
macro_rules! create_default_file_type_enums {
    ($type_name: ident: $data_name: ident, $($file_type:ident with type $data_type: ty: in $file_name: literal);* ) => {
        #[derive(Debug, Clone)]
        pub enum $type_name {
            $(
                $file_type
            ),*
        }

        #[derive(Debug, Clone)]
        pub enum $data_name {
            $(
                $file_type(DefaultFileData<$data_type>)
            ),*
        }

        impl DefaultFileTypeMethods for $type_name {
            type Data = $data_name;
            /// Creates a DefaultFileType based on the filename, returns None if unknown
            fn from_path(path: &RumbasPath) -> Option<Self> {
                let file_name = path.project().file_stem();
                match file_name {
                    Some(f) => match f.to_str() {
                        $(
                            Some($file_name) => Some(Self::$file_type),
                        )*
                        _ => None
                    }
                    _ => None
                }
            }

            /// Read the given path as the data needed for this DefaultFileType
            fn read_as_data(&self, path: &RumbasPath) -> YamlResult<Self::Data> {
                let file = FileToLoad { file_path: path.clone(), locale_dependant: false };
                let loaded_file = crate::support::file_manager::CACHE.read_file(file);
                if let Some(LoadedFile::Normal(l)) = loaded_file {
                    match self {
                        $(
                        Self::$file_type => {
                            let n: $data_type = parse_yaml(&l.content, path.clone())?;
                            Ok($data_name::$file_type( DefaultFileData { data: n, path: path.clone() }))
                        }
                        )*
                    }
                } else { unreachable!() }
            }
        }
    };
}

/// Apply all defaults files to the given exam
macro_rules! handle_exam {
    ($default_files:expr, $exam: expr, $handle_seq: expr, $handle_menu: expr, $handle_diag: expr) => {
        {
            let exam = &mut $exam.0;
            // TODO: diagnostic
            log::debug!("Found {} default exam files.", $default_files.len());
            for default_file in $default_files.iter() {
                    let default_data = default_file.read_as_data().unwrap(); //TODO Move this so file reader reads them
                    match default_data {
                        DefaultExamData::SequentialNavigation(n) => {
                            $handle_seq(&n.data, exam)
                        }
                        DefaultExamData::MenuNavigation(n) => {
                            $handle_menu(&n.data, exam)
                        }
                        DefaultExamData::DiagnosticNavigation(n) => {
                            $handle_diag(&n.data, exam)
                        }
                        DefaultExamData::Timing(t) => exam.timing.overwrite(&Value::Normal(t.data)),
                        DefaultExamData::Feedback(f) => exam.feedback.overwrite(&Value::Normal(f.data)),
                        DefaultExamData::NumbasSettings(f) => exam.numbas_settings.overwrite(&Value::Normal(f.data)),
                        DefaultExamData::Locales(f) => exam.locales.overwrite(&Value::Normal(f.data)),
                    }
            }
        }
    }
}

/// Apply all defaults files to the given question
macro_rules! handle_question {
    ($default_files:expr, $question: expr) => {
{
    let question = $question;
    log::debug!("Found {} default question files.", $default_files.len());
    for default_file in $default_files.iter() {
            log::debug!("Reading {}", default_file.get_path().display());
            let default_data = default_file.read_as_data().unwrap(); //TODO
            match default_data {
                DefaultQuestionData::Question(q) => {
                    question
                        .overwrite(&q.data);
                }
                DefaultQuestionData::QuestionPartJME(p) => handle_question_parts!(question, Box::new(QuestionPartJMEInputEnum(p.data.clone())), JME),
                DefaultQuestionData::QuestionPartGapFillGapJME(p) => handle_question_parts!(gap question, Box::new(QuestionPartJMEInputEnum(p.data.clone())), JME),
                DefaultQuestionData::QuestionPartGapFill(p) => handle_question_parts!(question, QuestionPartGapFillInputEnum(p.data.clone()), GapFill),
                DefaultQuestionData::QuestionPartChooseOne(p) => handle_question_parts!(question, QuestionPartChooseOneInputEnum(p.data.clone()), ChooseOne),
                DefaultQuestionData::QuestionPartGapFillGapChooseOne(p) => handle_question_parts!(gap question, QuestionPartChooseOneInputEnum(p.data.clone()), ChooseOne),
                DefaultQuestionData::QuestionPartChooseMultiple(p) => handle_question_parts!(question, QuestionPartChooseMultipleInputEnum(p.data.clone()), ChooseMultiple),
                DefaultQuestionData::QuestionPartGapFillGapChooseMultiple(p) => handle_question_parts!(gap question, QuestionPartChooseMultipleInputEnum(p.data.clone()), ChooseMultiple),
                DefaultQuestionData::QuestionPartMatchAnswersWithItems(p) => handle_question_parts!(question, QuestionPartMatchAnswersWithItemsInputEnum(p.data.clone()), MatchAnswersWithItems),
                DefaultQuestionData::QuestionPartGapFillGapMatchAnswersWithItems(p) => handle_question_parts!(gap question, QuestionPartMatchAnswersWithItemsInputEnum(p.data.clone()), MatchAnswersWithItems),
                DefaultQuestionData::QuestionPartMatrix(p) => handle_question_parts!(question, QuestionPartMatrixInputEnum(p.data.clone()), Matrix),
                DefaultQuestionData::QuestionPartGapFillGapMatrix(p) => handle_question_parts!(gap question, QuestionPartMatrixInputEnum(p.data.clone()), Matrix),
                DefaultQuestionData::QuestionPartNumberEntry(p) => handle_question_parts!(question, QuestionPartNumberEntryInputEnum(p.data.clone()), NumberEntry),
                DefaultQuestionData::QuestionPartGapFillGapNumberEntry(p) => handle_question_parts!(gap question, QuestionPartNumberEntryInputEnum(p.data.clone()), NumberEntry),
                DefaultQuestionData::QuestionPartPatternMatch(p) => handle_question_parts!(question, QuestionPartPatternMatchInputEnum(p.data.clone()), PatternMatch),
                DefaultQuestionData::QuestionPartGapFillGapPatternMatch(p) => handle_question_parts!(gap question, QuestionPartPatternMatchInputEnum(p.data.clone()), PatternMatch),
                DefaultQuestionData::QuestionPartInformation(p) => handle_question_parts!(question, QuestionPartInformationInputEnum(p.data.clone()), Information),
                DefaultQuestionData::QuestionPartGapFillGapInformation(p) => handle_question_parts!(gap question, QuestionPartInformationInputEnum(p.data.clone()), Information),
                DefaultQuestionData::QuestionPartExtension(p) => handle_question_parts!(question, QuestionPartExtensionInputEnum(p.data.clone()), Extension),
                DefaultQuestionData::QuestionPartGapFillGapExtension(p) => handle_question_parts!(gap question, QuestionPartExtensionInputEnum(p.data.clone()), Extension),

            }

    }
}
}
}

/// Apply all defaults files to the question parts (or gaps) of the given exam
macro_rules! handle_question_parts {
    ($question: expr, $p: expr, $type: ident) => {
        if let Value(Some(ValueType::Normal(ref mut parts))) = $question.parts {
            parts.iter_mut().for_each(|part_value| {
                if let ValueType::Normal(QuestionPartInput::Builtin(ref mut part)) = part_value {
                    if let QuestionPartBuiltinInput::$type(_) = &part {
                        part.overwrite(&QuestionPartBuiltinInput::$type($p.clone()))
                    }
                    if let Value(Some(ValueType::Normal(ref mut steps))) = &mut part.get_steps() {
                        steps.iter_mut().for_each(|part| {
                            if let ValueType::Normal(QuestionPartInput::Builtin(
                                QuestionPartBuiltinInput::$type(_),
                            )) = &part
                            {
                                part.overwrite(&ValueType::Normal(QuestionPartInput::Builtin(
                                    QuestionPartBuiltinInput::$type($p.clone()),
                                )))
                            }
                        })
                    }
                }
            });
        }
    };
    (gap $question: expr, $p: expr, $type: ident) => {
        if let Value(Some(ValueType::Normal(ref mut parts))) = $question.parts {
            parts.iter_mut().for_each(|part_value| {
                if let ValueType::Normal(QuestionPartInput::Builtin(
                    QuestionPartBuiltinInput::GapFill(ref mut gap_fill),
                )) = part_value
                {
                    if let Value(Some(ValueType::Normal(ref mut gaps))) = gap_fill.0.gaps {
                        gaps.iter_mut().for_each(|gap| {
                            if let ValueType::Normal(QuestionPartInput::Builtin(
                                QuestionPartBuiltinInput::$type(_),
                            )) = &gap
                            {
                                gap.overwrite(&ValueType::Normal(QuestionPartInput::Builtin(
                                    QuestionPartBuiltinInput::$type($p.clone()),
                                )))
                            }
                        })
                    }
                }
            })
        }
    };
}

use create_default_file_type_enums;
use handle_exam;
use handle_question;
use handle_question_parts;
