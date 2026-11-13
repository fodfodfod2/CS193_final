use mlua::Table;

use super::*;

pub fn indent(s: &str, n: usize) -> String {
    s.replace("\n", &(String::from("\n") + &"\t".repeat(n)))
}

pub fn vec_to_str<T: fmt::Display>(title: &str, v: &Vec<T>) -> String {
    if v.is_empty() {
        return String::from("");
    }

    let mut res = String::from("{title}:\n");
    for i in v {
        res.push_str("\n\t{i}");
    }
    res
}

pub trait Parsable {
    fn parse(t: Table) -> Self;
}
