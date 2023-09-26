CREATE TABLE country (
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

CREATE TRIGGER trigger_row_edit_update_timestamp_for_country
BEFORE UPDATE ON country
FOR EACH ROW
EXECUTE FUNCTION update_updated_on();

-- Insert for Country
INSERT INTO country (name, slug, description_long, description_short, image_link, thumbnail_link, gallery, created, edited)
VALUES ('United Arab Emirates', 'united-arab-emirates', 'Long description for United Arab Emirates', 'Short description for United Arab Emirates', 'image_link_here', 'thumbnail_link_here', ARRAY['gallery1', 'gallery2'], CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
       ('Netherlands', 'netherlands', 'Long description for Netherlands', 'Short description for Netherlands', 'image_link_here', 'thumbnail_link_here', ARRAY['gallery1', 'gallery2'], CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
       ('France', 'france', 'Long description for France', 'Short description for France', 'image_link_here', 'thumbnail_link_here', ARRAY['gallery1', 'gallery2'], CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
       ('South Africa', 'south-africa', 'Long description for South Africa', 'Short description for South Africa', 'image_link_here', 'thumbnail_link_here', ARRAY['gallery1', 'gallery2'], CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
       ('United States', 'united-states', 'Long description for United States', 'Short description for United States', 'image_link_here', 'thumbnail_link_here', ARRAY['gallery1', 'gallery2'], CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
       ('Thailand', 'thailand', 'Long description for Thailand', 'Short description for Thailand', 'image_link_here', 'thumbnail_link_here', ARRAY['gallery1', 'gallery2'], CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
       ('Australia', 'australia', 'Long description for Australia', 'Short description for Australia', 'image_link_here', 'thumbnail_link_here', ARRAY['gallery1', 'gallery2'], CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
       ('India', 'india', 'Long description for India', 'Short description for India', 'image_link_here', 'thumbnail_link_here', ARRAY['gallery1', 'gallery2'], CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
       ('Jamaica', 'jamaica', 'Long description for Jamaica', 'Short description for Jamaica', 'image_link_here', 'thumbnail_link_here', ARRAY['gallery1', 'gallery2'], CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
       ('United Kingdom', 'united-kingdom', 'Long description for United Kingdom', 'Short description for United Kingdom', 'image_link_here', 'thumbnail_link_here', ARRAY['gallery1', 'gallery2'], CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
