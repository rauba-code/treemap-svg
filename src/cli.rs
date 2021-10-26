use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "treemap-svg")]
pub struct Opt {
    /// The input file, in YAML
    /// 
    /// TODO: complete doc
    #[structopt(short, long, parse(from_os_str))]
    pub input : PathBuf,
    
    /// The configuration file, in YAML
    /// 
    /// TODO: complete doc
    #[structopt(short, long, parse(from_os_str))]
    pub config : Option<PathBuf>,
    
    /// The output file, containing the SVG data
    #[structopt(short, long, parse(from_os_str))]
    pub output : PathBuf
}