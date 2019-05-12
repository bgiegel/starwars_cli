#[macro_use]
extern crate serde_derive;
extern crate reqwest;

use reqwest::Response;
use reqwest::Error;
use std::io;
use std::fmt;
use std::num::ParseIntError;

#[derive(Deserialize, Debug)]
struct People {
    name: String,
    url: String,
    height:String,
    mass:String,
    hair_color: String,
    eye_color: String
}

impl fmt::Display for People {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name: {}\nHeight: {} cm\nHair: {}\n", self.name, self.height, self.hair_color)
    }
}

#[derive(Deserialize, Debug)]
struct PeopleResponse {
    count: u32,
    next: String,
    previous: Option<String>,
    results: Vec<People>
}

fn main() -> Result<(), Error> {
    let people_url = format!("https://swapi.co/api/people/?page={page}", page = "1");
    let mut response:Response = reqwest::get(&people_url)?;

    let people_response: PeopleResponse = response.json()?;

    println!("{:?}", people_response);

    println!("Welcome to the StarWars wiki CLI !");
    println!("Choose the character you want to know more about.");

    loop {
        for (i, people) in people_response.results.iter().enumerate() {
            println!("{} - {}", i+1, people.name)
        }
        match read_choice() {
            Ok(index) => {
                match people_response.results.get(index) {
                    None => println!("Wrong index"),
                    Some(people) => println!("{}", people)
                };
            }
            Err(_err) => println!("Not a number")
        };
    }
}

fn read_choice() -> Result<usize, ParseIntError> {
    println!("Enter character number : ");
    let mut choice = String::new();
    match io::stdin().read_line(&mut choice) {
        Ok(_n) => {}
        Err(error) => panic!(error),
    }

    choice.trim().parse::<i32>().map(|ch| ch as usize - 1)
}
