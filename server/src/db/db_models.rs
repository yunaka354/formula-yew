use crate::{
    db::connection::PooledConnection,
    models::{
        ChartResponse, ConstructorResponse, DriverResponse, LapLineChartData, PitstopResponse,
        RaceResponse, RaceResultResponse, SeasonResponse,
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
    collections::{HashMap, HashSet},
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
    pub async fn post(conn: &mut PooledConnection) {
        use crate::db::schema::seasons;

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
                .get_result(conn);

            if let Err(e) = result {
                println!("Error inserting season {}: {}", season.season, e);
            }
        }
    }

    pub fn get(season: i32, conn: &mut PooledConnection) -> Season {
        use crate::db::schema::seasons;
        seasons::table
            .filter(seasons::season.eq(season))
            .first::<Season>(conn)
            .expect("loading error")
    }

    pub fn get_by_id(id: i32, conn: &mut PooledConnection) -> Season {
        use crate::db::schema::seasons;
        seasons::table
            .filter(seasons::id.eq(id))
            .first::<Season>(conn)
            .expect("loading error")
    }

    pub fn is_exist(conn: &mut PooledConnection) -> bool {
        use crate::db::schema::seasons::dsl::*;
        let results = seasons.load::<Season>(conn);

        match results {
            Ok(s) => !s.is_empty(),
            Err(e) => {
                println!("Error loading seasons: {}", e);
                false
            }
        }
    }

    pub async fn generate_response(
        conn: &mut PooledConnection,
    ) -> Result<Vec<SeasonResponse>, Error> {
        use crate::db::schema::seasons::dsl::*;
        // check if season data is already in the database
        if !Season::is_exist(conn) {
            println!("Season data is not in the database. Fetch from Ergast API.");
            // if not, fetch season data from Ergast API and insert it into the database
            Season::post(conn).await;
        }

        let results = seasons.load::<Season>(conn);

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
    pub async fn post(season: &Season, conn: &mut PooledConnection) -> () {
        use crate::db::schema::races;

        let params = URLParams {
            limit: 100,
            offset: 0,
        };
        let response = Ergast::race(season.season, params)
            .await
            .expect("failed to fetch races");

        for race in response.table.races {
            let season = Season::get(race.season, conn);
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
                .get_result(conn);

            if let Err(e) = result {
                println!("Error inserting race {} {}: {}", race.season, race.round, e);
            }
        }
    }

    pub fn get_races_in_season(season: &Season, conn: &mut PooledConnection) -> Vec<Race> {
        use crate::db::schema::races;
        races::table
            .filter(races::season.eq(season.id))
            .load::<Race>(conn)
            .expect("loading error")
            .into_iter()
            .collect::<Vec<Race>>()
    }

    pub fn get(season: &Season, round: i32, conn: &mut PooledConnection) -> Option<Race> {
        use crate::db::schema::races;
        let result = races::table
            .filter(races::season.eq(season.id).and(races::round.eq(round)))
            .load::<Race>(conn);
        result.ok().and_then(|mut v| v.pop())
    }

    pub fn is_exist(season: &Season, conn: &mut PooledConnection) -> bool {
        let results = Race::get_races_in_season(&season, conn);
        if results.is_empty() {
            return false;
        }
        true
    }

    pub async fn generate_response(
        season: &Season,
        conn: &mut PooledConnection,
    ) -> Vec<RaceResponse> {
        // check if race data is already in the database
        if !Race::is_exist(&season, conn) {
            println!("Race data is not in the database. Fetch from Ergast API.");
            // if not, fetch race data from Ergast API and insert it into the database
            Race::post(&season, conn).await;
        }

        let results = Race::get_races_in_season(season, conn);

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
    pub fn get(conn: &mut PooledConnection) -> Vec<Driver> {
        use crate::db::schema::drivers;
        drivers::table
            .load::<Driver>(conn)
            .expect("loading error")
            .into_iter()
            .collect::<Vec<Driver>>()
    }

    pub fn get_by_id(id: &str, conn: &mut PooledConnection) -> Driver {
        use crate::db::schema::drivers;
        drivers::table
            .filter(drivers::id.eq(id))
            .first::<Driver>(conn)
            .expect("loading error")
    }

    pub async fn post(conn: &mut PooledConnection) -> () {
        use crate::db::schema::drivers;

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
                .get_result(conn);

            if let Err(e) = result {
                println!("Error inserting driver {}: {}", driver.driver_id, e);
            }
        }
    }

    pub fn is_exist(conn: &mut PooledConnection) -> bool {
        let results = Driver::get(conn);
        if results.is_empty() {
            return false;
        }
        true
    }

    pub fn generate_response(conn: &mut PooledConnection) -> Vec<DriverResponse> {
        use crate::db::schema::drivers::dsl::*;

        let results = drivers.load::<Driver>(conn);

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
    pub fn get(conn: &mut PooledConnection) -> Vec<Constructor> {
        use crate::db::schema::constructors;
        constructors::table
            .load::<Constructor>(conn)
            .expect("loading error")
            .into_iter()
            .collect::<Vec<Constructor>>()
    }

    pub fn get_by_id(id: &str, conn: &mut PooledConnection) -> Constructor {
        use crate::db::schema::constructors;
        constructors::table
            .filter(constructors::id.eq(id))
            .first::<Constructor>(conn)
            .expect("loading error")
    }

    pub async fn post(conn: &mut PooledConnection) -> () {
        use crate::db::schema::constructors;

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
                .get_result(conn);

            if let Err(e) = result {
                println!(
                    "Error inserting constructor {}: {}",
                    constructor.constructor_id, e
                );
            }
        }
    }

    pub fn is_exist(conn: &mut PooledConnection) -> bool {
        let results = Constructor::get(conn);
        if results.is_empty() {
            return false;
        }
        true
    }

    pub fn generate_response(conn: &mut PooledConnection) -> Vec<ConstructorResponse> {
        use crate::db::schema::constructors::dsl::*;

        let results = constructors.load::<Constructor>(conn);

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

    // return color hex code for the team
    pub fn team_color(&self) -> &str {
        match self.id.as_str() {
            "mercedes" => "#00D2BE",
            "ferrari" => "#DC0000",
            "red_bull" => "#0600EF",
            "mclaren" => "#FF8700",
            "alpine" => "#0090FF",
            "alphatauri" => "#2B4562",
            "alfa" => "#900000",
            "haas" => "#252525",
            "williams" => "#005AFF",
            "aston_martin" => "#006F62",
            _ => "#000000",
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
    pub fn get(race: &Race, conn: &mut PgConnection) -> Vec<Standing> {
        use crate::db::schema::standings;
        standings::table
            .filter(standings::race.eq(race.id))
            .load::<Standing>(conn)
            .expect("loading error")
            .into_iter()
            .collect::<Vec<Standing>>()
    }

    pub async fn post(race: &Race, conn: &mut PooledConnection) -> () {
        use crate::db::schema::standings;

        let season = Season::get_by_id(race.season, conn);
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
            let driver = Driver::get(conn)
                .into_iter()
                .find(|d| d.id == standing.driver.driver_id)
                .unwrap();
            let constructor = Constructor::get(conn)
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
                .get_result(conn);

            if let Err(e) = result {
                println!(
                    "Error inserting standing {} {}: {}",
                    race.season, race.round, e
                );
            }
        }
    }

    pub fn is_exist(race: &Race, conn: &mut PooledConnection) -> bool {
        let results = Standing::get(race, conn);
        if results.is_empty() {
            return false;
        }
        true
    }

    pub async fn generate_response(
        race: &Race,
        conn: &mut PooledConnection,
    ) -> ChartResponse<String, i32> {
        if !Standing::is_exist(&race, conn) {
            println!("Standing data is not in the database. Fetch from Ergast API.");
            // if not, fetch standing data from Ergast API and insert it into the database
            Standing::post(&race, conn).await;
        }

        let results = Standing::get(race, conn);
        let mut x = Vec::new();
        let mut y = Vec::new();
        let mut color = Vec::new();

        for entity in results {
            let driver = Driver::get_by_id(&entity.driver_id, conn);
            let constructor = Constructor::get_by_id(&entity.constructor_id, conn);
            x.push(driver.code.unwrap_or("NA".to_string()));
            y.push(entity.points);
            color.push(constructor.team_color().to_string());
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
    pub fn get(race: &Race, conn: &mut PooledConnection) -> Vec<(Laptime, Option<Pitstop>, Driver, RaceResult)> {
        use crate::db::schema::laptimes;
        use crate::db::schema::pitstops;
        use crate::db::schema::drivers;
        use crate::db::schema::race_results;
        laptimes::table
            // join to laptime table
            .left_join(pitstops::table
                .on(
                    laptimes::race_id.eq(pitstops::race_id)
                    .and(laptimes::driver_id.eq(pitstops::driver_id))
                    .and(laptimes::lap_number.eq(pitstops::lap_number))
                )
            )
            // join to driver table
            .inner_join(drivers::table.on(laptimes::driver_id.eq(drivers::id)))
            // join to race results table
            .inner_join(race_results::table.on(laptimes::race_id.eq(race_results::race_id).and(laptimes::driver_id.eq(race_results::driver_id))))
            .filter(laptimes::race_id.eq(race.id))
            .load::<(Laptime, Option<Pitstop>, Driver, RaceResult)>(conn)
            .expect("loading error")
            .into_iter()
            .collect::<Vec<(Laptime, Option<Pitstop>, Driver, RaceResult)>>()
    }

    pub async fn post(race: &Race, conn: &mut PooledConnection) -> () {
        use crate::db::schema::laptimes;

        let season = Season::get_by_id(race.season, conn);
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
                let driver = Driver::get_by_id(&timing.driver_id, conn);
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
                    .get_result(conn);

                if let Err(e) = result {
                    println!(
                        "Error inserting laptime {} {} lap:{}: {}",
                        &race.id, &driver.id, &lap_number, e
                    );
                }
            }
        }
    }

    pub fn is_exist(race: &Race, conn: &mut PooledConnection) -> bool {
        let results = Laptime::get(race, conn);
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

    pub async fn generate_response(
        race: &Race,
        exclude_pitstop: bool,
        conn: &mut PooledConnection,
    ) -> Vec<LapLineChartData> {
        if !Laptime::is_exist(&race, conn) {
            println!("Laptime data is not in the database. Fetch from Ergast API.");
            // if not, fetch laptime data from Ergast API and insert it into the database
            Laptime::post(&race, conn).await;
        }
        if !Pitstop::is_exist(&race, conn) {
            println!("Pitstop data is not in the database. Fetch from Ergast API.");
            // if not, fetch pitstop data from Ergast API and insert it into the database
            Pitstop::post(&race, conn).await;
        }

        let laps = Laptime::get(&race, conn);
        let mut map = HashMap::new();
        let mut pitstop_set = HashSet::new(); // set to chech if the driver pitted in the lap

        
        for (_laptime, pitstop, driver, _race_result) in &laps {
            if let Some(pit) = pitstop {
                pitstop_set.insert((driver.id.clone(), pit.lap_number - 1)); // add inlap.
                pitstop_set.insert((driver.id.clone(), pit.lap_number));
                pitstop_set.insert((driver.id.clone(), pit.lap_number + 1)); // add outlap.
            }
        }

        for (laptime, _pitstop, driver, race_result) in laps {
            let time = Laptime::convert_lap_time_text_to_f64(&laptime.lap_time);

            // skip if exclude_pitstop is true and the driver pitted in the lap
            if exclude_pitstop && pitstop_set.contains(&(driver.id.clone(), laptime.lap_number)) {
                continue;
            }

            // get reference to the entry in the map
            let entry = map
                .entry(driver.id.clone())
                .or_insert(LapLineChartData::new(driver.id, race_result, conn));

            // push lap number into x axis
            entry.laps.push(laptime.lap_number);

            // push lap time into y axis
            entry.laptime.push(time.unwrap());
        }

        // convert map to vector
        let mut vec = map
            .into_iter()
            .map(|(_, v)| v)
            .collect::<Vec<LapLineChartData>>();

        // vec.sort_by(|a, b| a.driver_id.partial_cmp(&b.driver_id).unwrap());
        vec.sort_by(|a, b| a.position.partial_cmp(&b.position).unwrap());
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
    pub fn get(race: &Race, conn: &mut PooledConnection) -> Vec<Pitstop> {
        use crate::db::schema::pitstops;
        pitstops::table
            .filter(pitstops::race_id.eq(race.id))
            .load::<Pitstop>(conn)
            .expect("loading error")
            .into_iter()
            .collect::<Vec<Pitstop>>()
    }

    pub async fn post(race: &Race, conn: &mut PooledConnection) -> () {
        use crate::db::schema::pitstops;

        let season = Season::get_by_id(race.season, conn);
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
            let duration = match pitstop.duration.parse::<BigDecimal>() {
                Ok(d) => d,
                Err(_) => {
                    // if duration is not available, set it to 0
                    BigDecimal::from(0)
                }
            };
            let driver = Driver::get_by_id(&pitstop.driver_id, conn);
            let new_pitstop = NewPitstop {
                race_id: &race.id,
                driver_id: &driver.id,
                lap_number: &pitstop.lap,
                pitstop_number: &pitstop.stop,
                pittime: &pitstop.time,
                duration: &duration,
            };

            println!(
                "Inserting pitstop {} {} stop{}",
                &race.id, &driver.id, &pitstop.stop
            );
            let result = diesel::insert_into(pitstops::table)
                .values(&new_pitstop)
                .returning(Pitstop::as_returning())
                .get_result(conn);

            if let Err(e) = result {
                println!(
                    "Error inserting pitstop {} {} stop{}: error: {}",
                    &race.id, &driver.id, &pitstop.stop, e
                );
            }
        }
    }

    pub fn is_exist(race: &Race, conn: &mut PooledConnection) -> bool {
        let results = Pitstop::get(race, conn);
        if results.is_empty() {
            return false;
        }
        true
    }

    pub async fn generate_response(
        race: &Race,
        conn: &mut PooledConnection,
    ) -> Vec<PitstopResponse> {
        if !Pitstop::is_exist(&race, conn) {
            println!("Pitstop data is not in the database. Fetch from Ergast API.");
            // if not, fetch pitstop data from Ergast API and insert it into the database
            Pitstop::post(&race, conn).await;
        }
        let pitstops = Pitstop::get(&race, conn);
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

#[derive(Queryable, Selectable, Debug, Serialize)]
#[diesel(table_name = crate::db::schema::race_results)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RaceResult {
    pub id: i32,
    pub race_id: i32,
    pub driver_id: String,
    pub constructor_id: String,
    pub position: i32,
    pub position_text: String,
    pub grid: i32,
    pub laps: i32,
    pub status: String,
    pub points: BigDecimal,
    pub created_at: SystemTime,
}

impl RaceResult {
    pub fn get(race: &Race, conn: &mut PooledConnection) -> Vec<RaceResult> {
        use crate::db::schema::race_results;
        race_results::table
            .filter(race_results::race_id.eq(race.id))
            .load::<RaceResult>(conn)
            .expect("loading error")
            .into_iter()
            .collect::<Vec<RaceResult>>()
    }

    pub async fn post(race: &Race, conn: &mut PooledConnection) -> () {
        use crate::db::schema::race_results;

        let season = Season::get_by_id(race.season, conn);
        let path = Path {
            year: season.season,
            round: Some(race.round),
        };

        let params = URLParams {
            limit: 2000,
            offset: 0,
        };
        let response = Ergast::results(path, params)
            .await
            .expect("failed to fetch results");

        let results = &response.table.races.get(0).unwrap().results;

        let results = match results {
            Some(l) => l,
            None => {
                println!("No race result data");
                return;
            }
        };

        for result in results {
            let driver = Driver::get_by_id(&result.driver.driver_id, conn);
            let contructor = Constructor::get_by_id(&result.constructor.constructor_id, conn);
            let new_race_result = NewRaceResult {
                race_id: &race.id,
                driver_id: &driver.id,
                constructor_id: &contructor.id,
                position: &result.position,
                position_text: &result.position_text,
                grid: &result.grid,
                laps: &result.laps,
                status: &result.status,
                points: &result.points.to_string().parse::<BigDecimal>().unwrap(),
            };

            println!("Inserting race_result {} {}", &race.id, &driver.id,);
            let result = diesel::insert_into(race_results::table)
                .values(&new_race_result)
                .returning(RaceResult::as_returning())
                .get_result(conn);

            if let Err(e) = result {
                println!(
                    "Error inserting race_result {} {}: error: {}",
                    &race.id, &driver.id, e
                );
            }
        }
    }

    pub fn is_exist(race: &Race, conn: &mut PooledConnection) -> bool {
        let results = RaceResult::get(race, conn);
        if results.is_empty() {
            return false;
        }
        true
    }

    pub async fn generate_response(
        race: &Race,
        conn: &mut PooledConnection,
    ) -> Vec<RaceResultResponse> {
        if !RaceResult::is_exist(&race, conn) {
            println!("RaceResult data is not in the database. Fetch from Ergast API.");
            // if not, fetch RaceResult data from Ergast API and insert it into the database
            RaceResult::post(&race, conn).await;
        }
        let race_results = RaceResult::get(&race, conn);
        let mut vec = Vec::new();
        for race_result in race_results {
            let driver = Driver::get_by_id(&race_result.driver_id, conn);
            let constructor = Constructor::get_by_id(&race_result.constructor_id, conn);
            let r = RaceResultResponse {
                id: race_result.id,
                position: race_result.position,
                position_text: race_result.position_text,
                code: driver.code.unwrap_or("NA".to_string()),
                given_name: driver.given_name,
                family_name: driver.family_name,
                points: race_result.points.to_f64().unwrap_or_default() as f32,
                status: race_result.status,
                constructor: constructor.name,
            };
            vec.push(r);
        }
        vec
    }
}

#[derive(Insertable)]
#[diesel(table_name = crate::db::schema::race_results)]
pub struct NewRaceResult<'a> {
    pub race_id: &'a i32,
    pub driver_id: &'a String,
    pub constructor_id: &'a String,
    pub position: &'a i32,
    pub position_text: &'a String,
    pub grid: &'a i32,
    pub laps: &'a i32,
    pub status: &'a String,
    pub points: &'a BigDecimal,
}
