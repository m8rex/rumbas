use crate::data::exam::{
    Exam, Feedback, Navigation, NumbasSettings, Question, QuestionPart, QuestionPartGapFill,
    QuestionPartJME, Timing,
};
use crate::data::optional_overwrite::OptionalOverwrite;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

//TODO Tests

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
                Some("questionpart.gapfill.gap.jme") => {
                    //TODO others etc
                    Some(DefaultFileType::QuestionPartGapFillGap(
                        QuestionPartType::JME,
                    ))
                }
                _ => None,
            },
            None => None,
        }
    }
    fn read_as_data(&self, path: &PathBuf) -> serde_json::Result<DefaultData> {
        let json = fs::read_to_string(path).unwrap();
        match self {
            DefaultFileType::Navigation => {
                let n: Navigation = serde_json::from_str(&json)?;
                Ok(DefaultData::Navigation(n))
            }
            DefaultFileType::Timing => {
                let t: Timing = serde_json::from_str(&json)?;
                Ok(DefaultData::Timing(t))
            }
            DefaultFileType::Feedback => {
                let f: Feedback = serde_json::from_str(&json)?;
                Ok(DefaultData::Feedback(f))
            }
            DefaultFileType::NumbasSettings => {
                let f: NumbasSettings = serde_json::from_str(&json)?;
                Ok(DefaultData::NumbasSettings(f))
            }
            DefaultFileType::Question => {
                let q: Question = serde_json::from_str(&json)?;
                Ok(DefaultData::Question(q))
            }
            DefaultFileType::QuestionPart(question_part_type) => match question_part_type {
                QuestionPartType::GapFill => {
                    let q: QuestionPartGapFill = serde_json::from_str(&json)?;
                    Ok(DefaultData::QuestionPart(QuestionPart::GapFill(q)))
                }
                QuestionPartType::JME => {
                    let q: QuestionPartJME = serde_json::from_str(&json)?;
                    Ok(DefaultData::QuestionPart(QuestionPart::JME(q)))
                }
            }, //TODO: reduce duplicate
            DefaultFileType::QuestionPartGapFillGap(question_part_type) => match question_part_type
            {
                QuestionPartType::GapFill => {
                    let q: QuestionPartGapFill = serde_json::from_str(&json)?;
                    Ok(DefaultData::QuestionPartGapFillGap(QuestionPart::GapFill(
                        q,
                    )))
                }
                QuestionPartType::JME => {
                    let q: QuestionPartJME = serde_json::from_str(&json)?;
                    Ok(DefaultData::QuestionPartGapFillGap(QuestionPart::JME(q)))
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

    pub fn read_as_data(&self) -> serde_json::Result<DefaultData> {
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
                    println!("{:?}", entry.path());
                }
            }
        }
    }

    result.into_iter().collect::<Vec<PathBuf>>()
}

pub fn combine_with_default_files(path: &Path, exam: &mut Exam) {
    let default_files = default_files(path);
    println!("Found {} default files.", default_files.len());
    for default_file in default_files.iter() {
        if !exam.empty_fields().is_empty() {
            println!("Reading {}", default_file.get_path().display());
            let default_data = default_file.read_as_data().unwrap(); //TODO
                                                                     //TODO: always call overwrite
            match default_data {
                DefaultData::Navigation(n) => exam.navigation.overwrite(&Some(n)),
                DefaultData::Timing(t) => exam.timing.overwrite(&Some(t)),
                DefaultData::Feedback(f) => exam.feedback.overwrite(&Some(f)),
                DefaultData::NumbasSettings(f) => exam.numbas_settings.overwrite(&Some(f)),
                DefaultData::Question(q) => {
                    if let Some(ref mut groups) = exam.question_groups {
                        groups.iter_mut().for_each(|qg| {
                            if let Some(ref mut questions) = &mut qg.questions {
                                questions.iter_mut().for_each(|question| {
                                    question.question_data.overwrite(&Some(q.clone()))
                                })
                            }
                        })
                    }
                }
                DefaultData::QuestionPart(p) => {
                    if let Some(ref mut groups) = exam.question_groups {
                        groups.iter_mut().for_each(|qg| {
                            if let Some(ref mut questions) = &mut qg.questions {
                                questions.iter_mut().for_each(|question| {
                                    if let Some(ref mut question_data) = question.question_data {
                                        if let Some(ref mut parts) = question_data.parts {
                                            parts.iter_mut().for_each(|part| {
                                                if let (
                                                    QuestionPart::GapFill(_),
                                                    QuestionPart::GapFill(_),
                                                ) = (&p, &part)
                                                {
                                                    part.overwrite(&p.clone())
                                                }
                                            })
                                        }
                                    }
                                })
                            }
                        })
                    }
                } //TODO: cleanup...
                DefaultData::QuestionPartGapFillGap(p) => {
                    if let Some(ref mut groups) = exam.question_groups {
                        groups.iter_mut().for_each(|qg| {
                            if let Some(ref mut questions) = &mut qg.questions {
                                questions.iter_mut().for_each(|question| {
                                    if let Some(ref mut question_data) = question.question_data {
                                        if let Some(ref mut parts) = question_data.parts {
                                            parts.iter_mut().for_each(|part| {
                                                if let QuestionPart::GapFill(gap_fill) = part {
                                                    if let Some(ref mut gaps) = gap_fill.gaps {
                                                        gaps.iter_mut().for_each(|gap| {
                                                            if let (
                                                                QuestionPart::JME(_),
                                                                QuestionPart::JME(_),
                                                            ) = (&p, &gap)
                                                            {
                                                                gap.overwrite(&p.clone())
                                                            }
                                                        })
                                                    }
                                                }
                                            })
                                        }
                                    }
                                })
                            }
                        })
                    }
                }
            }
        }
    }
}
