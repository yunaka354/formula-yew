use crate::{
    db::connection::establish_connection,
    models::{RaceResponse, SeasonResponse},
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
