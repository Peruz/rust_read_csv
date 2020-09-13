#![feature(test)]
extern crate test;

use serde::Deserialize;
use std::fs::File;
use clap::{Arg, App};
use std::io::{BufRead, BufReader};
use arrow;

fn main() {
    let file = get_args();
    println!("reading {}", file);
    let citypop = std_string_buffer(&file);
    println!("{:?}", citypop);
    let citypop = std_lines(&file);
    println!("{:?}", citypop);
    std_byte_buffer(&file);
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
    fn new(capacity: usize) -> CityPop {
        let city: Vec<String> = Vec::with_capacity(capacity);
        let state: Vec<String> = Vec::with_capacity(capacity);
        let population: Vec<Option<u32>> = Vec::with_capacity(capacity);
        let latitude: Vec<f64> = Vec::with_capacity(capacity);
        let longitude: Vec<f64> = Vec::with_capacity(capacity);
        let citypop: CityPop = CityPop {city, state, population, latitude, longitude};
        citypop
    }
}

fn arrow_with_schema(arg_file: &String) {
    let schema = arrow::datatypes::Schema::new(vec![
            arrow::datatypes::Field::new("city", arrow::datatypes::DataType::Utf8, false),
            arrow::datatypes::Field::new("state", arrow::datatypes::DataType::Utf8, false),
            arrow::datatypes::Field::new("population", arrow::datatypes::DataType::Int64, false),
            arrow::datatypes::Field::new("latitude", arrow::datatypes::DataType::Float64, false),
            arrow::datatypes::Field::new("longitude", arrow::datatypes::DataType::Float64, false),
        ]);
    let builder = arrow::csv::ReaderBuilder::new()
            .infer_schema(None)
            .has_header(true)
            .with_schema(std::sync::Arc::new(schema))
            .with_delimiter(b',')
            .with_batch_size(11000);
    let file = File::open(arg_file).unwrap();
    let mut csv = builder.build(file).unwrap();
    let batch = csv.next().unwrap().unwrap();
}

fn csv_with_serde(arg_file: &String) -> CityPop {
    let file = File::open(arg_file).expect("could not open file ");
    let mut reader = csv::Reader::from_reader(file);
    let mut citypop = CityPop::new(4000 as usize);
    for result in reader.deserialize() {
        let record: Record = result.unwrap();
        citypop.add_record(record)
    }
    citypop
}

fn csv_no_serde(arg_file: &String) -> CityPop {
    let file = File::open(arg_file).expect("could not open file");
    let mut rdr = csv::Reader::from_reader(file);
    let mut citypop = CityPop::new(4000 as usize);
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

fn csv_byte_record_deserialize(arg_file: &String) -> CityPop {
    let file = File::open(arg_file).unwrap();
    let mut reader = csv::Reader::from_reader(file);
    let mut record = csv::ByteRecord::new();
    let mut citypop = CityPop::new(4000 as usize);
    let header = csv::ByteRecord::from(
        vec!["city", "state", "population", "latitude", "longitude"]
    );
    while reader.read_byte_record(&mut record).unwrap() {
        let c: Record = record.deserialize(Some(&header)).unwrap();
    }
    citypop
}

fn std_string_buffer(arg_file: &String) -> CityPop {
    let file = File::open(arg_file).unwrap();
    let mut filebuffer = BufReader::new(file);
    let mut linebuffer = String::new();
    let mut citypop = CityPop::new(10000 as usize);
    filebuffer.read_line(&mut linebuffer).unwrap();
    linebuffer.clear();
    while filebuffer.read_line(&mut linebuffer).unwrap() != 0 {
        let mut l_split = linebuffer.split(',');
        citypop.city.push(
            match l_split.next() {
                Some(v) => String::from(v),
                None => String::from("None")
            }
        );
        citypop.state.push(
            match l_split.next() {
                Some(v) => String::from(v),
                None => String::from("None")
            }
        );
        citypop.population.push(
            match l_split.next() {
                Some(v) => match v.parse() {
                    Ok(ok) => Some(ok),
                    Err(_) => None,
                },
                None => None,
            }
        );
        citypop.latitude.push(
            match l_split.next() {
                Some(v) => match v.parse() {
                    Ok(ok) => ok,
                    Err(_) => std::f64::NAN,
                },
                None => std::f64::NAN,
            }
        );
        citypop.longitude.push(
            match l_split.next() {
                Some(v) => match v.trim_end().parse() {
                    Ok(ok) => ok,
                    Err(_) => std::f64::NAN
                },
                None => std::f64::NAN,
            }
        );
        linebuffer.clear()
    }
    return citypop
}

fn std_lines(arg_file: &String) -> CityPop {
    let file = File::open(arg_file).unwrap();
    let buf = BufReader::new(file);
    let mut citypop = CityPop::new(10000 as usize);
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
                None => String::from("None")
            }
        );
        citypop.state.push(
            match l_split.next() {
                Some(v) => String::from(v),
                None => String::from("None")
            }
        );
        citypop.population.push(
            match l_split.next() {
                Some(v) => match v.parse() {
                    Ok(ok) => Some(ok),
                    Err(_) => None,
                },
                None => None,
            }
        );
        citypop.latitude.push(
            match l_split.next() {
                Some(v) => match v.parse() {
                    Ok(ok) => ok,
                    Err(_) => std::f64::NAN,
                },
                None => std::f64::NAN,
            }
        );
        citypop.longitude.push(
            match l_split.next() {
                Some(v) => match v.parse() {
                    Ok(ok) => ok,
                    Err(_) => std::f64::NAN
                },
                None => std::f64::NAN,
            }
        );
    }
    return citypop
}

