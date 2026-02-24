// Colors
const GREEN: &str = "\x1b[32m";
const RED: &str = "\x1b[31m";
const YELLOW: &str = "\x1b[33m";
const BLUE: &str = "\x1b[34m";
const CYAN: &str = "\x1b[36m";
const RESET: &str = "\x1b[0m";

pub fn print_success(msg: &str) {
    println!("{} {}{}", GREEN, msg, RESET);
}

pub fn print_error(msg: &str) {
    eprintln!("{} {}{}", RED, msg, RESET);
}

pub fn print_warning(msg: &str) {
    println!("{} {}{}", YELLOW, msg, RESET);
}

pub fn print_progress(msg: &str) {
    println!("{} {}{}", CYAN, msg, RESET);
}

pub fn print_table(headers: &[&str], rows: &[Vec<String>]) {
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
    
    // Print headers
    println!();
    for (i, header) in headers.iter().enumerate() {
        print!("{:<width$} ", header, width = col_widths[i]);
    }
    println!();
    
    // Print separator
    for &width in &col_widths {
        print!("{:-<width$} ", "");
    }
    println!();
    
    // Print rows
    for row in rows {
        for (i, cell) in row.iter().enumerate() {
            print!("{:<width$} ", cell, width = col_widths[i]);
        }
        println!();
    }
    println!();
}
