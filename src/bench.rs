use std::fs::File;
use std::io::prelude::*;
use std::fs::OpenOptions;
use std::time::{Instant, SystemTime};
use std::fmt::Debug;
use std::ops::Add;
use std::io::{BufWriter};
use std::fs::read_dir;
use std::collections::BTreeMap;
use rbtree::RBTree;

use rand_pcg::Mcg128Xsl64;
use rand::Rng;
use criterion::black_box;

use uint::{Typable};
use ma_titan::default::immutable::{Int, STree};
use super::{SAMPLE_NEW, SAMPLE_PRED, REPEATS};

const SEED: u128 = 0xcafef00dd15ea5e5;
/// Diese Methode lädt die Testdaten aus ./testdata/{u40,u48,u64}/ und konstruiert mit Hilfe dieser eine
/// Datenstruktur T. Dabei wird die Laufzeit gemessen.
pub fn static_build_benchmark<E: Typable + From<u64> + Copy + Debug, T: PredecessorSetStatic<E>>(data: &str, name: &str, var: u32) {
    println!("Starte Laufzeitmessung new(). Datenstruktur: {}, Datentyp {}, Datensatz: {}", E::TYPE, T::TYPE, data);

    let bench_start = Instant::now();
    std::fs::create_dir_all(format!("./output/new/{}/",E::TYPE)).unwrap();

    let mut result = BufWriter::new(OpenOptions::new()
        .read(true)
        .write(true)
        .truncate(true)
        .create(true)
        .open(format!("output/new/{}/{}_{}_{}_{}.txt",E::TYPE, T::TYPE, name, data.replace("/", "_"), var)).unwrap());
    
    for dir in read_dir(format!("testdata/{}/{}/",data, E::TYPE)).unwrap() {
        let path = dir.unwrap().path();
        if path.to_str().unwrap().contains("git") {
            continue;
        }
        
        if data != "bwt_runs" {
            let i: u32 = path.to_str().unwrap().split('^').skip(1).next().unwrap().split('.').next().unwrap().parse().unwrap();
            if i != var {
                continue;
            }
        } else {
            if !path.to_str().unwrap().contains(var.to_string().as_str()) {
                continue;
            }
        }
        
        println!("{:?}",path);
        

        
        for i in 0..SAMPLE_NEW {
            let values = read_from_file::<E>(path.to_str().unwrap()).unwrap();
            let len = values.len();
            let now = Instant::now();
            let result_ds = T::new(values);
            ::std::mem::size_of_val(&result_ds);
            
            let elapsed_time = now.elapsed().as_nanos();
            writeln!(result, "RESULT algo={}_{} method=new size={} time={} unit=ns i={}",T::TYPE, name, len, elapsed_time, i).unwrap(); 
            
            result.flush().unwrap();
        }
        
    }
    println!("Laufzeitmessung der Datenstrukturerzeugung beendet. Dauer {} Sekunden", bench_start.elapsed().as_secs())
}

#[allow(dead_code)]
pub fn create_input<E: Typable + Add<E, Output=E> + Into<u64> + std::fmt::Display + Copy + Debug + From<u64> + Into<u64>>(data: &str) {
    std::fs::create_dir_all(format!("input/pred/{}/{}/", data, E::TYPE)).unwrap();

    for dir in read_dir(format!("testdata/{}/{}/",data, E::TYPE)).unwrap() {
        let path = dir.unwrap().path();

        println!("{:?}",path);
        
        let values = read_from_file::<E>(path.to_str().unwrap()).unwrap();
      

        let values_len = values.len();

        let test_values = get_test_values(values[0]+E::from(1u64),values[values_len-1]);

        write_to_file(format!("input/pred/{}/{}/min{}_max{}.data",data, E::TYPE, values[0],values[values_len-1]).to_string(), &test_values).unwrap();
    }
}