// just for reference
fn std_lines_onlyloop(arg_file: &String) { 
    let file = File::open(arg_file).unwrap();
    let buf = BufReader::new(file);
    for _l in buf.lines().skip(1) {
        continue;
    }
}

// just for reference
fn std_string_buffer_onlywhile(arg_file: &String) { 
    let file = File::open(arg_file).unwrap();
    let mut filebuffer = BufReader::new(file);
    let mut linebuffer = String::new();
    filebuffer.read_line(&mut linebuffer).unwrap();
    linebuffer.clear();
    while filebuffer.read_line(&mut linebuffer).unwrap() != 0 {
        linebuffer.clear()
    }
}

fn std_byte_buffer(arg_file: &String) {
    let file = File::open(arg_file).unwrap();
    let mut filebuffer = BufReader::new(file);
    let mut linebuffer = Vec::new();
    while filebuffer.read_until(b'\n', &mut linebuffer).unwrap() != 0 {
        // println!("{:?}", linebuffer);
        let linebuffer_split: Vec<_> = linebuffer.split(|i| *i == 44).collect();
        // println!("{:?}", linebuffer_split);
    }
}


// BENCHMARKS

#[bench]
fn bench_std_lines(b: &mut test::Bencher) {
    let file = String::from("uspop.csv");
    b.iter(|| {
        let citypop = std_lines(&file);
        test::black_box(citypop);
    });
}

#[bench]
fn bench_std_string_buffer(b: &mut test::Bencher) {
    let file = String::from("uspop.csv");
    b.iter(|| {
        let citypop = std_string_buffer(&file);
        test::black_box(citypop);
    });
}

#[bench]
fn bench_csv_no_serde(b: &mut test::Bencher) {
    let file = String::from("uspop.csv");
    b.iter(|| {
        let citypop = csv_no_serde(&file);
        test::black_box(citypop);
    });
}

#[bench]
fn bench_csv_with_serde(b: &mut test::Bencher) {
    let file = String::from("uspop.csv");
    b.iter(|| {
        let citypop = csv_with_serde(&file);
        test::black_box(citypop);
    });
}

#[bench]
fn bench_std_lines_onlyloop(b: &mut test::Bencher) {
    let file = String::from("uspop.csv");
    b.iter(|| {
        std_lines_onlyloop(&file);
    });
}

#[bench]
fn bench_std_string_buffer_onlywhile(b: &mut test::Bencher) {
    let file = String::from("uspop.csv");
    b.iter(|| {
        std_string_buffer_onlywhile(&file);
    });
}

// #[bench]
// fn bench_std_byte_buffer_onlywhile(b: &mut test::Bencher) {
//     let file = String::from("uspop.csv");
//     b.iter(|| {
//         std_byte_buffer(&file);
//     });
// }

#[bench]
fn bench_csv_byte_record_deserialize(b: &mut test::Bencher) {
    let file = String::from("uspop.csv");
    b.iter(|| {
        csv_byte_record_deserialize(&file);
    });
}

#[bench]
fn bench_arrow_with_schema(b: &mut test::Bencher) {
    let file = String::from("uspop.csv");
    b.iter(|| {
        arrow_with_schema(&file);
    });
}
