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

const TEST_SPEED:bool = true;

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

fn make_sparse_vector(size:u64, sparsity:f64) -> Vec<String> {
    let mut sparse_vector: Vec<String> = Vec::new();
    let mut rng = rand::thread_rng();

    for i in 0..size {
        if rng.gen_bool(sparsity) {
            sparse_vector.push("hello world".to_owned());
        } else {
            sparse_vector.push("".to_owned());
        }
    }

    sparse_vector
}


fn make_sparse_array(size: u64, sparsity:f64) -> SparseArray {
    let mut sparse_array = SparseArray::create(size);
    let mut rng = rand::thread_rng();

    for i in 0..size {
        if rng.gen_bool(sparsity) {

            // let s: String = i.to_string();
            // let element = String::from("some_value of ".to_owned() + &s );
            let element = String::from("hello world").to_owned();
            sparse_array.append(element, i);
        }
       
    }
    sparse_array.finalize();
    return sparse_array;

}

fn make_sparse_array3(size: u64, sparsity:f64) -> SparseArray {
    let mut sparse_array = SparseArray::create(size);
    let mut rng = rand::thread_rng();

    for i in 0..size {
        if rng.gen_bool(sparsity) {

            // let s: String = i.to_string();
            // let element = String::from("some_value of ".to_owned() + &s );
            let element = String::from("hello world").to_owned();
            sparse_array.append(element, i);
        } else {
            let element = String::from("").to_owned();
            sparse_array.append(element, i);
        }
       
    }
    sparse_array.finalize();
    return sparse_array;

}



fn make_sparse_array2() -> SparseArray {
    let mut sparse_array = SparseArray::create(20);
    for i in 0..20 {
        if i%3 == 0 {

            let s: String = i.to_string();
            let element = String::from("some_value of ".to_owned() + &s );
            sparse_array.append(element, i);
        }
       
    }
    sparse_array.finalize();
    return sparse_array;
}

fn test_validity_sparse_array(){
    let GET_INDEX_NUM = 5;
    let mut sparse_array = make_sparse_array2();
    let mut s = String::from("nothing");
    sparse_array.print_everything(5);
    sparse_array.get_at_rank(4, &mut s);
    println!("get_at_rank of 4 is = {}", &s);
    sparse_array.get_at_index(5, &mut s);
    println!("get_at_index of 5 is = {}", &s);

    let x = sparse_array.get_index_of(GET_INDEX_NUM);
    println!("get_index_of {} is = {}", GET_INDEX_NUM, x);
    let x = sparse_array.num_elem_at(18);
    println!("num_elem_at 18 is = {}", x);
}




fn test_speed_of_funcs_wrt_size(){
    let num_operations = 100_000;
    let sizes = [1_000, 10_000, 100_000, 1_000_000];
    let sparsities = [0.01, 0.05, 0.1];
    let mut rng = rand::thread_rng();


    for size in sizes {
        for sparsity in sparsities {
            if TEST_SPEED {
                println!("Testing size = {} and sparsity = {}", size, sparsity);
                let mut sparse_array = make_sparse_array(size, sparsity);
                sparse_array.finalize();
    
                let num_elts = sparse_array.num_elem();
    
                let mut s = String::from("nothing");
    
    
                // 
                // TESTING GET AT INDEX
                //
                // println!("Testing get at index");
                let start = Instant::now();
                // For num_operations time
                for _ in 0..num_operations {
                    let index = rng.gen_range(0..size-1);
                    // Get the at index index
                    sparse_array.get_at_index(index, &mut s);
                }
                let duration = start.elapsed();
                // return the duration as a float in seconds
                let x = duration.as_secs_f64();
                println!("t = {} for get at index", x);
    
    
    
    
                // TESTING NUM ELEMENTS AT:
                // println!("Testing num_elem_at");
                let start = Instant::now();
                // For num_operations time
                for _ in 0..num_operations {
                    let index = rng.gen_range(0..size-1);
                    // Get the at index index
                    sparse_array.num_elem_at(index);
                }
                let duration = start.elapsed();
                // return the duration as a float in seconds
                let x = duration.as_secs_f64();
                println!("t = {} for num_elem_at", x);
    
    
    
                // TESTING GET AT RANK
                // println!("Testing get_at_rank");
                let start = Instant::now();
                // For num_operations time
                for _ in 0..num_operations {
                    let index = rng.gen_range(0..num_elts-1);
                    // Get the at index index
                    sparse_array.get_at_rank(index, &mut s);
                }
                let duration = start.elapsed();
                // return the duration as a float in seconds
                let x = duration.as_secs_f64();
                println!("t = {} for get_at_rank", x);
    
    
    
                //
                // TESTING get_index_of
                let start = Instant::now();
                // For num_operations time
                for _ in 0..num_operations {
                    let index = rng.gen_range(0..num_elts-1);
                    // Get the at index index
                    sparse_array.get_index_of(index);
                }
                let duration = start.elapsed();
                // return the duration as a float in seconds
                let x = duration.as_secs_f64();
                println!("t = {} for get_index_of", x);
    
            } else {

                // Testing size of structure
                let mut sparse_array = make_sparse_array(size, sparsity);
                sparse_array.finalize();
                let num_elts = sparse_array.num_elem();
                let total_size = sparse_array.get_overhead();
                println!("Regular: bit_vector bits={}, num_elts={}, size_in_bytes={}", size, num_elts, total_size);

                // let mut sparse_array = make_sparse_array3(size, sparsity);
                // sparse_array.finalize();
                // let num_elts = sparse_array.num_elem();
                // let total_size2 = sparse_array.get_overhead();
                // println!("Empty String: bit_vector bits={}, num_elts={}, size_in_bytes={}", size, num_elts, total_size2);
                
                // let proportion = total_size2 as f64 / total_size as f64 ;
                // println!("proportion is {}", proportion);
                let test_vec = make_sparse_vector(size, sparsity);
                let size_vec = test_vec.iter().map(|s| s.len()).sum::<usize>();
                println!("test_vec size = {}", size_vec);
                let proportion = size_vec as f64 / total_size as f64 ;
                println!("proportion is {}", proportion);
            }
          





        }
    }


}
// Writeup: For this programming task, test your implementation by
// generating sparse arrays of a few different lengths (e.g. 1000, 10000, 100000, 1000000) 
// and having various sparsity (e.g. 1%, 5%, 10%). How does the speed of the different 
// functions vary as a factor of the overall size? 
//      How about as a function of the overall sparsity? 
//      Finally, try and estimate how the size of your sparse array in memory compares
//       to what the size would be if all of the 0 elements were instead explicitly stored as “empty” elements 
//          (e.g. as empty strings). How much space do you save? 
//                      How do your savings depend on sparsity?
fn run_sparse_array_experiments(){

}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    // test_rank_support();
    // test_select_support();
    test_validity_sparse_array();
    test_speed_of_funcs_wrt_size();



}
