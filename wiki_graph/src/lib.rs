use std::{
    fs::{read, File},
    io::{BufRead, BufReader, Read, Seek, SeekFrom},
    path::Path,
    time::Instant, convert::TryInto,
};

use bincode2::{deserialize, serialize};
use bzip2::read::BzDecoder;
use serde::{Deserialize, Serialize};

pub struct IndexData {
    pub offset: usize,
    pub id: usize,
}

pub fn get_article_offset_id(index_file: &str, article_title: &str) -> Option<IndexEntry> {
    println!("Opening index located at {}", index_file);

    let index_file = match File::open(index_file) {
        Ok(file) => file,
        Err(e) => panic!("couldn't open {}: {}", index_file, e),
    };
    let index_file = BufReader::new(index_file);

    for article_data in index_file.lines() {
        let article_data = article_data.unwrap();
        if article_data.contains(article_title) {
            let data: Vec<&str> = article_data.split(':').collect();

            let offset: usize = data[0].parse().expect("couldn't parse article byte offset");
            let id: usize = data[1].parse().expect("couldn't parse article ID");
            let title = data[2].trim().to_string();
            return Some(IndexEntry { title, offset, id });
        }
    }

    None
}

pub fn count_lines(index_file: &str) -> usize {
    let index_file = File::open(index_file).unwrap();
    let index_file = BufReader::new(index_file);
    let lines = index_file.lines();
    let mut count = 0;
    for _line in lines {
        count += 1;
    }
    return count;
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexEntry {
    pub title: String,
    pub id: usize,
    pub offset: usize,
}

pub fn build_index(index_file: &str) -> Vec<IndexEntry> {
    let num_articles = count_lines(&index_file);

    let mut index: Vec<IndexEntry> = Vec::with_capacity(num_articles);

    let index_file = File::open(index_file).unwrap();
    let index_file = BufReader::new(index_file);

    for article_data in index_file.lines() {
        let article_data = article_data.unwrap();
        let data: Vec<&str> = article_data.split(':').collect();

        let offset: usize = data[0].parse().expect("couldn't parse article byte offset");
        let id: usize = data[1].parse().expect("couldn't parse article ID");
        let title = data[2].trim().to_string();

        index.push(IndexEntry { title, id, offset });
    }
    let t = Instant::now();
    index.sort_unstable_by(|a, b| a.title.cmp(&b.title));
    println!("{:?} seconds elapsed to sort index", t.elapsed());

    index
}

pub fn get_article_offset_id_from_index(
    index: &Vec<IndexEntry>,
    article_title: &str,
) -> Option<IndexEntry> {
    let result = index.binary_search_by(|a| a.title.cmp(&article_title.to_string()));

    match result {
        Ok(i) => Some(index[i].clone()),
        Err(_) => {
            println!("article \'{}\' not found", article_title);
            None
        }
    }
}

pub fn get_article(data_file: &Path, offset: usize, _id: usize) -> String {
    let data_file =
        File::open(data_file).expect(stringify!("couldn't open file {}", data_file.display()));
    let mut data_file = BufReader::new(data_file);
    let offset: u64 = offset.try_into().expect("offset too large???");
    data_file.seek(SeekFrom::Start(offset)).expect(stringify!("couldn't seek to offset {}", offset));
    let mut decompressor = BzDecoder::new(data_file);
    let mut contents = String::new();
    decompressor
        .read_to_string(&mut contents)
        .expect("failed to decompress");

    contents
}
pub fn save_index(index: &Vec<IndexEntry>, file_name: &str) {
    let binary_data = serialize(index).unwrap();
    std::fs::write(file_name, binary_data).unwrap();
}

pub fn load_index(file_name: &str) -> Vec<IndexEntry> {
    let binary_data = read(file_name).unwrap();
    let index: Vec<IndexEntry> = deserialize(&binary_data).unwrap();

    index
}
