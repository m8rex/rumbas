use crate::data::exam::Exam;
use crate::data::feedback::Feedback;
use crate::data::gapfill::QuestionPartGapFill;
use crate::data::information::QuestionPartInformation;
use crate::data::jme::QuestionPartJME;
use crate::data::multiple_choice::QuestionPartChooseOne;
use crate::data::navigation::Navigation;
use crate::data::numbas_settings::NumbasSettings;
use crate::data::number_entry::QuestionPartNumberEntry;
use crate::data::optional_overwrite::OptionalOverwrite;
use crate::data::pattern_match::QuestionPartPatternMatch;
use crate::data::question::Question;
use crate::data::question_part::QuestionPart;
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

#[derive(Debug)]
pub enum DefaultFileType {
    Navigation,
    Timing,
    Feedback,
    NumbasSettings,
    Question,
    QuestionPart(QuestionPartType),
    QuestionPartGapFillGap(QuestionPartType),
}

#[derive(Debug)]
pub enum QuestionPartType {
    JME,
    GapFill,
    ChooseOne,
    NumberEntry,
    PatternMatch,
    Information,
}

pub enum DefaultData {
    Navigation(Navigation),
    Timing(Timing),
    Feedback(Feedback),
    NumbasSettings(NumbasSettings),
    Question(Question),
    QuestionPart(QuestionPart),
    QuestionPartGapFillGap(QuestionPart),
}

