diesel::table! {
    histories (id) {
        id -> Integer,
        url -> Text,
        body -> Text,
        method -> Text,
        headers -> Text,
        created_at -> Date
    }
}