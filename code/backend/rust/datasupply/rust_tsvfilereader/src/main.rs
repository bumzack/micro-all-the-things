use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

use serde_json::json;

use common::TsvLine;

#[tokio::main]
async fn main() -> io::Result<()> {
    let filename =
        "/Users/bumzack/stoff/micro-all-the-things/imdb_data/title.principals.tsv".to_string();
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    for line in reader.lines().take(100) {
        let original = line.unwrap();
        let entries = original
            .clone()
            .split("\t")
            .map(|s| s.to_string())
            .collect();

        let tsv = TsvLine {
            entries,
            original,
        };

        println!("tsv {:?}", &tsv);

        let json = json!(&tsv).to_string();

        let client = reqwest::Client::new();
        match client
            .post("http://localhost:18104/api/principal")
            .body(json)
            .send()
            .await
        {
            Ok(res) => println!("request ok. response  {:?}", &res),
            Err(e) => println!("error request {:?}", e),
        }
    }
    println!("DONE");
    Ok(())
}
