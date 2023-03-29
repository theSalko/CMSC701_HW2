use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use plotters::prelude::*;
use rand::Rng;
use std::time::Instant;
use std::env;

mod bit_vector;
mod rank_support;
mod utils;
mod select_support;
mod sparse_array;

use bit_vector::BitVector;
use rank_support::RankSupport;
use select_support::SelectSupport;
use sparse_array::SparseArray;
    

fn benchmark_rank(bitvector_size: usize, num_operations: usize) -> f64 {
    let mut rng = rand::thread_rng();
    println!("running for size {}", bitvector_size);

    let mut bit_vector = BitVector::new(bitvector_size); 

    for i in 0..bitvector_size {
        bit_vector.set(i, rng.gen_bool(0.5));
    }

    // Initialize it
    let rank_support = RankSupport::new(&bit_vector);

    let start = Instant::now();

    // For num_operations time
    for _ in 0..num_operations {
        let index = rng.gen_range(0..bitvector_size-1);
        // Get the 1 rank of the index
        rank_support.rank1(index);
    }

    let duration = start.elapsed();
    // return the duration as a float in seconds
    duration.as_secs_f64()
}

fn benchmark_size(bitvector_size: usize) -> usize {
    let mut rng = rand::thread_rng();
    println!("running size benchmark for size {}", bitvector_size);
    let mut bit_vector = BitVector::new(bitvector_size); 
    for i in 0..bitvector_size {
        bit_vector.set(i, rng.gen_bool(0.5));
    }
    
    let rank_support = RankSupport::new(&bit_vector);

    rank_support.overhead()

}


fn test_rank_support(){

    let num_operations = 100_000;
    let sizes = [1_000, 10_000, 100_000, 1_000_000, 10_000_000, 100_000_000];


    let root = BitMapBackend::new("plot.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    
    // TODO: Modify Chart Builder template
    let mut chart = ChartBuilder::on(&root)
        .caption("BitVector Size vs Time", ("Arial", 24).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0u32..*sizes.last().unwrap() as u32, 0.0..1.0)
        .unwrap();

    chart.configure_mesh().x_desc("Size").y_desc("Time").draw().unwrap();

    let size_time: Vec<(u32, f64)> = sizes
        .iter()
        .map(|&size| {
            let time = benchmark_rank(size, num_operations);
            (size as u32, time)
        })
        .collect();

    chart
        .draw_series(LineSeries::new(size_time, &RED))
        .unwrap()
        .label("Size vs Time")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart.configure_series_labels().draw().unwrap();

    let mut all_times= Vec::new();
    let mut all_sizes = Vec::new();
    for size in sizes {
        all_times.push(benchmark_rank(size, num_operations));
        all_sizes.push(benchmark_size(size));
    }

    for i in 0..all_times.len() {
        println!("for size = {} time was = {} and total size was = {}",sizes[i], all_times[i], all_sizes[i]);
    }
}


fn test_select_support(){
    for j in 0..10 {
        let bitvector_size = 190;
        let mut rng = rand::thread_rng();
        let mut bit_vector = BitVector::new(bitvector_size); 
        for i in 0..bitvector_size {
            bit_vector.set(i, rng.gen_bool(0.05*(j as f64)));
        }
        bit_vector.print_bit_vector(10);
        let rank_support = RankSupport::new(&bit_vector);
        let select_support = SelectSupport::new(&rank_support);
        let selected_rank = 6;
        let y = select_support.select1(selected_rank);

        println!("y is {} for selected rank {}", y, selected_rank);
        println!("");
    }
}



fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    // test_rank_support();
    test_select_support();
    
}

// // use bit_vector::BitVector;
// // use rank_support::RankSupport;

// // fn main() {
// //     // Test rank support on bit vectors of various sizes
// //     // Generate plots for time and overhead
// //     let bit_vector = BitVector::load("bit_vector.bin").unwrap();
// //     let rank_support = RankSupport::load(&bit_vector, "rank_support.bin").unwrap();
// // }
// // src/main.rs
// mod bit_vector;
// mod rank_support;
// mod utils;

// use bit_vector::BitVector;
// use rank_support::RankSupport;

// fn main() {
//     let mut bit_vector = BitVector::new(100); // Create a bit vector with 100 bits

//     // Set some bits
//     bit_vector.set(5, true);
//     bit_vector.set(10, true);
//     bit_vector.set(30, true);
//     bit_vector.set(36, true);
//     bit_vector.set(50, true);
//     bit_vector.set(70, true);


//     let rank_support = RankSupport::new(&bit_vector);
//     let start = 1;
//     let end = 64;
//     // let opt_number = bit_vector.get_bits_as_number(start, end);
//     // let number = match opt_number {
//     //     Some(x) => x,
//     //     None => 0
//     // };
//     let number_2 = bit_vector.interpret_as_u64_int(start, end);
//     println!("Number we got is and {}", number_2);
//     println!("Rank at position 10: {}", rank_support.rank1(10));
//     println!("Rank at position 30: {}", rank_support.rank1(30));
//     println!("Rank at position 50: {}", rank_support.rank1(50));
//     println!("Rank at position 70: {}", rank_support.rank1(70));
//     println!("Rank at position 71: {}", rank_support.rank1(71));
//     //Rank at position 10: 2
//     // Rank at position 30: 3
//     // Rank at position 50: 5
//     // Rank at position 70: 6
//     println!("BitVector:");
//     bit_vector.print_bit_vector(10);
//     rank_support.print_chunks_and_super_chunks();
// }