table! {
    favoriteproducts (id) {
        id -> Varchar,
        userid -> Varchar,
        product -> Varchar,
        timestamp -> Timestamp,
    }
}

table! {
    favoriteshops (id) {
        id -> Varchar,
        userid -> Varchar,
        shop -> Varchar,
        timestamp -> Timestamp,
    }
}

table! {
    purchases (id) {
        id -> Varchar,
        userid -> Varchar,
        product -> Varchar,
        shop -> Varchar,
        timestamp -> Timestamp,
    }
}

table! {
    wishlist (id) {
        id -> Varchar,
        userid -> Varchar,
        product -> Varchar,
        timestamp -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    favoriteproducts,
    favoriteshops,
    purchases,
    wishlist,
);
