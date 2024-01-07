use crate::{
    db::connection::establish_connection,
    models::{ConstructorResponse, DriverResponse, RaceResponse, SeasonResponse},
};
use chrono::{NaiveDate, NaiveTime};
use diesel::prelude::*;
use diesel::result::Error;
use ergast_rust::{api::URLParams, ergast::Ergast};
use std::time::{SystemTime, UNIX_EPOCH};

fn combine_date_and_time(date_str: &str, time_str: &str) -> Result<SystemTime, chrono::ParseError> {
    let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")?;
    let time = NaiveTime::parse_from_str(time_str, "%H:%M:%SZ")?;
    let datetime = NaiveDate::and_time(&date, time);
    // Convert to SystemTime
    Ok(UNIX_EPOCH + std::time::Duration::from_secs(datetime.timestamp() as u64))
}

fn convert_system_time_to_string(time: SystemTime) -> String {
    let datetime = chrono::DateTime::<chrono::Utc>::from(time);
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::db::schema::seasons)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Season {
    pub id: i32,
    pub season: i32,
    pub url: String,
    pub created_at: SystemTime,
}

#[derive(Insertable)]
#[diesel(table_name = crate::db::schema::seasons)]
pub struct NewSeason<'a> {
    pub season: &'a i32,
    pub url: &'a String,
}

impl Season {
    pub async fn post() {
        use crate::db::schema::seasons;

        let mut connection = establish_connection();
        let params = URLParams {
            limit: 100,
            offset: 0,
        };
        let response = Ergast::seasons(params)
            .await
            .expect("failed to fetch seasons");

        for season in response.table.seasons {
            let new_season = NewSeason {
                season: &season.season,
                url: &season.url,
            };
            println!("Inserting season {}", season.season);
            let result = diesel::insert_into(seasons::table)
                .values(&new_season)
                .returning(Season::as_returning())
                .get_result(&mut connection);

            if let Err(e) = result {
                println!("Error inserting season {}: {}", season.season, e);
            }
        }
    }

    pub fn get(season: i32) -> Season {
        use crate::db::schema::seasons;
        let mut connection = establish_connection();
        seasons::table
            .filter(seasons::season.eq(season))
            .first::<Season>(&mut connection)
            .expect("loading error")
    }

    pub fn is_exist() -> bool {
        use crate::db::schema::seasons::dsl::*;

        let mut connection = establish_connection();
        let results = seasons.load::<Season>(&mut connection);

        match results {
            Ok(s) => !s.is_empty(),
            Err(e) => {
                println!("Error loading seasons: {}", e);
                false
            }
        }
    }

    pub fn generate_response() -> Result<Vec<SeasonResponse>, Error> {
        use crate::db::schema::seasons::dsl::*;

        let mut connection = establish_connection();
        let results = seasons.load::<Season>(&mut connection);

        match results {
            Ok(s) => {
                let mut v = s
                    .iter()
                    .map(|s| SeasonResponse {
                        season: s.season,
                        url: s.url.clone(),
                    })
                    .collect::<Vec<SeasonResponse>>();
                v.reverse(); // descending order
                Ok(v)
            }
            Err(e) => {
                println!("Error loading seasons: {}", e);
                Err(e)
            }
        }
    }
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::db::schema::races)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Season))]
pub struct Race {
    id: i32,
    season: i32,
    round: i32,
    url: String,
    race_name: String,
    event_time: SystemTime,
    created_at: SystemTime,
}

#[derive(Insertable)]
#[diesel(table_name = crate::db::schema::races)]
pub struct NewRace<'a> {
    season: &'a i32,
    round: &'a i32,
    url: &'a str,
    race_name: &'a str,
    event_time: &'a SystemTime,
}

impl Race {
    pub async fn post(season: &Season) -> () {
        use crate::db::schema::races;

        let mut connection = establish_connection();
        let params = URLParams {
            limit: 100,
            offset: 0,
        };
        let response = Ergast::race(season.season, params)
            .await
            .expect("failed to fetch races");

        for race in response.table.races {
            let season = Season::get(race.season);
            let new_race = NewRace {
                season: &season.id,
                round: &race.round,
                url: &race.url,
                race_name: &race.race_name,
                event_time: &combine_date_and_time(&race.date, &race.time.unwrap()).unwrap(),
            };
            println!("Inserting race season:{} round:{}", race.season, race.round);
            let result = diesel::insert_into(races::table)
                .values(&new_race)
                .returning(Race::as_returning())
                .get_result(&mut connection);

            if let Err(e) = result {
                println!("Error inserting race {} {}: {}", race.season, race.round, e);
            }
        }
    }

    pub fn get(season: &Season) -> Vec<Race> {
        use crate::db::schema::races;
        let mut connection = establish_connection();
        races::table
            .filter(races::season.eq(season.id))
            .load::<Race>(&mut connection)
            .expect("loading error")
            .into_iter()
            .collect::<Vec<Race>>()
    }

    pub fn is_exist(season: &Season) -> bool {
        let results = Race::get(&season);
        if results.is_empty() {
            return false;
        }
        true
    }

