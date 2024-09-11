use std::{
    io::{Error, Write},
    sync::Mutex,
};

use colog::format::CologStyle;
pub use colored::Color;
use colored::Colorize;
use log::{Level, LevelFilter, Record};

pub struct LoggerConfig {
    pub module: Option<&'static str>,
    pub level: LevelFilter,
    pub file_color: Color,
    pub line_color: Color,
}

impl Default for LoggerConfig {
    fn default() -> Self {
        Self {
            module: None,
            level: LevelFilter::Info,
            file_color: Color::BrightBlack,
            line_color: Color::BrightBlue,
        }
    }
}

pub fn init_logger(config: LoggerConfig) {
    let mut builder = env_logger::Builder::new();
    builder.filter(config.module, config.level);
    builder.format(colog::formatter(CustomStatefulLogger::new(config)));
    builder.init();
}

#[derive(Default)]
pub struct CustomStatefulLogger {
    line: Mutex<usize>,
    config: LoggerConfig,
}

impl CustomStatefulLogger {
    fn new(config: LoggerConfig) -> Self {
        CustomStatefulLogger {
            config,
            ..Default::default()
        }
    }
}

impl CologStyle for CustomStatefulLogger {
    fn level_token(&self, level: &Level) -> &str {
        match *level {
            Level::Error => "ERR",
            Level::Warn => "WRN",
            Level::Info => "INF",
            Level::Debug => "DBG",
            Level::Trace => "TRC",
        }
    }

    fn prefix_token(&self, level: &Level) -> String {
        let line = self.line.lock().unwrap();
        let now = chrono::Local::now().format("%H:%M:%S");

        let line = format!("{:4} {}", line, now);
        format!(
            "{} {}",
            line,
            colog::format::default_prefix_token(self, level)
        )
    }

    fn format(
        &self,
        buf: &mut env_logger::fmt::Formatter,
        record: &Record<'_>,
    ) -> Result<(), Error> {
        *self.line.lock().unwrap() += 1;
        let file = record.file().unwrap_or_default().to_string();
        let line = match record.line() {
            Some(line) => format!(":{}", line.to_string()),
            None => "".to_string(),
        };
        let sep = self.line_separator();
        let prefix = self.prefix_token(&record.level());

        // default_format(self, buf, record)
        buf.write_fmt(format_args!(
            "{} {}\n     {}{}\n",
            prefix,
            record.args().to_string().replace('\n', &sep),
            file.color(self.config.file_color),
            line.color(self.config.line_color)
        ))
    }
}
