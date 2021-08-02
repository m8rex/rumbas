use crate::data::diagnostic_exam::DiagnosticExam;
use crate::data::exam::Exam;
use crate::data::feedback::Feedback;
use crate::data::gapfill::QuestionPartGapFill;
use crate::data::information::QuestionPartInformation;
use crate::data::jme::QuestionPartJME;
use crate::data::multiple_choice::QuestionPartChooseMultiple;
use crate::data::multiple_choice::QuestionPartChooseOne;
use crate::data::multiple_choice::QuestionPartMatchAnswersWithItems;
use crate::data::navigation::{
    DiagnosticNavigation, MenuNavigation, NormalNavigation, SequentialNavigation,
};
use crate::data::normal_exam::NormalExam;
use crate::data::numbas_settings::NumbasSettings;
use crate::data::number_entry::QuestionPartNumberEntry;
use crate::data::optional_overwrite::*;
use crate::data::pattern_match::QuestionPartPatternMatch;
use crate::data::question::Question;
use crate::data::question_part::{QuestionPart, QuestionPartBuiltin};
use crate::data::template::{Value, ValueType};
use crate::data::timing::Timing;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

//TODO Tests
//Questionnavigation?? -> in question?

#[derive(Debug)]
pub struct DefaultFile {
    r#type: DefaultFileType,
    path: PathBuf,
}

macro_rules! create_enum_structs {
    ( $($file_type:ident$([$file_type_data: ty])?, $data_type: ty, $file_name: literal);* ) => {
        #[derive(Debug)]
        pub enum DefaultFileType {
            $(
                $file_type $(($file_type_data))?
            ),*
        }

        pub enum DefaultData {
            $(
                $file_type($data_type)
            ),*
        }

        impl DefaultFileType {
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

create_enum_structs!(
SequentialNavigation, SequentialNavigation, "navigation";
MenuNavigation, MenuNavigation, "navigation.menu";
DiagnosticNavigation, DiagnosticNavigation, "navigation.diagnostic";
Timing, Timing, "timing";
Feedback, Feedback, "feedback";
NumbasSettings, NumbasSettings, "numbas_settings";
Question, Question, "question";
QuestionPartJME, QuestionPartJME, "questionpart.jme";
QuestionPartGapFill, QuestionPartGapFill, "questionpart.gapfill";
QuestionPartChooseOne, QuestionPartChooseOne, "questionpart.choose_one";
QuestionPartChooseMultiple, QuestionPartChooseMultiple, "questionpart.choose_multiple";
QuestionPartMatchAnswersWithItems, QuestionPartMatchAnswersWithItems, "questionpart.match_answers";
QuestionPartNumberEntry, QuestionPartNumberEntry, "questionpart.number_entry";
QuestionPartPatternMatch, QuestionPartPatternMatch, "questionpart.pattern_match";
QuestionPartInformation, QuestionPartInformation, "questionpart.information";
QuestionPartGapFillGapJME, QuestionPartJME, "questionpart.gapfill.gap.jme";
QuestionPartGapFillGapChooseOne, QuestionPartChooseOne, "questionpart.gapfill.gap.choose_one";
QuestionPartGapFillGapChooseMultiple, QuestionPartChooseMultiple, "questionpart.gapfill.gap.choose_multiple";
QuestionPartGapFillGapMatchAnswersWithItems, QuestionPartMatchAnswersWithItems, "questionpart.gapfill.gap.match_answers";
QuestionPartGapFillGapNumberEntry, QuestionPartNumberEntry, "questionpart.gapfill.gap.number_entry";
QuestionPartGapFillGapPatternMatch, QuestionPartPatternMatch, "questionpart.gapfill.gap.pattern_match";
QuestionPartGapFillGapInformation, QuestionPartInformation, "questionpart.gapfill.gap.information"
);

impl DefaultFile {
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

    pub fn read_as_data(&self) -> serde_yaml::Result<DefaultData> {
        self.r#type.read_as_data(&self.path)
    }

    pub fn get_path(&self) -> PathBuf {
        self.path.clone()
    }
}

fn default_files(path: &Path) -> Vec<DefaultFile> {
    let paths = default_file_paths(path);
    let usefull_paths = paths
        .into_iter()
        .map(|p| DefaultFile::from(&p))
        .filter(|p| p.is_some());
    usefull_paths.map(|p| p.unwrap()).collect()
}

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

macro_rules! handle_question_parts {
    ($exam: expr, $p: expr, $type: ident) => {
        if let Value(Some(ValueType::Normal(ref mut groups))) = $exam.question_groups {
            groups.iter_mut().for_each(|qg_value| {
                if let Some(ValueType::Normal(ref mut qg)) = &mut qg_value.0 {
                    if let Some(ValueType::Normal(ref mut questions)) = &mut qg.questions.0 {
                        questions.iter_mut().for_each(|question_value| {
                            if let Some(ValueType::Normal(ref mut question)) = &mut question_value.0
                            {
                                if let Some(ValueType::Normal(ref mut question_data)) =
                                    question.question_data.0
                                {
                                    if let Some(ValueType::Normal(ref mut parts)) =
                                        question_data.parts.0
                                    {
                                        //TODO: others etc
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
                                                if let Value(Some(ValueType::Normal(
                                                    ref mut steps,
                                                ))) = &mut part.get_steps()
                                                {
                                                    steps.iter_mut().for_each(|part| {
                                                        if let QuestionPart::Builtin(
                                                            QuestionPartBuiltin::$type(_),
                                                        ) = &part
                                                        {
                                                            part.overwrite(&QuestionPart::Builtin(
                                                                QuestionPartBuiltin::$type(
                                                                    $p.clone(),
                                                                ),
                                                            ))
                                                        }
                                                    })
                                                }
                                            }
                                        });
                                    }
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
                                if let Some(ValueType::Normal(ref mut question_data)) =
                                    question.question_data.0
                                {
                                    if let Some(ValueType::Normal(ref mut parts)) =
                                        question_data.parts.0
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
                                                                QuestionPartBuiltin::$type(
                                                                    $p.clone(),
                                                                ),
                                                            ))
                                                        }
                                                    })
                                                }
                                            }
                                        })
                                    }
                                }
                            }
                        })
                    }
                }
            })
        }
    };
}

macro_rules! handle {
    ($path: expr, $exam: expr, $handle_seq: expr, $handle_menu: expr, $handle_diag: expr) => {
{
    let path = $path;
    let exam = $exam;
    // TODO: diagnostic
    let default_files = default_files(path);
    //println!("Found {} default files.", default_files.len());
    for default_file in default_files.iter() {
        if !exam.check().is_empty() {
            log::info!("Reading {}", default_file.get_path().display()); //TODO: debug
            let default_data = default_file.read_as_data().unwrap(); //TODO
                                                                     //TODO: always call overwrite
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
                                                .overwrite(&Value::Normal(q.clone()));
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

            }
        }
    }
}
}
}

pub fn combine_with_default_files(path: &Path, exam: &mut Exam) {
    if let Exam::Normal(ref mut e) = exam {
        handle!(
            path,
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
            path,
            e,
            |_n: &SequentialNavigation, _e: &mut DiagnosticExam| (),
            |_n: &MenuNavigation, _e: &mut DiagnosticExam| (),
            |n: &DiagnosticNavigation, e: &mut DiagnosticExam| e
                .navigation
                .overwrite(&Value::Normal(n.clone()))
        );
    }
}
