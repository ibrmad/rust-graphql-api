table! {
    cars (id) {
        id -> Int4,
        name -> Varchar,
        maker -> Varchar,
        model -> Varchar,
        make_year -> Int4,
        comfort_level -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    cars_users (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        car_id -> Nullable<Int4>,
        car_plate_number -> Varchar,
        car_color -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    users (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        phone_number -> Varchar,
        verification_code -> Nullable<Int4>,
        is_phone_verified -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

joinable!(cars_users -> cars (car_id));
joinable!(cars_users -> users (user_id));

allow_tables_to_appear_in_same_query!(
    cars,
    cars_users,
    users,
);