    pub fn generate_response(season: &Season) -> Vec<RaceResponse> {
        let results = Race::get(season);
        results
            .iter()
            .map(|race| RaceResponse {
                season: season.season,
                round: race.round,
                race_name: race.race_name.clone(),
                circuit_name: "placeholder".to_string(),
                date: convert_system_time_to_string(race.event_time),
            })
            .collect::<Vec<RaceResponse>>()
    }
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::db::schema::drivers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Driver {
    pub id: String,
    pub permanent_number: Option<i32>,
    pub code: Option<String>,
    pub given_name: String,
    pub family_name: String,
    pub date_of_birth: NaiveDate,
    pub nationality: String,
    pub created_at: SystemTime,
}

impl Driver {
    pub fn get() -> Vec<Driver> {
        use crate::db::schema::drivers;
        let mut connection = establish_connection();
        drivers::table
            .load::<Driver>(&mut connection)
            .expect("loading error")
            .into_iter()
            .collect::<Vec<Driver>>()
    }

    pub async fn post() -> () {
        use crate::db::schema::drivers;

        let mut connection = establish_connection();
        let params = URLParams {
            limit: 1000,
            offset: 0,
        };
        let response = Ergast::drivers(params)
            .await
            .expect("failed to fetch drivers");

        for driver in response.table.drivers {
            let naive_date = NaiveDate::parse_from_str(&driver.date_of_birth, "%Y-%m-%d")
                .map_err(|e| format!("Error parsing date: {}", e));

            if let Err(e) = naive_date {
                println!("Error parsing date: {}", e);
                continue;
            }

            let new_driver = NewDriver {
                id: &driver.driver_id,
                permanent_number: driver.permanent_number,
                code: driver.code,
                given_name: &driver.given_name,
                family_name: &driver.family_name,
                date_of_birth: &naive_date.unwrap(),
                nationality: &driver.nationality,
            };

            println!("Inserting driver {}", driver.driver_id);
            let result = diesel::insert_into(drivers::table)
                .values(&new_driver)
                .returning(Driver::as_returning())
                .get_result(&mut connection);

            if let Err(e) = result {
                println!("Error inserting driver {}: {}", driver.driver_id, e);
            }
        }
    }

    pub fn is_exist() -> bool {
        let results = Driver::get();
        if results.is_empty() {
            return false;
        }
        true
    }

    pub fn generate_response() -> Vec<DriverResponse> {
        use crate::db::schema::drivers::dsl::*;

        let mut connection = establish_connection();
        let results = drivers.load::<Driver>(&mut connection);

        match results {
            Ok(s) => s
                .into_iter()
                .map(|d| DriverResponse {
                    id: d.id,
                    permanent_number: d.permanent_number,
                    code: d.code,
                    given_name: d.given_name,
                    family_name: d.family_name,
                    date_of_birth: d.date_of_birth,
                    nationality: d.nationality,
                })
                .collect::<Vec<DriverResponse>>(),
            Err(e) => {
                println!("Error loading drivers: {}", e);
                vec![]
            }
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = crate::db::schema::drivers)]
pub struct NewDriver<'a> {
    pub id: &'a String,
    pub permanent_number: Option<i32>,
    pub code: Option<String>,
    pub given_name: &'a String,
    pub family_name: &'a String,
    pub date_of_birth: &'a NaiveDate,
    pub nationality: &'a String,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::db::schema::constructors)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Constructor {
    pub id: String,
    pub url: String,
    pub name: String,
    pub nationality: String,
    pub created_at: SystemTime,
}

impl Constructor {
    pub fn get() -> Vec<Constructor> {
        use crate::db::schema::constructors;
        let mut connection = establish_connection();
        constructors::table
            .load::<Constructor>(&mut connection)
            .expect("loading error")
            .into_iter()
            .collect::<Vec<Constructor>>()
    }

    pub async fn post() -> () {
        use crate::db::schema::constructors;

        let mut connection = establish_connection();
        let params = URLParams {
            limit: 1000,
            offset: 0,
        };
        let response = Ergast::constructors(params)
            .await
            .expect("failed to fetch constructors");

        for constructor in response.table.constructors {
            let new_constructor = NewConstructor {
                id: &constructor.constructor_id,
                url: &constructor.url,
                name: &constructor.name,
                nationality: &constructor.nationality,
            };

            println!("Inserting constructor {}", constructor.constructor_id);
            let result = diesel::insert_into(constructors::table)
                .values(&new_constructor)
                .returning(Constructor::as_returning())
                .get_result(&mut connection);

            if let Err(e) = result {
                println!(
                    "Error inserting constructor {}: {}",
                    constructor.constructor_id, e
                );
            }
        }
    }

    pub fn is_exist() -> bool {
        let results = Constructor::get();
        if results.is_empty() {
            return false;
        }
        true
    }

    pub fn generate_response() -> Vec<ConstructorResponse> {
        use crate::db::schema::constructors::dsl::*;

        let mut connection = establish_connection();
        let results = constructors.load::<Constructor>(&mut connection);

        match results {
            Ok(s) => s
                .into_iter()
                .map(|d| ConstructorResponse {
                    id: d.id,
                    url: d.url,
                    name: d.name,
                    nationality: d.nationality,
                })
                .collect::<Vec<ConstructorResponse>>(),
            Err(e) => {
                println!("Error loading drivers: {}", e);
                vec![]
            }
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = crate::db::schema::constructors)]
pub struct NewConstructor<'a> {
    pub id: &'a String,
    pub url: &'a String,
    pub name: &'a String,
    pub nationality: &'a String,
}
