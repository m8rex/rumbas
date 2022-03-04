use super::compile::{
    compile_internal, CompilationContext, FileCompilationContext, InternalCompilationResult,
};
use crate::cli::check::CheckResult;
use rayon::prelude::*;
use rumbas::support::rc::find_root;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::env;
use std::path::Path;
use std::path::PathBuf;

pub fn create_editor_output(matches: &clap::ArgMatches) {
    match create_editor_output_internal(matches.to_owned().into()) {
        Ok(_) => (),
        Err(_) => std::process::exit(1),
    }
}

#[derive(Debug, Clone)]
pub struct EditorOutputContext {
    pub output_path: PathBuf,
    pub url_prefix: String,
}

impl From<clap::ArgMatches> for EditorOutputContext {
    fn from(matches: clap::ArgMatches) -> Self {
        Self {
            output_path: Path::new(&matches.value_of("OUTPUT_PATH").unwrap().to_string())
                .to_path_buf(),
            url_prefix: matches.value_of("URL_PREFIX").unwrap().to_string(),
        }
    }
}

fn find_complete_outputs(
    scorm_compilation_result: InternalCompilationResult,
    folder_compilation_result: InternalCompilationResult,
) -> Vec<PathBuf> {
    let s = scorm_compilation_result
        .created_output_paths
        .into_iter()
        .collect::<HashSet<_>>();
    folder_compilation_result
        .created_output_paths
        .into_iter()
        .filter(|p| {
            let scorm = p.with_extension("zip");
            s.contains(&scorm)
        })
        .collect()
}

pub fn create_editor_output_internal(context: EditorOutputContext) -> Result<(), ()> {
    let root = find_root();
    println!("{:?}", root);

    let compile_paths = vec!["exams".to_string()];
    println!("Compiling scorm packages.");
    let scorm_compilation_result = compile_internal(
        CompilationContext {
            compile_paths: compile_paths.clone(),
        },
        FileCompilationContext {
            use_scorm: true,
            as_zip: true,
            minify: true,
        },
    );

    println!("Compiling (preview) exam folders.");
    let folder_compilation_result = compile_internal(
        CompilationContext {
            compile_paths: compile_paths.clone(),
        },
        FileCompilationContext {
            use_scorm: false,
            as_zip: false,
            minify: false,
        },
    );

    let matching = find_complete_outputs(scorm_compilation_result, folder_compilation_result);
    println!("{:?}", matching);

    let handshake = ApiHandshake::default();
    let handshake_path = context.output_path.join("handshake.json");
    handshake.write(&handshake_path);

    let projects = vec![ApiProject::new(0, matching.len())];
    let project_path = context.output_path.join("projects.json");
    let s = serde_json::to_string_pretty(&projects).expect("Json generation failed");
    std::fs::write(project_path, s).expect("Writing file failed");

    let available_exams_path = context.output_path.join("available_exams.json");
    let exams: Vec<_> = matching
        .into_iter()
        .map(|p| ApiExam::new(&p, &context.url_prefix))
        .collect();
    let s = serde_json::to_string_pretty(&exams).expect("Json generation failed");
    std::fs::write(available_exams_path, s).expect("Writing file failed");

    Ok(())
}

#[derive(Clone, Serialize, Deserialize)]
struct ApiHandshake {
    numbas_editor: usize,     // always 1
    site_title: &'static str, // for now
}

impl Default for ApiHandshake {
    fn default() -> Self {
        Self {
            numbas_editor: 1,
            site_title: "Rumbas editor output result",
        }
    }
}

impl ApiHandshake {
    fn write(&self, file: &Path) {
        let s = serde_json::to_string_pretty(self).expect("Json generation failed");
        std::fs::write(file, s).expect("Writing file failed");
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct ApiProject {
    name: String,
    #[serde(rename = "pk")]
    remote_id: usize,
    description: String,
    homepage: String,
    url: String,
    owner: ApiUser,
    num_questions: usize,
    num_exams: usize,
}

impl ApiProject {
    fn new(num_questions: usize, num_exams: usize) -> Self {
        Self {
            name: "Main project".to_string(),
            remote_id: 0,
            description: "All exams in the rumbas repo".to_string(),
            homepage: "".to_string(),
            url: "".to_string(),
            owner: Default::default(),
            num_questions,
            num_exams,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct ApiUser {
    #[serde(rename = "profile")]
    profile_url: String,
    full_name: String,
}

impl Default for ApiUser {
    fn default() -> Self {
        Self {
            profile_url: "".to_string(),
            full_name: "Rumbas".to_string(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct ApiExam {
    pub url: String,
    pub name: String,
    #[serde(rename = "project")]
    pub project_url: String,
    #[serde(rename = "edit")]
    pub edit_url: String,
    pub author: ApiUser,
    pub metadata: ApiMetadata,
    #[serde(rename = "download")]
    pub zip_url: String,
    #[serde(rename = "preview")]
    pub preview_url: String,
}

impl ApiExam {
    pub fn new(p: &Path, url_prefix: &String) -> Self {
        let url = format!("{}/{}", url_prefix, p.display());
        let zip_url = format!("{}/{}", url_prefix, p.display());
        let preview_url = format!("{}/{}", url_prefix, p.display());
        Self {
            url,
            name: "todo".to_string(),
            project_url: "todo".to_string(),
            edit_url: "todo".to_string(),
            author: ApiUser::default(),
            metadata: ApiMetadata {
                description: "".to_string(),
            },
            zip_url,
            preview_url,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct ApiMetadata {
    pub description: String,
}
