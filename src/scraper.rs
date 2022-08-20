use reqwest::blocking::Client;
use scraper::{Html, Selector};
use bevy::prelude::*;
use reqwest::Error;
use open;
use std::collections::HashSet;
use std::env::home_dir;
use std::fs::File;
use std::io::prelude::*;
use std::io;

extern crate reqwest;
extern crate select;

use select::document::Document;
use select::predicate::{Name,Class,Predicate};


//https://www.scrapingbee.com/blog/web-scraping-rust/
pub struct ScraperPlugin;

impl Plugin for ScraperPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system_to_stage(StartupStage::PostStartup, scraper);
    }
}

fn scraper() {

    let mut file = File::create("horoscope.txt").unwrap();
    let mut owned_url: String = "https://www.astrology.com/horoscope/daily/".to_owned();
    let mut sign: String = sign_getter();

    /*loop{
        sign = sign_getter();
        if horoscopes().contains(&sign){
            break
        }
    }*/

    owned_url.push_str(&sign);
    owned_url.push_str(".html");
    let response = reqwest::blocking::get(
        owned_url.as_str(),
    )
    .unwrap();

    let main = Document::from_read(response).unwrap();

    let mut inner = main.find(Name("span"));

    let mut count:i32= 0;
    for i in inner {
        count = count + 1;
        if count == 2 {
            writeln!(&mut file, "{}", i.text()).unwrap();

        }
    }
    let opened = match open::with("horoscope.txt", "notepad"){
        Err(why) => panic!("couldn't open: {}",why),
        Ok(file) => file,
    };

}
    
fn sign_getter() -> String{
    let mut sign = String::new();
    println!("Enter your sign");
    io::stdin()
        .read_line(&mut sign)
        .expect("Failed to read input");
    sign.to_string()
}

fn horoscopes()-> HashSet<String>{
    let mut horoscopes = HashSet::new();
    horoscopes.insert("aries".to_string());
    horoscopes.insert("taurus".to_string());
    horoscopes.insert("gemini".to_string());
    horoscopes.insert("cancer".to_string());
    horoscopes.insert("leo".to_string());
    horoscopes.insert("virgo".to_string());
    horoscopes.insert("libra".to_string());
    horoscopes.insert("scorpio".to_string());
    horoscopes.insert("sagittarius".to_string());
    horoscopes.insert("capricorn".to_string());
    horoscopes.insert("aquarius".to_string());
    horoscopes.insert("pisces".to_string());
    horoscopes
    
}