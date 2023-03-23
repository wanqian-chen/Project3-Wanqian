use chrono::prelude::*;
use chrono::{DateTime, TimeZone, Utc};
use chrono_tz::Asia::Tokyo;

// get date in Tokyo
pub fn get_date_tokyo() -> String {
    let tokyo_time = Utc::now().with_timezone(&Tokyo);
    let tokyo = tokyo_time.format("%Y-%m-%d").to_string();
    tokyo
}

// get time in Tokyo
pub fn get_time_tokyo() -> String {
    let tokyo_time = Utc::now().with_timezone(&Tokyo);
    let tokyo = tokyo_time.format("%H:%M:%S").to_string();
    tokyo
}

// get date in Paris
pub fn get_date_paris() -> String {
    let paris_time = Utc::now().with_timezone(&chrono_tz::Europe::Paris);
    let paris = paris_time.format("%Y-%m-%d").to_string();
    paris
}

// get time in Paris
pub fn get_time_paris() -> String {
    let paris_time = Utc::now().with_timezone(&chrono_tz::Europe::Paris);
    let paris = paris_time.format("%H:%M:%S").to_string();
    paris
}

// get date in New York
pub fn get_date_ny() -> String {
    let ny_time = Utc::now().with_timezone(&chrono_tz::America::New_York);
    let ny = ny_time.format("%Y-%m-%d").to_string();
    ny
}

// get time in New York
pub fn get_time_ny() -> String {
    let ny_time = Utc::now().with_timezone(&chrono_tz::America::New_York);
    let ny = ny_time.format("%H:%M:%S").to_string();
    ny
}

// get date in London
pub fn get_date_london() -> String {
    let london_time = Utc::now().with_timezone(&chrono_tz::Europe::London);
    let london = london_time.format("%Y-%m-%d").to_string();
    london
}

// get time in London
pub fn get_time_london() -> String {
    let london_time = Utc::now().with_timezone(&chrono_tz::Europe::London);
    let london = london_time.format("%H:%M:%S").to_string();
    london
}

// get date in Sydney
pub fn get_date_sydney() -> String {
    let sydney_time = Utc::now().with_timezone(&chrono_tz::Australia::Sydney);
    let sydney = sydney_time.format("%Y-%m-%d").to_string();
    sydney
}

// get time in Sydney
pub fn get_time_sydney() -> String {
    let sydney_time = Utc::now().with_timezone(&chrono_tz::Australia::Sydney);
    let sydney = sydney_time.format("%H:%M:%S").to_string();
    sydney
}

// get date in San Francisco
pub fn get_date_sf() -> String {
    let sf_time = Utc::now().with_timezone(&chrono_tz::America::Los_Angeles);
    let sf = sf_time.format("%Y-%m-%d").to_string();
    sf
}

// get time in San Francisco
pub fn get_time_sf() -> String {
    let sf_time = Utc::now().with_timezone(&chrono_tz::America::Los_Angeles);
    let sf = sf_time.format("%H:%M:%S").to_string();
    sf
}

// get date in Beijing
pub fn get_date_beijing() -> String {
    let beijing_time = Utc::now().with_timezone(&chrono_tz::Asia::Shanghai);
    let beijing = beijing_time.format("%Y-%m-%d").to_string();
    beijing
}

// get time in Beijing
pub fn get_time_beijing() -> String {
    let beijing_time = Utc::now().with_timezone(&chrono_tz::Asia::Shanghai);
    let beijing = beijing_time.format("%H:%M:%S").to_string();
    beijing
}