use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub struct IndexData {
    pub offset: usize,
    pub id: usize,
}

pub fn get_article_offset_id(index_file: &str, article_title: &str) -> Option<IndexData> {
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
            let _title = data[2].trim();
            return Some(IndexData { offset, id });
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
#[derive(Debug)]
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

    index
}
