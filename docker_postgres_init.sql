----------------------------------------
--             WARNING!               --
--  if you change the table schema,   --
--  you will have to also change the  --
--               code!                --
----------------------------------------
-- Create Team Members Table
CREATE TABLE employee (
  id SERIAL PRIMARY KEY,
  name VARCHAR(50) NOT NULL,
  position VARCHAR(50) NOT NULL,
  bio TEXT NOT NULL,
  image_url VARCHAR(255) NOT NULL
);

-- Create Blog Categories Table
CREATE TABLE blog_category (
  id SERIAL PRIMARY KEY,
  category VARCHAR(50) NOT NULL
);

-- Create Blog Posts Table
CREATE TABLE blog (
  id SERIAL PRIMARY KEY,
  title VARCHAR(100) NOT NULL,
  content TEXT NOT NULL,
  publish_date TIMESTAMP NOT NULL,
  category_id INT NOT NULL REFERENCES blog_category(id)
);

-- Create Job Listings Table
CREATE TABLE job_listing (
  id SERIAL PRIMARY KEY,
  title VARCHAR(100) NOT NULL,
  description TEXT NOT NULL,
  publish_date TIMESTAMP NOT NULL
);

-- Create Product Features Table
CREATE TABLE product_feature (
  id SERIAL PRIMARY KEY,
  title VARCHAR(100) NOT NULL,
  description TEXT NOT NULL
);

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
  created TIMESTAMP DEFAULT NOW(),
  edited TIMESTAMP DEFAULT NOW()
);

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
  created TIMESTAMP DEFAULT NOW(),
  edited TIMESTAMP DEFAULT NOW()
);

-- for states/provinences/regions/teritory/reservation/military base/etc basically for anything that is the next largest division of a country
CREATE TABLE region (
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
  created TIMESTAMP DEFAULT NOW(),
  edited TIMESTAMP DEFAULT NOW()
);

CREATE TABLE city (
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
  featured_city BOOLEAN NOT NULL,
  tags TEXT,
  created TIMESTAMP DEFAULT NOW(),
  edited TIMESTAMP DEFAULT NOW()
);

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
  featured_partner_vendor BOOLEAN NOT NULL,
  continent integer references continent(id) NOT NULL,
  country integer references country(id) NOT NULL,
  region integer references region(id) NOT NULL,
  city integer references city(id) NOT NULL,
  latatude FLOAT NOT NULL,
  longitude FLOAT NOT NULL,
  email TEXT NOT NULL,
  phone VARCHAR(16) NOT NULL,
  address TEXT,
  website_link TEXT NOT NULL,
  tags TEXT,
  created TIMESTAMP DEFAULT NOW(),
  edited TIMESTAMP DEFAULT NOW()
);

-- I want to change the name of the "collection_id" and "item_id" to be something significantly more descriptive
-- restaurant table has significant differences from the csv joe gave me
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
  featured_restaurant BOOLEAN NOT NULL,
  partner_vendor integer references partner_vendor(id) NOT NULL,
  continent integer references continent(id) NOT NULL,
  country integer references country(id) NOT NULL,
  region integer references region(id) NOT NULL,
  city integer references city(id) NOT NULL,
  latatude FLOAT NOT NULL,
  longitude FLOAT NOT NULL,
  email TEXT NOT NULL,
  phone VARCHAR(16) NOT NULL,
  address TEXT,
  website_link TEXT NOT NULL,
  tags TEXT,
  created TIMESTAMP DEFAULT NOW(),
  edited TIMESTAMP DEFAULT NOW()
);

CREATE TABLE hotel (
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
  featured_hotel BOOLEAN NOT NULL,
  partner_vendor integer references partner_vendor(id) NOT NULL,
  continent integer references continent(id) NOT NULL,
  country integer references country(id) NOT NULL,
  region integer references region(id) NOT NULL,
  city integer references city(id) NOT NULL,
  latatude FLOAT NOT NULL,
  longitude FLOAT NOT NULL,
  email TEXT NOT NULL,
  phone VARCHAR(16) NOT NULL,
  address TEXT,
  website_link TEXT NOT NULL,
  tags TEXT,
  created TIMESTAMP DEFAULT NOW(),
  edited TIMESTAMP DEFAULT NOW()
);

