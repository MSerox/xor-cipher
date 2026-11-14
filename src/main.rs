use std::{
    fs::File,
    io::{self, BufReader, BufWriter, ErrorKind, Read, Write},
};

enum AppError {
    Io(io::Error),
    CustomRule(String),
}

impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        AppError::Io(error)
    }
}

fn main() {
    loop {
        let file_path =
            match get_input("Exit by enter /q\nPlease enter the path of your file (without \"\"):")
            {
                Ok(value) => value,
                Err(error) => match error {
                    AppError::Io(e) => {
                        println!("Error: {}", e);
                        continue;
                    }
                    AppError::CustomRule(msg) => {
                        println!("Error: {}", msg);
                        continue;
                    }
                },
            };
        if file_path == "/q" {
            break;
        }

        let file = match File::open(file_path) {
            Ok(f) => f,
            Err(e) => match e.kind() {
                ErrorKind::PermissionDenied => {
                    println!("Error: Permission denied.");
                    continue;
                }
                ErrorKind::NotFound => {
                    println!("Error: File not found.");
                    continue;
                }
                _ => {
                    println!("Error: {}", e);
                    continue;
                }
            },
        };

        let key = match get_input("Please enter your key:") {
            Ok(value) => value,
            Err(error) => match error {
                AppError::Io(e) => {
                    println!("Error: {}", e);
                    continue;
                }
                AppError::CustomRule(msg) => {
                    println!("Error: {}", msg);
                    continue;
                }
            },
        };
        if key == "/q" {
            break;
        }

        let key: Vec<u8> = key.trim().bytes().collect();

        if key.is_empty() {
            println!("Key can't be empty!");
            continue;
        }

        let out_path = match get_input("Enter the out path with file extension and name:") {
            Ok(value) => value,
            Err(error) => match error {
                AppError::Io(e) => {
                    println!("Error: {}", e);
                    continue;
                }
                AppError::CustomRule(msg) => {
                    println!("Error: {}", msg);
                    continue;
                }
            },
        };
        if out_path == "/q" {
            break;
        }

        let out_file = match File::create(out_path) {
            Ok(f) => f,
            Err(e) => match e.kind() {
                ErrorKind::PermissionDenied => {
                    println!("Error: Permission denied.");
                    continue;
                }
                ErrorKind::NotFound => {
                    println!("Error: Path not found.");
                    continue;
                }
                _ => {
                    println!("Error: {}", e);
                    continue;
                }
            },
        };

        let file_size = match file.metadata() {
            Ok(meta) => meta.len(),
            Err(e) => {
                println!("Error getting file metadata: {}", e);
                continue;
            }
        };
        let mut total_read: usize = 0;

        let mut reader = BufReader::new(file);
        let mut writer = BufWriter::with_capacity(1024 * 1024, out_file);
        const BUFFER_SIZE: usize = 64 * 1024;
        let mut buffer = [0; BUFFER_SIZE];

        let mut index = 0;
        let mut last_printed = 0;
        let print_threshold = file_size as usize / 100;
        println!("Starting processing...");
        loop {
            let n = match reader.read(&mut buffer) {
                Ok(n) => n,
                Err(e) => {
                    println!("Error reading file: {}", e);
                    break;
                }
            };

            if n == 0 {
                break;
            }

            for byte in buffer.iter_mut().take(n) {
                *byte ^= key[index % key.len()];
                index += 1;
            }

            if let Err(e) = writer.write_all(&buffer[..n]) {
                println!("Error writing to file: {}", e);
                break;
            }

            total_read += n as usize;

            if total_read - last_printed >= print_threshold {
                let percentage = (total_read as f64 / file_size as f64) * 100.0;
                print!(
                    "\rProgress: {:.2}%  [{}/{}] bytes",
                    percentage, total_read, file_size
                );
                if let Err(e) = io::stdout().flush() {
                    println!("Error flushing stdout: {}", e);
                    break;
                }

                last_printed = total_read;
            }
        }
        println!(
            "\rProgress: 100.00% [{}/{}] bytes - Done.",
            file_size, file_size
        );
    }
}

fn get_input(message: &str) -> Result<String, AppError> {
    let mut variable = String::new();
    println!("{message}");
    io::stdin().read_line(&mut variable)?;
    let variable = variable.trim();
    if variable.is_empty() {
        return Err(AppError::CustomRule("Input can't be empty".to_string()));
    }
    Ok(variable.to_string())
}