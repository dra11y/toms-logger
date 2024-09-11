use std::{
    io::{Error, Write},
    sync::Mutex,
};

use colog::format::CologStyle;
use colored::Colorize;
use log::{Level, LevelFilter, Record};

pub fn init_logger(module: Option<&str>, level: LevelFilter) {
    let mut builder = env_logger::Builder::new();
    builder.format(colog::formatter(CustomStatefulLogger::default()));
    builder.filter(module, level);
    builder.init();
}

#[derive(Default)]
pub struct CustomStatefulLogger {
    line: Mutex<usize>,
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
            file.bright_black(),
            line.blue()
        ))
    }
}