CREATE TABLE hotel_room (
  id SERIAL PRIMARY KEY,
  name varchar(50) NOT NULL,
  hotel_id integer references hotel(id) NOT NULL,
  description_short varchar(100) NOT NULL,
  description_long varchar(200) NOT NULL,
  video_link TEXT,
  image_link TEXT NOT NULL,
  image_link_2 TEXT NOT NULL,
  thumbnail_link TEXT NOT NULL,
  gallery TEXT[] NOT NULL,
  tags TEXT,
  created TIMESTAMP DEFAULT NOW(),
  edited TIMESTAMP DEFAULT NOW()
);

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
  featured_activity BOOLEAN NOT NULL,
  partner_vendor integer references partner_vendor(id) NOT NULL,
  continent integer references continent(id) NOT NULL,
  country integer references country(id) NOT NULL,
  region integer references region(id) NOT NULL,
  city integer references city(id) NOT NULL,
  latatude FLOAT NOT NULL,
  longitude FLOAT NOT NULL,
  email TEXT NOT NULL,
  phone VARCHAR(16) NOT NULL,
  address TEXT,
  website_link TEXT NOT NULL,
  tags TEXT,
  created TIMESTAMP DEFAULT NOW(),
  edited TIMESTAMP DEFAULT NOW()
);

CREATE TABLE event (
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
  featured_event BOOLEAN NOT NULL,
  ticket_link TEXT NOT NULL,
  partner_vendor integer references partner_vendor(id) NOT NULL,
  continent integer references continent(id) NOT NULL,
  country integer references country(id) NOT NULL,
  region integer references region(id) NOT NULL,
  city integer references city(id) NOT NULL,
  latatude FLOAT NOT NULL,
  longitude FLOAT NOT NULL,
  email TEXT NOT NULL,
  phone VARCHAR(16) NOT NULL,
  address TEXT,
  website_link TEXT NOT NULL,
  tags TEXT,
  created TIMESTAMP DEFAULT NOW(),
  edited TIMESTAMP DEFAULT NOW()
);

CREATE TABLE event_details (
  id SERIAL PRIMARY KEY,
  name VARCHAR(50) NOT NULL,
  slug VARCHAR(50) NOT NULL,
  collection_id uuid NOT NULL,
  item_id uuid NOT NULL,
  event_date DATE NOT NULL,
  event_artist_slug VARCHAR(100) NOT NULL,
  venue_name VARCHAR(100) NOT NULL,
  event_time TIME NOT NULL,
  city integer references city(id) NOT NULL,
  country integer references country(id) NOT NULL,
  region integer references region(id) NOT NULL,
  ticket_link TEXT NOT NULL,
  gallery TEXT[] NOT NULL,
  tags TEXT,
  created TIMESTAMP DEFAULT NOW(),
  edited TIMESTAMP DEFAULT NOW()
);

CREATE OR REPLACE FUNCTION update_updated_on()
RETURNS TRIGGER AS $$
BEGIN
  NEW.edited = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_row_edit_update_timestamp_for_continent
BEFORE UPDATE ON continent
FOR EACH ROW
EXECUTE FUNCTION update_updated_on();

CREATE TRIGGER trigger_row_edit_update_timestamp_for_country
BEFORE UPDATE ON country
FOR EACH ROW
EXECUTE FUNCTION update_updated_on();

CREATE TRIGGER trigger_row_edit_update_timestamp_for_region
BEFORE UPDATE ON country
FOR EACH ROW
EXECUTE FUNCTION update_updated_on();

CREATE TRIGGER trigger_row_edit_update_timestamp_for_city
BEFORE UPDATE ON city
FOR EACH ROW
EXECUTE FUNCTION update_updated_on();

CREATE TRIGGER trigger_row_edit_update_timestamp_for_partner_vendor
BEFORE UPDATE ON partner_vendor
FOR EACH ROW
EXECUTE FUNCTION update_updated_on();

CREATE TRIGGER trigger_row_edit_update_timestamp_for_partner_restaurant
BEFORE UPDATE ON restaurant
FOR EACH ROW
EXECUTE FUNCTION update_updated_on();

