use crate::bit_writer::BitWriter;
use crate::huffman::HuffmanNode;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Read, Seek, Write};

pub fn compress(filename: String) {
    let f = File::open(&filename).unwrap();
    let w = File::create(filename + ".hof").unwrap();
    let mut buffered_writer = BufWriter::new(w);

    let mut reader = std::io::BufReader::new(f);
    let mut buffer = [0u8; 8142];
    let mut freq: HashMap<u8, i32> = HashMap::new();

    loop {
        let bites_read = reader.read(&mut buffer).unwrap();
        if bites_read == 0 { break; }

        for byte in &buffer[..bites_read] {
            *freq.entry(*byte).or_insert(0) += 1;
        }
    }
    reader.rewind().unwrap();
    
    let unique_symbols = freq.len() as u16;
    buffered_writer.write_all(&unique_symbols.to_le_bytes()).unwrap();

    for (&byte, &count) in &freq {
        buffered_writer.write_all(&[byte]).unwrap();
        buffered_writer.write_all(&(count as u32).to_le_bytes()).unwrap();
    }
    
    let huffman_tree = HuffmanNode::from_frequencies(freq);
    let dictionary = huffman_tree.get_dictionary(None);
    
    let mut bit_writer = BitWriter::new(buffered_writer);

    loop {
        let bites_read = reader.read(&mut buffer).unwrap();
        if bites_read == 0 { break; }

        for byte in &buffer[..bites_read] {
            if let Some(code) = dictionary.get(byte) {
                bit_writer.write_bits(*code, code.bit_width() as u8).expect("Error al escribir bits");
            }
        }
    }

    bit_writer.finish().expect("Error al finalizar la escritura");
    println!("¡Compresión finalizada con metadatos guardados!");
}