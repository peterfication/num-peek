use std::collections::HashSet;
use std::error::Error;
use std::hash::Hash;

use npyz::Deserialize;
use ordered_float::{OrderedFloat, PrimitiveFloat};

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
    U64 {
        count: usize,
        unique_values: Vec<u64>,
        min: u64,
        max: u64,
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

                (npyz::TypeChar::Uint, 1) => value_stats_for_uint_type::<u8>(npy)?,
                (npyz::TypeChar::Uint, 2) => value_stats_for_uint_type::<u16>(npy)?,
                (npyz::TypeChar::Uint, 4) => value_stats_for_uint_type::<u32>(npy)?,
                (npyz::TypeChar::Uint, 8) => value_stats_for_uint_type::<u64>(npy)?,

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

/// Helper function to compute statistics for unsigned integer types.
fn value_stats_for_uint_type<T>(
    npy: npyz::NpyFile<&[u8]>,
) -> Result<Option<ValueStats>, Box<dyn Error>>
where
    T: Eq + Hash + Ord + Copy + Into<u64>,
    T: Deserialize,
{
    let data: Vec<T> = npy.data::<T>()?.collect::<Result<_, _>>()?;
    if data.is_empty() {
        Ok(None)
    } else {
        let count = data.len();
        let mut unique_numbers: Vec<_> = HashSet::<T>::from_iter(data).into_iter().collect();
        unique_numbers.sort_unstable();

        Ok(Some(ValueStats::U64 {
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
    value_stats_for_float_type::<f32>(npy, |count, min, max, unique_values| ValueStats::F32 {
        count,
        min,
        max,
        unique_values,
    })
}

/// Helper function to compute statistics for f64 type.
fn value_stats_for_float64_type(
    npy: npyz::NpyFile<&[u8]>,
) -> Result<Option<ValueStats>, Box<dyn Error>> {
    value_stats_for_float_type::<f64>(npy, |count, min, max, unique_values| ValueStats::F64 {
        count,
        min,
        max,
        unique_values,
    })
}

/// Helper function to compute statistics for float types (f32, f64).
fn value_stats_for_float_type<T>(
    npy: npyz::NpyFile<&[u8]>,
    make_stats: impl Fn(usize, T, T, Vec<T>) -> ValueStats,
) -> Result<Option<ValueStats>, Box<dyn Error>>
where
    T: PartialOrd + Copy + 'static + PrimitiveFloat,
    T: Deserialize,
{
    let data: Vec<_> = npy.data::<T>()?.collect::<Result<_, _>>()?;
    if data.is_empty() {
        Ok(None)
    } else {
        let count = data.len();
        let unique_numbers: Vec<_> = get_unique_float(data);
        match (unique_numbers.first(), unique_numbers.last()) {
            (Some(first), Some(last)) => Ok(Some(make_stats(
                count,
                *first,
                *last,
                unique_numbers.into_iter().collect(),
            ))),
            _ => unreachable!("unique_numbers should not be empty due to is_empty check"),
        }
    }
}

/// Get unique values of a vec of numbers (int or float)
pub fn get_unique_float<T>(input: Vec<T>) -> Vec<T>
where
    T: Copy,
    OrderedFloat<T>: std::hash::Hash + Eq + Ord,
{
    let wrapped: Vec<OrderedFloat<T>> = input.iter().map(|&x| OrderedFloat(x)).collect();
    let unique_set: HashSet<OrderedFloat<T>> = wrapped.into_iter().collect();
    let mut unique_vec: Vec<OrderedFloat<T>> = unique_set.into_iter().collect();
    unique_vec.sort();
    unique_vec.into_iter().map(|ordered| ordered.0).collect()
}