CREATE TRIGGER trigger_row_edit_update_timestamp_for_partner_hotel
BEFORE UPDATE ON hotel
FOR EACH ROW
EXECUTE FUNCTION update_updated_on();

CREATE TRIGGER trigger_row_edit_update_timestamp_for_partner_hotel_room
BEFORE UPDATE ON hotel_room
FOR EACH ROW
EXECUTE FUNCTION update_updated_on();

CREATE TRIGGER trigger_row_edit_update_timestamp_for_activity
BEFORE UPDATE ON activity
FOR EACH ROW
EXECUTE FUNCTION update_updated_on();

CREATE TRIGGER trigger_row_edit_update_timestamp_for_event
BEFORE UPDATE ON event
FOR EACH ROW
EXECUTE FUNCTION update_updated_on();

CREATE TRIGGER trigger_row_edit_update_timestamp_for_event_details
BEFORE UPDATE ON event_details
FOR EACH ROW
EXECUTE FUNCTION update_updated_on();

-- inserts for employee
insert into employee (name, position, bio, image_url) values ('tom', 'ceo', 'cool dude', 'https://google.com/1');
insert into employee (name, position, bio, image_url) values ('sean', 'cto', 'cool guy', 'https://google.com/2');
insert into employee (name, position, bio, image_url) values ('john', 'cfo', 'cool bro', 'https://google.com/3');

-- inserts for blog_category
insert into blog_category (category) values ('dumb');
insert into blog_category (category) values ('young');
insert into blog_category (category) values ('broke');

-- inserts for blog
insert into blog (title, content, publish_date, category_id) values ('how to wipe your face', 'its easy', '2023-08-03 12:34:56', 1);
insert into blog (title, content, publish_date, category_id) values ('runing with scissors is easy', 'sometimes dangerous', '2023-08-04 12:34:56', 1);
insert into blog (title, content, publish_date, category_id) values ('how to jump', 'just jump', '2023-08-05 12:34:56', 1);

-- job_listing
insert into job_listing (title, description, publish_date) values ('software dev', 'cool vibes', '2023-08-04 12:34:56');
insert into job_listing (title, description, publish_date) values ('dev', 'cool', '2023-08-05 12:34:56');
insert into job_listing (title, description, publish_date) values ('developer', 'vibes', '2023-08-06 12:34:56');

-- product feature
insert into product_feature (title, description) values ('thing', 'function');
insert into product_feature (title, description) values ('product', 'feature');
insert into product_feature (title, description) values ('tool', 'use');


-- continent
INSERT INTO continent (name, slug, description_long, description_short, image_link, thumbnail_link, special_offer_image_link, video_link, gallery)
VALUES ('Asia', 'asia', 'Long description of Asia', 'Short description of Asia', 'image_asia.jpg', 'thumbnail_asia.jpg', 'special_offer_asia.jpg', 'video_asia.mp4', ARRAY['image1.jpg', 'image2.jpg']);
INSERT INTO continent (name, slug, description_long, description_short, image_link, thumbnail_link, special_offer_image_link, video_link, gallery, tags)
VALUES ('Europe', 'europe', 'Long description of Europe', 'Short description of Europe', 'image_europe.jpg', 'thumbnail_europe.jpg', 'special_offer_europe.jpg', 'video_europe.mp4', ARRAY['image1.jpg', 'image2.jpg'], 'travel, culture');
INSERT INTO continent (name, slug, description_long, description_short, image_link, thumbnail_link, video_link, gallery, tags)
VALUES ('Africa', 'africa', 'Long description of Africa', 'Short description of Africa', 'image_africa.jpg', 'thumbnail_africa.jpg', 'video_africa.mp4', ARRAY['image1.jpg', 'image2.jpg'], 'safari, nature');

