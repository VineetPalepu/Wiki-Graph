// TODO: Make a hierarchical wiki browser, i.e. when you click a link it keeps track of which article you came from and when you finish an article it goes back to the parent article

fn main() {
    let dir = r"C:\Users\Vineet Palepu\Downloads\enwiki-20220101-pages-articles-multistream\";
    let index_file = "enwiki-20220101-pages-articles-multistream-index.txt";
    let index_file = format!("{}{}", dir, index_file);

    println!("{} articles in index", wiki_graph::count_lines(&index_file));

    let article = "OpenHistoricalMap";
    let result = wiki_graph::get_article_offset_id(&index_file, article);
    match result {
        Some(data) => {
            println!(
                "article {article} found with id {} and offset {}",
                data.id, data.offset
            );
        }
        None => println!("article {article} not found"),
    }
}
