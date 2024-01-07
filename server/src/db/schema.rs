// @generated automatically by Diesel CLI.

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

diesel::joinable!(races -> seasons (season));

diesel::allow_tables_to_appear_in_same_query!(races, seasons,);
