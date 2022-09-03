use std::{path::Path, fs::File, io::Read};

fn main() {
    let article = "CrCs2O4";
    let result = get_article_offset_id(article);

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

    let mut index_data = String::new();
    index_file.read_to_string(&mut index_data);

    for article_data in index_data.lines()
    {
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