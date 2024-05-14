// @generated automatically by Diesel CLI.

diesel::table! {
    placemarks (id) {
        id -> Int4,
        post_id -> Int4,
        lat -> Float8,
        lon -> Float8,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        user_id -> Int4,
        img_url -> Nullable<Varchar>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
    }
}

diesel::joinable!(placemarks -> posts (post_id));
diesel::joinable!(posts -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    placemarks,
    posts,
    users,
);
