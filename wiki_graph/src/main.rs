// TODO: Make a hierarchical wiki browser, i.e. when you click a link it keeps track of which article you came from and when you finish an article it goes back to the parent article
use std::fs::write;
use std::fs::File;
use std::path::Path;
use std::time::Instant;
use wiki_graph::*;

fn main() {
    let dir = r"C:\Users\Vineet Palepu\Downloads\enwiki-20220101-pages-articles-multistream\";
    let index_file = "enwiki-20220101-pages-articles-multistream-index.txt";
    let data_file = "enwiki-20220101-pages-articles-multistream.xml.bz2";
    let index_file = format!("{}{}", dir, index_file);
    let data_file = format!("{}{}", dir, data_file);
    let cache_file = r"data\index.dat";

    let file = File::open(cache_file);

    let index = match file {
        // if cache file exists
        Ok(_file) => {
            println!("found cached index at: {}", cache_file);
            let t = Instant::now();
            let index = load_index(cache_file);
            println!("loading cached index took {:?} seconds", t.elapsed());

            index
        }
        // otherwise, build index and save it
        Err(_) => {
            println!("no cached index found");
            println!("building index from index file at: {}", index_file);
            let t = Instant::now();
            let index = build_index(&index_file);
            println!("building index took {:?} seconds", t.elapsed());
            println!("saving index");
            let t = Instant::now();
            save_index(&index, cache_file);
            println!("saving built index took {:?} seconds", t.elapsed());

            index
        }
    };

    let article = "Academic conference";
    let result = get_article_offset_id_from_index(&index, article);
    match result {
        Some(data) => {
            println!(
                "article {article} found with id {} and offset {}",
                data.id, data.offset
            );
            let contents = get_article(Path::new(&data_file), article, data.offset, data.id);
            write(r"data\out.xml", &contents).unwrap();
            write(r"data\out.txt", get_wikitext(&contents)).unwrap();

            let neighbors = get_article_neighbors(&index, Path::new(&data_file), article);
            for n in neighbors
            {
                println!("{}", n);
            }
            
        }
        None => println!("article {article} not found"),
    }
}
