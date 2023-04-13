use std::collections::HashMap;
use std::fs;

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Blupp {
    pub name: String,
    pub count: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Stuff {
    pub characters: Option<Vec<HashMap<String, usize>>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FacetCounts {
    pub facet_fields: Option<HashMap<String, Vec<HashMap<String, usize>>>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchResult {
    pub facet_counts: Option<FacetCounts>,
}

fn main() {
    let contents = fs::read_to_string("/Users/bumzack/stoff/micro-all-the-things/code/backend/rust/tooling/serialize_facets/src/res1.json").expect("Should have been able to read the file");

    // let a = get_facets();
    let search_result: SearchResult = serde_json::from_str(&contents).unwrap();
    println!("search_result {:?}", &search_result);

    // let s = get_data();

    // let dict: Stuff1 = serde_json::from_str(&s).unwrap();
    // println!("dict {:?}", &dict);

    //  let dict: Stuff = serde_json::from_str(&s).unwrap();

    //println!("dict {:?}", &dict);
}

fn get_facets() -> String {
    let s = " { \
    \"facetDistribution\":{
    \"genres\":{
      \"Action\":20,
      \"Adventure\":7,
    
      \"Thriller\":3
    },
    \"rating\":{
      \"2\":1,
  
      \"9.8\":1
    }
  } 
  }";
    s.to_string()
}

fn get_data() -> String {
    "{ \"characters\":[
    \"Terminator\",65,
    \"John Connor\",11,
    \"Sarah Connor\",8,
    \"Self\",8,
    \"T-1000\",6,
    \"Kuzya\",5,
    \"Kyle Reese\",5,
    \"Nastya\",5,
    \"Sasha\",5,
    \"Self - Host\",5,
    \"Maykl\",4,
    \"Darth Vader\",3,
    \"Eliot\",3,
    \"Matt\",3,
    \"Olga\",3,
    \"Ricky\",3,
    \"Rob\",3,
    \"Voz inicio\",3,
    \"Alla\",2,
    \"Clyde\",2,
    \"Luke Skywalker\",2,
    \"Marty McFly\",2,
    \"Police officer\",2,
    \"Professional Kid Reviewer\",2,
    \"RoboCop\",2,
    \"Scientist\",2,
    \"Aaron\",1,
    \"Adam\",1,
    \"Alan Schaefer\",1,
    \"Aleksandar 'Sasa' Popadic\",1,
    \"Alf Git\",1,
    \"Alfred Pennyworth\",1,
    \"Algot\",1,
    \"Alice\",1,
    \"Andrew\",1,
    \"Anne Gale Hurd\",1,
    \"Ash\",1,
    \"Aspiring Actor\",1,
    \"Atreyu\",1,
    \"Baby Porn\",1,
    \"Bad Guy # 1\",1,
    \"Bad Terminator\",1,
    \"Baltimore Drug Dealer\",1,
    \"Bar Cowboy\",1,
    \"Batman\",1,
    \"Bed Bugs Dad\",1,
    \"Biker #1\",1,
    \"Biker #2\",1,
    \"Bill\",1,
    \"Billy\",1,
    \"Black Stallion\",1,
    \"Boatswain\",1,
    \"Body Guard\",1,
    \"Boomstick - Host\",1,
    \"Boots the Monkey\",1,
    \"Box Office Attendant\",1,
    \"Boy losing his mask\",1,
    \"Boyfriend\",1,
    \"Brett\",1,
    \"Bruce\",1,
    \"Bryan Mills\",1,
    \"Bug Guy\",1,
    \"Bully-boy #1\",1,
    \"Bully-boy #2\",1,
    \"Bum\",1,
    \"Businessman\",1,
    \"Camarero\",1,
    \"Captain\",1,
    \"Carrie\",1,
    \"Chief Nowaczyk\",1,
    \"Chinese Guru\",1,
    \"Chris\",1,
    \"Circus Worker\",1,
    \"Claire\",1,
    \"Concessionist\",1,
    \"Corporal Yaeger\",1,
    \"Cyborg\",1,
    \"Cérna\",1,
    \"Dad\",1,
    \"Damian\",1,
    \"Dan Bull\",1,
    \"Danny\",1,
    \"Davi\",1,
    \"Derek\",1,
    \"Det. Derrick Parker\",1,
    \"Detective Dan Lewis\",1,
    \"Detective Joe Loughran\",1,
    \"Detective Ken Clancy\",1,
    \"Detective Mark Valencia\",1,
    \"Detective Pikachu\",1,
    \"Dirk Fate\",1,
    \"Doberman\",1,
    \"Doble Harry\",1,
    \"Doc Brown\",1,
    \"Doctor\",1,
    \"Donald\",1,
    \"Doorman\",1,
    \"Dr. Evil\",1,
    \"Dr. Silverman\",1,
    \"Dragisa 'Giga' Popadic\",1] }"
        .to_string()
}
