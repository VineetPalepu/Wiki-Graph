use std::{path::Path, fs::{File, self, read_dir}, io::{Read, BufReader, BufRead}, time::Instant};

// TODO: Make a hierarchical wiki browser, i.e. when you click a link it keeps track of which article you came from and when you finish an article it goes back to the parent article

fn main() {    
    let article = "Category:Japan rail succession modules";
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
    let path = Path::new(r"C:\Users\Vineet Palepu\Downloads\enwiki-20220101-pages-articles-multistream\enwiki-20220101-pages-articles-multistream-index.txt");

    println!("Opening index located at {}", path.display());

    let index_file = match File::open(path)
    {
        Ok(file) => file,
        Err(e) => panic!("couldn't open {}: {}", path.display(), e),
    };
    let index_file = BufReader::new(index_file);

    for article_data in index_file.lines()
    {
        
        let article_data = article_data.unwrap();
        if article_data.contains(article_title)
        {
            let data: Vec<&str> = article_data.split(":").collect();

            let offset: usize = data[0].parse().expect("couldn't parse article byte offset");
            let id: usize = data[1].parse().expect("couldn't parse article ID");
            let title = data[2].trim();

            return Some(IndexData{offset, id});

        }
    }

    None

}