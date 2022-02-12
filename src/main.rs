mod app;
mod structs;

use structopt::StructOpt;
use structs::cli::Parameter;

fn main() {

    let opt = Parameter::from_args();
    println!("{:#?}", opt);

    app::exec();
}
