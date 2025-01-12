// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Unsigned<Integer>,
        #[max_length = 255]
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
