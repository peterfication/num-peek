use byte_unit::{Byte, UnitType};

use crate::analyze::{NpyAnalysis, ValueStats};

/// Presents the analysis results to the console.
pub fn present_analysis(file_path: &str, analysis: &NpyAnalysis) {
    println!("Peek into {file_path}");
    println!("----------------------------------------");
    println!("Dimensions: {}", analysis.dimensions);
    println!("Shape: {:?}", analysis.shape);
    println!("Type: {}", analysis.dtype_string);
    println!(
        "Bytes: {}",
        Byte::from(analysis.total_bytes).get_appropriate_unit(UnitType::Binary)
    );
    println!("----------------------------------------");

    match &analysis.stats {
        Some(ValueStats::F64 {
            count,
            unique_values,
            min,
            max,
        }) => {
            print_stats(count, unique_values, min, max);
        }
        Some(ValueStats::I64 {
            count,
            unique_values,
            min,
            max,
        }) => {
            print_stats(count, unique_values, min, max);
        }
        None => {
            println!(
                "Unsupported dtype for unique value calculation {}",
                analysis.dtype_string
            );
        }
    }
}

fn print_stats<T, U>(count: &usize, unique_values: &U, min: &T, max: &T)
where
    T: std::fmt::Debug + std::fmt::Display,
    U: std::fmt::Debug + std::ops::Deref<Target = [T]>,
{
    println!("Number of values: {count}");
    println!("Number of unique values: {}", unique_values.len());
    println!("Unique values: {unique_values:?}");
    println!("Min value: {min}");
    println!("Max value: {max}");
}
