use log::info;
use simplelog::*;
use std::fs::File;


pub fn init_logging(){
    let log_file = File::create("log").unwrap();
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            WriteLogger::new(LevelFilter::Info, Config::default(), log_file),
        ]
    ).unwrap();
    info!("#################################");
    info!("Logging initialized");
}