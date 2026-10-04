use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use crate::huffman::HuffmanNode;
use crate::utils::read_header;
use crate::bit_reader::BitReader;

pub fn decompress(filename: String) {
    let output_filename = if filename.ends_with(".hof") {
        filename[..filename.len() - 4].to_string()
    } else {
        format!("{}.dec", filename)
    };

    let f = File::open(&filename).expect("No se pudo abrir el archivo .hof");
    let mut reader = BufReader::new(f);
    
    let freq = read_header(&mut reader);
    let huffman_tree = HuffmanNode::from_frequencies(freq);
    
    let mut bit_reader = BitReader::new(reader);
    
    let out_file = File::create(&output_filename).expect("No se pudo crear el archivo descomprimido");
    let mut writer = BufWriter::new(out_file);
    
    loop {
        if let Some(decoded_byte) = huffman_tree.decode_byte(&mut bit_reader) {
            writer.write_all(&[decoded_byte]).unwrap();
        } else {
            break;
        }
    }

    writer.flush().unwrap();
    println!("¡Archivo descomprimido con éxito en: {}!", output_filename);
}