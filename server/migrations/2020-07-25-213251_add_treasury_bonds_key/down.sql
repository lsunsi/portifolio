ALTER TABLE treasury_bonds DROP COLUMN key RESTRICT;

ALTER TABLE assets DROP CONSTRAINT assets_kind_check;
ALTER TABLE assets ADD CONSTRAINT assets_kind_check CHECK (kind in ('treasury', 'etf'));

ALTER TABLE treasury_bonds ALTER COLUMN kind SET DEFAULT 'treasury';
ALTER TABLE treasury_bonds DROP CONSTRAINT treasuries_kind_check;
ALTER TABLE treasury_bonds ADD CONSTRAINT treasuries_kind_check CHECK (kind = 'treasury');

ALTER TABLE treasury_bonds RENAME TO treasuries;
