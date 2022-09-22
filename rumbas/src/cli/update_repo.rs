use rumbas::support::rc;
use rumbas::updates;
use std::path::Path;

pub fn update_repo() {
    let rc_res = rc::read(Path::new("."));
    match rc_res {
        Ok(rc) => {
            let new_rc_opt = updates::update(rc);
            if let Some(new_rc) = new_rc_opt {
                new_rc.write().expect("Failed writing rc file");
            } else {
                log::info!("No updates needed!")
            }
        }
        Err(e) => {
            log::error!("Could not parse rc file: {}", e);
            std::process::exit(1)
        }
    }
}
