DROP INDEX treasuries_maturity_date_key_key;

ALTER TABLE treasury_bonds
ADD CONSTRAINT treasuries_maturity_date_key
UNIQUE (maturity_date);