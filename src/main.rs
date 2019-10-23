pub mod bench;
use bench::*;
use ma_titan::default::immutable::{Int, STree};

use std::fs::read_dir;

pub const SAMPLE_SIZE: usize = 100;
pub const REPEATS: usize = 100_000;


fn main() {	
    for dir in read_dir(format!("testdata/normal/u40/")).unwrap() {
        let path = dir.unwrap().path();

        println!("{:?}",path);
        
        let values = read_from_file::<uint::u40>(path.to_str().unwrap()).unwrap();
      

        let values_len = values.len();

        let test_values = get_test_values(values[0]+1u32,values[values_len-1]);

        let bs = BinarySearch::new(values.clone());
        let stree = STree::new(values.clone());
        
        for (i,val) in test_values.into_iter().enumerate() {
            if i % 1000 == 0{
                println!("{}", val);
            }
            if bs.predecessor(val) != stree.predecessor(val) {
                panic!("Gesucht: {} , bs_found {:?}, stree_found {:?}, data={:?}", val, bs.predecessor(val), stree.predecessor(val), path);
            }

            if bs.successor(val) != stree.successor(val) {
                panic!("Gesucht: {} , bs_found {:?}, stree_found {:?}, data={:?}", val, bs.successor(val), stree.successor(val), path);
            }

            
        }
        
    }

}