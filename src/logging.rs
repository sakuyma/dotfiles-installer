use log::LevelFilter;
use chrono::Local;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};

static LOG_ENABLED: AtomicBool = AtomicBool::new(false);

pub fn init(log_enabled: bool) -> Result<(), Box<dyn std::error::Error>> {
    if !log_enabled {
        LOG_ENABLED.store(false, Ordering::SeqCst);
        return Ok(());
    }
    
    LOG_ENABLED.store(true, Ordering::SeqCst);
    
    // Create cache directory
    let home = std::env::var("HOME")?;
    let log_dir = PathBuf::from(home)
        .join(".cache")
        .join("dotfiles-installer");
    
    fs::create_dir_all(&log_dir)?;
    
    let log_file = log_dir.join("install.log");
    
    // Create archive directory for old logs
    let archive_dir = log_dir.join("archive");
    fs::create_dir_all(&archive_dir)?;
    
    // Archive old log if it exists
    if log_file.exists() {
        let timestamp = Local::now().format("%Y%m%d_%H%M%S");
        let archive_path = archive_dir.join(format!("install_{}.log", timestamp));
        fs::rename(&log_file, &archive_path)?;
    }
    
    // Setup fern logger
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}] [{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                message
            ))
        })
        .level(LevelFilter::Info)
        .chain(fern::log_file(log_file)?)
        .chain(std::io::stdout())
        .apply()?;
    
    Ok(())
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
            log::info!("[STEP] {}", format_args!($($arg)*));
        }
    };
}

#[macro_export]
macro_rules! log_success {
    ($($arg:tt)*) => {
        if $crate::logging::is_log_enabled() {
            log::info!("[SUCCESS] {}", format_args!($($arg)*));
        }
    };
}

#[macro_export]
macro_rules! log_warning {
    ($($arg:tt)*) => {
        if $crate::logging::is_log_enabled() {
            log::warn!("[WARNING] {}", format_args!($($arg)*));
        }
    };
}

#[macro_export]
macro_rules! log_progress {
    ($($arg:tt)*) => {
        if $crate::logging::is_log_enabled() {
            log::info!("[PROGRESS] {}", format_args!($($arg)*));
        }
    };
}
