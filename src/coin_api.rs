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
extern crate chrono;

use self::chrono::*;

use std::io::*;
use std::fs::File;

use super::rustc_serialize::json::Json as Json;
use super::rustc_serialize::json as json;

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

#[allow(dead_code)]
impl Owner {
    pub fn new(object_id: &'static str) -> Owner {
        Owner { __type: "Pointer", className: "_User", objectId: object_id }
    }
}

#[derive(RustcEncodable)]
struct ExpenseDate {
    __type: String,
    iso: String
}

#[allow(dead_code)]
impl ExpenseDate {
    pub fn new<S>(iso: S) -> ExpenseDate where S: Into<String> {
        ExpenseDate { __type: "Date".to_string(), iso: iso.into() }
    }
}

#[derive(RustcEncodable)]
#[allow(non_snake_case)]
struct Expense {
    amount: i32,
    title: &'static str,
    expenseId: &'static str,
    owner: Owner,
    creationDate: ExpenseDate
}

#[allow(dead_code)]
impl Expense {
    pub fn new(amount: i32, title: &'static str, creation_date: ExpenseDate, owner: Owner) -> Expense {
        let expense_id = "";
        Expense { amount: amount, title: title, expenseId: expense_id, owner: owner, creationDate: creation_date }
    }
}




pub struct CoinApi {
    parse: Parse,
    login: String,
    password: String,
}

#[allow(dead_code)]
impl CoinApi {

    // MARK: Initialization

    pub fn new<S>(user_login: S, user_password: S) -> CoinApi where S: Into<String> {
        CoinApi{ parse: Parse::new(COIN_APP_ID, COIN_API_KEY), login: user_login.into(), password: user_password.into() }
    }

    // MARK: API Calls

    pub fn login(&self) {
        &self.parse.login(&self.login, &self.password);
    }

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
        let creation_date = ExpenseDate::new("2015-05-30T18:02:52.249Z");
        let owner = Owner::new("b1OFAr3yTN");
        let expense = Expense::new(amount, title, creation_date, owner);

        let serialized_expense = json::encode(&expense).unwrap();
        println!("Expense: {}", serialized_expense);

        &self.parse.create_object("CNExpense", &serialized_expense);
    }
    
    // MARK: Input parsing
    
    pub fn parse_file(&mut self, path: &str) -> Vec<Expense> {
        //println!("Parsing file at: {}", path);
        
        match File::open(path) {
            Ok(mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents);
                
                let mut days_count = 0;
                let mut expense_count = 0;
                //let mut expense_data: Vec<&str> = vec![];
                
                let mut date: ExpenseDate = ExpenseDate::new("");
                for line in contents.lines() {
                    if line.starts_with("<div>") && line.len() > 5 {
                        if line.starts_with("<div><b>") && line.ends_with("</b></div>") && line.chars().nth(10).unwrap() == '.' {
                            // Date line
                            let iso_date_string = format!("{:?}", UTC.ymd(2015, 7, 8).and_hms(9, 10, 11));
                            println!("{}", iso_date_string);
                            date = ExpenseDate::new(&line[8..line.len() - 10]);
                            days_count += 1;
                        } else if line.starts_with("<div>") && line.ends_with("</div>") && line.len() > 18 {
                            // Expense line
                            let expense = &line[5..line.len() - 6];
                            let parts: Vec<&str> = expense.splitn(2, " - ").collect();
                            if parts.len() == 2 {
                                println!("{} - {} : {}", date.iso, parts[0], parts[1]);
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
