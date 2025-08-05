-- Add migration script here
-- Drop existing tables in reverse order of dependencies
DROP TABLE IF EXISTS villages CASCADE;
DROP TABLE IF EXISTS districts CASCADE;
DROP TABLE IF EXISTS regencies CASCADE;
DROP TABLE IF EXISTS provinces CASCADE;

-- Create function to update updated_at timestamp (needs to be created before triggers)
CREATE OR REPLACE FUNCTION update_modified_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create provinces table with updated column name
CREATE TABLE IF NOT EXISTS provinces (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    province_name VARCHAR(100) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create triggers for provinces
CREATE TRIGGER update_provinces_modtime
    BEFORE UPDATE ON provinces
    FOR EACH ROW
    EXECUTE FUNCTION update_modified_column();

-- Create regencies table with updated column names
CREATE TABLE IF NOT EXISTS regencies (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    regency_name VARCHAR(100) NOT NULL,
    province_id UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT fk_province FOREIGN KEY (province_id) REFERENCES provinces(id) ON DELETE CASCADE
);

-- Create trigger for regencies
CREATE TRIGGER update_regencies_modtime
    BEFORE UPDATE ON regencies
    FOR EACH ROW
    EXECUTE FUNCTION update_modified_column();

-- Create districts table with updated column names
CREATE TABLE IF NOT EXISTS districts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    district_name VARCHAR(100) NOT NULL,
    regency_id UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT fk_regency FOREIGN KEY (regency_id) REFERENCES regencies(id) ON DELETE CASCADE
);

-- Create trigger for districts
CREATE TRIGGER update_districts_modtime
    BEFORE UPDATE ON districts
    FOR EACH ROW
    EXECUTE FUNCTION update_modified_column();

-- Create villages table with updated column names and added postal_code
CREATE TABLE IF NOT EXISTS villages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    village_name VARCHAR(100) NOT NULL,
    postal_code VARCHAR(10) NOT NULL,
    district_id UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT fk_district FOREIGN KEY (district_id) REFERENCES districts(id) ON DELETE CASCADE
);

-- Create trigger for villages
CREATE TRIGGER update_villages_modtime
    BEFORE UPDATE ON villages
    FOR EACH ROW
    EXECUTE FUNCTION update_modified_column();

-- Create indexes for performance (after all tables are created)
CREATE INDEX IF NOT EXISTS idx_provinces_name ON provinces(province_name);
CREATE INDEX IF NOT EXISTS idx_regencies_name ON regencies(regency_name);
CREATE INDEX IF NOT EXISTS idx_regencies_province_id ON regencies(province_id);
CREATE INDEX IF NOT EXISTS idx_districts_name ON districts(district_name);
CREATE INDEX IF NOT EXISTS idx_districts_regency_id ON districts(regency_id);
CREATE INDEX IF NOT EXISTS idx_villages_name ON villages(village_name);
CREATE INDEX IF NOT EXISTS idx_villages_postal_code ON villages(postal_code);
CREATE INDEX IF NOT EXISTS idx_villages_district_id ON villages(district_id);
