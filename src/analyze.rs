use std::collections::HashSet;
use std::error::Error;
use std::hash::Hash;

use npyz::Deserialize;
use ordered_float::OrderedFloat;

/// A struct to hold the results of the NPY file analysis.
#[derive(Debug)]
pub struct NpyAnalysis {
    pub dimensions: usize,
    pub shape: Vec<u64>,
    pub dtype_string: String,
    pub stats: Option<ValueStats>,
    pub total_bytes: usize,
}

/// An enum to hold statistics for different supported numeric types.
#[derive(Debug)]
pub enum ValueStats {
    I64 {
        count: usize,
        unique_values: Vec<i64>,
        min: i64,
        max: i64,
    },
    F16 {
        count: usize,
        unique_values: Vec<half::f16>,
        min: half::f16,
        max: half::f16,
    },
    F32 {
        count: usize,
        unique_values: Vec<f32>,
        min: f32,
        max: f32,
    },
    F64 {
        count: usize,
        unique_values: Vec<f64>,
        min: f64,
        max: f64,
    },
}

/// Analyzes the NPY file and returns a struct with the results.
pub fn analyze_npy(file_path: &str) -> Result<NpyAnalysis, Box<dyn std::error::Error>> {
    let bytes = std::fs::read(file_path)?;
    let total_bytes = bytes.len();
    let npy = npyz::NpyFile::new(&bytes[..])?;

    let header = npy.header();
    let dimensions = header.shape().len();
    let shape = header.shape().to_vec();
    let dtype = npy.header().dtype();

    let (dtype_string, stats) = match dtype {
        npyz::DType::Plain(plain) => {
            let bits = plain.size_field() * 8;
            let dtype_str = format!("{:?}{}", plain.type_char(), bits);

            let stats = match (plain.type_char(), plain.size_field()) {
                (npyz::TypeChar::Int, 1) => value_stats_for_int_type::<i8>(npy)?,
                (npyz::TypeChar::Int, 2) => value_stats_for_int_type::<i16>(npy)?,
                (npyz::TypeChar::Int, 4) => value_stats_for_int_type::<i32>(npy)?,
                (npyz::TypeChar::Int, 8) => value_stats_for_int_type::<i64>(npy)?,

                (npyz::TypeChar::Float, 2) => value_stats_for_float16_type(npy)?,
                (npyz::TypeChar::Float, 4) => value_stats_for_float32_type(npy)?,
                (npyz::TypeChar::Float, 8) => value_stats_for_float64_type(npy)?,

                _ => None, // Unsupported type for detailed stats
            };
            (dtype_str, stats)
        }
        _ => (format!("{dtype:?}"), None),
    };

    Ok(NpyAnalysis {
        dimensions,
        shape,
        dtype_string,
        stats,
        total_bytes,
    })
}

/// Helper function to compute statistics for integer types.
fn value_stats_for_int_type<T>(
    npy: npyz::NpyFile<&[u8]>,
) -> Result<Option<ValueStats>, Box<dyn Error>>
where
    T: Eq + Hash + Ord + Copy + Into<i64>,
    T: Deserialize,
{
    let data: Vec<T> = npy.data::<T>()?.collect::<Result<_, _>>()?;
    if data.is_empty() {
        Ok(None)
    } else {
        let count = data.len();
        let mut unique_numbers: Vec<_> = HashSet::<T>::from_iter(data).into_iter().collect();
        unique_numbers.sort_unstable();

        Ok(Some(ValueStats::I64 {
            count,
            min: (*unique_numbers
                .first()
                .expect("unique_numbers should not be empty after non-empty data"))
            .into(),
            max: (*unique_numbers
                .last()
                .expect("unique_numbers should not be empty after non-empty data"))
            .into(),
            unique_values: unique_numbers.iter().map(|&x| x.into()).collect(),
        }))
    }
}

/// Helper function to compute statistics for f16 type.
fn value_stats_for_float16_type(
    npy: npyz::NpyFile<&[u8]>,
) -> Result<Option<ValueStats>, Box<dyn Error>> {
    let data: Vec<half::f16> = npy.data::<half::f16>()?.collect::<Result<_, _>>()?;

    if data.is_empty() {
        Ok(None)
    } else {
        let count = data.len();

        let mut unique_numbers: Vec<_> = data
            .into_iter()
            .map(|x: half::f16| x.to_bits())
            .collect::<HashSet<_>>()
            .into_iter()
            .map(half::f16::from_bits)
            .collect();

        unique_numbers.sort_by_key(|a| a.to_bits());

        match (unique_numbers.first(), unique_numbers.last()) {
            (Some(first), Some(last)) => Ok(Some(ValueStats::F16 {
                count,
                min: *first,
                max: *last,
                unique_values: unique_numbers.into_iter().collect(),
            })),
            _ => unreachable!("unique_numbers should not be empty due to is_empty check"),
        }
    }
}

/// Helper function to compute statistics for f32 type.
fn value_stats_for_float32_type(
    npy: npyz::NpyFile<&[u8]>,
) -> Result<Option<ValueStats>, Box<dyn Error>> {
    let data: Vec<_> = npy.data::<f32>()?.collect::<Result<_, _>>()?;
    if data.is_empty() {
        Ok(None)
    } else {
        let count = data.len();
        let mut unique_numbers: Vec<_> =
            HashSet::<OrderedFloat<f32>>::from_iter(data.into_iter().map(OrderedFloat))
                .into_iter()
                .collect();
        unique_numbers.sort_unstable();

        match (unique_numbers.first(), unique_numbers.last()) {
            (Some(first), Some(last)) => Ok(Some(ValueStats::F32 {
                count,
                min: first.0,
                max: last.0,
                unique_values: unique_numbers.into_iter().map(|n| n.0).collect(),
            })),
            _ => unreachable!("unique_numbers should not be empty due to is_empty check"),
        }
    }
}

/// Helper function to compute statistics for f64 type.
fn value_stats_for_float64_type(
    npy: npyz::NpyFile<&[u8]>,
) -> Result<Option<ValueStats>, Box<dyn Error>> {
    let data: Vec<_> = npy.data::<f64>()?.collect::<Result<_, _>>()?;
    if data.is_empty() {
        Ok(None)
    } else {
        let count = data.len();
        let mut unique_numbers: Vec<_> =
            HashSet::<OrderedFloat<f64>>::from_iter(data.into_iter().map(OrderedFloat))
                .into_iter()
                .collect();
        unique_numbers.sort_unstable();

        match (unique_numbers.first(), unique_numbers.last()) {
            (Some(first), Some(last)) => Ok(Some(ValueStats::F64 {
                count,
                min: first.0,
                max: last.0,
                unique_values: unique_numbers.into_iter().map(|n| n.0).collect(),
            })),
            _ => unreachable!("unique_numbers should not be empty due to is_empty check"),
        }
    }
}
