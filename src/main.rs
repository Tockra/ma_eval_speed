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
    let mut result = BufWriter::new(OpenOptions::new()
        .read(true)
        .write(true)
        .truncate(true)
        .create(true)
        .open("output/mphf_vs_bs.txt").unwrap());

    eval_mphf(&mut result);
    eval_binary_search(&mut result);
       
}

fn eval_binary_search(result: &mut BufWriter<std::fs::File>) {

    std::thread::sleep(std::time::Duration::from_millis(1000));

      
    for _ in 0..SAMPLE_SIZE {
            
        for i in 2..2048 {
            let keys = Box::new(build_uniform(i));
            let objects = Box::new(vec![0_u64; i as usize]);
            

            let iter = keys.iter();
            let mut x = 0;
            let now = Instant::now();
            for key in iter {
                x = match keys.binary_search(&key) {
                    Ok(x) => objects[x],
                    _ => panic!("get in internal wurde mit ungültigem Schlüssel {} aufgerufen. {:?}", *key,keys),
                };
            }
            let elapsed_time = now.elapsed().as_nanos();
            std::mem::size_of_val(&x);

            writeln!(result, "RESULT algo=hashmap_bs<(u16,u64)> size={} time_per_anfrage={}",i,elapsed_time as f64/(i as f64)).unwrap(); 
            result.flush().unwrap();
        }
    }
}

fn eval_mphf(result: &mut BufWriter<std::fs::File>) {
    std::thread::sleep(std::time::Duration::from_millis(1000));
    
    for _ in 0..SAMPLE_SIZE {
        for i in 2..2048 {
            let keys = build_uniform(i);
            let objects = vec![0_u64;i as usize];
            let hash_map = Mphf::new(2.0, &keys);
        
            let iter = keys.iter();
            let mut x = 0;
            let now = Instant::now();
            for key in iter {
                x = objects[hash_map.try_hash(&key).unwrap() as usize];
            }
            let elapsed_time = now.elapsed().as_nanos();
            std::mem::size_of_val(&x);

            writeln!(result, "RESULT algo=mphf<u16,u64> size={} time_per_anfrage={}",i,elapsed_time as f64/(i as f64)).unwrap(); 
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
    result.sort();
    result
}

