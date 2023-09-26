CREATE TABLE country (
  id SERIAL PRIMARY KEY,
  name varchar(50) NOT NULL,
  slug varchar(50) NOT NULL,
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

INSERT INTO country (
  name, 
  slug, 
  description_long, 
  description_short, 
  image_link, 
  thumbnail_link, 
  gallery
) VALUES (
  'MISSING DATA', 
  'missing-data', 
  'This is a placeholder for missing data.', 
  'Placeholder for missing data.', 
  'https://i.imgur.com/hfM1J8s.png', 
  'https://i.imgur.com/hfM1J8s.png', 
  '{}'
);

INSERT INTO country (name, slug, description_long, description_short, image_link, thumbnail_link, special_offer_image_link, video_link, gallery, tags)
VALUES ('United States', 'usa', 'Long description of the United States.', 'Short description of USA.', 'https://example.com/usa.jpg', 'https://example.com/thumbnail/usa.jpg', 'https://example.com/special_offer/usa.jpg', 'https://youtube.com/usa', ARRAY['https://example.com/image1.jpg', 'https://example.com/image2.jpg'], 'country, travel');
INSERT INTO country (name, slug, description_long, description_short, image_link, thumbnail_link, special_offer_image_link, video_link, gallery, tags)
VALUES ('France', 'france', 'Long description of France.', 'Short description of France.', 'https://example.com/france.jpg', 'https://example.com/thumbnail/france.jpg', 'https://example.com/special_offer/france.jpg', 'https://youtube.com/france', ARRAY['https://example.com/image1.jpg', 'https://example.com/image2.jpg'], 'country, travel');
INSERT INTO country (name, slug, description_long, description_short, image_link, thumbnail_link, special_offer_image_link, video_link, gallery, tags)
VALUES ('Japan', 'japan', 'Long description of Japan.', 'Short description of Japan.', 'https://example.com/japan.jpg', 'https://example.com/thumbnail/japan.jpg', 'https://example.com/special_offer/japan.jpg', 'https://youtube.com/japan', ARRAY['https://example.com/image1.jpg', 'https://example.com/image2.jpg'], 'country, travel');
