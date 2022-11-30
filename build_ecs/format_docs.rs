use regex::Regex;
use lazy_static::lazy_static;

pub fn format_docs(doc: &str) -> String {
    lazy_static! {
        static ref RE_URL1: Regex = Regex::new(r"(https?://[^\]]+)\[([^\]]+)\]").unwrap();
        static ref RE_URL2: Regex = Regex::new(r"(([^\]])\(| )(https?://[^ )]+)(\.|\)| )").unwrap();
        static ref RE_HIGHLIGHT: Regex = Regex::new(r"\[([a-zA-Z0-9]+)\]([^(])").unwrap();
        static ref RE_NEWLINE: Regex = Regex::new(r"\\n").unwrap();
        //static ref RE_QUOTE: Regex = Regex::new("\\\\\"").unwrap();
    }
    let res = doc.to_string();
    let res = RE_URL1.replace_all(&res, "[$2]($1)").to_string();
    let res = RE_URL2.replace_all(&res, "$1<$3>$4").to_string();
    let res = RE_HIGHLIGHT.replace_all(&res, "\\[$1\\]$2").to_string();
    let res = RE_NEWLINE.replace_all(&res, "\n\n").to_string();
    //let res = RE_QUOTE.replace_all(&res, "\"").to_string();
    //assert_eq!(doc,res);
    res.replace('\n', "\n\n")
}