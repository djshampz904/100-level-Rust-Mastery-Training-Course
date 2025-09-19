use analyzer::{
    read_csv,
    print_columns,
    print_csv,
    calculate_column_stats,
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

#[derive(Parser, Debug)]
enum Commands {
    Print {
        #[clap(long, short)]
        path: String,
        #[clap(long, short, default_value=",")]
        delimiter: String,
    },
    Headers {
        #[clap(long, short)]
        path: String,
        #[clap(long, short, default_value=",")]
        delimiter: String,
    },
    Stats {
        #[clap(long, short)]
        path: String,
        #[clap(long, short, default_value=",")]
        delimiter: String,
        #[clap(long, short, default_value="id")]
        column: String,
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>> { 
    let args = Cli::parse();
    
    match args.command {
        Some(Commands::Print { path, delimiter }) => {
            let my_delimiter = delimiter.chars().next().unwrap();
            let my_data = read_csv(&path, my_delimiter)?; 
            print_csv(my_data);
        }
        Some(Commands::Headers { path, delimiter }) => {
            let my_delimiter = delimiter.chars().next().unwrap();
            let my_data = read_csv(&path, my_delimiter)?;
            print_columns(my_data);
        }
        Some(Commands::Stats { path, delimiter, column }) => {
            let my_delimiter = delimiter.chars().next().unwrap();
            let my_data = read_csv(&path, my_delimiter)?;
            let my_column = column.as_str();

            calculate_column_stats(my_data, my_column);
        }
        _ =>  {
            println!("No arguments passed");
        }
    };
        

    Ok(())
}
