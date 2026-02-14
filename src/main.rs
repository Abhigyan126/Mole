use clap::Parser;
use std::{env,path::PathBuf, fs, path::Path, fmt::Write};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to drill
    #[arg(short, long)]
    path: String,

    #[arg(short, long)]
    size: bool,

    #[arg(short = 'o', long)]
    save: Option<String>,
}

fn print_dir_tree(path: &Path, prefix: String) -> std::io::Result<()> {
    let mut entries: Vec<_> = fs::read_dir(path)?
        .filter_map(Result::ok)
        .collect();

    entries.sort_by_key(|e| e.path());

    let count = entries.len();

    for (i, entry) in entries.into_iter().enumerate() {
        let entry_path = entry.path();
        let is_last = i == count - 1;

        let connector = if is_last { "└── " } else { "├── " };

        println!(
            "{}{}{}",
            prefix,
            connector,
            entry_path.file_name()
                .unwrap_or_default()
                .to_string_lossy()
        );

        if entry.file_type()?.is_dir() {
            let new_prefix = if is_last {
                format!("{}    ", prefix)
            } else {
                format!("{}│   ", prefix)
            };

            print_dir_tree(&entry_path, new_prefix)?;
        }
    }

    Ok(())
}

fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = 1024 * KB;
    const GB: u64 = 1024 * MB;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

fn print_dir_tree_with_size(path: &Path, prefix: String) -> std::io::Result<()> {
    let mut entries: Vec<_> = fs::read_dir(path)?
        .filter_map(Result::ok)
        .collect();

    entries.sort_by_key(|e| e.path());
    let count = entries.len();

    for (i, entry) in entries.into_iter().enumerate() {
        let entry_path = entry.path();
        let is_last = i == count - 1;
        let connector = if is_last { "└── " } else { "├── " };

        let file_type = entry.file_type()?;

        if file_type.is_dir() {
            println!(
                "{}{}{}",
                prefix,
                connector,
                entry_path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy(),
            );

            let new_prefix = if is_last {
                format!("{}    ", prefix)
            } else {
                format!("{}│   ", prefix)
            };

            print_dir_tree_with_size(&entry_path, new_prefix)?;
        } else if file_type.is_file() {
            let size = entry.metadata()?.len();
            let formatted_size = format_size(size);

            println!(
                "{}{}{} | {}",
                prefix,
                connector,
                entry_path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy(),
                formatted_size,
            );
        }
    }

    Ok(())
}


fn print_dir_tree_save(
    path: &Path,
    prefix: String,
    buffer: &mut String,
) -> std::io::Result<()> {
    let mut entries: Vec<_> = fs::read_dir(path)?
        .filter_map(Result::ok)
        .collect();

    entries.sort_by_key(|e| e.path());
    let count = entries.len();

    for (i, entry) in entries.into_iter().enumerate() {
        let entry_path = entry.path();
        let is_last = i == count - 1;
        let connector = if is_last { "└── " } else { "├── " };

        writeln!(
            buffer,
            "{}{}{}",
            prefix,
            connector,
            entry_path.file_name().unwrap_or_default().to_string_lossy()
        ).unwrap();

        if entry.file_type()?.is_dir() {
            let new_prefix = if is_last {
                format!("{}    ", prefix)
            } else {
                format!("{}│   ", prefix)
            };

            print_dir_tree_save(&entry_path, new_prefix, buffer)?;
        }
    }

    Ok(())
}

fn print_dir_tree_save_size(
    path: &Path,
    prefix: String,
    buffer: &mut String,
) -> std::io::Result<()> {
    let mut entries: Vec<_> = fs::read_dir(path)?
        .filter_map(Result::ok)
        .collect();

    entries.sort_by_key(|e| e.path());
    let count = entries.len();

    for (i, entry) in entries.into_iter().enumerate() {
        let entry_path = entry.path();
        let is_last = i == count - 1;
        let connector = if is_last { "└── " } else { "├── " };

        if entry.file_type()?.is_dir() {
            writeln!(
                buffer,
                "{}{}{}",
                prefix,
                connector,
                entry_path.file_name().unwrap_or_default().to_string_lossy(),
            ).unwrap();

            let new_prefix = if is_last {
                format!("{}    ", prefix)
            } else {
                format!("{}│   ", prefix)
            };

            print_dir_tree_save_size(&entry_path, new_prefix, buffer)?;
        } else {
            let size = entry.metadata()?.len();
            let formatted_size = format_size(size);

            writeln!(
                buffer,
                "{}{}{} | {}",
                prefix,
                connector,
                entry_path.file_name().unwrap_or_default().to_string_lossy(),
                formatted_size,
            ).unwrap();
        }
    }

    Ok(())
}

fn main() {
    let args = Args::parse();

    let path = match args.path.as_str() {
        "." => env::current_dir().unwrap(),
        _ => PathBuf::from(&args.path),
    };

    // ---------------- SAVE MODE ----------------
    if let Some(save_arg) = args.save {
        let mut save_path = match save_arg.as_str() {
            "." => env::current_dir().unwrap(),
            _ => PathBuf::from(save_arg),
        };

        if save_path.is_dir() {
            save_path.push("mole"); // default filename
        }

        save_path.set_extension("md");

        // 🔥 Use ONE buffer
        let mut buffer = String::new();

        let result = if args.size {
            print_dir_tree_save_size(&path, String::new(), &mut buffer)
        } else {
            print_dir_tree_save(&path, String::new(), &mut buffer)
        };

        match result {
            Ok(()) => {
                let wrapped = format!(
"```text
{}
```",
                    buffer
                );

                if let Err(e) = fs::write(&save_path, wrapped) {
                    eprintln!("Failed to save file: {}", e);
                } else {
                    println!("Saved to {:?}", save_path);
                }
            }
            Err(e) => eprintln!("Error building tree: {}", e),
        }

        return;
    }

    // ---------------- DISPLAY MODE ----------------
    let result = if args.size {
        print_dir_tree_with_size(&path, String::new())
    } else {
        print_dir_tree(&path, String::new())
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
    }
}
