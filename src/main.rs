use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use walkdir::WalkDir;

/// search in a string for a pattern and return all found pattern occurrences
/// :parameter
///     * `input_string` - string to search in
///     * `pattern` - pattern to search for
/// :return
///     * `cites` - all occurrences of pattern
fn pattern_extraction(input_string: &str, pattern: &Regex) -> Vec<String> {
    let mut cites: Vec<String> = Vec::new();
    for (_, [m]) in pattern
        .captures_iter(input_string)
        .map(|caps| caps.extract())
    {
        cites.push(m.to_string())
    }
    cites
}

/// check if every stated item is also referenced - eg every bibitem is cited at least once
/// :parameter
///     * `stated` - stated items to be cited/ referenced (eg bibitem)
///     * `used` - stated items to be cited/ referenced (eg cite)
///     * `method` - what is searched for (eg cite)
/// :return
///     * `None`
fn check_used(stated: &[String], used: &[String], method: &str) {
    for s in stated.iter() {
        if !used.contains(s) {
            eprintln!("{} [ {} ] used but never referenced", &method, &s);
        }
    }
}

/// check in all tex file in this of deeper directories if all labels are referenced and all bibitems are cited
/// :parameter
///     * `base_path` - base directory path in which tex files are stored
/// :return
///     * `None`
fn usage_check(base_path: &String) {
    let label_re = Regex::new(r"\\label\{([^}]*)\}").expect("Could not create label regex pattern");
    let ref_re = Regex::new(r"\\ref\{([^}]*)\}").expect("Could not create ref regex pattern");

    let cite_re = Regex::new(r"\\cite.?\{([^}]*)\}").expect("Could not create cite regex pattern");
    let bibitem_re =
        Regex::new(r"\\bibitem\{([^}]*)\}").expect("Could not create bibitem regex pattern");

    let mut total_bibitems: Vec<String> = Vec::new();
    let mut total_cites: Vec<String> = Vec::new();
    let mut total_labels: Vec<String> = Vec::new();
    let mut total_refs: Vec<String> = Vec::new();
    for entry in WalkDir::new(base_path).into_iter().filter_map(|e| e.ok()) {
        if let Some(entry_file_name) = entry.file_name().to_str() {
            if entry_file_name.ends_with(".tex") {
                let entry_path = entry.path().to_str().unwrap_or_else(|| {
                    panic!("Can not generate str from filepath [ {} ]", entry_file_name)
                });
                // add format to tell which file and path cant be opened
                let file = File::open(entry_path)
                    .unwrap_or_else(|_| panic!("File [ {} ] not found", &entry_path));
                let buffer = BufReader::new(file).lines();
                for (ci, i) in buffer.enumerate() {
                    let line = i.unwrap_or_else(|_| {
                        panic!(
                            "Can not read line [ {} ] of file [ {} ]",
                            ci, entry_file_name
                        )
                    });
                    total_bibitems.append(&mut pattern_extraction(&line, &bibitem_re));
                    total_cites.append(&mut pattern_extraction(&line, &cite_re));
                    total_labels.append(&mut pattern_extraction(&line, &label_re));
                    total_refs.append(&mut pattern_extraction(&line, &ref_re));
                }
            }
        }
    }
    check_used(&total_bibitems, &total_cites, "bibitem");
    check_used(&total_labels, &total_refs, "label");
}
/*
-- check the order of bibitems
*/

fn main() {
    usage_check(&"./".to_string());
}
