// TODO: Make a hierarchical wiki browser, i.e. when you click a link it keeps track of which article you came from and when you finish an article it goes back to the parent article

fn main() {
    let article = "OpenHistoricalMap";
    let result = wiki_graph::get_article_offset_id(article);
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