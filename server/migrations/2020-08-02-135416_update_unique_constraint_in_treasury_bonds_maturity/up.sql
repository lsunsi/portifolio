ALTER TABLE treasury_bonds
DROP CONSTRAINT treasuries_maturity_date_key;

CREATE UNIQUE INDEX treasuries_maturity_date_key_key
ON treasury_bonds (key, maturity_date);
