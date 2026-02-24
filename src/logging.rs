use chrono::Local;
use log::LevelFilter;
use std::sync::atomic::{AtomicBool, Ordering};

static LOG_ENABLED: AtomicBool = AtomicBool::new(false);

pub fn init(log_enabled: bool) {
    if !log_enabled {
        LOG_ENABLED.store(false, Ordering::SeqCst);
        return;
    }

    LOG_ENABLED.store(true, Ordering::SeqCst);

    env_logger::Builder::new()
        .filter_level(LevelFilter::Info)
        .format(|buf, record| {
            use std::io::Write;

            let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
            let level = match record.level() {
                log::Level::Error => "ERROR",
                log::Level::Warn => "WARN",
                log::Level::Info => "INFO",
                log::Level::Debug => "DEBUG",
                log::Level::Trace => "TRACE",
            };

            writeln!(buf, "[{}] {} {}", timestamp, level, record.args())
        })
        .init();
}

pub fn is_log_enabled() -> bool {
    LOG_ENABLED.load(Ordering::SeqCst)
}

#[macro_export]
macro_rules! output {
    ($($arg:tt)*) => {
        if $crate::logging::is_log_enabled() {
            log::info!($($arg)*);
        } else {
            println!($($arg)*);
        }
    };
}

#[macro_export]
macro_rules! error_output {
    ($($arg:tt)*) => {
        if $crate::logging::is_log_enabled() {
            log::error!($($arg)*);
        } else {
            eprintln!($($arg)*);
        }
    };
}

#[macro_export]
macro_rules! log_step {
    ($($arg:tt)*) => {
        if $crate::logging::is_log_enabled() {
            log::info!("{}", format_args!($($arg)*));
        }
    };
}

#[macro_export]
macro_rules! log_success {
    ($($arg:tt)*) => {
        if $crate::logging::is_log_enabled() {
            log::info!("{}", format_args!($($arg)*));
        }
    };
}

#[macro_export]
macro_rules! log_warning {
    ($($arg:tt)*) => {
        if $crate::logging::is_log_enabled() {
            log::warn!("{}", format_args!($($arg)*));
        }
    };
}

#[macro_export]
macro_rules! log_progress {
    ($($arg:tt)*) => {
        if $crate::logging::is_log_enabled() {
            log::info!("{}", format_args!($($arg)*));
        }
    };
}
