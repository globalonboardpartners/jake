-- formally known as "business"
CREATE TABLE partner_vendor (
  id SERIAL PRIMARY KEY,
  name varchar(50) NOT NULL,
  slug varchar(50) NOT NULL,
  description_short varchar(100) NOT NULL,
  description_long varchar(200) NOT NULL,
  video_link TEXT,
  image_link TEXT NOT NULL,
  image_link_2 TEXT NOT NULL,
  thumbnail_link TEXT NOT NULL,
  gallery TEXT[] NOT NULL,
  featured_partner_vendor FLOAT NOT NULL,
  continent integer references continent(id) NOT NULL,
  country integer references country(id) NOT NULL,
  region integer references region(id) NOT NULL,
  city integer references city(id) NOT NULL,
  latitude FLOAT NOT NULL,
  longitude FLOAT NOT NULL,
  email TEXT NOT NULL,
  phone VARCHAR(16) NOT NULL,
  address TEXT,
  website_link TEXT NOT NULL,
  tags TEXT,
  created TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
  edited TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TRIGGER trigger_row_edit_update_timestamp_for_partner_vendor
BEFORE UPDATE ON partner_vendor
FOR EACH ROW
EXECUTE FUNCTION update_updated_on();

INSERT INTO partner_vendor (
  name,
  slug,
  description_short,
  description_long,
  video_link,
  image_link,
  image_link_2,
  thumbnail_link,
  gallery,
  featured_partner_vendor,
  continent,
  country,
  region,
  city,
  latitude,
  longitude,
  email,
  phone,
  address,
  website_link,
  tags
)
VALUES (
  'MISSING DATA',
  'missing-data',
  'This is a placeholder for missing data.',
  'This entry is used to reference an entry that does not exist and is just missing data.',
  NULL,
  'https://i.imgur.com/hfM1J8s.png',
  'https://i.imgur.com/hfM1J8s.png',
  'https://i.imgur.com/hfM1J8s.png',
  ARRAY['missing', 'data'],
  0.0,
  1,
  1,
  1,
  1,
  0.0,
  0.0,
  'missing.data@example.com',
  '000-000-0000',
  NULL,
  'https://i.imgur.com/hfM1J8s.png',
  NULL
);

INSERT INTO partner_vendor (
  name,
  slug,
  description_short,
  description_long,
  video_link,
  image_link,
  image_link_2,
  thumbnail_link,
  gallery,
  featured_partner_vendor,
  continent,
  country,
  region,
  city,
  latitude,
  longitude,
  email,
  phone,
  address,
  website_link,
  tags
) VALUES (
  'VendorName',                     -- name
  'vendor-name',                    -- slug
  'This is a short description.',   -- description_short
  'This is a longer description.',  -- description_long
  'http://www.example.com/video',   -- video_link
  'http://www.example.com/image1',  -- image_link
  'http://www.example.com/image2',  -- image_link_2
  'http://www.example.com/thumb',   -- thumbnail_link
  '{"img1", "img2", "img3"}',       -- gallery
  4.5,                             -- featured_partner_vendor
  1,                               -- continent
  2,                               -- country
  3,                               -- region
  1,                               -- city
  40.7128,                         -- latitude
  -74.0060,                        -- longitude
  'email@example.com',             -- email
  '+1-123-456-7890',               -- phone
  '123 Example St, City, Country', -- address
  'http://www.example.com',        -- website_link
  'tag1,tag2,tag3'                 -- tags
);

INSERT INTO partner_vendor (
  name,
  slug,
  description_short,
  description_long,
  video_link,
  image_link,
  image_link_2,
  thumbnail_link,
  gallery,
  featured_partner_vendor,
  continent,
  country,
  region,
  city,
  latitude,
  longitude,
  email,
  phone,
  address,
  website_link,
  tags
) VALUES (
  'Chuck E. Cheese''s',                     -- name
  'chuck-e-cheeses',                        -- slug
  'Family entertainment center.',           -- description_short
  'A place where a kid can be a kid.',      -- description_long
  'http://www.example.com/chuckecheese',    -- video_link
  'http://www.example.com/image_chuck1',    -- image_link
  'http://www.example.com/image_chuck2',    -- image_link_2
  'http://www.example.com/thumb_chuck',     -- thumbnail_link
  '{"gallery1", "gallery2", "gallery3"}',   -- gallery
  4.5,                                      -- featured_partner_vendor
  1,                                        -- continent
  1,                                        -- country
  1,                                        -- region
  1,                                        -- city
  40.7128,                                  -- latitude
  -74.0060,                                 -- longitude
  'contact@chuckecheese.com',               -- email
  '+1-123-456-7890',                        -- phone
  '123 Fun St, Anytown, USA',                -- address
  'http://www.chuckecheese.com',            -- website_link
  'kids,fun,games,pizza'                    -- tags
);

INSERT INTO partner_vendor (
  name,
  slug,
  description_short,
  description_long,
  video_link,
  image_link,
  image_link_2,
  thumbnail_link,
  gallery,
  featured_partner_vendor,
  continent,
  country,
  region,
  city,
  latitude,
  longitude,
  email,
  phone,
  address,
  website_link,
  tags
) VALUES (
  'Boondocks',                                  -- name
  'boondocks',                                  -- slug
  'Outdoor adventure services',                 -- description_short
  'Boondocks offers a range of outdoor activities, from kayaking to hiking.',  -- description_long
  'http://www.boondocks.com/video',             -- video_link
  'http://www.boondocks.com/image1',            -- image_link
  'http://www.boondocks.com/image2',            -- image_link_2
  'http://www.boondocks.com/thumbnail',         -- thumbnail_link
  '{"gallery_img1", "gallery_img2", "gallery_img3"}',  -- gallery
  5.0,                                         -- featured_partner_vendor
  1,                                           -- continent (reference)
  1,                                         -- country (reference)
  1,                                         -- region (reference)
  1,                                         -- city (reference)
  40.7128,                                     -- latitude
  -74.0060,                                    -- longitude
  'contact@boondocks.com',                     -- email
  '+1-123-456-7890',                           -- phone
  '123 Boondocks St, Adventure City, USA',     -- address
  'http://www.boondocks.com',                  -- website_link
  'outdoor,adventure,kayaking'                 -- tags
);