impl DefaultFileType {
    fn from(path: &Path) -> Option<DefaultFileType> {
        let file_name = path.file_stem();
        match file_name {
            Some(f) => match f.to_str() {
                Some("navigation") => Some(DefaultFileType::Navigation),
                Some("timing") => Some(DefaultFileType::Timing),
                Some("feedback") => Some(DefaultFileType::Feedback),
                Some("numbas_settings") => Some(DefaultFileType::NumbasSettings),
                Some("question") => Some(DefaultFileType::Question),
                Some("questionpart.gapfill") => {
                    //TODO others etc
                    Some(DefaultFileType::QuestionPart(QuestionPartType::GapFill))
                }
                Some("questionpart.choose_one") => {
                    Some(DefaultFileType::QuestionPart(QuestionPartType::ChooseOne))
                }
                Some("questionpart.number_entry") => {
                    Some(DefaultFileType::QuestionPart(QuestionPartType::NumberEntry))
                }
                Some("questionpart.jme") => {
                    Some(DefaultFileType::QuestionPart(QuestionPartType::JME))
                }
                Some("questionpart.pattern_match") => Some(DefaultFileType::QuestionPart(
                    QuestionPartType::PatternMatch,
                )),
                Some("questionpart.information") => {
                    Some(DefaultFileType::QuestionPart(QuestionPartType::Information))
                }
                Some("questionpart.gapfill.gap.jme") => {
                    //TODO others etc
                    Some(DefaultFileType::QuestionPartGapFillGap(
                        QuestionPartType::JME,
                    ))
                }
                Some("questionpart.gapfill.gap.number_entry") => Some(
                    DefaultFileType::QuestionPartGapFillGap(QuestionPartType::NumberEntry),
                ),
                Some("questionpart.gapfill.gap.pattern_match") => Some(
                    DefaultFileType::QuestionPartGapFillGap(QuestionPartType::PatternMatch),
                ),
                _ => None,
            },
            None => None,
        }
    }
    fn read_as_data(&self, path: &PathBuf) -> serde_yaml::Result<DefaultData> {
        let yaml = fs::read_to_string(path).unwrap();
        match self {
            DefaultFileType::Navigation => {
                let n: Navigation = serde_yaml::from_str(&yaml)?;
                Ok(DefaultData::Navigation(n))
            }
            DefaultFileType::Timing => {
                let t: Timing = serde_yaml::from_str(&yaml)?;
                Ok(DefaultData::Timing(t))
            }
            DefaultFileType::Feedback => {
                let f: Feedback = serde_yaml::from_str(&yaml)?;
                Ok(DefaultData::Feedback(f))
            }
            DefaultFileType::NumbasSettings => {
                let f: NumbasSettings = serde_yaml::from_str(&yaml)?;
                Ok(DefaultData::NumbasSettings(f))
            }
            DefaultFileType::Question => {
                let q: Question = serde_yaml::from_str(&yaml)?;
                Ok(DefaultData::Question(q))
            }
            DefaultFileType::QuestionPart(question_part_type) => match question_part_type {
                QuestionPartType::GapFill => {
                    let q: QuestionPartGapFill = serde_yaml::from_str(&yaml)?;
                    Ok(DefaultData::QuestionPart(QuestionPart::GapFill(q)))
                }
                QuestionPartType::JME => {
                    let q: QuestionPartJME = serde_yaml::from_str(&yaml)?;
                    Ok(DefaultData::QuestionPart(QuestionPart::JME(q)))
                }
                QuestionPartType::ChooseOne => {
                    let q: QuestionPartChooseOne = serde_yaml::from_str(&yaml)?;
                    Ok(DefaultData::QuestionPart(QuestionPart::ChooseOne(q)))
                }
                QuestionPartType::NumberEntry => {
                    let q: QuestionPartNumberEntry = serde_yaml::from_str(&yaml)?;
                    Ok(DefaultData::QuestionPart(QuestionPart::NumberEntry(q)))
                }
                QuestionPartType::PatternMatch => {
                    let q: QuestionPartPatternMatch = serde_yaml::from_str(&yaml)?;
                    Ok(DefaultData::QuestionPart(QuestionPart::PatternMatch(q)))
                }
                QuestionPartType::Information => {
                    let q: QuestionPartInformation = serde_yaml::from_str(&yaml)?;
                    Ok(DefaultData::QuestionPart(QuestionPart::Information(q)))
                }
            }, //TODO: reduce duplicate
            DefaultFileType::QuestionPartGapFillGap(question_part_type) => match question_part_type
            {
                QuestionPartType::GapFill => {
                    let q: QuestionPartGapFill = serde_yaml::from_str(&yaml)?;
                    Ok(DefaultData::QuestionPartGapFillGap(QuestionPart::GapFill(
                        q,
                    )))
                }
                QuestionPartType::JME => {
                    let q: QuestionPartJME = serde_yaml::from_str(&yaml)?;
                    Ok(DefaultData::QuestionPartGapFillGap(QuestionPart::JME(q)))
                }
                QuestionPartType::ChooseOne => {
                    let q: QuestionPartChooseOne = serde_yaml::from_str(&yaml)?;
                    Ok(DefaultData::QuestionPartGapFillGap(
                        QuestionPart::ChooseOne(q),
                    ))
                }
                QuestionPartType::NumberEntry => {
                    let q: QuestionPartNumberEntry = serde_yaml::from_str(&yaml)?;
                    Ok(DefaultData::QuestionPartGapFillGap(
                        QuestionPart::NumberEntry(q),
                    ))
                }
                QuestionPartType::PatternMatch => {
                    let q: QuestionPartPatternMatch = serde_yaml::from_str(&yaml)?;
                    Ok(DefaultData::QuestionPartGapFillGap(
                        QuestionPart::PatternMatch(q),
                    ))
                }
                QuestionPartType::Information => {
                    let q: QuestionPartInformation = serde_yaml::from_str(&yaml)?;
                    Ok(DefaultData::QuestionPartGapFillGap(
                        QuestionPart::Information(q),
                    ))
                }
            },
        }
    }
}

impl DefaultFile {
    fn from(path: &Path) -> Option<DefaultFile> {
        let default_type: Option<DefaultFileType> = DefaultFileType::from(&path);
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
        let defaults_path = a.with_file_name("defaults");
        if defaults_path.is_dir() {
            for entry in defaults_path.read_dir().expect("read_dir call failed") {
                if let Ok(entry) = entry {
                    result.insert(entry.path()); //TODO: order files from the folder
                                                 //println!("{:?}", entry.path());
                }
            }
        }
    }

    result.into_iter().collect::<Vec<PathBuf>>()
}

