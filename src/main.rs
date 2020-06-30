#![feature(test)]
extern crate test;

use serde::Deserialize;
use ndarray::prelude::*;
use std::fs::File;
use clap::{Arg, App};
use std::io::{BufRead, BufReader};


fn main() {
    let file = get_args();
    let (lat, long) = csv_no_serde_ndarray(&file);
    println!("{}", lat);
    println!("{}", long);
    let (lat, long) = csv_serde_ndarray(&file);
    println!("{}", lat);
    println!("{}", long);
    let (lat, long) = buffer_ndarray(&file);
    println!("{}", lat);
    println!("{}", long);
    let _vv_strings = buffer_string(&file);
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
    // let file = String::from(matches.value_of("file").unwrap_or("default.csv"));
    let file = String::from(matches.value_of("file").unwrap_or_default());
    return file
}


#[derive(Debug, Deserialize)]
struct Record {
    city: String,
    state: String,
    population: Option<u64>,
    latitude: f64,
    longitude: f64,
}


fn csv_serde_ndarray(arg_file: &String) -> (ndarray::Array1<f64>, ndarray::Array1<f64>) {
    let mut lat_vec: Vec<f64> = Vec::new();
    let mut long_vec: Vec<f64> = Vec::new();
    // allocating with capacity doesn't change much, too small the csv?!
    // let mut lat_vec: Vec<f64> = Vec::with_capacity(7000); 
    // let mut long_vec: Vec<f64> = Vec::with_capacity(7000);
    let file = File::open(arg_file).expect("could not open file ");
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.deserialize() {
        match result {
            Err(e) => println!("Error reading line {}", e),
            Ok(v) => {
                let record: Record = v;
                lat_vec.push(record.latitude);
                long_vec.push(record.longitude);
            },

        }
    }
    let array_lat = Array::from(lat_vec);
    let array_long = Array::from(long_vec);
    return (array_lat, array_long);
}


fn csv_no_serde_ndarray(arg_file: &String) -> (ndarray::Array1<f64>, ndarray::Array1<f64>) {
    let mut lat_vec: Vec<f64> = Vec::new();
    let mut long_vec: Vec<f64> = Vec::new();
    let file = File::open(arg_file).expect("could not open file");
    let mut rdr = csv::Reader::from_reader(file);
    for result_row in rdr.records() {
        let record_row = result_row.unwrap();
        let _row_city = &record_row[0];
        let _row_state = &record_row[1];
        let _row_pop: Option<f64> = record_row[2].parse().ok();
        let row_latitude: f64 = record_row[3].parse().expect("no latitude");
        let row_longitude: f64 = record_row[4].parse().expect("no londitude");
        lat_vec.push(row_latitude);
        long_vec.push(row_longitude);
    }
    let array_lat = Array::from(lat_vec);
    let array_long = Array::from(long_vec);
    return (array_lat, array_long);
}



fn buffer_string(arg_file: &String) -> Vec<Vec<String>> {
    let file = File::open(arg_file).unwrap();
    let buf = BufReader::new(file);
    let string_vec : Vec<Vec<String>> = buf.lines()
        .map(|l| l.unwrap().split(',')
            .map(|s| s.to_string())
            .collect())
        .collect();
    return string_vec
}


fn buffer_ndarray(arg_file: &String) -> (ndarray::Array1<f64>, ndarray::Array1<f64>) {
    let file = File::open(arg_file).unwrap();
    let buf = BufReader::new(file);
    let mut city_vec: Vec<String> = Vec::new();
    let mut lat_vec: Vec<f64> = Vec::new();
    let mut long_vec: Vec<f64> = Vec::new();
    for l in buf.lines().skip(1) {
        let l_unwrap = match l {
            Ok(l_ok) =>  l_ok, 
            Err(l_err) => {
                println!("Err, could not read/unwrap line {}", l_err);
                continue;
            }
        };
        let mut l_split = l_unwrap.split(',');
        let l0: String = match l_split.nth(0) {
            Some(v) => String::from(v),
            None => {
                println!("could not get string for first column");
                String::from("None")
            }
        };
        let l3: f64 = match l_split.nth(2) {
            Some(v) => match v.parse() {
                Ok(ok) => ok,
                Err(_) => std::f64::NAN,
            },
            None => {
                println!("missing latitude");
                std::f64::NAN
            },
        };
        let l4: f64 = match l_split.nth(0) {
            Some(v) => match v.parse() {
                Ok(ok) => ok,
                Err(_) => std::f64::NAN,
            },
            None => {
                println!("missing longitude");
                std::f64::NAN
            },
        };
        city_vec.push(l0);
        lat_vec.push(l3);
        long_vec.push(l4);
    }
    let lat_array = Array::from(lat_vec);
    let long_array = Array::from(long_vec);
    return(lat_array, long_array)
}


// BENCHMARKS

#[bench]
fn testing_csv_serde_ndarary(b: &mut test::Bencher) {
    let file = String::from("uspop.csv");
    b.iter(|| {
        let (lat, long) = csv_serde_ndarray(&file);
        test::black_box((lat, long));
    });
}

#[bench]
fn testing_csv_no_serde_ndarray(b: &mut test::Bencher) {
    let file = String::from("uspop.csv");
    b.iter(|| {
        let (lat, long) = csv_no_serde_ndarray(&file);
        test::black_box((lat, long));
    });
}

#[bench]
fn testing_buffer_ndarray(b: &mut test::Bencher) {
    let file = String::from("uspop.csv");
    b.iter(|| {
        let (lat, long) = buffer_ndarray(&file);
        test::black_box((lat, long));
    });
}

#[bench]
fn testing_buffer_string(b: &mut test::Bencher) {
    let file = String::from("uspop.csv");
    b.iter(|| {
        let line_string = buffer_string(&file);
        test::black_box(line_string);
    });
}
