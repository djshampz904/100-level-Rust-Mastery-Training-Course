use analyzer::{
    read_csv,
    print_columns
};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name="csv-analyzer",
    version="1.0",
    about="Tool for analyzing csv files"
)]

struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>
}

enum Commands {
    Print {
        #[clap(long, short)]
        path: String,
        delimiter: String,
    }
}



fn main() -> Result<(), Box<dyn std::error::Error>> { 
    let args = Args::parse();
    


    Ok(())
}
