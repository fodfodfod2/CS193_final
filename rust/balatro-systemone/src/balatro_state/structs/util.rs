use mlua::Table;

use crate::ApiChoice;

use super::*;

pub fn indent(s: &str, n: usize) -> String {
    s.replace("\n", &(String::from("\n") + &"\t".repeat(n)))
}

pub fn vec_to_str<T: fmt::Display>(title: &str, v: &Vec<T>) -> String {
    if v.is_empty() {
        return String::from("");
    }

    let mut res = format!("{}:\n", title);
    for i in v {
        res += &format!("\n\t{}", i);
    }
    res
}

pub trait Parsable: Sized {
    fn parse(t: Table) -> Result<Self, &'static str>;
}

pub trait Buyable {
    fn get_item_name(&self) -> &str;
    fn get_price(&self) -> i16;
}

pub fn get_choice<T: Buyable + fmt::Display>(i: &T) -> ApiChoice {
    let n = i.get_item_name();
    let p = i.get_price();
    ApiChoice::new(&format!("Buy {n}"), &format!("Buy {n} --{i}-- for {p}$"))
}

pub fn parse_type_vec<T: Parsable>(table: Table) -> Vec<T> {
    let mut counter = 1;
    let mut res: Vec<T> = Vec::new();
    loop {
        let cur_card: Table = match table.get(counter) {
            Ok(v) => v,
            Err(_) => {
                counter -= 1;
                return res;
            }
        };

        res.push(T::parse(cur_card).unwrap());
        counter += 1;
    }
    res
}
