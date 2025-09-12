use clap::Parser;

#[derive(Parser)]
#[command(author, version)]
#[command(name = "num-peek")]
#[command(
    about = "A CLI tool that can peek into `*.npy` files."
)]
struct Cli {
    /// Path to the *.npy file
    #[arg(value_name = "FILE_PATH")]
    file_path: String,
}

fn main() {
    let cli = Cli::parse();
    println!("Peek into {}", cli.file_path);
}