-- country
INSERT INTO country (name, slug, description_long, description_short, image_link, thumbnail_link, special_offer_image_link, video_link, gallery, tags)
VALUES ('United States', 'usa', 'Long description of the United States.', 'Short description of USA.', 'https://example.com/usa.jpg', 'https://example.com/thumbnail/usa.jpg', 'https://example.com/special_offer/usa.jpg', 'https://youtube.com/usa', ARRAY['https://example.com/image1.jpg', 'https://example.com/image2.jpg'], 'country, travel');
INSERT INTO country (name, slug, description_long, description_short, image_link, thumbnail_link, special_offer_image_link, video_link, gallery, tags)
VALUES ('France', 'france', 'Long description of France.', 'Short description of France.', 'https://example.com/france.jpg', 'https://example.com/thumbnail/france.jpg', 'https://example.com/special_offer/france.jpg', 'https://youtube.com/france', ARRAY['https://example.com/image1.jpg', 'https://example.com/image2.jpg'], 'country, travel');
INSERT INTO country (name, slug, description_long, description_short, image_link, thumbnail_link, special_offer_image_link, video_link, gallery, tags)
VALUES ('Japan', 'japan', 'Long description of Japan.', 'Short description of Japan.', 'https://example.com/japan.jpg', 'https://example.com/thumbnail/japan.jpg', 'https://example.com/special_offer/japan.jpg', 'https://youtube.com/japan', ARRAY['https://example.com/image1.jpg', 'https://example.com/image2.jpg'], 'country, travel');

-- region
INSERT INTO region (name, slug, description_long, description_short, image_link, thumbnail_link, special_offer_image_link, video_link, gallery, tags)
VALUES ('California', 'california', 'Long description of California.', 'Short description of California.', 'https://example.com/california.jpg', 'https://example.com/thumbnail/california.jpg', 'https://example.com/special_offer/california.jpg', 'https://youtube.com/california', ARRAY['https://example.com/image1.jpg', 'https://example.com/image2.jpg'], 'region, travel');
INSERT INTO region (name, slug, description_long, description_short, image_link, thumbnail_link, special_offer_image_link, video_link, gallery, tags)
VALUES ('Provence', 'provence', 'Long description of Provence.', 'Short description of Provence.', 'https://example.com/provence.jpg', 'https://example.com/thumbnail/provence.jpg', 'https://example.com/special_offer/provence.jpg', 'https://youtube.com/provence', ARRAY['https://example.com/image1.jpg', 'https://example.com/image2.jpg'], 'region, travel');
INSERT INTO region (name, slug, description_long, description_short, image_link, thumbnail_link, special_offer_image_link, video_link, gallery, tags)
VALUES ('Kanto', 'kanto', 'Long description of Kanto.', 'Short description of Kanto.', 'https://example.com/kanto.jpg', 'https://example.com/thumbnail/kanto.jpg', 'https://example.com/special_offer/kanto.jpg', 'https://youtube.com/kanto', ARRAY['https://example.com/image1.jpg', 'https://example.com/image2.jpg'], 'region, travel');

-- city
INSERT INTO city (name, slug, description_long, description_short, image_link, thumbnail_link, special_offer_image_link, video_link, gallery, featured_city, tags)
VALUES ('New York', 'new-york', 'A bustling metropolis with iconic landmarks.', 'Experience the city that never sleeps.', 'http://example.com/images/new_york.jpg', 'http://example.com/images/thumbnails/new_york.jpg', 'http://example.com/images/special_offer/new_york.jpg', 'http://example.com/videos/new_york.mp4', ARRAY['http://example.com/gallery/image1.jpg', 'http://example.com/gallery/image2.jpg'], false, 'travel, city, USA');
INSERT INTO city (name, slug, description_long, description_short, image_link, thumbnail_link, special_offer_image_link, video_link, gallery, featured_city, tags)
VALUES ('Paris', 'paris', 'The city of love and lights.', 'Explore the romantic atmosphere of Paris.', 'http://example.com/images/paris.jpg', 'http://example.com/images/thumbnails/paris.jpg', 'http://example.com/images/special_offer/paris.jpg', 'http://example.com/videos/paris.mp4', ARRAY['http://example.com/gallery/image3.jpg', 'http://example.com/gallery/image4.jpg'], true, 'travel, city, France, romance');
INSERT INTO city (name, slug, description_long, description_short, image_link, thumbnail_link, video_link, gallery, featured_city, tags)
VALUES ('Tokyo', 'tokyo', 'A modern city with a blend of tradition and technology.', 'Experience the unique culture of Tokyo.', 'http://example.com/images/tokyo.jpg', 'http://example.com/images/thumbnails/tokyo.jpg', 'http://example.com/videos/tokyo.mp4', ARRAY['http://example.com/gallery/image3.jpg', 'http://example.com/gallery/image4.jpg'], false, 'travel, city, Japan');

