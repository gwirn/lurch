use regex::Regex;
use std::env;

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

/*
 figure and tables
\label
\ref

citations
\bibitem
\cite

-- check if everything is referenced
    -- check for labels but also for bibitems
-- check the order of bibitmes

*/

fn main() {
    let input_string =
        "thist is a test \\cite{bibitem9} \\label{Table1} and \\ref{Table1} here the string continues \\citep{bibitem1} \\bibitem{bibitem1} sdf*~d a(sadf \\bibitem{bibitem2} asdf asdf~*(f)".to_string();

    let label_re = Regex::new(r"\\label\{([^}]*)\}").expect("Could not create regex pattern");
    let ref_re = Regex::new(r"\\ref\{([^}]*)\}").expect("Could not create regex pattern");

    let cite_re = Regex::new(r"\\cite.?\{([^}]*)\}").expect("Could not create regex pattern");
    let bibitem_re = Regex::new(r"\\bibitem\{([^}]*)\}").expect("Could not create regex pattern");

    let cites = pattern_extraction(&input_string, &cite_re);
    let bibitems = pattern_extraction(&input_string, &bibitem_re);
    let labels = pattern_extraction(&input_string, &label_re);
    let refs = pattern_extraction(&input_string, &ref_re);

    println!("{:?}", &cites);
    println!("{:?}", &bibitems);
    println!("{:?}", &labels);
    println!("{:?}", &refs);
    check_used(&bibitems, &cites, "bibitem");
    check_used(&labels, &refs, "label");
    let args: Vec<_> = env::args().collect();
    println!(">>> {:?}{:?}", args[1], args[2])
}
