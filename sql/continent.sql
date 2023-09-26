-- formally known as "Region"
CREATE TABLE continent (
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

CREATE TRIGGER trigger_row_edit_update_timestamp_for_continent
BEFORE UPDATE ON continent
FOR EACH ROW
EXECUTE FUNCTION update_updated_on();

INSERT INTO continent (
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
  ARRAY['https://i.imgur.com/hfM1J8s.png']
);

INSERT INTO continent (name, slug, description_long, description_short, image_link, thumbnail_link, special_offer_image_link, video_link, gallery)
VALUES ('Asia', 'asia', 'Long description of Asia', 'Short description of Asia', 'image_asia.jpg', 'thumbnail_asia.jpg', 'special_offer_asia.jpg', 'video_asia.mp4', ARRAY['image1.jpg', 'image2.jpg']);
INSERT INTO continent (name, slug, description_long, description_short, image_link, thumbnail_link, special_offer_image_link, video_link, gallery, tags)
VALUES ('Europe', 'europe', 'Long description of Europe', 'Short description of Europe', 'image_europe.jpg', 'thumbnail_europe.jpg', 'special_offer_europe.jpg', 'video_europe.mp4', ARRAY['image1.jpg', 'image2.jpg'], 'travel, culture');
INSERT INTO continent (name, slug, description_long, description_short, image_link, thumbnail_link, video_link, gallery, tags)
VALUES ('Africa', 'africa', 'Long description of Africa', 'Short description of Africa', 'image_africa.jpg', 'thumbnail_africa.jpg', 'video_africa.mp4', ARRAY['image1.jpg', 'image2.jpg'], 'safari, nature');
