use colored::*;

pub fn print_success(msg: &str) {
    println!("{}", msg.green());
}

pub fn print_error(msg: &str) {
    eprintln!("{}", msg.red());
}

pub fn print_warning(msg: &str) {
    println!("{}", msg.yellow());
}

pub fn print_progress(msg: &str) {
    println!("{}", msg.cyan());
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

pub fn print_info(msg: &str) {
    println!("{}", msg.blue());
}

pub fn print_debug(msg: &str) {
    if cfg!(debug_assertions) {
        println!("{}", msg.bright_magenta());
    }
}

pub fn print_key_value(key: &str, value: &str) {
    println!("  {}: {}", key.cyan(), value.white());
}

pub fn print_check(label: &str, success: bool) {
    let mark = if success { "✓".green() } else { "✗".red() };
    println!("  {} {}", mark, label);
}
