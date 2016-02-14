use std::process::Command;

pub fn command(input: &str) -> String {
    let mut parts = input.split_whitespace();
    let command = parts.next();
    match command {
        Some(command) => {
            let args = parts.collect::<Vec<_>>();
            let output = Command::new(command)
                             .args(&args)
                             .output()
                             .map_err(|e| format!("Failed to run command '{}': {}", command, e));

            let text = output.and_then(|o| {
                if o.status.success() {
                    Ok(String::from_utf8_lossy(&(o.stdout)).into_owned())
                } else {
                    let err = String::from_utf8_lossy(&(o.stderr)).into_owned();
                    Err(format!("Command '{}' exited with error: {}", command, err))
                }
            });

            text.unwrap_or_else(|e| format!("ERROR: {}", e)).lines().collect::<Vec<_>>().join(" ")
        }
        None => "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_command() {
        assert_eq!(command(""), "");
    }

    #[test]
    fn non_utf8_output() {
        assert_eq!(command("echo -e \\xce\\xce"), "\u{fffd}\u{fffd}");
    }

    #[test]
    fn utf8_output() {
        assert_eq!(command("echo hallo"), "hallo");
    }

    #[test]
    fn invalid_command() {
        assert_eq!(command("hanspeter"),
                   "ERROR: Failed to run command 'hanspeter': No such file or directory (os \
                    error 2)");
    }

    #[test]
    fn failed_command() {
        assert_eq!(command("sleep hans"),
                   "ERROR: Command 'sleep' exited with error: sleep: invalid time interval \
                    \u{2018}hans\u{2019} Try 'sleep --help' for more information.");
    }
}

