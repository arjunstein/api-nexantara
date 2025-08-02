-- Enable UUID extension if not already enabled
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- Create provinces table
CREATE TABLE IF NOT EXISTS provinces (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create regencies table
CREATE TABLE IF NOT EXISTS regencies (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL,
    province_id UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT fk_province FOREIGN KEY (province_id) REFERENCES provinces(id) ON DELETE CASCADE
);

-- Create districts table
CREATE TABLE IF NOT EXISTS districts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL,
    regency_id UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT fk_regency FOREIGN KEY (regency_id) REFERENCES regencies(id) ON DELETE CASCADE
);

-- Create villages table
CREATE TABLE IF NOT EXISTS villages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL,
    district_id UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT fk_district FOREIGN KEY (district_id) REFERENCES districts(id) ON DELETE CASCADE
);

-- Create postal_codes table
CREATE TABLE IF NOT EXISTS postal_codes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    code VARCHAR(10) NOT NULL,
    village_id UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT fk_village FOREIGN KEY (village_id) REFERENCES villages(id) ON DELETE CASCADE
);

-- Create indexes for better query performance

-- Indexes for provinces
CREATE INDEX IF NOT EXISTS idx_provinces_name ON provinces(name);
CREATE INDEX IF NOT EXISTS idx_provinces_created_at ON provinces(created_at);

-- Indexes for regencies
CREATE INDEX IF NOT EXISTS idx_regencies_province_id ON regencies(province_id);
CREATE INDEX IF NOT EXISTS idx_regencies_name ON regencies(name);
CREATE INDEX IF NOT EXISTS idx_regencies_created_at ON regencies(created_at);

-- Indexes for districts
CREATE INDEX IF NOT EXISTS idx_districts_regency_id ON districts(regency_id);
CREATE INDEX IF NOT EXISTS idx_districts_name ON districts(name);
CREATE INDEX IF NOT EXISTS idx_districts_created_at ON districts(created_at);

-- Indexes for villages
CREATE INDEX IF NOT EXISTS idx_villages_district_id ON villages(district_id);
CREATE INDEX IF NOT EXISTS idx_villages_name ON villages(name);
CREATE INDEX IF NOT EXISTS idx_villages_created_at ON villages(created_at);

-- Indexes for postal_codes
CREATE INDEX IF NOT EXISTS idx_postal_codes_code ON postal_codes(code);
CREATE INDEX IF NOT EXISTS idx_postal_codes_village_id ON postal_codes(village_id);
CREATE INDEX IF NOT EXISTS idx_postal_codes_created_at ON postal_codes(created_at);

-- Add unique constraints
ALTER TABLE postal_codes ADD CONSTRAINT uq_postal_code UNIQUE (code);

-- Create function to update updated_at timestamp
CREATE OR REPLACE FUNCTION update_modified_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create triggers to update updated_at automatically
CREATE TRIGGER update_provinces_modtime
BEFORE UPDATE ON provinces
FOR EACH ROW EXECUTE FUNCTION update_modified_column();

CREATE TRIGGER update_regencies_modtime
BEFORE UPDATE ON regencies
FOR EACH ROW EXECUTE FUNCTION update_modified_column();

CREATE TRIGGER update_districts_modtime
BEFORE UPDATE ON districts
FOR EACH ROW EXECUTE FUNCTION update_modified_column();

CREATE TRIGGER update_villages_modtime
BEFORE UPDATE ON villages
FOR EACH ROW EXECUTE FUNCTION update_modified_column();

CREATE TRIGGER update_postal_codes_modtime
BEFORE UPDATE ON postal_codes
FOR EACH ROW EXECUTE FUNCTION update_modified_column();

-- Add comment for each table
COMMENT ON TABLE provinces IS 'Master table for storing province data';
COMMENT ON TABLE regencies IS 'Master table for storing regency/city data, child of provinces';
COMMENT ON TABLE districts IS 'Master table for storing district data, child of regencies';
COMMENT ON TABLE villages IS 'Master table for storing village data, child of districts';
COMMENT ON TABLE postal_codes IS 'Master table for storing postal code data, child of villages';
