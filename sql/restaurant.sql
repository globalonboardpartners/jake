CREATE TABLE restaurant (
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
  menu_gallery TEXT[] NOT NULL,
  featured_restaurant FLOAT NOT NULL,
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

CREATE TRIGGER trigger_row_edit_update_timestamp_for_partner_restaurant
BEFORE UPDATE ON restaurant
FOR EACH ROW
EXECUTE FUNCTION update_updated_on();

INSERT INTO restaurant (name, slug, description_short, description_long, video_link, image_link, image_link_2, thumbnail_link, gallery, menu_gallery, featured_restaurant, partner_vendor, continent, country, region, city, latitude, longitude, email, phone, website_link, tags)
VALUES ('Sample Restaurant 1', 'sample-restaurant-1', 'Short description for Sample Restaurant 1', 'Long description for Sample Restaurant 1', 'https://www.youtube.com/watch?v=sample_video', 'https://www.example.com/images/sample1.jpg', 'https://www.example.com/images/sample1_2.jpg', 'https://www.example.com/images/sample1_thumb.jpg', ARRAY['https://www.example.com/gallery/sample1_1.jpg', 'https://www.example.com/gallery/sample1_2.jpg'], ARRAY['https://www.example.com/menu/sample1_1.jpg', 'https://www.example.com/menu/sample1_2.jpg'], 1.0, 1, 1, 1, 1, 1, 40.7128, -74.0060, 'info@sample1.com', '+123456789', 'https://www.sample1.com', 'tag1, tag2, tag3');
INSERT INTO restaurant (name, slug, description_short, description_long, video_link, image_link, image_link_2, thumbnail_link, gallery, menu_gallery, featured_restaurant, partner_vendor, continent, country, region, city, latitude, longitude, email, phone, website_link, tags)
VALUES ('Sample Restaurant 2', 'sample-restaurant-2', 'Short description for Sample Restaurant 2', 'Long description for Sample Restaurant 2', 'https://www.youtube.com/watch?v=sample_video_2', 'https://www.example.com/images/sample2.jpg', 'https://www.example.com/images/sample2_2.jpg', 'https://www.example.com/images/sample2_thumb.jpg', ARRAY['https://www.example.com/gallery/sample2_1.jpg', 'https://www.example.com/gallery/sample2_2.jpg'], ARRAY['https://www.example.com/menu/sample2_1.jpg', 'https://www.example.com/menu/sample2_2.jpg'], 1.0, 1, 1, 1, 1, 1, 34.0522, -118.2437, 'info@sample2.com', '+987654321', 'https://www.sample2.com', 'tag4, tag5');
INSERT INTO restaurant (name, slug, description_short, description_long, video_link, image_link, image_link_2, thumbnail_link, gallery, menu_gallery, featured_restaurant, partner_vendor, continent, country, region, city, latitude, longitude, email, phone, website_link, tags)
VALUES ('Sample Restaurant 3', 'sample-restaurant-3', 'Short description for Sample Restaurant 3', 'Long description for Sample Restaurant 3', '', '', '', '', ARRAY['https://www.example.com/gallery/sample3_1.jpg', 'https://www.example.com/gallery/sample3_2.jpg'], ARRAY['https://www.example.com/menu/sample3_1.jpg', 'https://www.example.com/menu/sample3_2.jpg'], 1.0, 1, 1, 1, 1, 1, 51.5074, -0.1278, 'info@sample3.com', '+444555666', 'https://www.sample3.com', 'tag6, tag7, tag8');
