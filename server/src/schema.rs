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
    etfs (id) {
        id -> Int4,
        kind -> Text,
        ticker -> Text,
    }
}

table! {
    portfolios (id) {
        id -> Int4,
    }
}

table! {
    trades (id) {
        id -> Int4,
        portfolio_id -> Int4,
        asset_id -> Int4,
        date -> Date,
        quantity -> Numeric,
        price -> Numeric,
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
joinable!(trades -> assets (asset_id));
joinable!(trades -> portfolios (portfolio_id));

allow_tables_to_appear_in_same_query!(
    asset_prices,
    assets,
    etfs,
    portfolios,
    trades,
    treasuries,
);
