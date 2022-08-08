use super::compile::{
    compile_internal, CompilationContext, FileCompilationContext, InternalCompilationResult,
};
use rumbas::support::rc::find_root;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::Path;
use std::path::PathBuf;

pub fn create_editor_output(output_path: String, url_prefix: String) {
    match create_editor_output_internal(EditorOutputContext {
        output_path: Path::new(&output_path).to_path_buf(),
        url_prefix,
    }) {
        Ok(_) => (),
        Err(_) => std::process::exit(1),
    }
}

#[derive(Debug, Clone)]
pub struct EditorOutputContext {
    pub output_path: PathBuf,
    pub url_prefix: String,
}

fn find_complete_outputs(
    scorm_compilation_result: InternalCompilationResult,
    folder_compilation_result: InternalCompilationResult,
) -> Vec<(PathBuf, String)> {
    let s = scorm_compilation_result
        .created_outputs
        .into_iter()
        .collect::<HashSet<_>>();
    folder_compilation_result
        .created_outputs
        .into_iter()
        .filter(|(p, _)| {
            let scorm = p.with_extension("zip");
            s.iter().any(|(p, _)| p == &scorm)
        })
        .collect()
}

pub fn create_editor_output_internal(context: EditorOutputContext) -> Result<(), &'static str> {
    let root = find_root();
    let root = root.ok_or("Missing rc file")?;
    println!("{:?}", root);

    let compile_paths = vec!["exams".to_string()];
    println!("Compiling scorm packages for exams.");
    let scorm_compilation_result = compile_internal(
        CompilationContext {
            compile_paths: compile_paths.clone(),
        },
        FileCompilationContext {
            use_scorm: true,
            as_zip: true,
            minify: true,
            output_folder: context.output_path.clone(),
        },
    );

    println!("Compiling (preview) exam html-outputs.");
    let folder_compilation_result = compile_internal(
        CompilationContext { compile_paths },
        FileCompilationContext {
            use_scorm: false,
            as_zip: false,
            minify: false,
            output_folder: context.output_path.clone(),
        },
    );

    let matching: Vec<_> =
        find_complete_outputs(scorm_compilation_result, folder_compilation_result)
            .into_iter()
            .map(|(p, name)| {
                (
                    p.as_path()
                        .strip_prefix(root.clone())
                        .expect("stripping to work")
                        .to_owned(),
                    name,
                )
            })
            .collect();

    if !context.output_path.exists() {
        std::fs::create_dir(&context.output_path).expect("creating a folder to work");
    }

    let handshake = ApiHandshake::default();
    let handshake_path = context.output_path.join("handshake.json");
    handshake.write(&handshake_path);

    let projects = vec![ApiProject::new(0, matching.len(), &context.url_prefix)];
    let project_path = context.output_path.join("projects.json");
    let s = serde_json::to_string_pretty(&projects).expect("Json generation failed");
    std::fs::write(project_path, s).expect("Writing file failed");

    let available_exams_path = context.output_path.join("available_exams.json");
    let exams: Vec<_> = matching
        .into_iter()
        .map(|(p, name)| ApiExam::new(&p, &name, &context.url_prefix))
        .collect();
    let s = serde_json::to_string_pretty(&exams).expect("Json generation failed");
    std::fs::write(available_exams_path, s).expect("Writing file failed");

    Ok(()) // TODO: fail if not everything compiled?
}

#[derive(Clone, Serialize, Deserialize)]
struct ApiHandshake {
    numbas_editor: usize, // always 1 for now
    site_title: &'static str,
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
    fn new(num_questions: usize, num_exams: usize, url_prefix: &String) -> Self {
        Self {
            name: "Main project".to_string(),
            remote_id: 0,
            description: "All exams in the rumbas repo".to_string(),
            homepage: url_prefix.to_string(),
            url: url_prefix.to_string(),
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
    pub fn new(p: &Path, name: &String, url_prefix: &String) -> Self {
        let url = format!("{}/{}", url_prefix, p.display());
        let zip_url = format!("{}.zip", url);
        let preview_url = url.clone();
        Self {
            url,
            name: name.to_owned(),
            project_url: url_prefix.to_string(),
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
