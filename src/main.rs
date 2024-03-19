use colored::*;
use serde::Deserialize;
use std::io;

// Struct to desierialize json from openweathermap api
#[#[derive(Deserialize, Debug)]
struct WeatherResponse{
    weather: Vec<Weather>,
    main: Main,
    wind: Wind,
    name: String,
}

// Struct to represent weather description
#[#[derive(Deserialize, Debug)]
struct Weather{
    description: String,
}

// Struct to represent the main weather parameters
#[#[derive(Deserialize, Debug)]
struct Main{
    temp: f64,
    humidity: f64,
    pressure: f64,
}

// Struct to represent wind information
#[#[derive(Deserialize, Debug)]
struct Wind{
    speed: f64,
}


