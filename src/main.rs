use std::{env,str};
use std::process::Command;

fn main() {
    let url: String = env::args().nth(1).expect("Need an url to look up to");
    for i in 97..123 {
        for j in 97..123 {
            let fmt_url = format!("https://www.{}.{}{}", url, str::from_utf8(&[i]).expect("err parsing to utf-8"), str::from_utf8(&[j]).expect("err parsing to utf-8"));
            let response = reqwest::blocking::get(&fmt_url);
            match response {
                Ok(res) => {
                    Command::new("printf").args([r"\e[32m%s\e[0m -- %s\n",&res.status().to_string(),&fmt_url]).status().expect("err to print");
                },
                _ => ()
            }
        }
    }
}
