use serde::{self, Serialize};
use uuid::Uuid;
use chrono::{Date, Local};

use std::time::SystemTime;

#[derive(Serialize, Debug)]
pub struct Temperature {
    temp_celsius: i32,
    temp_fahrenheit: i32,
    time: SystemTime,
    #[serde(with = "date_serde")]
    date: Date<Local>,
    uuid: Uuid
}

impl Temperature {
    pub fn new(temp_c: i32, temp_f: i32, u: &Uuid) -> Temperature {
        Temperature {
            temp_celsius: temp_c,
            temp_fahrenheit: temp_f,
            time: SystemTime::now(),
            date: Local::today(),
            uuid: *u
        }
    }

    pub fn current_reading(&self) {
        println!("{:?}", format!("Current temperature (C): {:?}\n
                                  Current temperature (F): {:?}\n
                                  Time stamped: {:?}\n
                                  Date stamped: {:?}",
                                    self.temp_celsius,
                                    self.temp_fahrenheit,
                                    self.time,
                                    self.date))
    }

    pub fn get_uuid(&self) -> uuid::Uuid {
        self.uuid
    }

    pub fn get_temp_celsius(&self) -> i32 {
        self.temp_celsius
    }

    pub fn get_temp_fahrenheit(&self) -> i32 {
        self.temp_fahrenheit
    }

    pub fn get_time(&self) -> SystemTime {
        self.time
    }

    pub fn get_date(&self) -> Date<Local> {
        self.date
    }
}

mod date_serde {
    use serde::{self, Serializer};
    use chrono::{Date, Local};

    const FORMAT: &'static str = "%Y-%m-%d";

    pub fn serialize<S>(
        date: &Date<Local>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }
}