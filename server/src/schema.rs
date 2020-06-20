table! {
    asset_prices (id) {
        id -> Int4,
        asset_id -> Int4,
        price -> Numeric,
        date -> Date,
    }
}

table! {
    assets (id) {
        id -> Int4,
        kind -> Text,
    }
}

table! {
    treasuries (id) {
        id -> Int4,
        kind -> Text,
        maturity_date -> Date,
    }
}

joinable!(asset_prices -> assets (asset_id));

allow_tables_to_appear_in_same_query!(
    asset_prices,
    assets,
    treasuries,
);
