// src/logging.rs

use flexi_logger::{Logger, WriteMode, FileSpec};
use log::LevelFilter;

pub fn init_logging() {
    Logger::try_with_str("debug")
        .unwrap()
        .log_to_file(FileSpec::default().basename("logs").suffix("log"))
        .write_mode(WriteMode::BufferAndFlush) // Буферизация для оптимизации записи
        .start()
        .expect("Не удалось инициализировать flexi_logger");
}
