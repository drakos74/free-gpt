pub mod file {
    use std::env;
    use std::fs::File;
    use std::io::{self, BufRead};

    pub fn load(filename: &str) -> io::Result<()> {
        // Open the file in read-only mode
        let file = File::open(filename).or_else(|e| {
            println!("Error occurred: {}", e); // Custom handling of error
            let _ = File::create("empty-file")?;
            File::open("empty-file")
        });

        let reader = io::BufReader::new(file?);

        // Iterate over each line in the file
        for line in reader.lines() {
            let line = line?; // Unwrap the result of reading a line

            // Skip empty lines or comments
            if line.trim().is_empty() || line.starts_with('#') {
                continue;
            }

            // Split the line into key and value by the first '='
            if let Some(eq_pos) = line.find('=') {
                let key = &line[0..eq_pos].trim(); // Key is everything before '='
                let value = &line[eq_pos + 1..].trim(); // Value is everything after '='

                // Set the environment variable
                env::set_var(key, value);
            }
        }

        // Example: Printing a specific environment variable
        match env::var("KEY_STATUS") {
            Ok(value) => println!("KEY_STATUS: {}", value),
            Err(e) => println!("Couldn't read KEY_STATUS: {}", e),
        }

        Ok(())
    }
}
