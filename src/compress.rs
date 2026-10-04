use crate::huffman::HuffmanNode;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Seek};

pub fn compress(filename: String) {
    let f = File::open(&filename).unwrap();
    let w = File::create(filename + ".hof").unwrap();
    let mut reader = std::io::BufReader::new(f);
    let mut writer = std::io::BufWriter::new(w);

    let mut buffer = [0u8; 8142];
    let mut bites_read = 0;
    let mut freq: HashMap<u8, i32> = HashMap::new();

    loop {
        bites_read = reader.read(&mut buffer).unwrap();

        if bites_read == 0 {
            break;
        }

        for byte in &buffer[..bites_read] {
            *freq.entry(*byte).or_insert(0) += 1;
        }
    }
    bites_read = 0;
    reader.rewind().unwrap();

    let huffman_tree = HuffmanNode::from_frequencies(freq);

    println!("{:?}", huffman_tree);

    loop {
        
    }
}
