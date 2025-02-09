use std::{
    fs,
    io::{BufRead, BufReader, Error},
};

// Read file
pub fn read_file(path: &str) -> Result<String, Error> {
    match fs::File::open(path) {
        Ok(file) => {
            let reader = BufReader::new(file);
            let mut content = String::new();
            for line in reader.lines() {
                match line {
                    Ok(line_string) => {
                        content.push_str(&line_string);
                        // Manually append a newline to preserve the original format
                        content.push('\n');
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }

            Ok(content)
        }
        Err(e) => Err(e),
    }
}
