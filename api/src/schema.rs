table! {
    player (player_id) {
        player_id -> Uuid,
        player_name -> Text,
        player_race_id -> Int4,
        player_alignment_id -> Int4,
        background_id -> Int4,
        class_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}
