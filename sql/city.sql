CREATE TABLE city (
  id SERIAL PRIMARY KEY,
  name varchar(50) UNIQUE NOT NULL,
  slug varchar(50) UNIQUE NOT NULL,
  description_long varchar(100) NOT NULL,
  description_short varchar(200) NOT NULL,
  image_link TEXT NOT NULL,
  thumbnail_link TEXT NOT NULL,
  special_offer_image_link TEXT,
  video_link TEXT,
  gallery TEXT[] NOT NULL,
  featured_city FLOAT NOT NULL,
  tags TEXT,
  created TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
  edited TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TRIGGER trigger_row_edit_update_timestamp_for_city
BEFORE UPDATE ON city
FOR EACH ROW
EXECUTE FUNCTION update_updated_on();

-- Insert for City
INSERT INTO city (name, slug, description_long, description_short, image_link, thumbnail_link, featured_city, gallery, created, edited)
VALUES ('MISSING DATA', 'missing-data', 'Long description for MISSING DATA', 'Short description for MISSING DATA', 'image_link_here', 'thumbnail_link_here', 0.0, ARRAY['gallery1', 'gallery2'], CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
       ('Dubai', 'dubai', 'Long description for Dubai', 'Short description for Dubai', 'image_link_here', 'thumbnail_link_here', 1.0, ARRAY['gallery1', 'gallery2'], CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
       ('Amsterdam', 'amsterdam', 'Long description for Amsterdam', 'Short description for Amsterdam', 'image_link_here', 'thumbnail_link_here', 1.0, ARRAY['gallery1', 'gallery2'], CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
       ('Paris', 'paris', 'Long description for Paris', 'Short description for Paris', 'image_link_here', 'thumbnail_link_here', 1.0, ARRAY['gallery1', 'gallery2'], CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
       ('Cape Town', 'cape-town', 'Long description for Cape Town', 'Short description for Cape Town', 'image_link_here', 'thumbnail_link_here', 1.0, ARRAY['gallery1', 'gallery2'], CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
       ('Aberdeen', 'aberdeen', 'Long description for Aberdeen', 'Short description for Aberdeen', 'image_link_here', 'thumbnail_link_here', 1.0, ARRAY['gallery1', 'gallery2'], CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
       ('New York City', 'new-york-city', 'Long description for New York City', 'Short description for New York City', 'image_link_here', 'thumbnail_link_here', 1.0, ARRAY['gallery1', 'gallery2'], CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
       ('Darwin', 'darwin', 'Long description for Darwin', 'Short description for Darwin', 'image_link_here', 'thumbnail_link_here', 1.0, ARRAY['gallery1', 'gallery2'], CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
       ('Melbourne', 'melbourne', 'Long description for Melbourne', 'Short description for Melbourne', 'image_link_here', 'thumbnail_link_here', 1.0, ARRAY['gallery1', 'gallery2'], CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
       ('Delhi', 'delhi', 'Long description for Delhi', 'Short description for Delhi', 'image_link_here', 'thumbnail_link_here', 1.0, ARRAY['gallery1', 'gallery2'], CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
       ('London', 'london', 'Long description for London', 'Short description for London', 'image_link_here', 'thumbnail_link_here', 1.0, ARRAY['gallery1', 'gallery2'], CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
       ('Penang', 'penang', 'Long description for Penang', 'Short description for Penang', 'image_link_here', 'thumbnail_link_here', 1.0, ARRAY['gallery1', 'gallery2'], CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
       ('Kuala Lumpur', 'kuala-lumpur', 'Long description for Kuala Lumpur', 'Short description for Kuala Lumpur', 'image_link_here', 'thumbnail_link_here', 1.0, ARRAY['gallery1', 'gallery2'], CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
       ('Hamburg', 'hamburg', 'Long description for Hamburg', 'Short description for Hamburg', 'image_link_here', 'thumbnail_link_here', 1.0, ARRAY['gallery1', 'gallery2'], CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
       ('Salt Lake City', 'salt-lake-city-a', 'Long description for Salt Lake City', 'Short description for Salt Lake City', 'image_link_here', 'thumbnail_link_here', 1.0, ARRAY['gallery1', 'gallery2'], CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
       ('Ho Chi Minh City', 'ho-chi-minh-city', 'Long description for Ho Chi Minh City', 'Short description for Ho Chi Minh City', 'image_link_here', 'thumbnail_link_here', 1.0, ARRAY['gallery1', 'gallery2'], CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
       ('Kota Bharu', 'kota-bharu', 'Long description for Kota Bharu', 'Short description for Kota Bharu', 'image_link_here', 'thumbnail_link_here', 1.0, ARRAY['gallery1', 'gallery2'], CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
       ('Kota Kinabalu', 'kota-kinabalu', 'Long description for Kota Kinabalu', 'Short description for Kota Kinabalu', 'image_link_here', 'thumbnail_link_here', 1.0, ARRAY['gallery1', 'gallery2'], CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
       ('Bangkok', 'bangkok-a', 'Long description for Bangkok', 'Short description for Bangkok', 'image_link_here', 'thumbnail_link_here', 1.0, ARRAY['gallery1', 'gallery2'], CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
       ('Kuching', 'kuching', 'Long description for Kuching', 'Short description for Kuching', 'image_link_here', 'thumbnail_link_here', 1.0, ARRAY['gallery1', 'gallery2'], CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
       ('Miri', 'miri', 'Long description for Miri', 'Short description for Miri', 'image_link_here', 'thumbnail_link_here', 1.0, ARRAY['gallery1', 'gallery2'], CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
       ('Oslo', 'oslo', 'Long description for Oslo', 'Short description for Oslo', 'image_link_here', 'thumbnail_link_here', 1.0, ARRAY['gallery1', 'gallery2'], CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
