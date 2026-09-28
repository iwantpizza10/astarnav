use xml::{EventReader, reader::{Error, XmlEvent}};
use std::{collections::{HashMap}, fs::File, io::BufReader};
use crate::osm_types as osm;

type MappedI64Item<T> = HashMap<i64, T>;

pub fn parse_osm(osm_data: BufReader<File>) -> Result<(MappedI64Item<osm::Node>, MappedI64Item<osm::Way>, MappedI64Item<osm::Relation>), Error> {
    let parser = EventReader::new(osm_data);

    // output
    let mut nodes: MappedI64Item<osm::Node> = HashMap::new();
    let mut ways: MappedI64Item<osm::Way> = HashMap::new();
    let mut relations: MappedI64Item<osm::Relation> = HashMap::new();

    // state
    let mut current_node: Option<osm::Node> = None;
    let mut current_way: Option<osm::Way> = None;
    let mut current_relation: Option<osm::Relation> = None;

    for evt in parser {
        let evt = evt?;

        match evt {
            XmlEvent::StartElement { name, attributes, .. } => {
                if let Some(ref mut current_node) = current_node {
                    match name.local_name.as_str() {
                        "tag" => {
                            let mut key = String::new();
                            let mut value = String::new();

                            for attr in attributes {
                                if attr.name.local_name == "k" {
                                    key = attr.value;
                                } else if attr.name.local_name == "v" {
                                    value = attr.value;
                                }
                            }

                            if key == "" || value == "" {
                                eprintln!("{current_node:?} - tag {name} could not find k/v (found {key}/{value})");
                            }

                            current_node.tags.push(osm::Tag::from_kv(key, value));
                        },

                        _ => {}
                    }
                } else if let Some(ref mut current_way) = current_way {
                    match name.local_name.as_str() {
                        "nd" => {
                            let mut reference = -1;

                            for attr in attributes {
                                if attr.name.local_name == "ref" {
                                    reference = attr.value.parse().expect("osm data could not parse ref");
                                }
                            }

                            if reference == -1 {
                                eprintln!("{current_way:?} - tag {name} could not find ref (found {reference})");
                            }

                            current_way.nodes.push(reference);
                        },

                        "tag" => {
                            let mut key = String::new();
                            let mut value = String::new();

                            for attr in attributes {
                                if attr.name.local_name == "k" {
                                    key = attr.value;
                                } else if attr.name.local_name == "v" {
                                    value = attr.value;
                                }
                            }

                            if key == "" || value == "" {
                                eprintln!("{current_way:?} - tag {name} could not find k/v (found {key}/{value})");
                            }

                            current_way.tags.push(osm::Tag::from_kv(key, value));
                        }

                        _ => {}
                    }
                } else if let Some(ref mut current_relation) = current_relation {
                    match name.local_name.as_str() {
                        "member" => {
                            let mut classification = osm::MemberClassification::Node;
                            let mut reference = -1;
                            let mut role = "".to_string();

                            for attr in attributes {
                                if attr.name.local_name == "type" {
                                    classification = attr.value.parse().expect("osm data could not parse type");
                                } else if attr.name.local_name == "ref" {
                                    reference = attr.value.parse().expect("osm data could not parse ref");
                                } else if attr.name.local_name == "role" {
                                    role = attr.value;
                                }
                            }

                            if classification == osm::MemberClassification::None || reference == -1 { // role is optional
                                eprintln!("{current_relation:?} - tag {name} could not find type/ref (found {classification:?}/{reference})");
                            }

                            current_relation.members.push(osm::Member::from_values(classification, reference, role));
                        },

                        "tag" => {
                            let mut key = String::new();
                            let mut value = String::new();

                            for attr in attributes {
                                if attr.name.local_name == "k" {
                                    key = attr.value;
                                } else if attr.name.local_name == "v" {
                                    value = attr.value;
                                }
                            }

                            if key == "" || value == "" {
                                eprintln!("{current_relation:?} - tag {name} could not find k/v (found {key}/{value})");
                            }

                            current_relation.tags.push(osm::Tag::from_kv(key, value));
                        }

                        _ => {}
                    }
                } else {
                    if name.local_name == "node" {
                        let mut node: osm::Node = Default::default();

                        for attr in attributes {
                            match attr.name.local_name.as_str() {
                                "id" => node.id = attr.value.parse().expect("osm data could not parse id"),
                                "lat" => node.lat = attr.value.parse().expect("osm data could not parse lat"),
                                "lon" => node.lon = attr.value.parse().expect("osm data could not parse lon"),
                                _ => {}
                            }
                        }

                        current_node = Some(node);
                    } else if name.local_name == "way" {
                        let mut way: osm::Way = Default::default();

                        for attr in attributes {
                            match attr.name.local_name.as_str() {
                                "id" => way.id = attr.value.parse().expect("osm data could not parse id"),
                                _ => {}
                            }
                        }

                        current_way = Some(way);
                    } else if name.local_name == "relation" {
                        let mut relation: osm::Relation = Default::default();

                        for attr in attributes {
                            match attr.name.local_name.as_str() {
                                "id" => relation.id = attr.value.parse().expect("osm data could not parse id"),
                                _ => {}
                            }
                        }

                        current_relation = Some(relation);
                    }
                }
            },

            XmlEvent::EndElement { name } => {
                if name.local_name == "node" {
                    let node = current_node.expect("node did not exist upon reaching node closing tag");
                    nodes.insert(node.id, node);

                    current_node = None;
                } else if name.local_name == "way" {
                    let way = current_way.expect("way did not exist upon reaching way closing tag");
                    ways.insert(way.id, way);

                    current_way = None;
                } else if name.local_name == "relation" {
                    let relation = current_relation.expect("relation did not exist upon reaching relation closing tag");
                    relations.insert(relation.id, relation);

                    current_relation = None;
                }
            },

            _ => {}
        }
    }

    Ok((nodes, ways, relations))
}

pub fn parse_neighbors(nodes: MappedI64Item<osm::Node>, ways: &MappedI64Item<osm::Way>) -> MappedI64Item<osm::Node> {
    let mut nodes = nodes;

    for (_, way) in ways {
        for node_idx in 0..way.nodes.len() {
            if node_idx == 0 {
                let node_id = way.nodes[node_idx];

                if let Some(node) = nodes.get_mut(&node_id) {
                    node.neighbors.insert(way.nodes[1]);
                }
            } else if node_idx == way.nodes.len() - 1 {
                let node_id = way.nodes[node_idx];

                if let Some(node) = nodes.get_mut(&node_id) {
                    node.neighbors.insert(way.nodes[way.nodes.len() - 2]); // -2 because -1 is just the last element lmao
                }
            } else {
                let node_id = way.nodes[node_idx];

                if let Some(node) = nodes.get_mut(&node_id) {
                    node.neighbors.insert(way.nodes[node_idx - 1]);
                    node.neighbors.insert(way.nodes[node_idx + 1]);
                }
            }
        }
    }

    nodes
}
