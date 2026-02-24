use log::LevelFilter;
use chrono::Local;

pub fn init(verbose: bool, quiet: bool) {
    let level = if quiet {
        LevelFilter::Error
    } else if verbose {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };
    
    env_logger::Builder::new()
        .filter_level(level)
        .format(|buf, record| {
            use std::io::Write;
            writeln!(
                buf,
                "{} [{:5}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .init();
}

// Macroses
#[macro_export]
macro_rules! log_step {
    ($($arg:tt)*) => {
        log::info!("🔧 {}", format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! log_success {
    ($($arg:tt)*) => {
        log::info!("{}", format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! log_warning {
    ($($arg:tt)*) => {
        log::warn!("{}", format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        log::error!("{}", format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        log::debug!("{}", format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! log_progress {
    ($($arg:tt)*) => {
        log::info!("{}", format_args!($($arg)*));
    };
}
