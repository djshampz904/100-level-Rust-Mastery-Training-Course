use txtprocessor;
use std::path::Path;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mycontent = txtprocessor::read_file(Path::new("data/poem.txt"))?;
    
    let line_count = txtprocessor::count_lines(&mycontent);
    let count_characters = txtprocessor::count_characters(&mycontent);
    let count_words = txtprocessor::count_words(&mycontent);

    println!("Word count: {}", count_words);
    println!("Line count: {}", line_count);
    println!("Character count: {}", count_characters);
    Ok(())

}
