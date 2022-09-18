use std::time::Instant;
use wiki_graph::*;

macro_rules! benchmark {
    ($code:expr) => {
        let t = Instant::now();
        $code;
        println!("{:?} seconds elapsed for: {}", t.elapsed(), stringify!($code));
    };
}

fn main() {
    bench_build_index();
    bench_count_lines();
    bench_get_article_offset_id();
}

fn bench_get_article_offset_id() {
    let dir = r"C:\Users\Vineet Palepu\Downloads\enwiki-20220101-pages-articles-multistream\";
    let index_file = "enwiki-20220101-pages-articles-multistream-index.txt";
    let index_file = format!("{}{}", dir, index_file);

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

    for article in articles {
        benchmark!(get_article_offset_id(&index_file, article));
    }
}

fn bench_count_lines()
{
    let dir = r"C:\Users\Vineet Palepu\Downloads\enwiki-20220101-pages-articles-multistream\";
    let index_file = "enwiki-20220101-pages-articles-multistream-index.txt";
    let index_file = format!("{}{}", dir, index_file);

    benchmark!(count_lines(&index_file));
}

fn bench_build_index()
{
    let dir = r"C:\Users\Vineet Palepu\Downloads\enwiki-20220101-pages-articles-multistream\";
    let index_file = "enwiki-20220101-pages-articles-multistream-index.txt";
    let index_file = format!("{}{}", dir, index_file);

    benchmark!(build_index(&index_file));
}
