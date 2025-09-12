use clap::Parser;

#[derive(Parser)]
#[command(author, version)]
#[command(name = "num-peek")]
#[command(
    about = "A CLI tool that can peek into `*.npy` files."
)]
struct Cli {
    /// Path to the *.npy file
    #[arg(value_name = "FILE_PATH", value_parser = validate_npy)]
    file_path: String,
}

fn validate_npy(string: &str) -> Result<String, String> {
    if string.ends_with(".npy") {
        Ok(string.to_string())
    } else {
        Err(String::from("File must have a .npy extension"))
    }
}

fn main() {
    let cli = Cli::parse();

    // Make sure the file exists
    // If not, print an error message and exit
    if !std::path::Path::new(&cli.file_path).exists() {
        eprintln!("Error: File '{}' does not exist.", cli.file_path);
        std::process::exit(1);
    }

    println!("Peek into {}", cli.file_path);
}
