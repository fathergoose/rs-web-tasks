// @generated automatically by Diesel CLI.

diesel::table! {
    things (id) {
        id -> Int4,
        title -> Varchar,
        details -> Nullable<Text>,
        active -> Bool,
    }
}