-- partner_vendor
INSERT INTO partner_vendor (name, slug, description_short, description_long, video_link, image_link, image_link_2, thumbnail_link, gallery, featured_partner_vendor, continent, country, region, city, latatude, longitude, email, phone, website_link)
VALUES ('Vendor1', 'vendor1', 'Short description for Vendor 1', 'Long description for Vendor 1', NULL, 'https://example.com/image1.jpg', 'https://example.com/image2.jpg', 'https://example.com/thumbnail.jpg', ARRAY['https://example.com/gallery_image1.jpg', 'https://example.com/gallery_image2.jpg'], FALSE, 1, 1, 1, 1, 40.7128, -74.0060, 'vendor1@example.com', '123-456-7890', 'https://vendor1website.com');
INSERT INTO partner_vendor (name, slug, description_short, description_long, video_link, image_link, image_link_2, thumbnail_link, gallery, featured_partner_vendor, continent, country, region, city, latatude, longitude, email, phone, website_link, tags)
VALUES ('Vendor2', 'vendor2', 'Short description for Vendor 2', 'Long description for Vendor 2', 'https://youtube.com/vendor2_video', 'https://example.com/vendor2_image1.jpg', 'https://example.com/vendor2_image2.jpg', 'https://example.com/vendor2_thumbnail.jpg', ARRAY['https://example.com/vendor2_gallery_image1.jpg', 'https://example.com/vendor2_gallery_image2.jpg'], TRUE, 1, 1, 1, 1, 51.5074, -0.1278, 'vendor2@example.com', '987-654-3210', 'https://vendor2website.com', 'tag1, tag2, tag3');
INSERT INTO partner_vendor (name, slug, description_short, description_long, video_link, image_link, image_link_2, thumbnail_link, gallery, featured_partner_vendor, continent, country, region, city, latatude, longitude, email, phone, website_link)
VALUES ('Vendor3', 'vendor3', 'Short description for Vendor 3', 'Long description for Vendor 3', NULL, 'https://example.com/vendor3_image1.jpg', 'https://example.com/vendor3_image2.jpg', 'https://example.com/vendor3_thumbnail.jpg', ARRAY['https://example.com/vendor3_gallery_image1.jpg', 'https://example.com/vendor3_gallery_image2.jpg'], FALSE, 1, 1, 1, 1, 48.8566, 2.3522, 'vendor3@example.com', '555-123-4567', 'https://vendor3website.com');

