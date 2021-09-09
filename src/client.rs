use std::error::Error;
use std::io;

pub struct Client {
    proposal_value: Option<i32>,
}

impl Client {
    fn new() -> Client {
        Client {
            proposal_value: None
        }
    }

    fn request_proposal() -> i32 {
        println!("Please input your integer proposal.");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Unexpected reading error");

        input.trim().parse::<i32>().expect("Unexpected parsing error")
    }
}