use std::{error::Error, fs::File, io::BufReader};

mod osm_parser;
mod osm_types;

const PATH: &str = "./osm/map.osm";

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open(PATH)?;
    let file = BufReader::new(file);
    let x = osm_parser::parse_osm(file)?;

    // !!!

    Ok(())
}
