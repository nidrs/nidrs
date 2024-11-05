// @generated automatically by Diesel CLI.

diesel::table! {
    downlogs (id) {
        id -> Int4,
        resource_id -> Int4,
        user_id -> Int4,
        status -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    resources (id) {
        id -> Int4,
        room_id -> Int4,
        name -> Varchar,
        size -> Int4,
        key -> Varchar,
        length -> Int4,
        creator_id -> Int4,
        down_count -> Int4,
        blank -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    rooms (id) {
        id -> Int4,
        name -> Varchar,
        blank -> Bool,
        creator_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        unionid -> Varchar,
        platform -> Varchar,
        openid -> Varchar,
        name -> Varchar,
        derive -> Varchar,
        out_ip -> Varchar,
        in_ip -> Varchar,
        blank -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users_extra (id) {
        id -> Int4,
        user_id -> Int4,
        first_launch_path -> Varchar,
        first_launch_scene -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    downlogs,
    resources,
    rooms,
    users,
    users_extra,
);
