// use std::io::{stdin};
// use reqwest::Response;
// use serde::{Deserialize};
// use colored::*;
// use dotenv::dotenv;
// use std::env;

// #[derive(Deserialize, Debug)]
// struct WeatherResponce {
//     weather: Vec<Weather>,
//     main: Main,
//     wind: Wind,
//     name: String,
// }

// #[derive(Debug, Deserialize)]
// struct Weather {
//     description: String,
// }

// #[derive(Deserialize, Debug)]
// struct Main {
//     temp: f64,
//     humidity: f64,
//     pressure: f64,
// }

// #[derive(Debug, Deserialize)]
// struct Wind {
//     speed: f64,
// }

// fn get_weather_info(city: &str, country_code: &str, api_key: &str) -> Result<WeatherResponce, reqwest::Error> {
//     let url = format!(
//         "https://api.openweathermap.org/data/2.5/weather?q={},{}&units=metric&appid={}",
//         city, country_code, api_key
//     );
//     let response = reqwest::blocking::get(&url)?;
//     let response_json: WeatherResponce = response.json()?;
//     Ok(response_json)
// }

// fn display_weather_info(response: &WeatherResponce) {
//     let description = &response.weather[0].description;
//     let temperature = &response.main.temp;
//     let humidity = &response.main.humidity;
//     let pressure = &response.main.pressure;
//     let wind_speed = &response.wind.speed;

//     let weather_text = format!(
//         "Weather in {}: {} {}\n\t
//         > Temperature: {:.1}°C,\n\t
//         > Humidity: {:.1}%,\n\t
//         > Pressure: {:.1} hPa,\n\t
//         > Wind Speed: {:.1} m/s",
//         response.name,
//         description,
//         get_temp_emoji(*temperature), // Dereference temperature here
//         temperature,
//         humidity,
//         pressure,
//         wind_speed,
//     );

//     let weather_text_colored = match description.as_str() {
//         "clear sky" => weather_text.bright_yellow(),
//         "few clouds" | "scattered clouds" | "broken clouds" => weather_text.bright_blue(),
//         _ => weather_text.normal(),
//     };

//     println!("{}", weather_text_colored);
// }

// fn get_temp_emoji(temperature: f64) -> &'static str {
//     if temperature < 0.0 {
//         "❄️"
//     } else if temperature >= 0.0 && temperature < 10.0 {
//         "☁️"
//     } else if temperature >= 10.0 && temperature < 30.0 {
//         "⛅️"
//     } else {
//         "☀️"
//     }
// }

// fn main() {
//     dotenv().ok(); // Correct way to load environment variables

//     // Get API Key from environment variable
//     let api_key = match env::var("WEATHER_API_TOKEN") {
//         Ok(key) => key,
//         Err(_) => {
//             eprintln!("{}", "Error: WEATHER_API_TOKEN is not set in .env file!".bright_red());
//             return;
//         }
//     };

//     println!("{}", "Welcome to Weather Station!".bright_yellow());

//     loop {
//         // Reading City
//         println!("{}", "Please enter the name of the city: ".bright_green());
//         let mut city = String::new();
//         stdin().read_line(&mut city).expect("Failed to read input");
//         let city = city.trim();

//         // Reading Country Code
//         println!("{}", "Please enter the country code (e.g., US, CA): ".bright_green());
//         let mut country_code = String::new();
//         stdin().read_line(&mut country_code).expect("Failed to read input");
//         let country_code = country_code.trim();

//         if city.is_empty() || country_code.is_empty() {
//             eprintln!("{}", "City or Country Code cannot be empty!".bright_red());
//             continue;
//         }

//         match get_weather_info(&city, &country_code, &api_key) {
//             Ok(response) => {
//                 display_weather_info(&response);
//             }
//             Err(err) => {
//                 eprintln!("{}", format!("Error: {}", err).bright_red());
//             }
//         }
//     }
// }

use std::time::Instant;

use colored::Colorize;

fn main() {
    let start = Instant::now();
    let mut sum:u128 = 0;
    for i in 1..200_000_000 {
        sum += i
    }

    let finish = start.elapsed();

    println!("{}, {:?} time and the total is {:?}", "This Code took ".bright_green(), finish , sum );
}