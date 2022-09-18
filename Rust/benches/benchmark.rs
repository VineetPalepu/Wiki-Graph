use std::time::Instant;
use wiki_graph::*;

macro_rules! benchmark {
    ($code:expr) => {
        let t = Instant::now();
        $code;
        println!("{:?} seconds elapsed", t.elapsed());
    };
}

fn main() {
    bench_get_article_offset_id();
}

fn bench_get_article_offset_id() {
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
        benchmark!(get_article_offset_id(article));
    }
}
