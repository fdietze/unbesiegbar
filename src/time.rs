extern crate time;

pub fn get_time(format: &str) -> String {
    let tm = time::now();
    time::strftime(format, &tm).unwrap_or_else(|e| format!("ERROR: {}", e))
}

#[cfg(test)]
mod tests {
    extern crate regex;
    use super::*;

    #[test]
    fn valid_format() {
        let re = regex::Regex::new(r"^\d{2}:\d{2}$").unwrap();
        assert!(re.is_match(&get_time("%H:%M")));
    }

    #[test]
    fn invalid_format() {
        assert_eq!(get_time("%q"), "ERROR: invalid format specifier: %q");
    }
}

