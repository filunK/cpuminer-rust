
use std::path::PathBuf;
use structopt::StructOpt;

// fn is_config_provided() {
    
// }

#[derive(StructOpt, Debug)]
#[structopt(name = "cpuminer-rust")]
pub struct Parameter {

    /// Parameter Configuration
    /// 
    /// indicates the parameters instead of command line parameters.  
    /// You can use ALL LONG Paameters in JSON file excepted of conf parameter.
    #[structopt(short = "c", long = "conf")]
    confFile: Option<PathBuf>,

    /// Algorithm
    /// 
    /// indicates what you want to use an argorithm.
    #[structopt(short = "a", long = "algo")]
    algorithm: Option<String>,
}

