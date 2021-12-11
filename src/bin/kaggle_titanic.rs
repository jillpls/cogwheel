use std::io::BufRead;
use csv::StringRecord;
use nalgebra::dvector;
use cogwheel::*;

fn main() {
    let mut cw = CogWheel::new(vec![2,10,10,10,2], None);
    cw.init_random_weights();
    println!("{:?}", cw);
    let r = cw.run(dvector!(1.0, 2.0)).unwrap_or_else(|x| {
        println!("{:?}", x);
        panic!();
    });
    let mut reader = parse_input("examples/kaggle_titanic/train.csv").unwrap();
    let headers = reader.headers().unwrap();
    let records = reader.records().map(|x| x.unwrap() ).collect::<Vec<StringRecord>>();
    for r in records {

    }
    println!("{:?}", records);

    println!("{}", r);
}

fn parse_input( path : &str ) -> Result<csv::Reader<std::io::BufReader<std::fs::File>>, Box<dyn std::error::Error>>{
    let file = std::fs::File::open(path)?;
    let reader = csv::Reader::from_reader(std::io::BufReader::new(file) );
    Ok(reader)
}