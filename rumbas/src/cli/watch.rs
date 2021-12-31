use std::path::Path;

use notify::{watcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::time::Duration;
pub fn watch(matches: &clap::ArgMatches) {
    let path = Path::new(matches.value_of("PATH").unwrap());
    if path.is_absolute() {
        log::error!("Absolute path's are not supported");
        return;
    }

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
            Ok(event) => println!("{:?}", event),
            Err(e) => log::error!("watch error: {:?}", e),
        }
    }
}
