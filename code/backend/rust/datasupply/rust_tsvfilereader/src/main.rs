use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

use common::TsvLine;

fn main() -> io::Result<()> {
    read_file()?;
    Ok(())
}

fn read_file() -> io::Result<()> {
    let filename = "/Users/gsc/stoff/micro-all-the-things/imdb_data/title.principals.tsv".to_string();
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    for line in reader.lines().take(10) {
        let original = line.unwrap();
        let entries = original.clone()
            .split("\t")
            .map(|s|s.to_string())
            .collect();
        let tsv = TsvLine {
            entries,
            original,
        };
        println!("tsv {:?}", &tsv);
    }
    println!("DONE");
    Ok(())
}