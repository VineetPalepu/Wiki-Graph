// TODO: Make a hierarchical wiki browser, i.e. when you click a link it keeps track of which article you came from and when you finish an article it goes back to the parent article

use std::path::Path;

use wiki_graph::get_article;

fn main() {
    let dir = r"C:\Users\Vineet Palepu\Downloads\enwiki-20220101-pages-articles-multistream\";
    let index_file = "enwiki-20220101-pages-articles-multistream-index.txt";
    let data_file = "enwiki-20220101-pages-articles-multistream.xml.bz2";
    let index_file = format!("{}{}", dir, index_file);

    let index = wiki_graph::build_index(&index_file);

    let head = &index[0..10];
    println!("{:?}", head);
    
    let article = "OpenHistoricalMap";
    //let result = wiki_graph::get_article_offset_id(&index_file, article);
    let result = wiki_graph::get_article_offset_id_from_index(&index, article);
    match result {
        Some(data) => {
            println!(
                "article {article} found with id {} and offset {}",
                data.id, data.offset
            );
            let contents = get_article(Path::new(data_file), data.offset, data.id);
            println!("{}", contents);
        }
        None => println!("article {article} not found"),
    }

}
