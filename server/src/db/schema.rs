// @generated automatically by Diesel CLI.

diesel::table! {
    constructors (id) {
        id -> Text,
        url -> Text,
        name -> Text,
        nationality -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    drivers (id) {
        id -> Text,
        permanent_number -> Nullable<Int4>,
        code -> Nullable<Text>,
        given_name -> Text,
        family_name -> Text,
        date_of_birth -> Date,
        nationality -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    laptimes (id) {
        id -> Int4,
        race_id -> Int4,
        driver_id -> Text,
        lap_number -> Int4,
        lap_time -> Text,
        position -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    pitstops (id) {
        id -> Int4,
        race_id -> Int4,
        driver_id -> Text,
        lap_number -> Int4,
        pitstop_number -> Int4,
        pittime -> Text,
        duration -> Numeric,
        created_at -> Timestamp,
    }
}

diesel::table! {
    race_results (id) {
        id -> Int4,
        race_id -> Int4,
        driver_id -> Text,
        constructor_id -> Text,
        position -> Int4,
        position_text -> Text,
        grid -> Int4,
        laps -> Int4,
        status -> Text,
        points -> Numeric,
        created_at -> Timestamp,
    }
}

diesel::table! {
    races (id) {
        id -> Int4,
        season -> Int4,
        round -> Int4,
        url -> Text,
        race_name -> Text,
        event_time -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    seasons (id) {
        id -> Int4,
        season -> Int4,
        url -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    standings (id) {
        id -> Int4,
        race -> Int4,
        driver_id -> Text,
        constructor_id -> Text,
        position -> Int4,
        position_text -> Text,
        points -> Int4,
        wins -> Int4,
        created_at -> Timestamp,
    }
}

diesel::joinable!(laptimes -> drivers (driver_id));
diesel::joinable!(laptimes -> races (race_id));
diesel::joinable!(pitstops -> drivers (driver_id));
diesel::joinable!(pitstops -> races (race_id));
diesel::joinable!(race_results -> constructors (constructor_id));
diesel::joinable!(race_results -> drivers (driver_id));
diesel::joinable!(race_results -> races (race_id));
diesel::joinable!(races -> seasons (season));
diesel::joinable!(standings -> constructors (constructor_id));
diesel::joinable!(standings -> drivers (driver_id));
diesel::joinable!(standings -> races (race));

diesel::allow_tables_to_appear_in_same_query!(
    constructors,
    drivers,
    laptimes,
    pitstops,
    race_results,
    races,
    seasons,
    standings,
);
