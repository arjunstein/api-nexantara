-- Dummy data for Indonesian administrative divisions

-- Clear existing data (in case of re-running)
TRUNCATE TABLE postal_codes CASCADE;
TRUNCATE TABLE villages CASCADE;
TRUNCATE TABLE districts CASCADE;
TRUNCATE TABLE regencies CASCADE;
TRUNCATE TABLE provinces CASCADE;

-- Insert provinces
INSERT INTO provinces (id, name) VALUES
    ('11111111-1111-1111-1111-111111111111', 'DKI Jakarta'),
    ('22222222-2222-2222-2222-222222222222', 'Jawa Barat'),
    ('33333333-3333-3333-3333-333333333333', 'Jawa Tengah'),
    ('44444444-4444-4444-4444-444444444444', 'Jawa Timur'),
    ('55555555-5555-5555-5555-555555555555', 'Bali');

-- Insert regencies (kabupaten/kota)
INSERT INTO regencies (id, name, province_id) VALUES
    -- DKI Jakarta
    ('aaaa1111-1111-1111-1111-111111111111', 'Jakarta Selatan', '11111111-1111-1111-1111-111111111111'),
    ('aaaa2222-2222-2222-2222-222222222222', 'Jakarta Pusat', '11111111-1111-1111-1111-111111111111'),
    -- Jawa Barat
    ('bbbb1111-1111-1111-1111-111111111111', 'Kota Bandung', '22222222-2222-2222-2222-222222222222'),
    ('bbbb2222-2222-2222-2222-222222222222', 'Kota Bogor', '22222222-2222-2222-2222-222222222222'),
    -- Jawa Tengah
    ('cccc1111-1111-1111-1111-111111111111', 'Kota Semarang', '33333333-3333-3333-3333-333333333333'),
    -- Jawa Timur
    ('dddd1111-1111-1111-1111-111111111111', 'Kota Surabaya', '44444444-4444-4444-4444-444444444444'),
    -- Bali
    ('eeee1111-1111-1111-1111-111111111111', 'Kota Denpasar', '55555555-5555-5555-5555-555555555555');

-- Insert districts (kecamatan)
INSERT INTO districts (id, name, regency_id) VALUES
    -- Jakarta Selatan
    ('a1a1a1a1-1111-1111-1111-111111111111', 'Kebayoran Baru', 'aaaa1111-1111-1111-1111-111111111111'),
    ('a1a1a1a1-2222-2222-2222-222222222222', 'Tebet', 'aaaa1111-1111-1111-1111-111111111111'),
    -- Bandung
    ('b1b1b1b1-1111-1111-1111-111111111111', 'Coblong', 'bbbb1111-1111-1111-1111-111111111111'),
    -- Bogor
    ('c1c1c1c1-1111-1111-1111-111111111111', 'Bogor Selatan', 'bbbb2222-2222-2222-2222-222222222222'),
    -- Semarang
    ('d1d1d1d1-1111-1111-1111-111111111111', 'Semarang Tengah', 'cccc1111-1111-1111-1111-111111111111'),
    -- Surabaya
    ('e1e1e1e1-1111-1111-1111-111111111111', 'Surabaya Pusat', 'dddd1111-1111-1111-1111-111111111111'),
    -- Denpasar
    ('f1f1f1f1-1111-1111-1111-111111111111', 'Denpasar Selatan', 'eeee1111-1111-1111-1111-111111111111');

-- Insert villages (kelurahan/desa)
INSERT INTO villages (id, name, district_id) VALUES
    -- Kebayoran Baru
    ('f1f1f1f1-1111-1111-1111-111111111111', 'Selong', 'a1a1a1a1-1111-1111-1111-111111111111'),
    ('f1f1f1f1-2222-2222-2222-222222222222', 'Gunung', 'a1a1a1a1-1111-1111-1111-111111111111'),
    -- Tebet
    ('f2f2f2f2-1111-1111-1111-111111111111', 'Tebet Timur', 'a1a1a1a1-2222-2222-2222-222222222222'),
    -- Coblong
    ('f3f3f3f3-1111-1111-1111-111111111111', 'Dago', 'b1b1b1b1-1111-1111-1111-111111111111'),
    -- Bogor Selatan
    ('f4f4f4f4-1111-1111-1111-111111111111', 'Batu Tulis', 'c1c1c1c1-1111-1111-1111-111111111111'),
    -- Semarang Tengah
    ('f5f5f5f5-1111-1111-1111-111111111111', 'Sekayu', 'd1d1d1d1-1111-1111-1111-111111111111'),
    -- Surabaya Pusat
    ('f6f6f6f6-1111-1111-1111-111111111111', 'Ketabang', 'e1e1e1e1-1111-1111-1111-111111111111'),
    -- Denpasar Selatan
    ('f7f7f7f7-1111-1111-1111-111111111111', 'Sanur', 'f1f1f1f1-1111-1111-1111-111111111111');

-- Insert postal codes
INSERT INTO postal_codes (code, village_id) VALUES
    ('12110', 'f1f1f1f1-1111-1111-1111-111111111111'), -- Selong, Kebayoran Baru
    ('12120', 'f1f1f1f1-2222-2222-2222-222222222222'), -- Gunung, Kebayoran Baru
    ('12820', 'f2f2f2f2-1111-1111-1111-111111111111'), -- Tebet Timur
    ('40135', 'f3f3f3f3-1111-1111-1111-111111111111'), -- Dago, Bandung
    ('16133', 'f4f4f4f4-1111-1111-1111-111111111111'), -- Batu Tulis, Bogor
    ('50125', 'f5f5f5f5-1111-1111-1111-111111111111'), -- Sekayu, Semarang
    ('60272', 'f6f6f6f6-1111-1111-1111-111111111111'), -- Ketabang, Surabaya
    ('80228', 'f7f7f7f7-1111-1111-1111-111111111111'); -- Sanur, Denpasar
