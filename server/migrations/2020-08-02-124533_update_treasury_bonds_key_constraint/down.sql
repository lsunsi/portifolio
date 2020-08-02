ALTER TABLE treasury_bonds
DROP CONSTRAINT treasury_bonds_key_check;

ALTER TABLE treasury_bonds
ADD CONSTRAINT treasury_bonds_key_check
CHECK (key in ('LFT', 'LTN', 'NTNB'));