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

static COIN_LOGIN: &'static str = "";
static COIN_PASSWORD: &'static str = "";



fn main() {

    let args: Vec<String> = env::args().collect();
    println!("args: {:?}", args);

    //let unwrapped_args: Vec<&str> = args.iter().map(|& ref x| println!("arg: {:?}", x));

    enum ParsingState {
        Empty,
        ExpectLogin,
        ExpectPassword,
        ExpectInputFile
    }

    let mut state: ParsingState = ParsingState::Empty;
    let mut login: &str = "";
    let mut password: &str = "";
    let mut input_file_path: &str = "";

    for arg in args.iter() {
        match arg.as_ref() {
            "-l" => {
                state = ParsingState::ExpectLogin;
                println!("state: login");
            },
            "-p" => {
                state = ParsingState::ExpectPassword;
                println!("state: password");
            }
            "-i" => {
                state = ParsingState::ExpectInputFile;
                println!("state: input file");
            }
            
            value @ _ => {
                match state {
                    ParsingState::ExpectLogin => {
                        state = ParsingState::Empty;
                        login = value;
                    }
                    ParsingState::ExpectPassword => {
                        state = ParsingState::Empty;
                        password = value;
                    }
                    ParsingState::ExpectInputFile => {
                        state = ParsingState::Empty;
                        input_file_path = value;
                    }
                    ParsingState::Empty => {
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
        let expenses = fake_coin_api.parse_file(input_file_path);
        println!("Parsed {} expenses", expenses.len());
    }
}
