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
    pub num_color: Color,
    pub timestamp_color: Color,
    pub timestamp_format: &'static str,
    pub file_color: Color,
    pub line_color: Color,
}

impl Default for LoggerConfig {
    fn default() -> Self {
        let dark_blue = Color::TrueColor {
            r: 50,
            g: 100,
            b: 150,
        };
        let dark_grey = Color::TrueColor {
            r: 100,
            g: 100,
            b: 100,
        };

        Self {
            module: None,
            level: LevelFilter::Info,
            num_color: Color::BrightBlack,
            timestamp_format: "%H:%M:%S",
            file_color: dark_grey,
            line_color: dark_blue,
            timestamp_color: dark_blue,
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
        let line = format!(
            "{:4}",
            self.line.lock().expect("Failed to lock log # mutex")
        );
        let now = chrono::Local::now().format(self.config.timestamp_format);

        let line = format!(
            "{} {}",
            line.color(self.config.num_color),
            now.to_string().color(self.config.timestamp_color)
        );
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
        *self.line.lock().expect("Failed to lock log # mutex") += 1;
        let file = record.file().unwrap_or_default().to_string();
        let line = match record.line() {
            Some(line) => format!(":{}", line),
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
