use std::collections::BTreeMap;
use std::fs;
use std::io::{self, Read};
use std::process;

use clap::{Parser, Subcommand};
use kvf::{parse, Entry};

/// A Kv Format Swiss-Army knife
#[derive(Parser)]
#[command(name = "kaiv", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Print the value for a given key from a KV file
    Get {
        /// The key to look up
        key: String,
        /// KV file to read (omit or use `-` for stdin)
        file: Option<String>,
    },
    /// Validate a KV file and report parse errors
    Check {
        /// KV file to validate (omit or use `-` for stdin)
        file: Option<String>,
    },
    /// Re-output a KV file in canonical form (sorted keys, no comments/blanks)
    Fmt {
        /// KV file to format (omit or use `-` for stdin)
        file: Option<String>,
    },
    /// Export or import KV data in various formats
    Export {
        #[command(subcommand)]
        format: ExportFormat,
    },
    /// Import data in various formats as KV
    Import {
        #[command(subcommand)]
        format: ImportFormat,
    },
}

#[derive(Subcommand)]
enum ExportFormat {
    /// Export a KV file as a JSON object
    Json {
        /// KV file to read (omit or use `-` for stdin)
        file: Option<String>,
    },
}

#[derive(Subcommand)]
enum ImportFormat {
    /// Import a JSON object as KV format
    Json {
        /// JSON file to read (omit or use `-` for stdin)
        file: Option<String>,
    },
}

/// Read input bytes from a file path or stdin.
/// A path of `None` or `"-"` means stdin.
fn read_input(file: Option<&str>) -> io::Result<Vec<u8>> {
    match file {
        None | Some("-") => {
            let mut buf = Vec::new();
            io::stdin().read_to_end(&mut buf)?;
            Ok(buf)
        }
        Some(path) => fs::read(path),
    }
}

/// Print all parse errors to stderr and exit with code 2.
fn report_parse_errors(errors: &[kvf::ParseError]) -> ! {
    for err in errors {
        eprintln!("{err}");
    }
    process::exit(2);
}

fn cmd_get(key: &str, file: Option<&str>) {
    let input = read_input(file).unwrap_or_else(|e| {
        eprintln!("Error reading input: {e}");
        process::exit(2);
    });

    let entries = parse(&input).unwrap_or_else(|errors| report_parse_errors(&errors));

    for entry in entries {
        if let Entry::Kv {
            key: k, value: v, ..
        } = entry
        {
            if k == key {
                println!("{v}");
                return;
            }
        }
    }

    // Key not found
    process::exit(1);
}

fn cmd_check(file: Option<&str>) {
    let input = read_input(file).unwrap_or_else(|e| {
        eprintln!("Error reading input: {e}");
        process::exit(1);
    });

    if let Err(errors) = parse(&input) {
        report_parse_errors(&errors);
    }
}

fn cmd_fmt(file: Option<&str>) {
    let input = read_input(file).unwrap_or_else(|e| {
        eprintln!("Error reading input: {e}");
        process::exit(2);
    });

    let entries = parse(&input).unwrap_or_else(|errors| report_parse_errors(&errors));

    // Collect KV pairs into a sorted map
    let mut map: BTreeMap<String, String> = BTreeMap::new();
    for entry in entries {
        if let Entry::Kv { key, value, .. } = entry {
            map.insert(key, value);
        }
    }

    for (k, v) in &map {
        println!("{k}={v}");
    }
}

fn cmd_export_json(file: Option<&str>) {
    let input = read_input(file).unwrap_or_else(|e| {
        eprintln!("Error reading input: {e}");
        process::exit(2);
    });

    let entries = parse(&input).unwrap_or_else(|errors| report_parse_errors(&errors));

    let mut map: BTreeMap<String, String> = BTreeMap::new();
    for entry in entries {
        if let Entry::Kv { key, value, .. } = entry {
            map.insert(key, value);
        }
    }

    let json = serde_json::to_string(&map).unwrap_or_else(|e| {
        eprintln!("Error serializing JSON: {e}");
        process::exit(2);
    });
    println!("{json}");
}

fn cmd_import_json(file: Option<&str>) {
    let input = read_input(file).unwrap_or_else(|e| {
        eprintln!("Error reading input: {e}");
        process::exit(1);
    });

    let json_str = std::str::from_utf8(&input).unwrap_or_else(|e| {
        eprintln!("Error reading JSON input as UTF-8: {e}");
        process::exit(1);
    });

    let value: serde_json::Value = serde_json::from_str(json_str).unwrap_or_else(|e| {
        eprintln!("Error parsing JSON: {e}");
        process::exit(1);
    });

    let obj = match value.as_object() {
        Some(o) => o,
        None => {
            eprintln!("Error: JSON input must be an object");
            process::exit(1);
        }
    };

    // Validate all keys and values first, then collect into sorted map
    let mut map: BTreeMap<String, String> = BTreeMap::new();
    for (k, v) in obj {
        if !is_valid_key(k) {
            eprintln!("Error: invalid key {k:?} — keys must match [A-Za-z_][A-Za-z0-9_]*");
            process::exit(1);
        }
        match v.as_str() {
            Some(s) => {
                map.insert(k.clone(), s.to_string());
            }
            None => {
                eprintln!("Error: value for key {k:?} is not a string");
                process::exit(1);
            }
        }
    }

    for (k, v) in &map {
        println!("{k}={v}");
    }
}

/// Returns true if the key matches `[A-Za-z_][A-Za-z0-9_]*`.
fn is_valid_key(key: &str) -> bool {
    let mut chars = key.chars();
    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
        _ => return false,
    }
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Get { key, file } => cmd_get(&key, file.as_deref()),
        Command::Check { file } => cmd_check(file.as_deref()),
        Command::Fmt { file } => cmd_fmt(file.as_deref()),
        Command::Export {
            format: ExportFormat::Json { file },
        } => cmd_export_json(file.as_deref()),
        Command::Import {
            format: ImportFormat::Json { file },
        } => cmd_import_json(file.as_deref()),
    }
}

#[cfg(test)]
mod tests {
    use super::is_valid_key;

    #[test]
    fn valid_keys() {
        assert!(is_valid_key("APP_NAME"));
        assert!(is_valid_key("_private"));
        assert!(is_valid_key("a"));
        assert!(is_valid_key("A1_b2"));
    }

    #[test]
    fn invalid_keys() {
        assert!(!is_valid_key(""));
        assert!(!is_valid_key("1starts_with_digit"));
        assert!(!is_valid_key("has-hyphen"));
        assert!(!is_valid_key("has.dot"));
        assert!(!is_valid_key("has space"));
    }
}
