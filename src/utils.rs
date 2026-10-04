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
