mod bench;
use bench::*;
use ma_titan::default::immutable::{Int, STree};
use uint::*;
use std::collections::BTreeMap;
use std::fmt::Debug;

pub const SAMPLE_SIZE: usize = 100;
pub const REPEATS: usize = 100_000;


fn main() {
	let args: Vec<String> = std::env::args().collect();

    if args.len() != 5 {
        println!("Bitte genau vier Argumente übergeben!");
    }
	
	match args[3].as_ref() {
		"u40" => stage1::<u40>(args),
		"u48" => stage1::<u48>(args),
		"u64" => stage1::<u64>(args),
		_ => panic!("Bitte verwende {} <stree|vebtree|btree|binary> <pred|new> <u40|u48|u64> <uniform|normal/bereich_viertel|normal/bereich_komplett|bwt_runs>",args[0]),
    }
}

fn stage1<T: Int + Typable + From<u64> + Copy + Debug>(args: Vec<String>) {
    match args[1].as_ref() {
        "stree" => stage2::<T,STree<T>>(args),
        "vebtree" => stage2::<T,VEBTree>(args),
        "btree" => stage2::<T,BTreeMap<T,T>>(args),
		"binary" => stage2::<T,BinarySearch<T>>(args),
        _ => panic!("Bitte verwende {} <stree|vebtree|btree|binary> <pred|new> <u40|u48|u64> <uniform|normal/bereich_viertel|normal/bereich_komplett|bwt_runs>",args[0]),
    }
}

fn stage2<T: Int + Typable + From<u64> + Copy + Debug, U: Clone + PredecessorSetStatic<T>>(args: Vec<String>) {
    match args[2].as_ref() {
		"new" => static_build_benchmark::<T,U>(args[4].as_ref()),
		"pred" => pred_and_succ_benchmark::<T,U>(args[4].as_ref()),
		_ => panic!("Bitte verwende {} <stree|vebtree|btree|binary> <pred|new> <u40|u48|u64> <uniform|normal/bereich_viertel|normal/bereich_komplett|bwt_runs>",args[0]),
	}
}