-- restaurant
INSERT INTO restaurant (name, slug, description_short, description_long, video_link, image_link, image_link_2, thumbnail_link, gallery, menu_gallery, featured_restaurant, partner_vendor, continent, country, region, city, latatude, longitude, email, phone, website_link, tags)
VALUES ('Sample Restaurant 1', 'sample-restaurant-1', 'Short description for Sample Restaurant 1', 'Long description for Sample Restaurant 1', 'https://www.youtube.com/watch?v=sample_video', 'https://www.example.com/images/sample1.jpg', 'https://www.example.com/images/sample1_2.jpg', 'https://www.example.com/images/sample1_thumb.jpg', ARRAY['https://www.example.com/gallery/sample1_1.jpg', 'https://www.example.com/gallery/sample1_2.jpg'], ARRAY['https://www.example.com/menu/sample1_1.jpg', 'https://www.example.com/menu/sample1_2.jpg'], TRUE, 1, 1, 1, 1, 1, 40.7128, -74.0060, 'info@sample1.com', '+123456789', 'https://www.sample1.com', 'tag1, tag2, tag3');
INSERT INTO restaurant (name, slug, description_short, description_long, video_link, image_link, image_link_2, thumbnail_link, gallery, menu_gallery, featured_restaurant, partner_vendor, continent, country, region, city, latatude, longitude, email, phone, website_link, tags)
VALUES ('Sample Restaurant 2', 'sample-restaurant-2', 'Short description for Sample Restaurant 2', 'Long description for Sample Restaurant 2', 'https://www.youtube.com/watch?v=sample_video_2', 'https://www.example.com/images/sample2.jpg', 'https://www.example.com/images/sample2_2.jpg', 'https://www.example.com/images/sample2_thumb.jpg', ARRAY['https://www.example.com/gallery/sample2_1.jpg', 'https://www.example.com/gallery/sample2_2.jpg'], ARRAY['https://www.example.com/menu/sample2_1.jpg', 'https://www.example.com/menu/sample2_2.jpg'], FALSE, 1, 1, 1, 1, 1, 34.0522, -118.2437, 'info@sample2.com', '+987654321', 'https://www.sample2.com', 'tag4, tag5');
INSERT INTO restaurant (name, slug, description_short, description_long, video_link, image_link, image_link_2, thumbnail_link, gallery, menu_gallery, featured_restaurant, partner_vendor, continent, country, region, city, latatude, longitude, email, phone, website_link, tags)
VALUES ('Sample Restaurant 3', 'sample-restaurant-3', 'Short description for Sample Restaurant 3', 'Long description for Sample Restaurant 3', '', '', '', '', ARRAY['https://www.example.com/gallery/sample3_1.jpg', 'https://www.example.com/gallery/sample3_2.jpg'], ARRAY['https://www.example.com/menu/sample3_1.jpg', 'https://www.example.com/menu/sample3_2.jpg'], TRUE, 1, 1, 1, 1, 1, 51.5074, -0.1278, 'info@sample3.com', '+444555666', 'https://www.sample3.com', 'tag6, tag7, tag8');

-- hotel
INSERT INTO hotel (name, slug, description_short, description_long, video_link, image_link, image_link_2, thumbnail_link, gallery, featured_hotel, partner_vendor, continent, country, region, city, latatude, longitude, email, phone, website_link)
VALUES ('Hotel ABC', 'hotel-abc', 'A luxury hotel', 'Hotel ABC is a luxurious hotel located...', 'https://www.youtube.com/hotel_abc_video', 'https://example.com/images/hotel_abc.jpg', 'https://example.com/images/hotel_abc_2.jpg', 'https://example.com/images/hotel_abc_thumbnail.jpg', '{"https://example.com/images/hotel_abc_gallery_1.jpg", "https://example.com/images/hotel_abc_gallery_2.jpg"}', TRUE, 1, 1, 1, 1, 1, 40.7128, -74.0060, 'info@hotelabc.com', '+1-123-456-7890', 'https://www.hotelabc.com');
INSERT INTO hotel (name, slug, description_short, description_long, video_link, image_link, image_link_2, thumbnail_link, gallery, featured_hotel, partner_vendor, continent, country, region, city, latatude, longitude, email, phone, website_link, tags)
VALUES ('Hotel XYZ', 'hotel-xyz', 'A cozy hotel', 'Hotel XYZ offers a cozy atmosphere...', 'https://www.youtube.com/hotel_xyz_video', 'https://example.com/images/hotel_xyz.jpg', 'https://example.com/images/hotel_xyz_2.jpg', 'https://example.com/images/hotel_xyz_thumbnail.jpg', '{"https://example.com/images/hotel_xyz_gallery_1.jpg", "https://example.com/images/hotel_xyz_gallery_2.jpg", "https://example.com/images/hotel_xyz_gallery_3.jpg"}', FALSE, 1, 1, 1, 1, 1, 34.0522, -118.2437, 'info@hotelxyz.com', '+1-987-654-3210', 'https://www.hotelxyz.com', '{"luxury", "cozy", "view"}');
INSERT INTO hotel (name, slug, description_short, description_long, video_link, image_link, image_link_2, thumbnail_link, gallery, featured_hotel, partner_vendor, continent, country, region, city, latatude, longitude, email, phone, website_link)
VALUES ('Seaside Resort', 'seaside-resort', 'Beachfront resort', 'Seaside Resort is a beachfront resort...', 'https://www.youtube.com/seaside_resort_video', 'https://example.com/images/seaside_resort.jpg', 'https://example.com/images/seaside_resort_2.jpg', 'https://example.com/images/seaside_resort_thumbnail.jpg', '{"https://example.com/images/seaside_resort_gallery_1.jpg", "https://example.com/images/seaside_resort_gallery_2.jpg", "https://example.com/images/seaside_resort_gallery_3.jpg"}', TRUE, 1, 1, 1, 1, 1, 25.7617, -80.1918, 'info@seasideresort.com', '+1-555-123-4567', 'https://www.seasideresort.com');

