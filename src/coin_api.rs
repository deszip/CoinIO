/*
    - init with user credentials
    - get categories/expenses count for user
    - get total for - period
                    - category

    IO
    - parsing evernote data
    - lint input file from evernote
    - upload parsed data
*/

extern crate xml;

use std::io::*;
use std::fs::File;

use super::rustc_serialize::json::Json as Json;
use super::rustc_serialize::json as json;

use self::xml::reader::{EventReader, XmlEvent};

use parse::Parse;

pub static COIN_APP_ID: &'static str = "BMx0eWt6Xtrle8CkjJyTOMwUVLf0vh5wM8Os5ljy";
pub static COIN_API_KEY: &'static str = "ZPV6tUDO71f9bwVWxc1iQl09St1o3ycEi5qXEoKs";



// MARK: Model structs
#[derive(RustcEncodable)]
#[allow(non_snake_case)]
struct Owner {
    __type: &'static str,
    className: &'static str,
    objectId: &'static str
}

impl Owner {
    pub fn new(object_id: &'static str) -> Owner {
        Owner { __type: "Pointer", className: "_User", objectId: object_id }
    }
}

#[derive(RustcEncodable)]
struct Date {
    __type: &'static str,
    iso: &'static str
}

impl Date {
    pub fn new(iso: &'static str) -> Date {
        Date { __type: "Date", iso: iso }
    }
}

#[derive(RustcEncodable)]
#[allow(non_snake_case)]
struct Expense {
    amount: i32,
    title: &'static str,
    expenseId: &'static str,
    owner: Owner,
    creationDate: Date
}

impl Expense {
    pub fn new(amount: i32, title: &'static str, creation_date: Date, owner: Owner) -> Expense {
        let expense_id = "";
        Expense { amount: amount, title: title, expenseId: expense_id, owner: owner, creationDate: creation_date }
    }
}




pub struct CoinApi {
    parse: Parse,
    login: &'static str,
    password: &'static str,
}

impl CoinApi {

    // MARK: Initialization

    pub fn new(user_login: &'static str, user_password: &'static str) -> CoinApi {
        CoinApi{ parse: Parse::new(COIN_APP_ID, COIN_API_KEY), login: user_login, password: user_password }
    }

    // MARK: API Calls

    pub fn expenses_count(&mut self) -> i32 {
        // build json predicate based on current user

        let predicate_string = "{\"owner\":{\"__type\":\"Pointer\",\"className\":\"_User\",\"objectId\":\"b1OFAr3yTN\"}}";

        let predicate = vec![("where", predicate_string)];
        let count_response = &self.parse.count_total("CNExpense", predicate);  // count for logged in user
        //let count_response = &self.parse.count("CNExpense");                   // overall count

        //println!("Response: {}", count_response);

        let response_data = Json::from_str(&count_response).unwrap();
        let expense_count = response_data.as_object().unwrap().get("count").unwrap().to_string().parse::<i32>().unwrap();

        expense_count
    }

    pub fn categories_count(&mut self) -> i32 {
        let predicate_string = "{\"owner\":{\"__type\":\"Pointer\",\"className\":\"_User\",\"objectId\":\"b1OFAr3yTN\"}}";

        let predicate = vec![("where", predicate_string)];
        let count_response = &self.parse.count_total("CNCategory", predicate);  // count for logged in user
        //let count_response = &self.parse.count("CNExpense");                   // overall count

        //println!("Response: {}", count_response);

        let response_data = Json::from_str(&count_response).unwrap();
        let expense_count = response_data.as_object().unwrap().get("count").unwrap().to_string().parse::<i32>().unwrap();

        expense_count
    }

    pub fn create_expense(&mut self, amount: i32, title: &'static str) {
        let creation_date = Date::new("2015-05-30T18:02:52.249Z");
        let owner = Owner::new("b1OFAr3yTN");
        let expense = Expense::new(amount, title, creation_date, owner);

        let serialized_expense = json::encode(&expense).unwrap();
        println!("Expense: {}", serialized_expense);

        &self.parse.create_object("CNExpense", &serialized_expense);
    }
    
    // MARK: Input parsing
    
    pub fn parseFile(&mut self, path: &str) -> Vec<Expense> {
        //println!("Parsing file at: {}", path);
        
        match File::open(path) {
            Ok(mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents);
                
                let mut days_count = 0;
                let mut expense_count = 0;
                let mut expense_data: Vec<&str> = vec![];
                
                for line in contents.lines() {
                    if line.starts_with("<div>") && line.len() > 5 {
                        if line.starts_with("<div><b>") && line.ends_with("</b></div>") && line.chars().nth(10).unwrap() == '.' {
                            // Date line
                            let date = &line[8..line.len() - 10];
                            //println!("{}", date);
                            days_count += 1;
                        } else if line.starts_with("<div>") && line.ends_with("</div>") && line.len() > 18 {
                            // Expense line
                            let expense = &line[5..line.len() - 6];
                            let parts: Vec<&str> = expense.splitn(2, " - ").collect();
                            if parts.len() == 2 {
                                //println!("    {}: {}", parts[0], parts[1]);
                                expense_data.app
                                expense_count += 1;
                            } else {
                               println!("Got mailformed entry: {}", line);
                            }
                        }
                    }
                }
                
                println!("Parsed {} days, with {} expenses", days_count, expense_count);
            }
            
            Err(error) => {
                println!("Error opening file: {}", error);
            }
        }
        
        vec![]
    }
}
