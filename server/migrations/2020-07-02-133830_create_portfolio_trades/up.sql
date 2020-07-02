CREATE TABLE portfolios (id SERIAL PRIMARY KEY);

CREATE TABLE trades (
	id SERIAL PRIMARY KEY,
	portfolio_id INT NOT NULL,
	asset_id INT NOT NULL,
	date DATE NOT NULL,
	quantity DECIMAL NOT NULL CHECK (quantity != 0),
	price DECIMAL NOT NULL CHECK (price > 0),
	FOREIGN KEY (asset_id) REFERENCES assets,
	FOREIGN KEY (portfolio_id) REFERENCES portfolios,
	FOREIGN KEY (asset_id, date) REFERENCES asset_prices(asset_id, date)
);
