use crate::cli::compile::{CompilationContext, FileCompilationContext};
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use rumbas::support::dependency_manager::DEPENDENCIES;
use rumbas::support::file_manager::RumbasRepoFileData;
use rumbas::support::file_manager::CACHE;
use rumbas_support::input::FileToLoad;
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;

pub fn watch(matches: &clap::ArgMatches) {
    watch_internal(matches.to_owned().into())
}

#[derive(Debug, Clone)]
pub struct WatchContext {
    pub watch_path: String,
    pub only_check: bool,
}

impl From<clap::ArgMatches> for WatchContext {
    fn from(matches: clap::ArgMatches) -> Self {
        Self {
            watch_path: matches.value_of("PATH").unwrap().to_string(),
            only_check: matches.is_present("only_check"),
        }
    }
}

fn watch_internal(context: WatchContext) {
    let path = Path::new(".");
    if path.is_absolute() {
        // TODO
        log::error!("Absolute path's are not supported");
        return;
    }

    let handler: Box<dyn WatchHandler> = if context.only_check {
        Box::new(WatchChecker)
    } else {
        Box::new(WatchCompiler)
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
                DebouncedEvent::Create(p) => handle_if_needed(&p, &handler), // Shouldn'tdo anything?
                DebouncedEvent::Write(p) => handle_if_needed(&p, &handler),
                DebouncedEvent::Chmod(p) => handle_if_needed(&p, &handler),
                DebouncedEvent::Remove(p) => handle_if_needed(&p, &handler),
                DebouncedEvent::Rename(previous, new) => (), // TODO do the rename in the dependencies?
                _ => (),
            },
            Err(e) => log::error!("watch error: {:?}", e),
        }
    }
}

fn to_relative_path(path: &Path) -> std::path::PathBuf {
    path.strip_prefix(std::env::current_dir().unwrap())
        .unwrap()
        .to_owned()
}

fn handle_if_needed(path: &Path, handler: &Box<dyn WatchHandler>) {
    let relative_path = to_relative_path(path);
    if relative_path.starts_with(crate::cli::compile::CACHE_FOLDER) {
        return ();
    } else if relative_path.starts_with(crate::cli::compile::OUTPUT_FOLDER) {
        return ();
    } else {
        handler.recompile_dependant(path)
    }
}

trait WatchHandler {
    fn handle_setup(&self, path: &str);
    fn handle_file(&self, path: &Path);
    fn recompile_dependant(&self, path: &Path) {
        let relative_path = to_relative_path(path);
        let file_data = RumbasRepoFileData::from(&relative_path);
        let relative_path = file_data.dependency_path();

        let file_to_remove: FileToLoad = file_data.into();
        CACHE.delete_file(file_to_remove);

        DEPENDENCIES.log_debug();

        let dependants = DEPENDENCIES.get_dependants(relative_path.to_path_buf());
        for dependant in dependants.iter() {
            self.handle_file(dependant);
        }
    }
}

pub struct WatchChecker;
impl WatchHandler for WatchChecker {
    fn handle_setup(&self, path: &str) {
        // TODO
        crate::cli::check::check_internal(path);
    }
    fn handle_file(&self, path: &Path) {
        crate::cli::check::check_file(path);
    }
}

pub struct WatchCompiler;
impl WatchCompiler {
    fn file_context() -> FileCompilationContext {
        FileCompilationContext {
            use_scorm: false,
            as_zip: false,
            minify: false,
        }
    }
}
impl WatchHandler for WatchCompiler {
    fn handle_setup(&self, path: &str) {
        // TODO
        crate::cli::compile::compile_internal(
            CompilationContext {
                compile_path: path.to_string(),
            },
            Self::file_context(),
        );
    }
    fn handle_file(&self, path: &Path) {
        crate::cli::compile::compile_file(&Self::file_context(), path);
    }
}