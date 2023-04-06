use std::{
    fs,
    path::{Path, PathBuf},
};

use mdbook::book::Summary;

fn main() {
    let paths = fs::read_dir("./test").unwrap();
    let summaries: Vec<String> = paths
        .into_iter()
        .filter(|x| x.as_ref().unwrap().file_name().to_str().unwrap() == "SUMMARY.md".to_owned())
        .map(|x| fs::read_to_string(x.unwrap().path()).unwrap())
        .collect();
    let length = summaries.len();

    let mut sum = mdbook::book::parse_summary(&summaries[0]).unwrap();
    println!("{sum:#?}");
    println!("{length}");
    let path = Path::new("./new_path");
    rebase_summary(path, sum);
}

fn rebase_summary(new_base: &Path, mut summary: Summary) -> Summary {
    let new_prefix: Vec<mdbook::book::SummaryItem> = summary
        .numbered_chapters
        .into_iter()
        .map(|x| append_new(x, new_base))
        .collect();
    println!("{new_prefix:#?}");

    todo!()
}

fn append_new(x: mdbook::book::SummaryItem, new_base: &Path) -> mdbook::book::SummaryItem {
    match x {
        mdbook::book::SummaryItem::Link(mut link) => {
            let mut new_path = PathBuf::new();
            new_path.push(new_base);
            new_path.push(link.location.unwrap());
            link.location = Some(new_path);
            mdbook::book::SummaryItem::Link(link)
        }
        mdbook::book::SummaryItem::Separator => mdbook::book::SummaryItem::Separator,
        mdbook::book::SummaryItem::PartTitle(ptitle) => {
            mdbook::book::SummaryItem::PartTitle(ptitle)
        }
    }
}
