use log::info;
use simplelog::*;
use std::fs::File;

pub fn create_logger() {
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Debug,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create("sorume-server.log").unwrap(),
        ),
    ])
    .unwrap();

    info!("Simplelog has been started successfully");
}
