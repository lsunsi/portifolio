ALTER TABLE treasuries RENAME TO treasury_bonds;

ALTER TABLE treasury_bonds ALTER COLUMN kind SET DEFAULT 'treasury_bond';
ALTER TABLE treasury_bonds DROP CONSTRAINT treasuries_kind_check;
ALTER TABLE treasury_bonds ADD CONSTRAINT treasuries_kind_check CHECK (kind = 'treasury_bond');

ALTER TABLE assets DROP CONSTRAINT assets_kind_check;
ALTER TABLE assets ADD CONSTRAINT assets_kind_check CHECK (kind in ('treasury_bond', 'etf'));

ALTER TABLE treasury_bonds
ADD COLUMN key TEXT NOT NULL CHECK (key in ('LFT', 'LTN', 'NTNB'));
