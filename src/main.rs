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
        expect_password,
        expect_input_file
    }

    let mut state: ParsingState = ParsingState::empty;
    let mut login: &str = "";
    let mut password: &str = "";
    let mut input_file_path: &str = "";

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
            "-i" => {
                state = ParsingState::expect_input_file;
                println!("state: input file");
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
                    ParsingState::expect_input_file => {
                        state = ParsingState::empty;
                        input_file_path = value;
                    }
                    ParsingState::empty => {
                        println!("Skipping: {}", value)
                    }
                }
            }
        }
    }
    
    // Handling login
    if login.len() > 0 && password.len() > 0 {
        let mut coin_api = CoinApi::new(login, password);
        coin_api.login();
        
        // Printing stats
        let categories_count = coin_api.categories_count();
        let expense_count = coin_api.expenses_count();
        println!("Categories: {}", categories_count);
        println!("Expenses: {}", expense_count);
    }

    // Handling input
    let mut fake_coin_api = CoinApi::new(COIN_LOGIN, COIN_PASSWORD);
    if input_file_path.len() > 0 {
        println!("Got path: {}", input_file_path);
        let expenses = fake_coin_api.parseFile(input_file_path);
        println!("Parsed {} expenses", expenses.len());
    }
}
