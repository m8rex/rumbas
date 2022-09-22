use super::compile::PassedRumbasCompileData;
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

#[derive(Debug, Clone)]
pub struct EditorOutputCompileData {
    pub locale: String,
    pub generated_path: PathBuf,
    pub generated_zip_path: PathBuf,
    pub exam_name: String,
    pub exam_path: PathBuf,
}

fn find_complete_outputs(
    scorm_compilation_result: InternalCompilationResult,
    folder_compilation_result: InternalCompilationResult,
    root: &PathBuf,
) -> Vec<EditorOutputCompileData> {
    let s = scorm_compilation_result.created_outputs;
    folder_compilation_result
        .created_outputs
        .into_iter()
        .filter_map(|result| {
            let scorm = s
                .iter()
                .find(|r| r.exam_path == result.exam_path && r.locale == result.locale);

            scorm.map(|scorm_result| EditorOutputCompileData {
                locale: result.locale,
                generated_path: result
                    .generated_path
                    .as_path()
                    .strip_prefix(root.clone())
                    .expect("stripping to work")
                    .to_owned(),
                generated_zip_path: scorm_result
                    .generated_path
                    .as_path()
                    .strip_prefix(root.clone())
                    .expect("stripping to work")
                    .to_owned(),
                exam_name: result.exam_name,
                exam_path: result.exam_path,
            })
        })
        .collect()
}

fn find_git_url() -> Option<String> {
    let repo = git2::Repository::discover(".").ok()?;
    let remote = repo.find_remote("origin").ok().or_else(|| {
        repo.remotes()
            .ok()
            .and_then(|remotes| remotes.get(0).and_then(|name| repo.find_remote(name).ok()))
    });
    remote.and_then(|r| r.url().map(|r| r.to_string()))
}

pub fn create_editor_output_internal(context: EditorOutputContext) -> Result<(), &'static str> {
    let root = find_root(&Path::new("."));
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
        find_complete_outputs(scorm_compilation_result, folder_compilation_result, &root);

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

    let remote_url = find_git_url().unwrap_or_default();
    let remote_url = remote_url
        .replace(":", "/")
        .replace(r"git@", "https://")
        .replace(".git", "");

    let available_exams_path = context.output_path.join("available_exams.json");
    let exams: Vec<_> = matching
        .into_iter()
        .map(|result| ApiExam::new(result, &context.url_prefix, &remote_url))
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
    pub fn new(
        compile_result: EditorOutputCompileData,
        url_prefix: &String,
        git_prefix: &String,
    ) -> Self {
        let url = format!("{}/{}", url_prefix, compile_result.generated_path.display());
        let preview_url = url.clone();
        let project_url = url.clone();
        let zip_url = format!(
            "{}/{}",
            url_prefix,
            compile_result.generated_zip_path.display()
        );
        let edit_url = format!(
            "{}/tree/master/{}",
            git_prefix,
            compile_result.exam_path.display()
        );
        Self {
            url,
            name: compile_result.exam_name.to_owned(),
            project_url,
            edit_url,
            author: ApiUser::default(),
            metadata: ApiMetadata {
                description: compile_result.exam_path.display().to_string(),
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
