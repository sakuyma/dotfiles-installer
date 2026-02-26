use colored::*;
use std::sync::atomic::{AtomicBool, Ordering};

static QUIET_MODE: AtomicBool = AtomicBool::new(false);

pub fn set_quiet(quiet: bool) {
    QUIET_MODE.store(quiet, Ordering::SeqCst);
}

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

pub fn print_info(msg: &str) {
    if !is_quiet() {
        println!("{}", msg.blue());
    }
}

pub fn print_key_value(key: &str, value: &str) {
    if !is_quiet() {
        println!("  {}: {}", key.cyan(), value.white());
    }
}

// For table output - suppress entirely in quiet mode
pub fn print_table(headers: &[&str], rows: &[Vec<String>]) {
    if is_quiet() {
        return;
    }

    if rows.is_empty() {
        return;
    }

    // Calculate column widths
    let mut col_widths: Vec<usize> = headers.iter().map(|h| h.len()).collect();
    for row in rows {
        for (i, cell) in row.iter().enumerate() {
            if i < col_widths.len() {
                col_widths[i] = col_widths[i].max(cell.len());
            }
        }
    }

    // Print headers (bold and cyan)
    println!();
    for (i, header) in headers.iter().enumerate() {
        print!("{} ", header.bold().cyan());
        print!("{}", " ".repeat(col_widths[i] - header.len()));
    }
    println!();

    // Print separator (gray)
    for &width in &col_widths {
        print!("{} ", "-".repeat(width).dimmed());
    }
    println!();

    // Print rows
    for row in rows {
        for (i, cell) in row.iter().enumerate() {
            if i == 0 {
                print!("{} ", cell.bright_white());
            } else {
                print!("{} ", cell.white());
            }
            print!("{}", " ".repeat(col_widths[i] - cell.len()));
        }
        println!();
    }
    println!();
}
