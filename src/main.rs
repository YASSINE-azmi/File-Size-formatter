use std::env;

#[derive(Debug)]
enum FileSize {
    Bytes(f64),
    Kilobytes(f64),
    Megabytes(f64),
    Gigabytes(f64),
    Terabytes(f64),
}

#[derive(Debug)]
struct Sizes {
    size: FileSize,
}

impl Sizes {
    fn parse(input: &str) -> Result<Sizes, String> {
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        
        if parts.len() != 2 {
            return Err(String::from("Invalid format. Please use a format like '300 kb'"));
        }

        let value = parts[0].parse::<f64>().map_err(|_| "Failed to parse the number. Ensure it's a valid digit.")?;
        
        let unit = parts[1].to_lowercase();

        let size = match unit.as_str() {
            "bytes" | "b" => FileSize::Bytes(value),
            "kb" => FileSize::Kilobytes(value),
            "mb" => FileSize::Megabytes(value),
            "gb" => FileSize::Gigabytes(value),
            "tb" => FileSize::Terabytes(value),
            _ => return Err(format!("Unrecognized size identifier: '{}'", unit)),
        };

        Ok(Sizes { size })
    }

    fn to_bytes(&self) -> f64 {
        match self.size {
            FileSize::Bytes(v) => v,
            FileSize::Kilobytes(v) => v * 1024.0,
            FileSize::Megabytes(v) => v * 1024.0 * 1024.0,
            FileSize::Gigabytes(v) => v * 1024.0 * 1024.0 * 1024.0,
            FileSize::Terabytes(v) => v * 1024.0 * 1024.0 * 1024.0 * 1024.0,
        }
    }

    fn display(&self) -> String {
        match self.size {
            FileSize::Bytes(v) => format!("{:.2} Bytes", v),
            FileSize::Kilobytes(v) => format!("{:.2} KB", v),
            FileSize::Megabytes(v) => format!("{:.2} MB", v),
            FileSize::Gigabytes(v) => format!("{:.2} GB", v),
            FileSize::Terabytes(v) => format!("{:.2} TB", v),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Error: Please provide a file size argument.");
        println!("Usage example: cargo run -- \"12 mb\"");
        return;
    }

    let input = &args[1];

    match Sizes::parse(input) {
        Ok(sizes_struct) => {
            println!("--- Debug Info ---");
            println!("{:?}\n", sizes_struct);

            println!("--- Computed Results ---");
            println!("Formatted Size: {}", sizes_struct.display());
            println!("Total in Bytes: {:.2} bytes", sizes_struct.to_bytes());
        },
        Err(e) => {
            println!("Error processing input: {}", e);
        }
    }
}