-- hotel_room
INSERT INTO hotel_room (name, hotel_id, description_short, description_long, video_link, image_link, image_link_2, thumbnail_link, gallery, tags)
VALUES ('Standard Room', 1, 'Cozy standard room', 'This room offers a comfortable stay with all basic amenities.', 'https://www.example.com/room_video', 'https://www.example.com/room_image.jpg', 'https://www.example.com/room_image2.jpg', 'https://www.example.com/room_thumbnail.jpg', ARRAY['https://www.example.com/room_gallery1.jpg', 'https://www.example.com/room_gallery2.jpg'], 'Standard, Cozy, Basic');
INSERT INTO hotel_room (name, hotel_id, description_short, description_long, video_link, image_link, image_link_2, thumbnail_link, gallery, tags)
VALUES ('Luxury Suite', 1, 'Spacious luxury suite', 'Indulge in luxury with this spacious suite, featuring modern amenities and breathtaking views.', 'https://www.example.com/suite_video', 'https://www.example.com/suite_image.jpg', 'https://www.example.com/suite_image2.jpg', 'https://www.example.com/suite_thumbnail.jpg', ARRAY['https://www.example.com/suite_gallery1.jpg', 'https://www.example.com/suite_gallery2.jpg', 'https://www.example.com/suite_gallery3.jpg'], 'Luxury, Suite, Spacious, View, Modern');
INSERT INTO hotel_room (name, hotel_id, description_short, description_long, video_link, image_link, image_link_2, thumbnail_link, gallery, tags)
VALUES ('Family Room', 1, 'Perfect for families', 'This family room is designed to accommodate families comfortably, ensuring a pleasant stay for everyone.', 'https://www.example.com/family_video', 'https://www.example.com/family_image.jpg', 'https://www.example.com/family_image2.jpg', 'https://www.example.com/family_thumbnail.jpg', ARRAY[]::TEXT[], 'Family, Spacious, Comfortable');

-- activity
INSERT INTO activity (name, slug, description_short, description_long, video_link, image_link, image_link_2, thumbnail_link, gallery, featured_activity, partner_vendor, continent, country, region, city, latatude, longitude, email, phone, website_link)
VALUES ('Hiking Adventure', 'hiking-adventure', 'Explore nature on foot', 'Join us for an exciting hiking adventure through scenic trails.', 'https://www.youtube.com/watch?v=xyz', 'https://example.com/images/hiking.jpg', 'https://example.com/images/hiking_2.jpg', 'https://example.com/images/hiking_thumb.jpg', ARRAY['https://example.com/gallery/image1.jpg', 'https://example.com/gallery/image2.jpg'], true, 1, 1, 1, 1, 1, 123.456, -78.901, 'contact@example.com', '+1234567890', 'https://www.example.com');
INSERT INTO activity (name, slug, description_short, description_long, video_link, image_link, image_link_2, thumbnail_link, gallery, featured_activity, partner_vendor, continent, country, region, city, latatude, longitude, email, phone, website_link, tags)
VALUES ('Cultural Tour', 'cultural-tour', 'Experience rich cultural heritage', 'Immerse yourself in the local culture and traditions on this guided tour.', 'https://www.youtube.com/watch?v=abc', 'https://example.com/images/cultural.jpg', 'https://example.com/images/cultural_2.jpg', 'https://example.com/images/cultural_thumb.jpg', ARRAY['https://example.com/gallery/image3.jpg', 'https://example.com/gallery/image4.jpg'], false, 1, 1, 1, 1, 1, 12.345, -67.890, 'info@example.com', '+0987654321', 'https://www.tourcompany.com', 'culture, heritage, guided tour');
INSERT INTO activity (name, slug, description_short, description_long, video_link, image_link, image_link_2, thumbnail_link, gallery, featured_activity, partner_vendor, continent, country, region, city, latatude, longitude, email, phone, website_link, tags)
VALUES ('Snorkeling Adventure', 'snorkeling-adventure', 'Discover marine life underwater', 'Explore the beautiful marine life with our snorkeling adventure.', 'https://www.youtube.com/watch?v=pqr', 'https://example.com/images/snorkeling.jpg', 'https://example.com/images/snorkeling_2.jpg', 'https://example.com/images/snorkeling_thumb.jpg', ARRAY['https://example.com/gallery/image5.jpg', 'https://example.com/gallery/image6.jpg'], true, 1, 1, 1, 1, 1, 23.456, -45.678, 'contact@snorkelers.com', '+9876543210', 'https://www.snorkelers.com', 'snorkeling, marine life, water sports');

