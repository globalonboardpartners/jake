CREATE TABLE activity (
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
  featured_activity FLOAT NOT NULL,
  partner_vendor integer references partner_vendor(id) NOT NULL,
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


CREATE TRIGGER trigger_row_edit_update_timestamp_for_activity
BEFORE UPDATE ON activity
FOR EACH ROW
EXECUTE FUNCTION update_updated_on();

INSERT INTO activity (
  name, 
  slug, 
  description_short, 
  description_long, 
  video_link, 
  image_link, 
  image_link_2, 
  thumbnail_link, 
  gallery, 
  featured_activity, 
  partner_vendor, 
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
  'MISSING DATA', 
  'missing-data', 
  'This is a placeholder for missing data.', 
  'This entry serves as a placeholder for any missing or unavailable data.', 
  NULL, 
  'https://i.imgur.com/hfM1J8s.png', 
  'https://i.imgur.com/hfM1J8s.png', 
  'https://i.imgur.com/hfM1J8s.png', 
  ARRAY['https://i.imgur.com/hfM1J8s.png'], 
  0.0, 
  1, 
  1, 
  1, 
  1, 
  1, 
  0.0, 
  0.0, 
  'missing@email.com', 
  '000-000-0000', 
  NULL, 
  'https://i.imgur.com/hfM1J8s.png', 
  NULL
);

-- activity
INSERT INTO activity (name, slug, description_short, description_long, video_link, image_link, image_link_2, thumbnail_link, gallery, featured_activity, partner_vendor, continent, country, region, city, latitude, longitude, email, phone, website_link)
VALUES ('Hiking Adventure', 'hiking-adventure', 'Explore nature on foot', 'Join us for an exciting hiking adventure through scenic trails.', 'https://www.youtube.com/watch?v=xyz', 'https://example.com/images/hiking.jpg', 'https://example.com/images/hiking_2.jpg', 'https://example.com/images/hiking_thumb.jpg', ARRAY['https://example.com/gallery/image1.jpg', 'https://example.com/gallery/image2.jpg'], 1.0, 1, 1, 1, 1, 1, 123.456, -78.901, 'contact@example.com', '+1234567890', 'https://www.example.com');
INSERT INTO activity (name, slug, description_short, description_long, video_link, image_link, image_link_2, thumbnail_link, gallery, featured_activity, partner_vendor, continent, country, region, city, latitude, longitude, email, phone, website_link, tags)
VALUES ('Cultural Tour', 'cultural-tour', 'Experience rich cultural heritage', 'Immerse yourself in the local culture and traditions on this guided tour.', 'https://www.youtube.com/watch?v=abc', 'https://example.com/images/cultural.jpg', 'https://example.com/images/cultural_2.jpg', 'https://example.com/images/cultural_thumb.jpg', ARRAY['https://example.com/gallery/image3.jpg', 'https://example.com/gallery/image4.jpg'], 1.0, 1, 1, 1, 1, 1, 12.345, -67.890, 'info@example.com', '+0987654321', 'https://www.tourcompany.com', 'culture, heritage, guided tour');
INSERT INTO activity (name, slug, description_short, description_long, video_link, image_link, image_link_2, thumbnail_link, gallery, featured_activity, partner_vendor, continent, country, region, city, latitude, longitude, email, phone, website_link, tags)
VALUES ('Snorkeling Adventure', 'snorkeling-adventure', 'Discover marine life underwater', 'Explore the beautiful marine life with our snorkeling adventure.', 'https://www.youtube.com/watch?v=pqr', 'https://example.com/images/snorkeling.jpg', 'https://example.com/images/snorkeling_2.jpg', 'https://example.com/images/snorkeling_thumb.jpg', ARRAY['https://example.com/gallery/image5.jpg', 'https://example.com/gallery/image6.jpg'], 1.0, 1, 1, 1, 1, 1, 23.456, -45.678, 'contact@snorkelers.com', '+9876543210', 'https://www.snorkelers.com', 'snorkeling, marine life, water sports');
