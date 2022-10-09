use reqwest::header::USER_AGENT;
use std::io::Read;
use text_colorizer::*;

fn get_weather() -> Result<(), reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    //let mut res = reqwest::blocking::get("https://api.weather.gov/points/39.7456,-97.0892").unwrap();
    let mut res = client.get("https://api.weather.gov/points/39.7456,-97.0892").header(USER_AGENT, "RustWeather").send()?;
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());
    println!("Body:\n{}", body);
    
    Ok(())
}

fn main() {
    println!("{}", "Getting Weather".green().bold());
    match get_weather() {
        Err(e) => println!("{}", e),
        _ => ()
    }
}
