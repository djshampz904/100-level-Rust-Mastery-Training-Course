use csv::{
    ReaderBuilder,
    Reader
};
use std::fs::File;

pub fn read_csv(path: &str, delimiter: char) -> Result<Reader<File>, Box<dyn std::error::Error>> {
    let mut csv_file = ReaderBuilder::new()
        .has_headers(true)
        .delimiter(delimiter as u8)
        .from_path(path)?;


    Ok(csv_file)
}

pub fn print_columns(mut data: Reader<File>) -> Result<(), Box<dyn std::error::Error>> {

    let mut myheaders = data.headers()?;
    
    for column in myheaders {
        let column_name = column;
        println!("{}", column_name);
    }
    Ok(())
}
