use std::{
    fs,
    path::{Path, PathBuf},
};

use clap::Parser;
use mdbook::book::{Link, Summary, SummaryItem};
use walkdir::{DirEntry, WalkDir};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the meta directory containing all the summary files
    #[arg(short, long)]
    meta_directory: PathBuf,
    /// Paths to "just a bunch of markdown"
    #[arg(short, long)]
    jabom: Vec<PathBuf>,
}

fn main() {
    let args = Args::parse();

    let paths = WalkDir::new(&args.meta_directory);
    let mut summaries: Vec<(String, PathBuf)> = paths
        .into_iter()
        .filter(|x| x.as_ref().unwrap().file_name().to_str().unwrap() == "SUMMARY.md")
        .map(|x| {
            (
                fs::read_to_string(x.as_ref().unwrap().path()).unwrap(),
                x.unwrap().path().parent().unwrap().to_path_buf(),
            )
        })
        .collect();
    let mut jabom: Vec<(String, Summary)> = args
        .jabom
        .into_iter()
        .map(|x| generate_summary_for_jabom(x))
        .collect();

    let length = summaries.len();

    let mut rebased_summaries: Vec<(String, Summary)> = summaries
        .iter()
        .map(|x| {
            let absolute_path = &x.1.canonicalize().unwrap();
            println!("{:?}", x.1);
            let sum = mdbook::book::parse_summary(&x.0).unwrap();
            (
                x.1.file_name().unwrap().to_string_lossy().to_string(),
                rebase_summary(absolute_path, sum),
            )
        })
        .collect();
    rebased_summaries.append(&mut jabom);

    let final_summary = rebased_summaries
        .into_iter()
        .fold(Summary::default(), |mut acc, mut x| {
            let ptitle = SummaryItem::PartTitle(x.0);
            // let sub_summary = SummaryItem::Link(Link {
            //     name: x.0,
            //     location: None,
            //     number: None,
            //     nested_items: x.1.numbered_chapters,
            // });
            // acc.prefix_chapters.append(&mut x.prefix_chapters);
            // acc.numbered_chapters.append(&mut x.numbered_chapters);
            // acc.suffix_chapters.append(&mut x.suffix_chapters);
            acc.numbered_chapters.push(ptitle);
            acc.numbered_chapters.append(&mut x.1.numbered_chapters);
            acc
        });
    let final_summary = output_summary(final_summary);
    println!("{final_summary:#?}");
    std::fs::write("SUMMARY.md", final_summary);
    // println!("{length}");
    // let absolute_path = &summaries[0].1.canonicalize().unwrap();
    // rebase_summary(&absolute_path, sum);
}

fn rebase_summary(new_base: &Path, summary: Summary) -> Summary {
    let new_numbered: Vec<mdbook::book::SummaryItem> = summary
        .numbered_chapters
        .into_iter()
        .map(|x| rebase(x, new_base))
        .collect();

    let new_prefix: Vec<mdbook::book::SummaryItem> = summary
        .prefix_chapters
        .into_iter()
        .map(|x| rebase(x, new_base))
        .collect();

    let new_suffix: Vec<mdbook::book::SummaryItem> = summary
        .suffix_chapters
        .into_iter()
        .map(|x| rebase(x, new_base))
        .collect();
    Summary {
        prefix_chapters: new_prefix,
        numbered_chapters: new_numbered,
        suffix_chapters: new_suffix,
        title: summary.title,
    }
    // todo!()
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
            new_path.push(link.location.unwrap());
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

fn output_summary_item(x: &SummaryItem, depth: u16) -> String {
    let mut indent = String::new();
    for _ in 0..depth {
        indent += "\t";
    }
    match x {
        SummaryItem::Link(link) => {
            let loc = if let Some(path) = &link.location {
                path.display().to_string()
            } else {
                String::new()
            };
            let mut s = format!("{}- [{}]({})\n", indent, link.name, loc);
            link.nested_items.iter().fold(s, |mut acc, x| {
                acc += &output_summary_item(x, depth + 1);
                acc
            })
        }
        SummaryItem::Separator => format!(""),
        SummaryItem::PartTitle(ptitle) => format!("{}# {ptitle}\n", indent),
    }
}

fn output_summary(x: Summary) -> String {
    let mut output = String::new();
    for x in x.numbered_chapters {
        output += &output_summary_item(&x, 0);
    }
    output
}

fn is_markdown(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.ends_with(".md"))
        .unwrap_or(false)
}

fn generate_summary_for_jabom(dir: PathBuf) -> (String, Summary) {
    let name = dir.file_name().unwrap().to_string_lossy().to_string();
    let mut sum = Summary {
        title: Some(name.clone()),
        suffix_chapters: vec![],
        prefix_chapters: vec![],
        numbered_chapters: vec![],
    };
    for entry in WalkDir::new(dir)
        .min_depth(1)
        .into_iter()
        .filter_entry(is_markdown)
    {
        let entry = entry.unwrap();
        println!("{:?}", entry.path());
        sum.numbered_chapters.push(SummaryItem::Link(Link {
            name: entry
                .path()
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .to_string(),
            location: Some(entry.path().to_path_buf().canonicalize().unwrap()),
            number: None,
            nested_items: vec![],
        }));
    }
    (name, sum)
}
