use colored::*;
use std::sync::atomic::{AtomicBool, Ordering};

static QUIET_MODE: AtomicBool = AtomicBool::new(false);

pub fn is_quiet() -> bool {
    QUIET_MODE.load(Ordering::SeqCst)
}

pub fn print_success(msg: &str) {
    if !is_quiet() {
        println!("{}", msg.green());
    }
}

pub fn print_error(msg: &str) {
    // Errors are always shown, even in quiet mode
    eprintln!("{}", msg.red());
}

pub fn print_warning(msg: &str) {
    if !is_quiet() {
        println!("{}", msg.yellow());
    }
}

pub fn print_progress(msg: &str) {
    if !is_quiet() {
        println!("{}", msg.cyan());
    }
}
