use std::{error::Error, fs::File, io::BufReader};

mod osm_parser;
mod osm_types;

const PATH: &str = "./osm/map.osm";

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open(PATH)?;
    let file = BufReader::new(file);
    let (nodes, ways, _) = osm_parser::parse_osm(file)?;
    let nodes = osm_parser::parse_neighbors(nodes, &ways);

    // ...

    Ok(())
}
