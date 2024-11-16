use clap::Parser;
use std::fs;
use std::path::PathBuf;
use std::time::{Duration, Instant};

mod filesystem;
mod replace;

/// Gitbook Flavoured Markdown to Standard Markdown
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Source Directory Path
    #[arg(short, long)]
    source: PathBuf,

    /// Output Directory Path
    #[arg(short, long)]
    output: PathBuf,
}

fn main() {
    let total_time_taken_start: Instant = Instant::now();

    let args = Args::parse();

    // Checking if the given Source directory exists
    if args.source.exists() {
        if !args.source.is_dir() {
            panic!("The given source path exists but is not a directory.");
        }
    } else {
        panic!("The given source path doesn't exist.")
    }

    // Checking if the given output directory exists. If not create the output directory
    if args.output.exists() {
        if !args.output.is_dir() {
            panic!("The given output path exists but is not a directory.");
        }
    }

    let md_files: Vec<filesystem::FileDetail> =
        filesystem::find_md_files(&args.source, &args.output);

    println!("[+] Number of Markdown files found: {}", md_files.len());

    println!("[+] Creating the output directores...");

    filesystem::create_output_directories(&md_files);

    println!("[+] Successfully created the output directories.");

    println!("[+] Modifying files and moving assets.");

    for each in md_files {
        let mut content: String = match fs::read_to_string(each.file_path) {
            Ok(result) => result,
            Err(why) => panic!("Failed to the read the file: {}", why),
        };

        content = replace::code(content);
        content = replace::embed_urls(content);
        content = replace::file_links(content, &each.file_dir, &each.output_assets_dir);
        content = replace::hints(content);
        content = replace::images(content, &each.file_dir, &each.output_assets_dir);
        content = replace::tabs(content);

        match fs::write(each.output_file_path, content) {
            Ok(_) => {}
            Err(why) => panic!("Failed to write file: {}", why),
        }
    }

    println!("[+] Successfully modified files and moved assets.");

    let total_time_taken: Duration = Instant::elapsed(&total_time_taken_start);

    println!(
        "Total Time Taken: {} seconds",
        total_time_taken.as_secs_f64()
    );
}
