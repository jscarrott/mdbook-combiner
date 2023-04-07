use std::{
    fs,
    path::{Path, PathBuf},
};

use mdbook::book::{Summary, SummaryItem};

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
        .map(|x| rebase(x, new_base))
        .collect();
    println!("{new_prefix:#?}");

    todo!()
}

// fn rebase(mut sum_item: SummaryItem, new_base: &Path) -> SummaryItem {
//     match sum_item {
//         mdbook::book::SummaryItem::Link(link) => {
//             let mut new_path = PathBuf::new();
//             new_path.push(new_base);
//             new_path.push(link.location.clone().unwrap());
//             link.location = Some(new_path);
//         }
//         mdbook::book::SummaryItem::Separator => (),
//         mdbook::book::SummaryItem::PartTitle(ptitle) => (),
//     };
//     sum_item
// }

fn rebase(x: mdbook::book::SummaryItem, new_base: &Path) -> mdbook::book::SummaryItem {
    match x {
        mdbook::book::SummaryItem::Link(mut link) => {
            let mut new_path = PathBuf::new();
            new_path.push(new_base);
            new_path.push(link.location.unwrap().push(path));
            link.location = Some(new_path);
            link.nested_items = link
                .nested_items
                .into_iter()
                .map(|x| rebase(x, new_base))
                .collect();
            mdbook::book::SummaryItem::Link(link)
        }
        mdbook::book::SummaryItem::Separator => mdbook::book::SummaryItem::Separator,
        mdbook::book::SummaryItem::PartTitle(ptitle) => {
            mdbook::book::SummaryItem::PartTitle(ptitle)
        }
    }
}
