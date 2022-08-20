table! {
    player (id) {
        id -> Int4,
        name -> Text,
        race_id -> Int4,
        alignment_id -> Int4,
        background_id -> Int4,
        class_id -> Int4,
        user_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    user (id) {
        id -> Int4,
        name -> Text,
        password -> Text,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

allow_tables_to_appear_in_same_query!(
    player,
    user,
);
