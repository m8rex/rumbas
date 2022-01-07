use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use rumbas::support::dependency_manager::DEPENDENCIES;
use rumbas::support::file_manager::RumbasRepoFileData;
use rumbas::support::file_manager::CACHE;
use rumbas_support::input::FileToLoad;
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;

pub fn watch(matches: &clap::ArgMatches) {
    let path = Path::new(".");
    if path.is_absolute() {
        log::error!("Absolute path's are not supported");
        return;
    }

    crate::cli::check::check_internal(matches.value_of("PATH").unwrap());

    log::info!("Watching {:?}", path.display());

    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Create a watcher object, delivering debounced events.
    // The notification back-end is selected based on the platform.
    let mut watcher = watcher(tx, Duration::from_secs(5)).unwrap();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(path, RecursiveMode::Recursive).unwrap();

    loop {
        match rx.recv() {
            Ok(event) => match event {
                DebouncedEvent::Create(p) => recompile_dependant(&p), // Shouldn'tdo anything?
                DebouncedEvent::Write(p) => recompile_dependant(&p),
                DebouncedEvent::Chmod(p) => recompile_dependant(&p),
                DebouncedEvent::Remove(p) => recompile_dependant(&p),
                DebouncedEvent::Rename(previous, new) => (), // TODO do the rename in the dependencies?
                _ => (),
            },
            Err(e) => log::error!("watch error: {:?}", e),
        }
    }
}

fn recompile_dependant(path: &Path) {
    // TODO: remove from file cache
    let relative_path = path.strip_prefix(std::env::current_dir().unwrap()).unwrap();
    let file_data = RumbasRepoFileData::from(&relative_path);
    let relative_path = file_data.dependency_path();

    let file_to_remove: FileToLoad = file_data.into();
    CACHE.delete_file(file_to_remove);

    DEPENDENCIES.log_debug();

    let dependants = DEPENDENCIES.get_dependants(relative_path.to_path_buf());
    for dependant in dependants.iter() {
        crate::cli::check::check_file(dependant);
    }
}
