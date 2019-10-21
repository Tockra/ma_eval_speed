pub mod bench;
use std::time::{Instant};
use std::io::BufWriter;
use std::fs::OpenOptions;
use std::collections::BTreeSet;
use std::io::prelude::*;
use boomphf::Mphf;

extern crate rand_distr;

pub const SAMPLE_SIZE: usize = 100;
pub const REPEATS: usize = 100_000;

use rand_distr::{Distribution, Uniform};

fn main() {

    
    std::fs::create_dir_all("./output/").unwrap();

    eval_mphf();

       
}

fn eval_mphf() {
    let mut result = BufWriter::new(OpenOptions::new()
        .read(true)
        .write(true)
        .truncate(true)
        .create(true)
        .open("output/mphf_vs_bs.txt").unwrap());
    std::thread::sleep(std::time::Duration::from_millis(1000));

    for i in 0..u16::max_value() {
        let keys = build_uniform(i);
        let objects = vec![0_u64;i as usize];
        let hash_map = Mphf::new(2.0, &keys);
        for i in 0..SAMPLE_SIZE {
            let iter = keys.iter();
            let now = Instant::now();
            for key in iter {
                let x = objects[hash_map.try_hash(&key).unwrap() as usize];
                 std::mem::size_of_val(&x);
            }
            let elapsed_time = now.elapsed().as_nanos();

            writeln!(result, "RESULT algo=mphf<u16,_> size={} time={} iterations={}",i,elapsed_time,SAMPLE_SIZE).unwrap(); 
            result.flush().unwrap();
        }
    }
}

fn build_uniform(max_value: u16) -> Vec<u16> {
    let between = Uniform::from(0u16..max_value);
    let mut rng = rand::thread_rng();
    let mut memory: BTreeSet<u16> = BTreeSet::new(); 
    let mut result = Vec::with_capacity(max_value as usize);
    for _ in 0..max_value {
        let mut random_val = between.sample(&mut rng);
        while memory.contains(&random_val) {
            random_val = between.sample(&mut rng);
        }

        memory.insert(random_val);
        result.push(random_val);
    }
    result
}