/// Lädt die Testdaten aus ./testdata/{u40,u48,u64}/ und erzeugt mit Hilfe dieser die zu testende Datenstruktur T. 
/// Anschließend werden 10000 gültige Vor- bzw. Nachfolger erzeugt und die Laufzeiten der Predecessor-Methode 
/// werden mit Hilfe dieser gemessen
pub fn pred_and_succ_benchmark<E: Typable + Into<u64> + Copy + Debug + From<u64> + Into<u64>, T: Clone + PredecessorSetStatic<E>>(data: &str, name: &str, var: u32) {
    println!("Starte Laufzeitmessung pred(). Datenstruktur: {}, Datentyp {}, Datensatz: {}", E::TYPE, T::TYPE, data);
    let bench_start = Instant::now();
    std::fs::create_dir_all(format!("./output/pred/{}",E::TYPE)).unwrap();
    let mut result = BufWriter::new(OpenOptions::new()
        .read(true)
        .write(true)
        .truncate(true)
        .create(true)
        .open(format!("output/pred/{}/{}_{}_{}_{}.txt",E::TYPE,T::TYPE, name, data.replace("/", "_"),var)).unwrap());
    for dir in read_dir(format!("testdata/{}/{}/",data, E::TYPE)).unwrap() {
        let path = dir.unwrap().path();
        if path.to_str().unwrap().contains("git") {
            continue;
        }

        if data != "bwt_runs" {
            let i: u32 = path.to_str().unwrap().split('^').skip(1).next().unwrap().split('.').next().unwrap().parse().unwrap();
            if i != var {
                continue;
            }
        } else {
            if !path.to_str().unwrap().contains(var.to_string().as_str()) {
                continue;
            }
        }

        println!("{:?}",path);

        {
        let values = read_from_file::<E>(path.to_str().unwrap()).unwrap();
        let size = values.len();
        let test_values = read_from_file::<E>(&format!("input/pred/{}/{}/min{}_max{}.data",data,E::TYPE,values[0].into(),values[size-1].into())).unwrap();
        let repeats = test_values.len();
        let data_structure = T::new(values);
        
        println!("Datenstruktur erstellt");
        for i in 0..SAMPLE_PRED {
            cache_clear();
            let iter = test_values.iter();
            let now = Instant::now();
            for elem in iter {
                data_structure.predecessor(*elem);
            }
            let elapsed_time = now.elapsed().as_nanos();
            if i % 10 == 0 {
                println!("Fortschritt: {}%",i*100/SAMPLE_PRED);
            }
             writeln!(result, "RESULT algo={}_{} method=predecessor size={} element_size={} time={} unit=ns test_values={} i={}",T::TYPE, name, size, std::mem::size_of::<E>(), elapsed_time, repeats, i).unwrap(); 
            result.flush().unwrap();
        }}
        {
        let values = read_from_file::<E>(path.to_str().unwrap()).unwrap();
        let size = values.len();
        println!("Test-Elemente eingelesen");
        let test_values = read_from_file::<E>(&format!("input/pred/{}/{}/min{}_max{}.data",data,E::TYPE,values[0].into(),values[size-1].into())).unwrap();
        println!("Test-Values eingelesen");
        let repeats = test_values.len();

        println!("Starte evaluierung pred()");
        let data_structure = T::new(values);
        
        println!("Datenstruktur erstellt");

        for i in 0..SAMPLE_PRED {
            cache_clear();
            let iter = test_values.iter();
            let now = Instant::now();
            for elem in iter {
                data_structure.successor(*elem);
            }
            let elapsed_time = now.elapsed().as_nanos();
            if i % 10 == 0 {
                println!("Fortschritt: {}%",i*100/SAMPLE_PRED);
            }
            writeln!(result, "RESULT algo={}_{} method=successor size={} element_size={} time={} unit=ns test_values={} i={}",T::TYPE, name, size, std::mem::size_of::<E>(), elapsed_time, repeats,i).unwrap(); 
            result.flush().unwrap();
        }}

    }
    println!("Laufzeitmessung der Predecessor- und Successor-Methoden beendet. Dauer {} Sekunden", bench_start.elapsed().as_secs())
}

fn get_test_values<E: Typable + Copy + From<u64> + Into<u64> >(min: E, max: E) -> Vec<E> {
    let mut state = Mcg128Xsl64::new(black_box(SEED));
    let mut test_values: Vec<E> = Vec::with_capacity(REPEATS);

    while test_values.len() != REPEATS {
        test_values.push(E::from(state.gen_range(min.into(),max.into())));
    }
    test_values
}

// Diese Methode löscht (hoffentlich) 12 Mbyte des Caches. 
pub fn cache_clear() {
    std::fs::create_dir_all("./cache").unwrap();

    let mut data = vec![0u64, 2u64];

    for i in 2 .. 3_750_000u64 {
        let sum = data[(i-1) as usize]*2+3+data[(i-2) as usize]*2+3;
        data.push(black_box(sum));
    }

    let mut buf = BufWriter::new(File::create(format!("cache/cache_{}",SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis())).unwrap());
    buf.write_fmt(format_args!("{}", data[data.len()-1])).unwrap();

    buf.flush().unwrap();
}

impl<T: Int>  PredecessorSetStatic<T> for RBTree<T,T> {
    fn new(elements: Box<[T]>) -> Self {
        let mut n: RBTree<T,T> = RBTree::new();
        for i in elements.iter() {
            n.insert(*i,*i);
        }
        n
    }

