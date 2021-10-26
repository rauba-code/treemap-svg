use structopt::StructOpt;
use crate::utils::UnwrapOrExt;
use crate::ds::Printer;

extern crate yaml_rust;
use yaml_rust::YamlLoader;

mod ds;
mod yaml_io;
mod utils;
mod cli;
mod algorithm;


fn main() {
    let opt = cli::Opt::from_args();
    let text = yaml_io::read_input(opt.input).unwrap_or_exit();
    
    let doc = &YamlLoader::load_from_str(&text).unwrap()[0];
    let root_node = &mut yaml_io::node_from_yaml(doc, None);
    let rect = ds::Rect { a: ds::Point { x: 20.0, y: 20.0 },
        off: ds::Point { y: 800.0, x: 800.0 }};
    algorithm::process_order_nodes(root_node, &rect);
    let layer = algorithm::compose(root_node, &rect);
    
    let g = ds::Graphic { id: "chart1".to_string(), lines : layer.lines,
        texts : layer.texts, rect};
    let svgx = ds::Svgx { width: 900, height: 900, g, id: "svg1".to_string() };
    
    yaml_io::write_data(opt.output, svgx.print().as_bytes()).unwrap_or_exit();
    //println!("{}", svgx.print());
}
