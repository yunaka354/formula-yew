// @generated automatically by Diesel CLI.

diesel::table! {
    seasons (id) {
        id -> Int4,
        season -> Int4,
        url -> Text,
        created_at -> Timestamp,
    }
}
