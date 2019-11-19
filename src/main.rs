pub mod bench;
use bench::*;
use ma_titan::default::immutable::{Int, STree};
use uint::*;
use std::collections::BTreeMap;
use std::fmt::{Display, Debug};
use std::ops::Add;
use rbtree::RBTree;

pub const SAMPLE_NEW: usize = 3;
pub const SAMPLE_PRED: usize = 100;
pub const REPEATS: usize = 100_000;


fn main() {
	let args: Vec<String> = std::env::args().collect();

    if args.len() != 7 {
        println!("Bitte verwende {} <stree|rbtree|btree|binary> <pred|new|gen-input> <u40|u48|u64> <uniform|normal|bwt_runs> <input-size=<1,..,32> <name>",args[0]);
        return;
    }
	
    if args[4] != "uniform" && args[4] != "normal" && args[4] != "bwt_runs" {
        println!("Bitte verwende {} <stree|rbtree|btree|binary> <pred|new|gen-input> <u40|u48|u64> <uniform|normal|bwt_runs> <input-size=<1,..,32> <name>",args[0]);
        return;
    } 

	match args[3].as_ref() {
		"u40" => stage1::<u40>(args),
		"u48" => stage1::<u48>(args),
		"u64" => stage1::<u64>(args),
		_ => println!("Bitte verwende {} <stree|rbtree|btree|binary> <pred|new|gen-input> <u40|u48|u64> <uniform|normal|bwt_runs> <input-size=<1,..,32> <name>",args[0]),
    }
}

fn stage1<T: 'static + Int + Typable + Display + Default + Add<T, Output=T> + From<u64> + Copy + Debug>(args: Vec<String>) {
    match args[1].as_ref() {
        "stree" => stage2::<T,STree<T>>(args),
        "rbtree" => stage2::<T,RBTree<T,T>>(args),
        "btree" => stage2::<T,BTreeMap<T,T>>(args),
		"binary" => stage2::<T,BinarySearch<T>>(args),
        _ => println!("Bitte verwende {} <stree|rbtree|btree|binary> <pred|new|gen-input> <u40|u48|u64> <uniform|normal|bwt_runs> <input-size=<1,..,32> <name>",args[0]),
    }
}

fn stage2<T: Int + Typable + Default +Display + Add<T, Output=T> + From<u64> + Copy + Debug, U: Clone + PredecessorSetStatic<T>>(args: Vec<String>) {
    match args[2].as_ref() {
		"new" => static_build_benchmark::<T,U>(args[4].as_ref(), args[6].as_ref(), args[5].parse::<u32>().unwrap()),
		"pred" => pred_and_succ_benchmark::<T,U>(args[4].as_ref(), args[6].as_ref(), args[5].parse::<u32>().unwrap()),
        "gen-input" => create_input::<T>(args[4].as_ref()),
		_ => println!("Bitte verwende {} <stree|rbtree|btree|binary> <pred|new|gen-input> <u40|u48|u64> <uniform|normal|bwt_runs> <input-size=<1,..,32> <name>",args[0]),
	}
}