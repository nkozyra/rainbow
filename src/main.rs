use regex::Regex;
use std::env;
use std::io::Read;
use std::process::ExitCode;

use reqwest::header::USER_AGENT;
use serde::{Deserialize, Serialize};
use text_colorizer::*;

#[derive(Serialize, Deserialize)]
struct LocationProperties {
    forecast: String,
}

#[derive(Serialize, Deserialize)]
struct LocationPayload {
    id: String,
    properties: LocationProperties,
}

fn process_data(ind: String) -> serde_json::Result<LocationPayload> {
    let p: LocationPayload = serde_json::from_str(&ind)?;

    Ok(p)
}


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ForecastPeriod {
    number: u8,
    name: String,
    temperature: u8,
    icon: String,
    short_forecast: String,
}

#[derive(Serialize, Deserialize)]
struct ForecastProperties {
    periods: Vec<ForecastPeriod>,
}

#[derive(Serialize, Deserialize)]
struct ForecastPayload {
    properties: ForecastProperties,
}

fn process_forecast(ind: String) -> serde_json::Result<ForecastPayload> {
    let p: ForecastPayload = serde_json::from_str(&ind)?;
    Ok(p)
}

fn get_emojis(ind: Vec<String>) -> Vec<String>  {
    let mut emojis: Vec<String> = Vec::new();

    let rain = Regex::new(r"(?i)(showers|rain)").unwrap();
    let thunder = Regex::new(r"(?i)(thunder)").unwrap();
    let mcloudy = Regex::new(r"(?i)(mostly cloudy)").unwrap();
    let pcloudy = Regex::new(r"(?i)(Partly Cloudy)").unwrap();
    let msunny = Regex::new(r"(?i)(Mostly Sunny)").unwrap();

    for v in ind.iter() {
        let mut e: String = "☀️".to_string();
        if rain.is_match(v) {
            e = "🌧".to_string();
        }
        if thunder.is_match(v) {
            e = "⛈".to_string();
        }
        if mcloudy.is_match(v) {
            e = "☁️".to_string();
        }
        if pcloudy.is_match(v) {
            e = "⛅️".to_string();
        }
        if msunny.is_match(v) {
            e = "🌤".to_string();
        }
        emojis.push(e.to_string());
    }

    emojis
}

fn get_weather(lat: f32, lng: f32) -> Result<(), reqwest::Error> {
    let client = reqwest::blocking::Client::new();

    let wurl = format!("https://api.weather.gov/points/{lat},{lng}", lat=lat, lng=lng);
    dbg!(&wurl);
    let mut res = client.get(wurl).header(USER_AGENT, "RustWeather").send()?;
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    let location_data: LocationPayload = match process_data(body) {
        Ok(p) => p,
        Err(error) => panic!("error with stuff: {:?}", error),
    };

    let mut forecast_res = client.get(location_data.properties.forecast).header(USER_AGENT, "RustWeather").send()?;
    let mut forecast_body = String::new();
    forecast_res.read_to_string(&mut forecast_body).unwrap();

    let forecast_data: ForecastPayload = match process_forecast(forecast_body) {
        Ok(p) => p,
        Err(error) => panic!("error with stuff: {:?}", error),
    };

    let mut highs: Vec<u8> = Vec::new();
    let mut lows: Vec<u8> = Vec::new();
    let mut labels: Vec<String> = Vec::new();
    let mut icons: Vec<String> = Vec::new();
    for (i, period) in forecast_data.properties.periods.iter().enumerate() {
        if i % 2 == 0 {
            highs.push(period.temperature);
            let mut l = period.name.to_string();
            if l == "Today" {
                l = "  ".to_string();
            }
            labels.push(l[..2].to_string());
            icons.push(period.short_forecast.to_string());
        } else {
            lows.push(period.temperature);
        }
    }

    let emojis: Vec<String> = get_emojis(icons);
    for label in labels.iter() {
        print!("  {}  ", label);
    }
    println!("");
    for emoji in emojis.iter() {
        print!("  {}   ", emoji);
    }
    println!("");
    for period in highs.iter() {
        print!("  {}  ", period);
    }
    println!("");
    for _ in highs.iter() {
        print!("  --  ");
    }
    println!("");
    for period in lows.iter() {
        print!("  {}  ", period);
    }

    Ok(())
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("ERROR: must supply a latitude/longitude argument (./rainbow \"30.1,-80.2\")");
        return ExitCode::FAILURE;
    }
    let mut bs = args[1].split(",");
    let lat = match bs.next().unwrap().parse::<f32>() {
        Ok(v) => v,
        Err(error) => panic!("error parsing lat: {:?}", error),
    };
    let lng = match bs.next().unwrap().parse::<f32>() {
        Ok(v) => v,
        Err(error) => panic!("error parsing lat: {:?}", error),
    };

    println!("{}", "Getting Weather".green().bold());
    match get_weather(lat, lng) {
        Err(e) => println!("{}", e),
        _ => ()
    }
    ExitCode::SUCCESS
}
