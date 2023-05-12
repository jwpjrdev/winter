use log::{Record, Level, Metadata};

use crate::cli::OutputFormat;

pub struct WinterLogger {
    output_format: OutputFormat,
}

impl log::Log for WinterLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::max()
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            if self.output_format == OutputFormat::Human {
                println!("{} - {}", record.level(), record.args());
            } else {
                use std::io::Write;
                // TODO: test conversion from arguments to string
                std::io::stdout().write(record.args().to_string().as_bytes());
            }
        }
    }

    fn flush(&self) {}
}
