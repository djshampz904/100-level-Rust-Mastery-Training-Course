use std::fs;
use std::path::Path;

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

