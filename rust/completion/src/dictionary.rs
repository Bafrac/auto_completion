use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn create_dictionary() -> Vec<String>
{   
    let dict_path = "/app/Dictionarys/Dictionary_harry_potter.txt";

    let mut dictionary = Vec::new();

    match File::open(dict_path)
    {
        Ok(file) =>
        {
            let reader = BufReader::new(file);
            for line in reader.lines()
            {
                if let Ok(line) = line
                {
                    let trimmed = line.trim();
                    if !trimmed.is_empty()
                    {
                        let word = trimmed.split_whitespace()
                            .next()
                            .unwrap_or("")
                            .to_string();
                        if !word.is_empty(){dictionary.push(word);}
                    }
                }
            }
            println!("Dictionary created with {} terms", dictionary.len());
        }
        Err(e) => eprintln!("Error reading dictionary: {}", e),
    }
    dictionary
}