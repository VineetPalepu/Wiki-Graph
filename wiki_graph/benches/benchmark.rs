use std::{path::Path, time::Instant};
use wiki_graph::*;

macro_rules! benchmark {
    ($code:expr) => {
        let t = Instant::now();
        $code;
        println!(
            "{:?} seconds elapsed for: {}",
            t.elapsed(),
            stringify!($code)
        );
    };
}

fn main()
{
    bench_count_lines();
    bench_build_index();
    bench_get_article_offset_id_from_index();
}

fn bench_count_lines()
{
    let dir = r"data";
    let index_file = "enwiki-20220101-pages-articles-multistream-index.txt";
    let index_file = format!("{dir}\\{index_file}");

    benchmark!(count_lines(&Path::new(&index_file)));
}

fn bench_build_index()
{
    let dir = r"data";
    let index_file = "enwiki-20220101-pages-articles-multistream-index.txt";
    let index_file = format!("{dir}\\{index_file}");

    benchmark!(build_index(&Path::new(&index_file)));
}

fn bench_get_article_offset_id_from_index()
{
    let dir = r"data";
    let index_file = "enwiki-20220101-pages-articles-multistream-index.txt";
    let index_file = format!("{dir}\\{index_file}");

    let articles = [
        "ArtificalLanguages",
        "Wireless application service provider",
        "TheFilmSchool",
        "Vahid Mirzadeh",
        "William Loren Batt",
        "Buddhist Tripitaka",
        "Blight remediation",
        "Town of Brookhaven",
        "Floppy disk interface",
        "OpenHistoricalMap",
    ];

    let index = build_index(&Path::new(&index_file));

    for article in articles
    {
        benchmark!(get_article_offset_id(&index, article));
    }
}
