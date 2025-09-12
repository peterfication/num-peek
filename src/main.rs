use std::collections::HashSet;

use clap::Parser;
use ordered_float::OrderedFloat;

#[derive(Parser)]
#[command(author, version)]
#[command(name = "num-peek")]
#[command(about = "A CLI tool that can peek into `*.npy` files.")]
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
    println!("----------------------------------------");
    analyze_npy(&cli.file_path).expect("Failed to analyze the npy file");
}

fn analyze_npy(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let bytes = std::fs::read(file_path)?;

    let npy = npyz::NpyFile::new(&bytes[..])?;

    println!("Dimensions: {}", npy.header().shape().len());
    println!("Shape: {:?}", npy.header().shape());
    let dtype = npy.header().dtype();

    match dtype {
        npyz::DType::Plain(plain) => {
            println!("Type: {:?}{}", plain.type_char(), plain.size_field());
            println!("----------------------------------------");

            match plain.type_char() {
                npyz::TypeChar::Float if plain.size_field() == 8 => {
                    let unique_numbers: HashSet<OrderedFloat<f64>> = npy
                        .data::<f64>()?
                        .map(|n| n.map(OrderedFloat))
                        .collect::<Result<HashSet<_>, _>>()?;

                    // Sort the unique numbers for consistent output
                    let unique_numbers: Vec<OrderedFloat<f64>> = {
                        let mut nums: Vec<OrderedFloat<f64>> = unique_numbers.into_iter().collect();
                        nums.sort_unstable();
                        nums
                    };

                    let min_value = unique_numbers.iter().min().unwrap();
                    let max_value = unique_numbers.iter().max().unwrap();

                    println!("Number of unique values: {}", unique_numbers.len());
                    println!("Unique values: {unique_numbers:?}");
                    println!("Min value: {min_value:?}");
                    println!("Max value: {max_value:?}");
                }
                npyz::TypeChar::Int if plain.size_field() == 8 => {
                    let unique_numbers: HashSet<i64> =
                        npy.data::<i64>()?.collect::<Result<HashSet<_>, _>>()?;

                    // Sort the unique numbers for consistent output
                    let unique_numbers: Vec<i64> = {
                        let mut nums: Vec<i64> = unique_numbers.into_iter().collect();
                        nums.sort_unstable();
                        nums
                    };

                    let max_value = unique_numbers.iter().max().unwrap();
                    let min_value = unique_numbers.iter().min().unwrap();

                    println!("Number of unique values: {}", unique_numbers.len());
                    println!("Unique values: {unique_numbers:?}");
                    println!("Min value: {min_value:?}");
                    println!("Max value: {max_value:?}");
                }
                _ => {
                    println!("Unsupported dtype for unique value calculation");
                }
            }
        }
        _ => return Err("Unsupported dtype".into()),
    }

    Ok(())
}
