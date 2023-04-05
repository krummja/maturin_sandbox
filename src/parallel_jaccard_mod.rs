use rayon::prelude::*;
use polars_core::utils::accumulate_dataframes_vertical;
use polars::prelude::*;


fn split_offsets(len: usize, n: usize) -> Vec<(usize, usize)> {
    let offsets: Vec<(usize, usize)>;

    offsets = vec![(0, len)];

    return offsets;

    // return if n == 1 {
    //     vec![(0, len)]
    // } else {
    //     let chunk_size = len / n;
    //     (0..n)
    //         .map(|partition| {
    //             let offset = partition * chunk_size;
    //             let len = if partition == (n - 1) {
    //                 len - offset
    //             } else {
    //                 chunk_size
    //             };
    //
    //             (partition * chunk_size, len)
    //         })
    //         .collect()
    // }
}
