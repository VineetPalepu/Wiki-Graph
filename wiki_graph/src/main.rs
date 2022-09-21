use regex::Regex;
use std::env;
use std::fs::read_dir;
use std::path::Path;
use std::path::PathBuf;
use wiki_graph::*;

// TODO: Make a hierarchical wiki browser, i.e. when you click a link it keeps track of which article you came from and when you finish an article it goes back to the parent article

// usage: wiki_graph <article_1> <article_2> ...
fn main()
{
    let articles: Vec<String> = env::args().skip(1).collect();

    // TODO: mdocment
    let data_folder = Path::new("data");
    let (index_file, data_file) = get_index_data_file_names(data_folder);
    let cache_file = Path::new("data\\index.dat");

    let wiki_db = WikiDB::new(
        Path::new(&index_file),
        Path::new(&data_file),
        Path::new(&cache_file),
    );

    for article in articles
    {
        println!("{article}: ");
        for neighbor in wiki_db.get_article_neighbors(&article).unwrap()
        {
            println!("\t{neighbor}");
        }
    }

    //println!("{}", wiki_db.get_article_text(article).unwrap());
    //println!("{:?}", wiki_db.get_article_neighbors(article).unwrap());
}

fn get_dir_files(dir: &Path) -> Vec<PathBuf>
{
    let mut files = Vec::new();
    for file in read_dir(dir).expect(&format!("error opening dir: {}", dir.display()))
    {
        if let Ok(file) = file
        {
            files.push(file.path());
        }
    }

    files
}

fn get_index_date_str(file_name: &str) -> Option<String>
{
    let regex = Regex::new(r"^enwiki-(\d{8})-pages-articles-multistream-index.txt$").unwrap();
    if let Some(capture) = regex.captures(file_name)
    {
        let date_str = capture[1].to_string();

        return Some(date_str);
    }

    None
}

fn get_index_data_file_names(data_folder: &Path) -> (PathBuf, PathBuf)
{
    let mut index_file = None;
    let mut data_file = None;

    let data_files = &get_dir_files(data_folder);

    for potential_index_file in data_files
    {
        //check if the file_name matches the index file name pattern
        if let Some(date_str) =
            get_index_date_str(potential_index_file.file_name().unwrap().to_str().unwrap())
        {
            index_file = Some(potential_index_file.clone());
            println!(
                "found index file: {}",
                potential_index_file.file_name().unwrap().to_str().unwrap()
            );

            let data_regex = Regex::new(&format!(
                r"^enwiki-{date_str}-pages-articles-multistream.xml.bz2$"
            ))
            .unwrap();
            for potential_data_file in data_files
            {
                if data_regex.is_match(potential_data_file.file_name().unwrap().to_str().unwrap())
                {
                    data_file = Some(potential_data_file.clone());
                    println!(
                        "found data file: {}",
                        potential_data_file.file_name().unwrap().to_str().unwrap()
                    );
                    break;
                }
            }
        }
    }

    if index_file == None
    {
        panic!(
            "couldn't find a index file: {}\\enwiki-########-pages-articles-multistream-index.txt",
            data_folder.display()
        );
    }
    if data_file == None
    {
        let date_str =
            get_index_date_str(index_file.unwrap().file_name().unwrap().to_str().unwrap()).unwrap();
        let data_file_name = format!("enwiki-{date_str}-pages-articles-multistream.xml.bz2");
        panic!(
            "couldn't find a data file: {}\\{data_file_name}",
            data_folder.display()
        );
    }

    (index_file.unwrap(), data_file.unwrap())
}
