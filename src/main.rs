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

    //let unwrapped_args: Vec<&str> = args.iter().map(|& ref x| println!("arg: {:?}", x));

    enum ParsingState {
        empty,
        expect_login,
        expect_password
    }

    let mut state: ParsingState = ParsingState::empty;
    let mut login: &str = "";
    let mut password: &str = "";

    for arg in args.iter() {
        match arg.as_ref() {
            "-l" => {
                state = ParsingState::expect_login;
                println!("state: login");
            },
            "-p" => {
                state = ParsingState::expect_password;
                println!("state: password");
            }
            value @ _ => {
                match state {
                    ParsingState::expect_login => {
                        state = ParsingState::empty;
                        login = value;
                    }
                    ParsingState::expect_password => {
                        state = ParsingState::empty;
                        password = value;
                    }
                    ParsingState::empty => {
                        println!("Skipping: {}", value)
                    }
                }
            }
        }
    }

    println!("Login: {}, password: {}", login, password);

    //let tail: Vec<&str> = args.tail().iter().map(|x| &x).collect();
    //println!("tail: {:?}", args);

/*
    match tail {
        [ref executable_path] => {
            panic!("Called with no arguments. Nothing to do.");
        }

        _ => {}
    }
*/

    /*
    let mut coin_api = CoinApi::new(COIN_LOGIN, COIN_PASSWORD);

    let categories_count = coin_api.categories_count();
    let expense_count = coin_api.expenses_count();

    println!("Categories: {}", categories_count);
    println!("Expenses: {}", expense_count);
    */

    //coin_api.create_expense(12, "Foo");
}
