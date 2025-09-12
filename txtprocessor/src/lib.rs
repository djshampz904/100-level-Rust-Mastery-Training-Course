use std::fs;
use std::error::Error;
use std::io::Write;
use std::fs::File;
use std::path::{ 
    Path,
    PathBuf
};
use std::collections::HashMap;

pub fn read_file(path: &Path) -> Result<String, std::io::Error> {
    match fs::read_to_string(path) {
        Ok(file) => Ok(file),
        Err(e) => {
            eprintln!("❌ Error reading file path: {}", e);
            Err(e)
        }
    }
}

pub fn count_lines(content: &str) -> usize {
    content.lines().collect::<Vec<&str>>().len()
}

pub fn count_characters(content: &str) -> usize {
    content.chars().collect::<Vec<char>>().len()
}

pub fn count_words(content: &str) -> usize {
    content.split_whitespace().collect::<Vec<&str>>().len()
}

pub fn write_file(path: &str, content: &str) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(path)?;
    write!(file, "{}", content)?;

    Ok(())
}

pub fn remove_duplicates(content: &str) -> String {
    let mut non_dups = String::new();
    let mut track_words: HashMap<String, usize> = HashMap::new();
    let content_lines = content.lines().collect::<Vec<&str>>();

    for line in content_lines {
        // vector for storing the extracted non dups words
        let mut extracted_words: Vec<&str> = Vec::new();

        // split the line and create a vector of words
        let line_content = line.split_whitespace().collect::<Vec<&str>>();

        // loop through the line and check the words
        for word in line_content {
            // put word in lowercase
            let lowercase_word = word.to_lowercase();
            let look_up = lowercase_word.as_str();

            // if word not already in the hash insert it or continue if its there
            if track_words.contains_key(look_up) {
                continue;
            }

            track_words.insert(lowercase_word, 1);
            extracted_words.push(word);
        }
        non_dups.push_str(extracted_words.join(" ").as_str());
        non_dups.push('\n');
    }
    non_dups
}


pub fn scan_duplicates(content: &str) -> String {
    let mut dups: HashMap<&str, usize> = HashMap::new();
    let mut mychars = content.chars().collect::<Vec<char>>();
    let mut new_content = String::new();

    // remove punctuations
    for (i, item) in mychars.iter().enumerate() {
    match *item {
        ',' | '.' | '/' | '?' | '!' | ':' | ';' | '\'' | '"' | '(' | ')' | '[' | ']' | '{' | 
            '}' | '-' | '_' | '@' | '#' | '$' | '%' | '^' | '&' | '*' | '+' | '=' | '<' | '>'
            | '\\' | '|' => {
                
            }
        _ => new_content.push(*item)
    }
    }

    let mut myhash: HashMap<&str, i32> = HashMap::new();

    let mywords = new_content.split_whitespace().collect::<Vec<&str>>();

    for word in mywords {
        *myhash.entry(word).or_insert(0) += 1;
    }
    

     new_content

}

#[cfg(test)]
mod test {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;

    #[test]
    fn test_filepath() {
        let result = read_file(Path::new("non_existent_file"));
        let file = NamedTempFile::new().unwrap();
        let result2 = read_file(Path::new(file.path().to_str().unwrap()));
        assert!(result2.is_ok());
        assert!(result.is_err());
    }

    #[test]
    fn test_linecount() -> Result<(), Box<dyn std::error::Error>> {
        let mut file = NamedTempFile::new().unwrap();
        let _ = file.write_all(b"Hello\nWorld");

        let result = read_file(Path::new(file.path().to_str().unwrap()))?;
        let line_count = count_lines(&result);
        assert_eq!(line_count, 2);
        
        Ok(())

    }

    #[test]
    fn test_charactercount() -> Result<(), Box<dyn std::error::Error>> {
        let mut file = NamedTempFile::new().unwrap();
        let _ = file.write_all(b"Hello\nWorld");

        let result = read_file(Path::new(file.path().to_str().unwrap()))?;
        let char_count = count_characters(&result);
        assert_eq!(char_count, 11);

        Ok(())
    }
        

}

