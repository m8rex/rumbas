use crate::cli::compile::{CompilationContext, FileCompilationContext};
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use rumbas::support::dependency_manager::DEPENDENCIES;
use rumbas::support::file_manager::RumbasRepoFileData;
use rumbas::support::file_manager::CACHE;
use rumbas::support::rc::within_repo;
use rumbas_support::input::FileToLoad;
use rumbas_support::path::RumbasPath;
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;

pub fn watch(watch_path: String, only_check: bool) {
    watch_internal(WatchContext {
        watch_path,
        only_check,
    })
}

#[derive(Debug, Clone)]
pub struct WatchContext {
    pub watch_path: String,
    pub only_check: bool,
}

fn watch_internal(context: WatchContext) {
    let path = Path::new(".");
    if path.is_absolute() {
        // TODO
        log::error!("Absolute path's are not supported");
        return;
    }

    let handler: &dyn WatchHandler = if context.only_check {
        &WatchChecker
    } else {
        &WatchCompiler
    };

    handler.handle_setup(&context.watch_path);

    log::info!("Watching {:?}", path.display());

    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Create a watcher object, delivering debounced events.
    // The notification back-end is selected based on the platform.
    let mut watcher = watcher(tx, Duration::from_secs(2)).unwrap();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(path, RecursiveMode::Recursive).unwrap();

    loop {
        match rx.recv() {
            Ok(event) => match event {
                DebouncedEvent::Create(p) => handle_if_needed(&p, handler), // Shouldn't do anything?
                DebouncedEvent::Write(p) => handle_if_needed(&p, handler),
                DebouncedEvent::Chmod(p) => handle_if_needed(&p, handler),
                DebouncedEvent::Remove(p) => handle_if_needed(&p, handler),
                DebouncedEvent::Rename(_previous, _new) => (), // TODO do the rename in the dependencies?
                _ => (),
            },
            Err(e) => log::error!("watch error: {:?}", e),
        }
    }
}

fn handle_if_needed(path: &Path, handler: &dyn WatchHandler) {
    if let Some(path) = within_repo(&path) {
        if path.in_main_folder(crate::cli::compile::CACHE_FOLDER)
            || path.in_main_folder(crate::cli::compile::OUTPUT_FOLDER)
        {
            // do nothing
        } else {
            handler.recompile_dependant(path)
        }
    }
}

trait WatchHandler {
    fn handle_setup(&self, path: &str);
    fn handle_file(&self, path: &RumbasPath);
    fn recompile_dependant(&self, path: RumbasPath) {
        let file_data = RumbasRepoFileData::from(path.clone());
        let relative_path = file_data.dependency_path();

        let file_to_remove: FileToLoad = file_data.into();
        CACHE.delete_file(file_to_remove);

        DEPENDENCIES.log_debug();

        let dependants = DEPENDENCIES.get_dependants(path);
        for dependant in dependants.iter() {
            self.handle_file(dependant);
        }
    }
}

pub struct WatchChecker;
impl WatchHandler for WatchChecker {
    fn handle_setup(&self, path: &str) {
        // TODO
        crate::cli::check::check_internal(vec![path.to_string()]);
    }
    fn handle_file(&self, path: &RumbasPath) {
        crate::cli::check::check_file(&path);
    }
}

pub struct WatchCompiler;
impl WatchCompiler {
    fn file_context() -> FileCompilationContext {
        FileCompilationContext {
            use_scorm: false,
            as_zip: false,
            minify: false,
            output_folder: Path::new(crate::cli::compile::OUTPUT_FOLDER).to_path_buf(),
        }
    }
}
impl WatchHandler for WatchCompiler {
    fn handle_setup(&self, path: &str) {
        // TODO
        crate::cli::compile::compile_internal(
            CompilationContext {
                compile_paths: vec![path.to_string()],
            },
            Self::file_context(),
        );
    }
    fn handle_file(&self, path: &RumbasPath) {
        crate::cli::compile::compile_file(&Self::file_context(), &path);
    }
}
