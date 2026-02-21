use std::fs;

pub fn is_laptop() -> bool {
    if let Ok(entries) = fs::read_dir("/sys/class/power_supply/") {
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();

            if name_str.starts_with("BAT") {
                let mut path = entry.path();
                path.push("type");

                if let Ok(typ) = fs::read_to_string(path) {
                    if typ.trim() == "Battery" {
                        return true;
                    }
                }
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_laptop() {
        let _ = is_laptop();
    }
}
