#![feature(test)]
extern crate test;

use serde::Deserialize;
use std::fs::File;
use clap::{Arg, App};
use std::io::{BufRead, BufReader};

fn main() {
    let file = get_args();
    println!("reading {}", file);
    let citypop = buffer_to_struct(&file);
    println!("{:?}", citypop);
}

fn get_args() -> String {
    let matches = App::new("My CSV reader")
        .version("0.1.0")
        .author("Luca Peruzzo")
        .about("Reading a csv file passed as argument")
        .arg(Arg::new("file")
                 .short('f')
                 .long("file")
                 .default_value("default.csv")
                 .takes_value(true))
        .get_matches();
    let file = String::from(matches.value_of("file").unwrap_or_default());
    return file
}

#[derive(Debug, Deserialize)]
struct Record {
    city: String,
    state: String,
    population: Option<u32>,
    latitude: f64,
    longitude: f64,
}

#[derive(Debug)]
struct CityPop {
    city: Vec<String>,
    state: Vec<String>,
    population: Vec<Option<u32>>,
    latitude: Vec<f64>,
    longitude: Vec<f64>,
}

impl CityPop {
    fn add_entry(&mut self, city: String, state: String, population: Option<u32>, latitude: f64, longitude: f64) {
        self.city.push(city);
        self.state.push(state);
        self.population.push(population);
        self.latitude.push(latitude);
        self.longitude.push(longitude);
    }
    fn add_record(&mut self, record: Record) {
        self.city.push(record.city);
        self.state.push(record.state);
        self.population.push(record.population);
        self.latitude.push(record.latitude);
        self.longitude.push(record.longitude);
    }
    fn new() -> CityPop {
        let city: Vec<String> = Vec::new();
        let state: Vec<String> = Vec::new();
        let population: Vec<Option<u32>> = Vec::new();
        let latitude: Vec<f64> = Vec::new();
        let longitude: Vec<f64> = Vec::new();
        let citypop: CityPop = CityPop {city, state, population, latitude, longitude};
        citypop
    }
}

fn csv_serde(arg_file: &String) -> CityPop {
    let file = File::open(arg_file).expect("could not open file ");
    let mut reader = csv::Reader::from_reader(file);
    let mut citypop = CityPop::new();
    for result in reader.deserialize() {
        let record: Record = result.unwrap();
        citypop.add_record(record)
    }
    citypop
}

fn csv_noserde(arg_file: &String) -> CityPop {
    let file = File::open(arg_file).expect("could not open file");
    let mut rdr = csv::Reader::from_reader(file);
    let mut citypop = CityPop::new();
    for result_row in rdr.records() {
        let record_row = result_row.unwrap();
        citypop.city.push(String::from(&record_row[0]));
        citypop.state.push(String::from(&record_row[1]));
        citypop.population.push(record_row[2].parse().ok());
        citypop.latitude.push(record_row[3].parse().expect("no latitude"));
        citypop.longitude.push(record_row[4].parse().expect("no londitude"));
    }
    return citypop
}

fn buffer_to_struct(arg_file: &String) -> CityPop {
    let file = File::open(arg_file).unwrap();
    let buf = BufReader::new(file);
    let mut citypop = CityPop::new();
    for l in buf.lines().skip(1) {
        let l_unwrap = match l {
            Ok(l_ok) =>  l_ok, 
            Err(l_err) => {
                println!("Err, could not read/unwrap line {}", l_err);
                continue;
            }
        };
        let mut l_split = l_unwrap.split(',');

        citypop.city.push(
            match l_split.next() {
                Some(v) => String::from(v),
                None => {
                    String::from("None")
                }
            }
        );
        citypop.state.push(
            match l_split.next() {
                Some(v) => String::from(v),
                None => {
                    String::from("None")
                }
            }
        );
        citypop.population.push(
            match l_split.next() {
                Some(v) => match v.parse() {
                    Ok(ok) => Some(ok),
                    Err(_) => None,
                },
                None => {
                    None
                },
            }
        );
        citypop.latitude.push(
            match l_split.next() {
                Some(v) => match v.parse() {
                    Ok(ok) => ok,
                    Err(_) => std::f64::NAN,
                },
                None => {
                    std::f64::NAN
                },
            }
        );
        citypop.longitude.push(
            match l_split.next() {
                Some(v) => match v.parse() {
                    Ok(ok) => ok,
                    Err(_) => std::f64::NAN,
                },
                None => {
                    std::f64::NAN
                },
            }
        );
    }
    return citypop
}

// just for reference
fn buffer_onlyloop(arg_file: &String) { 
    let file = File::open(arg_file).unwrap();
    let buf = BufReader::new(file);
    for l in buf.lines() {
        continue;
    }
}

// BENCHMARKS

#[bench]
fn bench_buffer_to_struct(b: &mut test::Bencher) {
    let file = String::from("uspop.csv");
    b.iter(|| {
        let citypop = buffer_to_struct(&file);
        test::black_box(citypop);
    });
}

#[bench]
fn bench_csv_noserde(b: &mut test::Bencher) {
    let file = String::from("uspop.csv");
    b.iter(|| {
        let citypop = csv_noserde(&file);
        test::black_box(citypop);
    });
}

#[bench]
fn bench_csv_serde(b: &mut test::Bencher) {
    let file = String::from("uspop.csv");
    b.iter(|| {
        let citypop = csv_serde(&file);
        test::black_box(citypop);
    });
}

#[bench]
fn bench_buffer_onlyloop(b: &mut test::Bencher) {
    let file = String::from("uspop.csv");
    b.iter(|| {
        buffer_onlyloop(&file);
    });
}