-- event
INSERT INTO event (name, slug, description_short, description_long, video_link, image_link, image_link_2, thumbnail_link, gallery, featured_event, ticket_link, partner_vendor, continent, country, region, city, latatude, longitude, email, phone, website_link)
VALUES ('Sample Event 1', 'sample-event-1', 'Short description of Sample Event 1', 'Long description of Sample Event 1', 'https://www.youtube.com/watch?v=video1', 'https://example.com/image1.jpg', 'https://example.com/image2.jpg', 'https://example.com/thumbnail.jpg', ARRAY['https://example.com/gallery/image1.jpg', 'https://example.com/gallery/image2.jpg'], TRUE, 'https://example.com/tickets', 1, 1, 1, 1, 1, 40.7128, -74.0060, 'email@example.com', '+1234567890', 'https://www.sampleevent1.com');
INSERT INTO event (name, slug, description_short, description_long, video_link, image_link, image_link_2, thumbnail_link, gallery, featured_event, ticket_link, partner_vendor, continent, country, region, city, latatude, longitude, email, phone, website_link, tags)
VALUES ('Sample Event 2', 'sample-event-2', 'Short description of Sample Event 2', 'Long description of Sample Event 2', 'https://www.youtube.com/watch?v=video2', 'https://example.com/image3.jpg', 'https://example.com/image4.jpg', 'https://example.com/thumbnail.jpg', ARRAY['https://example.com/gallery/image3.jpg', 'https://example.com/gallery/image4.jpg'], FALSE, 'https://example.com/tickets', 1, 1, 1, 1, 1, 51.5074, -0.1278, 'event@example.com', '+9876543210', 'https://www.sampleevent2.com', 'music, concert, entertainment');

-- event details
INSERT INTO event_details (name, slug, collection_id, item_id, event_date, event_artist_slug, venue_name, event_time, city, country, region, ticket_link, gallery, tags)
VALUES ('Concert in the Park', 'concert-park', 'd9e21c3c-4c9d-4a18-9b79-8aa5e936e0a7', '5a3b7cd2-7b97-4465-bd6d-9bc5be8a8a07', '2023-08-15', 'artist-name', 'Central Park', '19:00:00', 1, 1, 1, 'https://www.example.com/tickets', '{image1.jpg, image2.jpg}', 'music, outdoor');
INSERT INTO event_details (name, slug, collection_id, item_id, event_date, event_artist_slug, venue_name, event_time, city, country, region, ticket_link, gallery, tags)
VALUES ('Comedy Show', 'comedy-show', 'b23e4a1f-3a78-47e6-8293-34c10be5a712', '56f13b1e-4efc-4126-9a82-3dc972a11d63', '2023-08-20', 'comedian-name', 'Laugh Factory', '20:30:00', 1, 1, 1, 'https://www.example.com/comedy-tickets', '{image3.jpg, image4.jpg}', 'comedy, entertainment');
INSERT INTO event_details (name, slug, collection_id, item_id, event_date, event_artist_slug, venue_name, event_time, city, country, region, ticket_link, gallery, tags)
VALUES ('Art Exhibition', 'art-exhibition', 'f1aa23a7-d0cd-4a46-bd2b-c2165e487b67', 'bc45ddcc-8d25-4c4f-a442-19b1c8623b9b', '2023-08-25', 'artist-name', 'Art Gallery', '14:00:00', 1, 1, 1, 'https://www.example.com/art-tickets', '{image5.jpg, image6.jpg}', 'art, exhibition');
