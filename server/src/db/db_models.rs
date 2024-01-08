use crate::{
    color_pallet::ColorPallet,
    db::connection::establish_connection,
    models::{
        ChartResponse, ConstructorResponse, DriverResponse, LapLineChartData, PitstopResponse,
        RaceResponse, SeasonResponse,
    },
};
use bigdecimal::{BigDecimal, ToPrimitive};
use chrono::{NaiveDate, NaiveTime};
use diesel::prelude::*;
use diesel::result::Error;
use ergast_rust::api::{Path, URLParams};
use ergast_rust::ergast::Ergast;
use serde::Serialize;
use std::{
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};

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

    pub fn get_by_id(id: i32) -> Season {
        use crate::db::schema::seasons;
        let mut connection = establish_connection();
        seasons::table
            .filter(seasons::id.eq(id))
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

    pub async fn generate_response() -> Result<Vec<SeasonResponse>, Error> {
        use crate::db::schema::seasons::dsl::*;
        // check if season data is already in the database
        if !Season::is_exist() {
            println!("Season data is not in the database. Fetch from Ergast API.");
            // if not, fetch season data from Ergast API and insert it into the database
            Season::post().await;
        }

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
#[allow(dead_code)]
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

    pub fn get_races_in_season(season: &Season) -> Vec<Race> {
        use crate::db::schema::races;
        let mut connection = establish_connection();
        races::table
            .filter(races::season.eq(season.id))
            .load::<Race>(&mut connection)
            .expect("loading error")
            .into_iter()
            .collect::<Vec<Race>>()
    }

    pub fn get(season: &Season, round: i32) -> Option<Race> {
        use crate::db::schema::races;
        let mut connection = establish_connection();
        let result = races::table
            .filter(races::season.eq(season.id).and(races::round.eq(round)))
            .load::<Race>(&mut connection);
        result.ok().and_then(|mut v| v.pop())
    }

    pub fn is_exist(season: &Season) -> bool {
        let results = Race::get_races_in_season(&season);
        if results.is_empty() {
            return false;
        }
        true
    }

    pub async fn generate_response(season: &Season) -> Vec<RaceResponse> {
        // check if race data is already in the database
        if !Race::is_exist(&season) {
            println!("Race data is not in the database. Fetch from Ergast API.");
            // if not, fetch race data from Ergast API and insert it into the database
            Race::post(&season).await;
        }

        let results = Race::get_races_in_season(season);

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

    pub fn get_by_id(id: &str) -> Driver {
        use crate::db::schema::drivers;
        let mut connection = establish_connection();
        drivers::table
            .filter(drivers::id.eq(id))
            .first::<Driver>(&mut connection)
            .expect("loading error")
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

    pub fn get_by_id(id: &str) -> Constructor {
        use crate::db::schema::constructors;
        let mut connection = establish_connection();
        constructors::table
            .filter(constructors::id.eq(id))
            .first::<Constructor>(&mut connection)
            .expect("loading error")
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

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::db::schema::standings)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Standing {
    pub id: i32,
    pub race: i32,
    pub driver_id: String,
    pub constructor_id: String,
    pub position: i32,
    pub position_text: String,
    pub points: i32,
    pub wins: i32,
    pub created_at: SystemTime,
}

impl Standing {
    pub fn get(race: &Race) -> Vec<Standing> {
        use crate::db::schema::standings;
        let mut connection = establish_connection();
        standings::table
            .filter(standings::race.eq(race.id))
            .load::<Standing>(&mut connection)
            .expect("loading error")
            .into_iter()
            .collect::<Vec<Standing>>()
    }

    pub async fn post(race: &Race) -> () {
        use crate::db::schema::standings;

        let mut connection = establish_connection();
        let season = Season::get_by_id(race.season);
        let path = Path {
            year: season.season,
            round: Some(race.round),
        };

        let params = URLParams {
            limit: 1000,
            offset: 0,
        };
        let response = Ergast::standings(path, params)
            .await
            .expect("failed to fetch standings");

        let standing_list = &response
            .table
            .standings_lists
            .get(0)
            .unwrap()
            .driver_standings;
        for standing in standing_list {
            let driver = Driver::get()
                .into_iter()
                .find(|d| d.id == standing.driver.driver_id)
                .unwrap();
            let constructor = Constructor::get()
                .into_iter()
                .find(|c| c.id == standing.constructors.get(0).unwrap().constructor_id)
                .unwrap();
            let new_standing = NewStanding {
                race: &race.id,
                driver_id: &driver.id,
                constructor_id: &constructor.id,
                position: &standing.position,
                position_text: &standing.position_text,
                points: &standing.points,
                wins: &standing.wins,
            };

            println!("Inserting standing {} {}", race.season, race.round);
            let result = diesel::insert_into(standings::table)
                .values(&new_standing)
                .returning(Standing::as_returning())
                .get_result(&mut connection);

            if let Err(e) = result {
                println!(
                    "Error inserting standing {} {}: {}",
                    race.season, race.round, e
                );
            }
        }
    }

    pub fn is_exist(race: &Race) -> bool {
        let results = Standing::get(race);
        if results.is_empty() {
            return false;
        }
        true
    }

    pub async fn generate_response(race: &Race) -> ChartResponse<String, i32> {
        if !Standing::is_exist(&race) {
            println!("Standing data is not in the database. Fetch from Ergast API.");
            // if not, fetch standing data from Ergast API and insert it into the database
            Standing::post(&race).await;
        }

        let results = Standing::get(race);
        let mut x = Vec::new();
        let mut y = Vec::new();
        let mut color = Vec::new();

        for entity in results {
            let driver = Driver::get_by_id(&entity.driver_id);
            let constructor = Constructor::get_by_id(&entity.constructor_id);
            x.push(driver.code.unwrap_or("NA".to_string()));
            y.push(entity.points);
            color.push(ColorPallet::get_color(constructor.name.as_str()));
        }
        ChartResponse {
            x,
            y,
            color: Some(color),
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = crate::db::schema::standings)]
pub struct NewStanding<'a> {
    pub race: &'a i32,
    pub driver_id: &'a String,
    pub constructor_id: &'a String,
    pub position: &'a i32,
    pub position_text: &'a String,
    pub points: &'a i32,
    pub wins: &'a i32,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::db::schema::laptimes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Laptime {
    pub id: i32,
    pub race_id: i32,
    pub driver_id: String,
    pub lap_number: i32,
    pub lap_time: String,
    pub position: i32,
    pub created_at: SystemTime,
}

impl Laptime {
    pub fn get(race: &Race) -> Vec<Laptime> {
        use crate::db::schema::laptimes;
        let mut connection = establish_connection();
        laptimes::table
            .filter(laptimes::race_id.eq(race.id))
            .load::<Laptime>(&mut connection)
            .expect("loading error")
            .into_iter()
            .collect::<Vec<Laptime>>()
    }

    pub async fn post(race: &Race) -> () {
        use crate::db::schema::laptimes;

        let mut connection = establish_connection();
        let season = Season::get_by_id(race.season);
        let path = Path {
            year: season.season,
            round: Some(race.round),
        };

        let params = URLParams {
            limit: 2000,
            offset: 0,
        };
        let response = Ergast::laps(path, params)
            .await
            .expect("failed to fetch laps");

        let laps = &response.table.races.get(0).unwrap().laps;

        let laps = match laps {
            Some(l) => l,
            None => {
                println!("No laps data");
                return;
            }
        };

        for lap in laps {
            let lap_number = lap.number;
            for timing in &lap.timings {
                let driver = Driver::get_by_id(&timing.driver_id);
                let new_laptime = NewLaptime {
                    race_id: &race.id,
                    driver_id: &driver.id,
                    lap_number: &lap_number,
                    lap_time: &timing.time,
                    position: &timing.position,
                };

                println!(
                    "Inserting laptime {} {} lap{}",
                    &race.id, &driver.id, lap_number
                );
                let result = diesel::insert_into(laptimes::table)
                    .values(&new_laptime)
                    .returning(Laptime::as_returning())
                    .get_result(&mut connection);

                if let Err(e) = result {
                    println!(
                        "Error inserting laptime {} {} lap:{}: {}",
                        &race.id, &driver.id, &lap_number, e
                    );
                }
            }
        }
    }

    pub fn is_exist(race: &Race) -> bool {
        let results = Laptime::get(race);
        if results.is_empty() {
            return false;
        }
        true
    }

    // convert text formatted like "m:ss.SSS" to f64
    pub fn convert_lap_time_text_to_f64(lap_time: &str) -> Result<f64, &str> {
        let parts: Vec<&str> = lap_time.split(':').collect();
        if parts.len() != 2 {
            return Err("Invalid time format");
        }

        let min_sec: Vec<&str> = parts[1].split('.').collect();
        if min_sec.len() != 2 {
            return Err("Invalid time format");
        }

        let minutes = match parts[0].parse::<f64>() {
            Ok(m) => m,
            Err(_) => return Err("Invalid minutes"),
        };

        let seconds = match min_sec[0].parse::<f64>() {
            Ok(s) => s,
            Err(_) => return Err("Invalid seconds"),
        };

        let milliseconds = match min_sec[1].parse::<f64>() {
            Ok(ms) => ms / 1000.0, // convert milliseconds to seconds
            Err(_) => return Err("Invalid milliseconds"),
        };

        Ok(minutes * 60.0 + seconds + milliseconds)
    }

    pub async fn generate_response(race: &Race) -> Vec<LapLineChartData> {
        if !Laptime::is_exist(&race) {
            println!("Laptime data is not in the database. Fetch from Ergast API.");
            // if not, fetch standing data from Ergast API and insert it into the database
            Laptime::post(&race).await;
        }

        let laps = Laptime::get(&race);
        let mut map = HashMap::new();
        for lap in laps {
            let driver_id = lap.driver_id.clone();
            let time = Laptime::convert_lap_time_text_to_f64(&lap.lap_time);
            let lap = lap.lap_number;
            let entry = map
                .entry(driver_id.clone())
                .or_insert(LapLineChartData::new(driver_id));
            entry.laps.push(lap);
            entry.laptime.push(time.unwrap());
        }
        let mut vec = map
            .into_iter()
            .map(|(_, v)| v)
            .collect::<Vec<LapLineChartData>>();
        vec.sort_by(|a, b| a.driver_id.partial_cmp(&b.driver_id).unwrap());
        vec
    }
}

#[derive(Insertable)]
#[diesel(table_name = crate::db::schema::laptimes)]
pub struct NewLaptime<'a> {
    pub race_id: &'a i32,
    pub driver_id: &'a String,
    pub lap_number: &'a i32,
    pub lap_time: &'a String,
    pub position: &'a i32,
}

#[derive(Queryable, Selectable, Debug, Serialize)]
#[diesel(table_name = crate::db::schema::pitstops)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Pitstop {
    pub id: i32,
    pub race_id: i32,
    pub driver_id: String,
    pub lap_number: i32,
    pub pitstop_number: i32,
    pub pittime: String,
    pub duration: BigDecimal,
    pub created_at: SystemTime,
}

impl Pitstop {
    pub fn get(race: &Race) -> Vec<Pitstop> {
        use crate::db::schema::pitstops;
        let mut connection = establish_connection();
        pitstops::table
            .filter(pitstops::race_id.eq(race.id))
            .load::<Pitstop>(&mut connection)
            .expect("loading error")
            .into_iter()
            .collect::<Vec<Pitstop>>()
    }

    pub async fn post(race: &Race) -> () {
        use crate::db::schema::pitstops;

        let mut connection = establish_connection();
        let season = Season::get_by_id(race.season);
        let path = Path {
            year: season.season,
            round: Some(race.round),
        };

        let params = URLParams {
            limit: 2000,
            offset: 0,
        };
        let response = Ergast::pitstops(path, params)
            .await
            .expect("failed to fetch pitstops");

        let pitstops = &response.table.races.get(0).unwrap().pitstops;

        let pitstops = match pitstops {
            Some(l) => l,
            None => {
                println!("No pitstop data");
                return;
            }
        };

        for pitstop in pitstops {
            let driver = Driver::get_by_id(&pitstop.driver_id);
            let new_pitstop = NewPitstop {
                race_id: &race.id,
                driver_id: &driver.id,
                lap_number: &pitstop.lap,
                pitstop_number: &pitstop.stop,
                pittime: &pitstop.time,
                duration: &pitstop.duration.parse::<BigDecimal>().unwrap(),
            };

            println!(
                "Inserting pitstop {} {} stop{}",
                &race.id, &driver.id, &pitstop.stop
            );
            let result = diesel::insert_into(pitstops::table)
                .values(&new_pitstop)
                .returning(Pitstop::as_returning())
                .get_result(&mut connection);

            if let Err(e) = result {
                println!(
                    "Error inserting pitstop {} {} stop{}: error: {}",
                    &race.id, &driver.id, &pitstop.stop, e
                );
            }
        }
    }

    pub fn is_exist(race: &Race) -> bool {
        let results = Pitstop::get(race);
        if results.is_empty() {
            return false;
        }
        true
    }

    pub async fn generate_response(race: &Race) -> Vec<PitstopResponse> {
        if !Pitstop::is_exist(&race) {
            println!("Pitstop data is not in the database. Fetch from Ergast API.");
            // if not, fetch pitstop data from Ergast API and insert it into the database
            Pitstop::post(&race).await;
        }
        let pitstops = Pitstop::get(&race);
        let vec = pitstops
            .iter()
            .map(|pitstop| PitstopResponse {
                driver_id: pitstop.driver_id.clone(),
                lap: pitstop.lap_number,
                duration: pitstop.duration.to_f64().unwrap_or_default(),
                stop: pitstop.pitstop_number,
            })
            .collect::<Vec<PitstopResponse>>();
        vec
    }
}

#[derive(Insertable)]
#[diesel(table_name = crate::db::schema::pitstops)]
pub struct NewPitstop<'a> {
    pub race_id: &'a i32,
    pub driver_id: &'a String,
    pub lap_number: &'a i32,
    pub pitstop_number: &'a i32,
    pub pittime: &'a String,
    pub duration: &'a BigDecimal,
}
