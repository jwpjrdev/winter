use clap::ValueEnum;
use colored::*;
use log::{LevelFilter, Metadata, Record, SetLoggerError, Level};
use std::fmt::Write;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum OutputFormat {
    #[default]
    Human,
    Json,
    JsonLines,
}

pub struct WinterLogger {
    output_format: OutputFormat,
    max_log_level: LevelFilter,
}

impl WinterLogger {
    pub fn init(output_format: OutputFormat, max_log_level: LevelFilter) -> Result<(), SetLoggerError> {
        let logger = Box::new(WinterLogger {
            output_format: output_format,
            max_log_level: max_log_level,
        });

        log::set_max_level(max_log_level);
        log::set_boxed_logger(logger)
    }
}

impl log::Log for WinterLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        // Trace > debug > info > warn > error
        metadata.level() <= self.max_log_level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            if self.output_format == OutputFormat::Human {
                let result = LogFormatter::format(record);
                println!("{result}");
            } else {
                use std::io::Write;
                std::io::stdout()
                    .write(record.args().to_string().as_bytes())
                    .unwrap();
            }
        }
    }

    fn flush(&self) {}
}

trait FormattedLevel {
    fn format_level(&self) -> ColoredString;
}

impl FormattedLevel for Level {
    fn format_level(&self) -> ColoredString {
        let string = self.to_string().to_lowercase().bold();
        match self {
            Level::Trace => string.blue(),
            Level::Debug => string.magenta(),
            Level::Info => string.green(),
            Level::Warn => string.yellow(),
            Level::Error => string.red(),
        }
    }
}

#[allow(unused_variables)]
struct LogFormatter<'a> {
    result: String,
    record: Record<'a>,
}

// #[allow(dead_code)]
impl<'a> LogFormatter<'a> {
    fn new(record: Record) -> LogFormatter {
        LogFormatter { result: String::new(), record: record }
    }
    
    pub fn format(record: &Record) -> String {
        let record2 = record.clone();
        let mut formatter = LogFormatter::new(record2);
        formatter.write_all();

        formatter.result
    }
    
    fn write_all(&mut self) {
        write!(self.result, "{}: ", self.record.level().format_level()).unwrap();
        if self.record.level() == Level::Trace {
            write!(self.result, "{} ", self.record.target()).unwrap();
            
            let file = self.record.file().unwrap_or("<unknown>");
            if let Some(line) = self.record.line() {
                write!(self.result, "[{}:{}]: ", file, line).unwrap();
            } else {
                write!(self.result, "[{}:<unknown>]: ", file).unwrap();
            }
        }
        
        write!(self.result, "{}", self.record.args()).unwrap();
    }
}