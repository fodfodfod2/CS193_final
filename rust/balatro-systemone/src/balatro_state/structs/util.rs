use mlua::Table;

use crate::ApiChoice;
use crate::game_interface::Action;

use super::*;

/*
 * This file just contains a few util functions multiple files use
 * that don't really fit in anywhere else
 */

/*
 * This function will indent a string by a certain number of tabs
 */
pub fn indent(s: &str, n: usize) -> String {
    s.replace("\n", &(String::from("\n") + &"\t".repeat(n)))
}

/*
 * This function will give a printout of a vector
 * title: the title of the printout will
 * returns a String of the Result
 */
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
    /*
     * This function will Parse a Self object from a table
     * and return Ok(the result) or Err(a str representing the err 
     * if one occured
     */
    fn parse(t: Table) -> Result<Self, &'static str>;
}

pub trait Buyable {
    /*
     * Get the Display name for the item
     */
    fn get_item_name(&self) -> &str;

    /*
     * This function will return the price of the object
     */
    fn get_price(&self) -> i16;

    /*
     * This function will return the action of buying the object with
     * a certain idx.
     *
     * this action will be performed to buy the card
     */
    fn get_action(&self, i: i16) -> Action;
}

/*
 * This function will return an ApiChoice representing buying
 * the item from the shop
 */
pub fn get_choice<T: Buyable + fmt::Display>(i: &T, idx: i16) -> ApiChoice {
    let n = i.get_item_name();
    let p = i.get_price();
    ApiChoice::new(
        &format!("Buy {n}"),
        &format!("Buy {n} --{i}-- for {p}$"),
        i.get_action(idx),
    )
}

/*
 * This function will parse an array of a certain type from a table
 * for example {[1]={<CARD>},[2]={<CARD>}} will turn into a vec containing
 * those two cards
 */
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
