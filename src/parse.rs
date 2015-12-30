extern crate hyper;

use std::io::Read;

use self::hyper::Client;
use self::hyper::Url;
use self::hyper::header::*;

static PARSE_APP_ID_HEADER_KEY: &'static str = "X-Parse-Application-Id";
static PARSE_API_KEY_HEADER_KEY: &'static str = "X-Parse-REST-API-Key";

static PARSE_LOGIN_URL_TEMPLATE: &'static str = "https://api.parse.com/1/login/";
static PARSE_CLASS_URL_TEMPLATE: &'static str = "https://api.parse.com/1/classes/[class]/";




pub struct Parse {
    client: Client,
    app_id: &'static str,
    api_key: &'static str,
}

pub enum ParseUrlError {
    InvalidBaseUrl
}

#[allow(dead_code)]
impl Parse {

    pub fn new(app_id: &'static str, api_key: &'static str) -> Parse {
        Parse { client: Client::new(), app_id: app_id, api_key: api_key}
    }

    pub fn login(&self, username: &str, password: &str) {
        let base_url = PARSE_LOGIN_URL_TEMPLATE.to_string();

        let mut parameters: Vec<(&str, &str)> = vec![];
        parameters.push(("username", username));
        parameters.push(("password", password));
        let login_url = build_query_url(&base_url, parameters).ok().unwrap();

        let headers = self.get_headers();

        match self.client.get(login_url).headers(headers).send() {
            Ok(mut response) => {
                let mut body = String::new();
                response.read_to_string(&mut body).unwrap();
                println!("Login: {}", body);
            },

            Err(error) => println!("Failed to login: {}", error),
        }
    }

    // MARK: Info requests

    pub fn count_total(&mut self, class_name: &str, predicate: Vec<(&str, &str)>) -> String {
        let base_url = PARSE_CLASS_URL_TEMPLATE.to_string().replace("[class]", class_name);

        let mut parameters = predicate;
        parameters.push(("count", "1"));
        parameters.push(("limit", "0"));
        let count_url = build_query_url(&base_url, parameters).ok().unwrap();

        let headers = self.get_headers();

        match self.client.get(count_url).headers(headers).send() {
            Ok(mut response) => {
                let mut body = String::new();
                response.read_to_string(&mut body).unwrap();
                body
            },

            Err(_) => "".to_string()
        }
    }

    pub fn count(&mut self, class_name: &str) -> String {
        let predicate = vec![];
        self.count_total(class_name, predicate)
    }
    
    // MARK: Create requests
    pub fn create_object(&mut self, class_name: &str, serialized_instance: &str) {
        let url = Url::parse(&(PARSE_CLASS_URL_TEMPLATE.to_string().replace("[class]", class_name))).ok().unwrap();
        let headers = self.get_headers();

        match self.client.post(url).headers(headers).body(serialized_instance).send() {
            Ok(mut response) => {
                let mut body = String::new();
                response.read_to_string(&mut body).unwrap();
                println!("{}", body);
            }

            Err(_) => {

            }
        }
    }
    
    // MARK Utilities

    fn get_headers(&self) -> Headers {
        let mut headers = Headers::new();
        let app_id = self.app_id.as_bytes().to_vec();
        let api_key = self.api_key.as_bytes().to_vec();

        headers.set_raw(PARSE_APP_ID_HEADER_KEY, vec![app_id]);
        headers.set_raw(PARSE_API_KEY_HEADER_KEY, vec![api_key]);

        headers
    }
}

// Utilities

fn build_query_url(base_url: &str, parameters: Vec<(&str, &str)>) -> Result<Url, ParseUrlError> {
    match Url::parse(base_url) {
        Ok(mut parsed_url) => {
            parsed_url.set_query_from_pairs(parameters.into_iter());
            Ok(parsed_url)
        },

        Err(_) => {
            Err(ParseUrlError::InvalidBaseUrl)
        }
    }
}
