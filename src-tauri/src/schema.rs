diesel::table! {
    histories (id) {
        id -> Integer,
        url -> Text,
        body -> Text,
        headers -> Text,
        created_at -> Date
    }
}