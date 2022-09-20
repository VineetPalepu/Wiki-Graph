use std::env;
use std::fs::read_dir;
// TODO: Make a hierarchical wiki browser, i.e. when you click a link it keeps track of which article you came from and when you finish an article it goes back to the parent article
use regex::Regex;
use std::fs::write;
use std::fs::File;
use std::path::Path;
use std::time::Instant;
use wiki_graph::*;

fn get_dir_files(dir: &str) -> Vec<String> {
    let mut files = Vec::new();
    for file in read_dir(dir).expect(&format!("error opening dir: {dir}")) {
        if let Ok(file) = file {
            let file_name = file.file_name();
            let file_name = file_name.to_str().unwrap();
            files.push(file_name.to_string());
        }
    }

    files
}

fn get_index_date_str(file_name: &str) -> Option<String> {
    let regex = Regex::new(r"^enwiki-(\d{8})-pages-articles-multistream-index.txt$").unwrap();
    if let Some(capture) = regex.captures(file_name) {
        let date_str = capture[1].to_string();

        return Some(date_str);
    }

    None
}

fn get_index_data_file_names(data_folder: &str) -> (String, String) {
    let mut index_file = None;
    let mut data_file = None;

    let data_files = &get_dir_files(data_folder);

    for potential_index_file in data_files {
        //check if the file_name matches the index file name pattern
        if let Some(date_str) = get_index_date_str(&potential_index_file) {
            index_file = Some(potential_index_file.clone());
            println!("found index file: {}", potential_index_file);

            let data_regex = Regex::new(&format!(
                r"^enwiki-{date_str}-pages-articles-multistream.xml.bz2$"
            ))
            .unwrap();
            for potential_data_file in data_files {
                if data_regex.is_match(&potential_data_file) {
                    data_file = Some(potential_data_file.clone());
                    println!("found data file: {}", potential_data_file);
                    break;
                }
            }
        }
    }

    if index_file == None {
        panic!("couldn't find a index file: {data_folder}\\enwiki-########-pages-articles-multistream-index.txt");
    }
    if data_file == None {
        let date_str = get_index_date_str(&index_file.unwrap()).unwrap();
        let data_file_name = format!("enwiki-{date_str}-pages-articles-multistream.xml.bz2");
        panic!("couldn't find a data file: {data_folder}\\{data_file_name}");
    }

    (index_file.unwrap(), data_file.unwrap())
}

// TODO:
// usage: wiki_graph "<article_1>" "<article_2>" ...
fn main() {
    let _args: Vec<String> = env::args().skip(1).collect();

    let data_folder = r"data";

    let cache_file = format!("{data_folder}\\index.dat");
    let index;
    let (index_file, data_file) = get_index_data_file_names(data_folder);
    if let Ok(_cache_file) = File::open(&cache_file) {
        println!("found cached index at: {}", cache_file);
        let t = Instant::now();
        index = load_index(&cache_file);
        println!("loading cached index took {:?} seconds", t.elapsed());
    } else {
        println!("no cached index found");
        println!("building index from index file at: {data_folder}\\{index_file}");
        let t = Instant::now();
        index = build_index(&format!("{data_folder}\\{index_file}"));
        println!("building index took {:?} seconds", t.elapsed());
        println!("saving index");
        let t = Instant::now();
        save_index(&index, &cache_file);
        println!("saving built index took {:?} seconds", t.elapsed());
    }

    let article = "Academic conference";
    let result = get_article_offset_id(&index, article);
    match result {
        Some(data) => {
            println!(
                "article {article} found with id {} and offset {}",
                data.id, data.offset
            );
            let contents = get_article(
                Path::new(&format!("{data_folder}\\{data_file}")),
                article,
                data.offset,
                data.id,
            );
            write(r"data\out.xml", &contents).unwrap();
            write(r"data\out.txt", get_wikitext(&contents)).unwrap();

            let neighbors = get_article_neighbors(
                &index,
                Path::new(&format!("{data_folder}\\{data_file}")),
                article,
            );
            for n in neighbors {
                println!("{}", n);
            }
        }
        None => println!("article {article} not found"),
    }
}