pub fn combine_with_default_files(path: &Path, exam: &mut Exam) {
    let default_files = default_files(path);
    //println!("Found {} default files.", default_files.len());
    for default_file in default_files.iter() {
        if !exam.empty_fields().is_empty() {
            //println!("Reading {}", default_file.get_path().display());
            let default_data = default_file.read_as_data().unwrap(); //TODO
                                                                     //TODO: always call overwrite
            match default_data {
                DefaultData::Navigation(n) => exam.navigation.overwrite(&Value::Normal(n)),
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
                DefaultData::QuestionPart(p) => {
                    if let Value(Some(ValueType::Normal(ref mut groups))) = exam.question_groups {
                        groups.iter_mut().for_each(|qg_value| {
                            if let Some(ValueType::Normal(ref mut qg)) = &mut qg_value.0 {
                                if let Some(ValueType::Normal(ref mut questions)) =
                                    &mut qg.questions.0
                                {
                                    questions.iter_mut().for_each(|question_value| {
                                        if let Some(ValueType::Normal(ref mut question)) =
                                            &mut question_value.0
                                        {
                                            if let Some(ValueType::Normal(ref mut question_data)) =
                                                question.question_data.0
                                            {
                                                if let Some(ValueType::Normal(ref mut parts)) =
                                                    question_data.parts.0
                                                {
                                                    //TODO: others etc
                                                    parts.iter_mut().for_each(|part_value| {
                                                        if let Some(ValueType::Normal(
                                                            ref mut part,
                                                        )) = &mut part_value.0
                                                        {
                                                            if let (
                                                                QuestionPart::GapFill(_),
                                                                QuestionPart::GapFill(_),
                                                            ) = (&p, &part)
                                                            {
                                                                part.overwrite(&p.clone())
                                                            } else if let (
                                                                QuestionPart::JME(_),
                                                                QuestionPart::JME(_),
                                                            ) = (&p, &part)
                                                            {
                                                                part.overwrite(&p.clone())
                                                            } else if let (
                                                                QuestionPart::ChooseOne(_),
                                                                QuestionPart::ChooseOne(_),
                                                            ) = (&p, &part)
                                                            {
                                                                part.overwrite(&p.clone())
                                                            } else if let (
                                                                QuestionPart::NumberEntry(_),
                                                                QuestionPart::NumberEntry(_),
                                                            ) = (&p, &part)
                                                            {
                                                                part.overwrite(&p.clone())
                                                            } else if let (
                                                                QuestionPart::PatternMatch(_),
                                                                QuestionPart::PatternMatch(_),
                                                            ) = (&p, &part)
                                                            {
                                                                part.overwrite(&p.clone())
                                                            } else if let (
                                                                QuestionPart::Information(_),
                                                                QuestionPart::Information(_),
                                                            ) = (&p, &part)
                                                            {
                                                                part.overwrite(&p.clone())
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
                } //TODO: cleanup...
                DefaultData::QuestionPartGapFillGap(p) => {
                    if let Value(Some(ValueType::Normal(ref mut groups))) = exam.question_groups {
                        groups.iter_mut().for_each(|qg_value| {
                            if let Some(ValueType::Normal(ref mut qg)) = &mut qg_value.0 {
                                if let Some(ValueType::Normal(ref mut questions)) =
                                    &mut qg.questions.0
                                {
                                    questions.iter_mut().for_each(|question_value| {
                                        let mut question = question_value.unwrap();
                                        if let Some(ValueType::Normal(ref mut question)) =
                                            &mut question_value.0
                                        {
                                            if let Some(ValueType::Normal(ref mut question_data)) =
                                                question.question_data.0
                                            {
                                                if let Some(ValueType::Normal(ref mut parts)) =
                                                    question_data.parts.0
                                                {
                                                    parts.iter_mut().for_each(|part_value| {
                                                        if let Some(ValueType::Normal(
                                                            ref mut part,
                                                        )) = &mut part_value.0
                                                        {
                                                            if let QuestionPart::GapFill(
                                                                ref mut gap_fill,
                                                            ) = part
                                                            {
                                                                if let Some(ValueType::Normal(
                                                                    ref mut gaps,
                                                                )) = gap_fill.gaps.0
                                                                {
                                                                    gaps.iter_mut().for_each(
                                                                        |gap| {
                                                                            if let (
                                                                                QuestionPart::JME(
                                                                                    _,
                                                                                ),
                                                                                QuestionPart::JME(
                                                                                    _,
                                                                                ),
                                                                            ) = (&p, &gap)
                                                                            {
                                                                                gap.overwrite(
                                                                                    &p.clone(),
                                                                                )
                                                                            }
                                                                            if let (
                                                                    QuestionPart::NumberEntry(_),
                                                                    QuestionPart::NumberEntry(_),
                                                                ) = (&p, &gap)
                                                                {
                                                                    gap.overwrite(&p.clone())
                                                                }
                                                                            if let (
                                                                    QuestionPart::PatternMatch(_),
                                                                    QuestionPart::PatternMatch(_),
                                                                ) = (&p, &gap)
                                                                {
                                                                    gap.overwrite(&p.clone())
                                                                }
                                                                        },
                                                                    )
                                                                }
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
                }
            }
        }
    }
}