    fn predecessor(&self,number: T) -> Option<T> {
        Some(*self.predecessor(number).unwrap())
    }

    fn successor(&self,number: T) -> Option<T>{
        None
    }

    /*fn contains(&self, number: T) -> bool {
        self.contains_key(&number)
    }*/

    const TYPE: &'static str = "RBTree";
}

#[derive(Clone)]
pub struct BinarySearch<T> {
    element_list: Box<[T]>
}

impl<T: Int>  PredecessorSetStatic<T> for BinarySearch<T> {
    fn new(elements: Box<[T]>) -> Self {
        Self {
            element_list: elements,
        }
    }

    fn predecessor(&self,number: T) -> Option<T> {
        if self.element_list.len() == 0 || number < self.element_list[0] {
            None
        } else {
            match self.element_list.binary_search(&number) {
                Ok(x) => Some(self.element_list[x]),
                Err(x) => Some(self.element_list[x-1])
            }
        }
    }

    fn successor(&self,number: T) -> Option<T>{
        if self.element_list.len() == 0 || number > self.element_list[self.element_list.len()-1] {
            None
        } else {
            match self.element_list.binary_search(&number) {
                Ok(x) => Some(self.element_list[x]),
                Err(x) => Some(self.element_list[x])
            }
        }
    }

    const TYPE: &'static str = "BinarySearch";
}

pub trait PredecessorSetStatic<T> {
    fn new(elements: Box<[T]>) -> Self;
    fn predecessor(&self,number: T) -> Option<T>;
    fn successor(&self,number: T) -> Option<T>; // Optional
    const TYPE: &'static str;
}

impl<T: Int> PredecessorSetStatic<T> for STree<T> {
    const TYPE: &'static str = "STree";

    fn new(elements: Box<[T]>) -> Self {
         STree::<T>::new(elements)
    }

    fn predecessor(&self,number: T) -> Option<T> {
        self.locate_or_pred(number).and_then(|x| Some(self.element_list[x]))
    }

    fn successor(&self,number: T) -> Option<T> {
        self.locate_or_succ(number).and_then(|x| Some(self.element_list[x]))
    }
}

impl<T: Int>  PredecessorSetStatic<T> for BTreeMap<T,T> {
    fn new(elements: Box<[T]>) -> Self {
        let mut n: BTreeMap<T,T> = BTreeMap::new();
        for i in elements.iter() {
            n.insert(*i,*i);
        }
        n
    }

    fn predecessor(&self,number: T) -> Option<T> {
        self.range(T::from(0)..number).last().map(|x| *x.0)
    }

    fn successor(&self,number: T) -> Option<T>{
        self.range(number..).next().map(|x| *x.0)
    }

    const TYPE: &'static str = "B-Baum";
}

pub fn read_from_file<T: Typable + From<u64> + Copy>(name: &str) -> std::io::Result<Box<[T]>> {
    let mut input = File::open(name)?;
    let mut lenv = Vec::new();
    std::io::Read::by_ref(&mut input).take(std::mem::size_of::<usize>() as u64).read_to_end(&mut lenv)?;
    let mut len: [u8; std::mem::size_of::<usize>()] = [0; std::mem::size_of::<usize>()];
    for (i,b) in lenv.iter().enumerate() {
        len[i] = *b;
    }
    let len: usize = usize::from_le_bytes(len);

    assert!(len == (std::fs::metadata(name)?.len() as usize - std::mem::size_of::<usize>())/ std::mem::size_of::<T>());

    let mut values: Vec<T> = Vec::with_capacity(len);
    while values.len() != len {
        let mut buffer = Vec::with_capacity(std::mem::size_of::<T>());
        std::io::Read::by_ref(&mut input).take(std::mem::size_of::<T>() as u64).read_to_end(&mut buffer)?;
        let mut next_value: u64 = 0;
        for i in 0..buffer.len() {
            next_value |= (buffer[i] as u64) << (8*i);
        }

        values.push(T::from(next_value));
    }
    Ok(values.into_boxed_slice())
}

/// Serializiert den übergebenen Vector und schreibt diesen in eine Datei namens `name`.
fn write_to_file<T: Typable + Copy + Into<u64>>(name: String, val: &[T]) -> std::io::Result<()>{
    let mut buf = BufWriter::new(File::create(name).unwrap());
    buf.write_all(&val.len().to_le_bytes())?;
    for &v in val {
        let v: u64 = v.into();
        buf.write_all(&v.to_le_bytes()[..std::mem::size_of::<T>()])?;
    }
    buf.flush()?;
    Ok(())
}