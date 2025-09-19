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

pub fn print_csv(mut data: Reader<File>) -> Result<(), Box<dyn std::error::Error>> {
    let my_records = data.records();

    for i in my_records {
        println!("{:?}", i);
    }

    Ok(())
}


pub fn print_columns(mut data: Reader<File>) -> Result<(), Box<dyn std::error::Error>> {

    let mut myheaders = data.headers()?;
    
    for column in myheaders {
        let column_name = column;
        println!("{}", column_name);
    }
    Ok(())
}

pub fn calculate_column_stats(mut data: Reader<File>, column_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut myheaders = data.headers()?;


    let index = myheaders.iter()
        .enumerate()
        .find(|(_, col)| *col == column_name)
        .map(|(i, _)| i).unwrap();

    println!("{:?}", index);

    
    Ok(())
}
