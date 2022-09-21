use std::{
    convert::TryInto,
    fs::{read, write, File},
    io::{BufRead, BufReader, Read, Seek, SeekFrom},
    path::{Path, PathBuf},
    time::Instant,
};

use bincode2::{deserialize, serialize};
use bzip2::read::BzDecoder;
use regex::Regex;
use serde::{Deserialize, Serialize};

impl WikiDB
{
    pub fn new(index_file: &Path, data_file: &Path, cache_file: &Path) -> WikiDB
    {
        let index;
        //let cache_file = format!("{data_folder}\\index.dat");
        if let Ok(_cache_file) = File::open(cache_file) {
            println!("found cached index at: {}", cache_file.display());
            let t = Instant::now();
            index = load_index(cache_file);
            println!("loading cached index took {:?} seconds", t.elapsed());
        } else {
            println!("no cached index found");
            println!("building index from index file at: {}", index_file.display());
            let t = Instant::now();
            index = build_index(index_file);
            println!("building index took {:?} seconds", t.elapsed());
            println!("saving index");
            let t = Instant::now();
            save_index(&index, cache_file);
            println!("saving built index took {:?} seconds", t.elapsed());
        }

        WikiDB { index, data: PathBuf::from(data_file) }
    }

    //TODO: maybe implement get_article_xml(article_title)

    pub fn get_article_xml(&self, article_title: &str) -> Option<String>
    {
        // get article location in data file
        let entry = get_article_offset_id(&self.index, article_title)?;

        // get article xml from data file
        let xml = get_article(&self.data, article_title, entry.offset, entry.id);

        Some(xml)
    }
    
    //TODO: implement get_article_text(article_title)
    pub fn get_article_text(&self, article_title: &str) -> Option<String>
    {
        let xml = self.get_article_xml(article_title)?;

        Some(get_wikitext(&xml))
    }

    //TODO: implement get_neighbors(article_title)
    pub fn get_article_neighbors(&self, article_title: &str) -> Result<Vec<String>, String>
    {
        let article_text = self.get_article_text(article_title);

        let article_text = match article_text
        {
            Some(s) => s,
            None => return Err(format!("article '{article_title}' not found")),
        };

        let re = Regex::new(r"\[\[([^\]]*)\]\]").unwrap();
        let mut neighbors: Vec<String> = vec![];

        for neighbor in re.captures_iter(&article_text) {
            let wikilink = neighbor[1].to_string();
            let title = match wikilink.find("|") {
                Some(i) => wikilink[0..i].to_string(),
                None => wikilink,
            };

            let title = title
                .chars()
                .enumerate()
                .map(|(i, c)| {
                    let s;
                    if i == 0 {
                        s = c.to_uppercase().to_string();
                    } else {
                        s = c.to_string();
                    }

                    s
                })
                .collect();

            neighbors.push(title);
        }

        Ok(neighbors)
    }
}

pub struct WikiDB
{
    pub index: Vec<IndexEntry>,
    pub data: PathBuf,
}

pub fn count_lines(index_file: &Path) -> usize {
    let index_file = File::open(index_file).unwrap();
    let index_file = BufReader::new(index_file);
    let lines = index_file.lines();
    let mut count = 0;
    for _line in lines {
        count += 1;
    }
    return count;
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexEntry {
    pub title: String,
    pub id: usize,
    pub offset: usize,
}

pub fn build_index(index_file: &Path) -> Vec<IndexEntry> {
    let num_articles = count_lines(&index_file);

    let mut index: Vec<IndexEntry> = Vec::with_capacity(num_articles);

    let index_file = File::open(index_file).unwrap();
    let index_file = BufReader::new(index_file);

    for article_data in index_file.lines() {
        let article_data = article_data.unwrap();
        let data: Vec<&str> = article_data.split(':').collect();

        let offset: usize = data[0].parse().expect("couldn't parse article byte offset");
        let id: usize = data[1].parse().expect("couldn't parse article ID");
        let title = data[2].trim().to_string();

        index.push(IndexEntry { title, id, offset });
    }
    let t = Instant::now();
    index.sort_unstable_by(|a, b| a.title.cmp(&b.title));
    println!("{:?} seconds elapsed to sort index", t.elapsed());

    index
}

pub fn get_article_offset_id(
    index: &Vec<IndexEntry>,
    article_title: &str,
) -> Option<IndexEntry> {
    let result = index.binary_search_by(|a| a.title.cmp(&article_title.to_string()));

    match result {
        Ok(i) => Some(index[i].clone()),
        Err(_) => {
            println!("article \'{}\' not found", article_title);
            None
        }
    }
}

pub fn get_article(data_file: &Path, title: &str, offset: usize, _id: usize) -> String {
    let data_file =
        File::open(data_file).expect(&format!("couldn't open file {}", data_file.display()));
    let mut data_file = BufReader::new(data_file);
    let offset: u64 = offset.try_into().expect("offset too large???");
    data_file
        .seek(SeekFrom::Start(offset))
        .expect(stringify!("couldn't seek to offset {}", offset));
    let mut decompressor = BzDecoder::new(data_file);
    let mut contents = String::new();
    decompressor
        .read_to_string(&mut contents)
        .expect("failed to decompress");

    let title_index = contents.find(&format!("<title>{}</title>", title));
    let article = match title_index {
        Some(title_index) => {
            let start_index = contents[..title_index]
                .rfind("<page>")
                .expect("couldn't find beginning of page");
            let end_index = contents[title_index..]
                .find("</page>")
                .map(|i| title_index + i + 7)
                .expect("couldn't find end of page");

            &contents[start_index..end_index]
        }
        None => panic!("article not found in block, implment multiple block reading"),
    };

    article.to_string()
}

pub fn get_wikitext(article_xml: &str) -> String {
    let start_index = article_xml
        .find("<text")
        .expect("couldn't find text XML tag");
    let start_index = article_xml[start_index..]
        .find(">")
        .expect("couldn't find text XML tag")
        + start_index
        + 1;
    let end_index = article_xml
        .find("</text>")
        .expect("couldn't find text XML closing tag");
    let wikitext = article_xml[start_index..end_index].to_string();

    wikitext
}

pub fn save_index(index: &Vec<IndexEntry>, index_file: &Path) {
    let binary_data = serialize(index).unwrap();
    write(index_file, binary_data).unwrap();
}

pub fn load_index(index_file: &Path) -> Vec<IndexEntry> {
    let binary_data = read(index_file).unwrap();
    let index: Vec<IndexEntry> = deserialize(&binary_data).unwrap();

    index
}
