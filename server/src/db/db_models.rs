use diesel::{prelude::*, result::Error};
use std::time::SystemTime;
use ergast_rust::{ergast::Ergast, api::URLParams};
use crate::{db::connection::establish_connection, models::SeasonResponse};

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
        let response = Ergast::seasons(params).await.expect("failed to fetch seasons"); 

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
                let v = s.iter()
                .map(|s| SeasonResponse {
                    season: s.season,
                    url: s.url.clone(),
                })
                .collect();
                Ok(v)
            },
            Err(e) => {
                println!("Error loading seasons: {}", e);
                Err(e)
            }
        }
    }
}