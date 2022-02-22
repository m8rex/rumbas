use super::compile::{
    compile_internal, CompilationContext, FileCompilationContext, InternalCompilationResult,
};
use crate::cli::check::CheckResult;
use rayon::prelude::*;
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
    pub output_path: String,
}

impl From<clap::ArgMatches> for EditorOutputContext {
    fn from(matches: clap::ArgMatches) -> Self {
        Self {
            output_path: matches.value_of("OUTPUT_PATH").unwrap().to_string(),
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

    println!("{:?}", scorm_compilation_result.created_output_paths);
    println!("{:?}", folder_compilation_result.created_output_paths);
    let matching = find_complete_outputs(scorm_compilation_result, folder_compilation_result);
    println!("{:?}", matching);
    Ok(())
}
