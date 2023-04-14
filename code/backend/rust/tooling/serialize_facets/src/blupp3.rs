// https://stackoverflow.com/questions/57903579/rust-serde-deserializing-a-mixed-array

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Order4 {
    price: String,
    whole_lot_volume: i64,
    lot_volume: f64,
}

#[derive(Debug, Deserialize)]
struct Ask4 {
    a: Order4,
}

#[derive(Debug, Deserialize)]
struct MyStruct4 {
    id: i32,
    a: Ask4,
}

pub fn blupp3() {
    let str = r#"
       [  
            1,  
            {
                "a":  [
                    "1.2345", 
                    5, 
                    9.8765
                 ]
            }
       ]
    "#
    .to_string();

    // let a = get_facets();
    let my_struct: MyStruct4 = serde_json::from_str(&str).unwrap();
    println!("my_struct {:?}", &my_struct);
}

pub fn blupp4() {
    let str = r#"
       [  
            1,  
            {
                "a":  [
                    "1.2345", 
                    5, 
                    9.8765
                 ]
            }
       ]
    "#
    .to_string();

    // let a = get_facets();
    let my_struct: MyStruct4 = serde_json::from_str(&str).unwrap();
    println!("my_struct {:?}", &my_struct);
}
