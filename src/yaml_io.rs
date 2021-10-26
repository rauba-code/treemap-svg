use std::io::{Read, Write};
use std::fs::File;
use std::path::{PathBuf};
use yaml_rust::Yaml;
use crate::algorithm::Node;

pub fn read_input(path : PathBuf) -> std::result::Result<String, String> {
    let display = path.display();
    
    let mut file = match File::open(&path) {
        Err(why) => return Err(format!("couldn't open {}: {}", display, why)),
        Ok(file) => file,
    };
    
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => Err(format!("couldn't read {}: {}", display, why)),
        Ok(_) => Ok(s)
    }
}



pub fn write_data(outpath : PathBuf, data : &[u8])
-> std::result::Result<(), String> {
    let mut output = match std::fs::File::create(&outpath) {
        Ok(s) => s,
        Err(err) => return Err(format!("couldn't open {}: {}", 
            outpath.to_str().unwrap_or_default(), err))
    };
    match output.write_all(data) {
        Ok(s) => Ok(s),
        Err(err) => Err(format!("couldn't write to {}: {}", 
            outpath.to_str().unwrap_or_default(), err))
    }
}

pub fn node_from_yaml(key : &Yaml, val : Option<&Yaml>) -> Node {
    match key {
        Yaml::Hash(h) => {
            let children : Vec<Node> = h.iter()
                    .map(|x| node_from_yaml(x.0, Some(x.1)))
                    .collect();
            let value = children.iter().map(|x| x.value).sum();
            Node { children, value, text: String::new() }
        }
        Yaml::String(text) => {
            match val {
                None => panic!(
                    "Error on parsing YAML input: no value is provided"),
                Some(va) => match va {
                    Yaml::Real(re) => {
                        Node { children: Vec::<Node>::new(),
                               value: re.parse::<f64>().unwrap(), 
                               text: text.clone() }
                    },
                    Yaml::Integer(re) => {
                        Node { children: Vec::<Node>::new(),
                               value: *re as f64, 
                               text: text.clone() }
                    },
                    Yaml::Hash(h) => {
                        let children : Vec<Node> = h.iter()
                                .map(|x| node_from_yaml(x.0, Some(x.1)))
                                .collect();
                        let value = children.iter().map(|x| x.value).sum();
                        Node { children, value, text: String::new() }
                    }
                    _ => panic!("Error on parsing YAML input: unexpected value")
                }
            }
            
        }
        _ => todo!()
    }
}