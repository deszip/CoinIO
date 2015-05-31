/*
@TODO:
 + move logic to separate module/lib
 + move URL building to separate struct
 - accept command line arguments
*/

extern crate getopts;
extern crate rustc_serialize;

#[allow(unused_imports)]
use self::rustc_serialize::json;
#[allow(unused_imports)]
use self::rustc_serialize::json::Json;
#[allow(unused_imports)]
use self::getopts::Options;
use std::env;
use coin_api::CoinApi;

mod parse;
mod coin_api;

static COIN_LOGIN: &'static str = "raf.rafiki@gmail.com";
static COIN_PASSWORD: &'static str = "cfd32";

fn main() {

    let args: Vec<String> = env::args().collect();
    println!("args: {:?}", args);

    let mut coin_api = CoinApi::new(COIN_LOGIN, COIN_PASSWORD);

    let categories_count = coin_api.categories_count();
    let expense_count = coin_api.expenses_count();

    println!("Categories: {}", categories_count);
    println!("Expenses: {}", expense_count);

    coin_api.create_expense(12, "Foo");
}
