use compress::compress;
use decompress::decompress;
use utils::{Command, validate_file};

mod compress;
mod decompress;
pub mod huffman;
mod utils;

fn main() {
    let (command, filename) = utils::get_command_filename();

    validate_file(filename.clone(), command.clone());

    match command {
        Command::Compress => {
            compress(filename);
        }
        Command::Decompress => {
            decompress(filename);
        }
    }
}
