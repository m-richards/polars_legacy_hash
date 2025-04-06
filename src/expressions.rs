use polars::{
    chunked_array::ops::arity::{
        try_binary_elementwise, try_ternary_elementwise, unary_elementwise,
    },
    prelude::*,
};

use polars::datatypes::{
    DataType::{Float64, String, Struct},
    Field,
};
use pyo3_polars::derive::polars_expr;


use std::hash::Hash;
use ahash::RandomState;
use polars_arrow::bitmap::utils::get_bit_unchecked;

use crate::pl_legacy_hashing::vector_hasher::integer_vec_hash;


// pub(crate) fn get_null_hash_value(random_state: &RandomState) -> u64 {
//     // we just start with a large prime number and hash that twice
//     // to get a constant hash value for null/None
//     let first = random_state.hash_one(3188347919usize);
//     random_state.hash_one(first)
// }



// fn insert_null_hash(chunks: &[ArrayRef], random_state: RandomState, buf: &mut Vec<u64>) {
//     let null_h = get_null_hash_value(&random_state);
//     let hashes = buf.as_mut_slice();

//     let mut offset = 0;
//     chunks.iter().for_each(|arr| {
//         if arr.null_count() > 0 {
//             let validity = arr.validity().unwrap();
//             let (slice, byte_offset, _) = validity.as_slice();
//             (0..validity.len())
//                 .map(|i| unsafe { get_bit_unchecked(slice, i + byte_offset) })
//                 .zip(&mut hashes[offset..])
//                 .for_each(|(valid, h)| {
//                     *h = [null_h, *h][valid as usize];
//                 })
//         }
//         offset += arr.len();
//     });
// }

// fn integer_vec_hash<T>(ca: &ChunkedArray<T>, random_state: RandomState, buf: &mut Vec<u64>)
// where
//     T: PolarsIntegerType,
//     T::Native: Hash,
// {
//     // Note that we don't use the no null branch! This can break in unexpected ways.
//     // for instance with threading we split an array in n_threads, this may lead to
//     // splits that have no nulls and splits that have nulls. Then one array is hashed with
//     // Option<T> and the other array with T.
//     // Meaning that they cannot be compared. By always hashing on Option<T> the random_state is
//     // the only deterministic seed.
//     buf.clear();
//     buf.reserve(ca.len());

//     #[allow(unused_unsafe)]
//     #[allow(clippy::useless_transmute)]
//     ca.downcast_iter().for_each(|arr| {
//         buf.extend(
//             arr.values()
//                 .as_slice()
//                 .iter()
//                 .copied()
//                 .map(|v| random_state.hash_one(v)),
//         );
//     });

//     insert_null_hash(ca.chunks(), random_state, buf)
// }

fn integer_vec_hash_adapter<T>(ca: &ChunkedArray<T>, random_state: RandomState, mut buf: Vec<u64>, name: PlSmallStr) -> PolarsResult<Series>
where
    T: PolarsIntegerType,
    T::Native: Hash,
{
    integer_vec_hash(ca, random_state, &mut buf); 
     // ops/various.rs/SeriesMethods/hash for Ok line
    Ok(UInt64Chunked::from_vec(name, buf).into_series()) 
}


#[polars_expr(output_type=UInt64)]
fn oldhash(inputs: &[Series]) -> PolarsResult<Series> {
    println!("Got series {:?}", inputs);
    let s = inputs.get(0).expect("no series received");

            let rs = RandomState::with_seeds(0, 0, 0, 0);
            let h:Vec<u64> = vec![];
            let ser_name: PlSmallStr = s.name().clone();

    match s.dtype() {

 // TODO floats / strings / etc
        DataType::Int64 => integer_vec_hash_adapter(s.i64().unwrap(), rs, h, ser_name),
        DataType::Int32 => integer_vec_hash_adapter(s.i32().unwrap(), rs, h, ser_name),
        DataType::UInt64 => integer_vec_hash_adapter(s.u64().unwrap(), rs, h, ser_name),
        DataType::UInt32 => integer_vec_hash_adapter(s.u32().unwrap(), rs, h, ser_name),
        _ => Err(PolarsError::InvalidOperation(
            "wyhash only works on strings or binary data".into(),
        )),
    }
}
