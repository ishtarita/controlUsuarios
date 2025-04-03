// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        creationdate -> Nullable<Date>,
        email -> Varchar,
        password -> Varchar,
    }
}
