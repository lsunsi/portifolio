CREATE TABLE assets (
	id SERIAL PRIMARY KEY,
	kind TEXT NOT NULL CHECK (kind in ('treasury')),
	UNIQUE (id, kind)
);

CREATE TABLE asset_prices (
	id SERIAL PRIMARY KEY,
	asset_id INTEGER NOT NULL REFERENCES assets,
	price DECIMAL NOT NULL CHECK (price > 0),
	date DATE NOT NULL,
	UNIQUE (asset_id, date)
);

CREATE TABLE treasuries (
	id int PRIMARY KEY,
	kind TEXT NOT NULL DEFAULT 'treasury' CHECK (kind = 'treasury'),
	maturity_date DATE NOT NULL,
	FOREIGN KEY (id, kind) REFERENCES assets (id, kind),
	UNIQUE(maturity_date)
);
