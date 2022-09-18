use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub struct IndexData {
    pub offset: usize,
    pub id: usize,
}

pub fn get_article_offset_id(article_title: &str) -> Option<IndexData> {
    let path = Path::new(
        r"C:\Users\Vineet Palepu\Downloads\enwiki-20220101-pages-articles-multistream\enwiki-20220101-pages-articles-multistream-index.txt",
    );

    println!("Opening index located at {}", path.display());

    let index_file = match File::open(path) {
        Ok(file) => file,
        Err(e) => panic!("couldn't open {}: {}", path.display(), e),
    };
    let index_file = BufReader::new(index_file);

    for article_data in index_file.lines() {
        let article_data = article_data.unwrap();
        if article_data.contains(article_title)
        {
            let data: Vec<&str> = article_data.split(":").collect();

            let offset: usize = data[0].parse().expect("couldn't parse article byte offset");
            let id: usize = data[1].parse().expect("couldn't parse article ID");
            let _title = data[2].trim();
            return Some(IndexData { offset, id });
        }
    }

    None
}
