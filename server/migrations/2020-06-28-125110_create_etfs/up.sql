ALTER TABLE assets DROP CONSTRAINT assets_kind_check;
ALTER TABLE assets ADD CONSTRAINT assets_kind_check CHECK (kind in ('treasury', 'etf'));

CREATE TABLE etfs (
	id int PRIMARY KEY,
	kind TEXT NOT NULL DEFAULT 'etf' CHECK (kind = 'etf'),
	ticker TEXT NOT NULL,
	FOREIGN KEY (id, kind) REFERENCES assets (id, kind),
	UNIQUE(ticker)
);
