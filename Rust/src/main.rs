use std::{path::Path, fs::File, io::{Read, BufReader, BufRead}, time::Instant};

fn main() {
    let article = "CrCs2O4";
    let t1 = Instant::now();
    let result = get_article_offset_id(article);
    println!("{:?} seconds to search index", t1.elapsed());

    match result
    {
        Some(data) => 
        {
            println!("article {article} found with id {} and offset {}", data.id, data.offset);
        },
        None => println!("article {article} not found"),
    }
}

struct IndexData
{
    offset: usize,
    id: usize,
}

fn get_article_offset_id(article_title: &str) -> Option<IndexData>
{
    let path = Path::new("E:\\Data\\enwiki-20211020-pages-articles-multistream-index.txt");
    let mut index_file = match File::open(path)
    {
        Ok(file) => file,
        Err(e) => panic!("couldn't open {}: {}", path.display(), e),
    };
    let index_file = BufReader::new(index_file);

    for article_data in index_file.lines()
    {
        let article_data = article_data.unwrap();
        let data: Vec<&str> = article_data.split(":").collect();

        let offset: usize = data[0].parse().expect("couldn't parse article byte offset");
        let id: usize = data[1].parse().expect("couldn't parse article ID");
        let title = data[2].trim();

        if title == article_title
        {
            return Some(IndexData{offset, id});
        }
    }

    None

}