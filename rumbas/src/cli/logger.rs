// See https://github.com/daboross/fern/blob/master/examples/pretty-colored.rs
pub fn setup(level: log::LevelFilter) -> Result<(), fern::InitError> {
    // configure colors for the whole line
    let colors_line = fern::colors::ColoredLevelConfig::new()
        .error(fern::colors::Color::Red)
        .warn(fern::colors::Color::Yellow)
        // we actually don't need to specify the color for debug and info, they are white by default
        .info(fern::colors::Color::White)
        .debug(fern::colors::Color::White)
        // depending on the terminals color scheme, this is the same as the background color
        .trace(fern::colors::Color::BrightBlack);

    // configure colors for the name of the level.
    // since almost all of them are the same as the color for the whole line, we
    // just clone `colors_line` and overwrite our changes
    let colors_level = colors_line.info(fern::colors::Color::Green);

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{color_line}{date}[{target}][{level}{color_line}] {message}\x1B[0m",
                color_line = format_args!(
                    "\x1B[{}m",
                    colors_line.get_color(&record.level()).to_fg_str()
                ),
                date = chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                target = record.target(),
                level = colors_level.color(record.level()),
                message = message
            ))
        })
        .level(level)
        .chain(std::io::stdout())
        //.chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}
