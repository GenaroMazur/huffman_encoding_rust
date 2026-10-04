use std::collections::HashMap;
use std::io::Read;

pub enum Command {
    Compress,
    Decompress,
}

impl Command {
    pub fn clone(&self) -> Command {
        match self {
            Command::Compress => Command::Compress,
            Command::Decompress => Command::Decompress,
        }
    }
}

pub fn get_command_filename() -> (Command, String) {
    let args = std::env::args().collect::<Vec<String>>();

    let command = match args[1].as_str() {
        "compress" => Command::Compress,
        "decompress" => Command::Decompress,
        _ => panic!("Invalid command"),
    };

    let filename = args.get(2).expect("Invalid filename").clone();

    (command, filename)
}

pub fn validate_file(filename: String, command: Command) {
    if !std::path::Path::new(&filename).exists() {
        panic!("File does not exist");
    }

    match command {
        Command::Compress => {
            if filename.ends_with(".hof") {
                panic!("File is already compressed");
            }
        }
        Command::Decompress => {
            if !filename.ends_with(".hof") {
                panic!("File is not compressed");
            }
        }
    }
}

pub fn read_header<R: Read>(reader: &mut R) -> HashMap<u8, i32> {
    let mut freq = HashMap::new();

    let mut num_symbols_buf = [0u8; 2];
    reader.read_exact(&mut num_symbols_buf).unwrap();
    let num_symbols = u16::from_le_bytes(num_symbols_buf);

    for _ in 0..num_symbols {
        let mut byte_buf = [0u8; 1];
        let mut count_buf = [0u8; 4];

        reader.read_exact(&mut byte_buf).unwrap();
        reader.read_exact(&mut count_buf).unwrap();

        let byte = byte_buf[0];
        let count = u32::from_le_bytes(count_buf) as i32;

        freq.insert(byte, count);
    }

    freq
}
