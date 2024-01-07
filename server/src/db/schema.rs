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

diesel::joinable!(races -> seasons (season));
diesel::joinable!(standings -> constructors (constructor_id));
diesel::joinable!(standings -> drivers (driver_id));
diesel::joinable!(standings -> races (race));

diesel::allow_tables_to_appear_in_same_query!(constructors, drivers, races, seasons, standings,);
