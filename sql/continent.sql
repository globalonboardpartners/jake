-- formally known as "Region"
CREATE TABLE continent (
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
  tags TEXT,
  created TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
  edited TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TRIGGER trigger_row_edit_update_timestamp_for_continent
BEFORE UPDATE ON continent
FOR EACH ROW
EXECUTE FUNCTION update_updated_on();

-- -- Insert for Continent
-- INSERT INTO continent (name, slug, description_long, description_short, image_link, thumbnail_link, gallery, created, edited)
-- VALUES ('Middle East', 'middle-east', 'Long description for Middle East', 'Short description for Middle East', 'image_link_here', 'thumbnail_link_here', ARRAY['gallery1', 'gallery2'], CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
--        ('Europe', 'europe', 'Long description for Europe', 'Short description for Europe', 'image_link_here', 'thumbnail_link_here', ARRAY['gallery1', 'gallery2'], CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
--        ('North America', 'north-america', 'Long description for North America', 'Short description for North America', 'image_link_here', 'thumbnail_link_here', ARRAY['gallery1', 'gallery2'], CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
--        ('Asia', 'asia', 'Long description for Asia', 'Short description for Asia', 'image_link_here', 'thumbnail_link_here', ARRAY['gallery1', 'gallery2'], CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
--        ('Caribbean', 'caribbean', 'Long description for Caribbean', 'Short description for Caribbean', 'image_link_here', 'thumbnail_link_here', ARRAY['gallery1', 'gallery2'], CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
--        ('Australia', 'australia', 'Long description for Australia', 'Short description for Australia', 'image_link_here', 'thumbnail_link_here', ARRAY['gallery1', 'gallery2'], CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

-- Insert for Continent
INSERT INTO continent (name, slug, description_long, description_short, image_link, thumbnail_link, gallery, created, edited)
VALUES ('MISSING DATA', 'missing-data', 'Long description for MISSING DATA', 'Short description for MISSING DATA', 'image_link_here', 'thumbnail_link_here', ARRAY['gallery1', 'gallery2'], CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
       ('Middle East', 'middle-east', 'Long description for Middle East', 'Short description for Middle East', 'image_link_here', 'thumbnail_link_here', ARRAY['gallery1', 'gallery2'], CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
       ('Europe', 'europe', 'Long description for Europe', 'Short description for Europe', 'image_link_here', 'thumbnail_link_here', ARRAY['gallery1', 'gallery2'], CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
       ('North America', 'north-america', 'Long description for North America', 'Short description for North America', 'image_link_here', 'thumbnail_link_here', ARRAY['gallery1', 'gallery2'], CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
       ('Asia', 'asia', 'Long description for Asia', 'Short description for Asia', 'image_link_here', 'thumbnail_link_here', ARRAY['gallery1', 'gallery2'], CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
       ('Caribbean', 'caribbean', 'Long description for Caribbean', 'Short description for Caribbean', 'image_link_here', 'thumbnail_link_here', ARRAY['gallery1', 'gallery2'], CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
       ('Australia', 'australia', 'Long description for Australia', 'Short description for Australia', 'image_link_here', 'thumbnail_link_here', ARRAY['gallery1', 'gallery2'], CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
