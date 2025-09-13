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
    F64 {
        count: usize,
        unique_values: Vec<f64>,
        min: f64,
        max: f64,
    },
    I64 {
        count: usize,
        unique_values: Vec<i64>,
        min: i64,
        max: i64,
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
                (npyz::TypeChar::Float, _size) => {
                    let data: Vec<f64> = npy.data::<f64>()?.collect::<Result<_, _>>()?;
                    if data.is_empty() {
                        None
                    } else {
                        let count = data.len();
                        let mut unique_numbers: Vec<_> = HashSet::<OrderedFloat<f64>>::from_iter(
                            data.into_iter().map(OrderedFloat),
                        )
                        .into_iter()
                        .collect();
                        unique_numbers.sort_unstable();

                        match (unique_numbers.first(), unique_numbers.last()) {
                            (Some(first), Some(last)) => Some(ValueStats::F64 {
                                count,
                                min: first.0,
                                max: last.0,
                                unique_values: unique_numbers.into_iter().map(|n| n.0).collect(),
                            }),
                            _ => unreachable!(
                                "unique_numbers should not be empty due to is_empty check"
                            ),
                        }
                    }
                }

                (npyz::TypeChar::Int, 1) => value_stats_for_int_type::<i8>(npy)?,
                (npyz::TypeChar::Int, 2) => value_stats_for_int_type::<i16>(npy)?,
                (npyz::TypeChar::Int, 4) => value_stats_for_int_type::<i32>(npy)?,
                (npyz::TypeChar::Int, 8) => value_stats_for_int_type::<i64>(npy)?,
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
