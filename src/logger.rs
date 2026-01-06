use chrono::Local;

pub struct Logger {
    verbose: bool,
}

impl Logger {
    pub fn new(verbose: bool) -> Logger {
        Logger { verbose }
    }

    #[allow(dead_code)]
    pub fn log_info(&self, message: &str) {
        self.base_log("INFO", message);
    }

    #[allow(dead_code)]
    pub fn log_debug(&self, message: &str) {
        self.base_log("DEBUG", message);
    }

    #[allow(dead_code)]
    pub fn log_warning(&self, message: &str) {
        self.base_log("WARNING", message);
    }

    #[allow(dead_code)]
    pub fn log_error(&self, message: &str) {
        self.base_log("ERROR", message);
    }

    #[allow(dead_code)]
    fn base_log(&self, log_level: &str, message: &str) {
        if self.verbose {
            let time_stamp: chrono::format::DelayedFormat<chrono::format::StrftimeItems<'_>> =
                Local::now().format("%Y-%m-%d %H:%M:%S");
            println!("{time_stamp} [{log_level}]: {message}");
        }
    }
}
