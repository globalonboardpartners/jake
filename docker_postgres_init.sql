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
  special_offer_image_link TEXT NOT NULL,
  video_link TEXT NOT NULL,
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
  special_offer_image_link TEXT NOT NULL,
  video_link TEXT NOT NULL,
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
  special_offer_image_link TEXT NOT NULL,
  video_link TEXT NOT NULL,
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
  special_offer_image_link TEXT NOT NULL,
  video_link TEXT NOT NULL,
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
  video_link TEXT NOT NULL,
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
  video_link TEXT NOT NULL,
  image_link TEXT NOT NULL,
  image_link_2 TEXT NOT NULL,
  thumbnail_link TEXT NOT NULL,
  gallery TEXT[] NOT NULL,
  menu_gallery TEXT[] NOT NULL,
  featured_restaurant BOOLEAN NOT NULL,
  partner_vendor_id integer references partner_vendor(id) NOT NULL,
  continent integer references continent(id) NOT NULL,
  country integer references country(id) NOT NULL,
  region integer references region(id) NOT NULL,
  city integer references city(id) NOT NULL,
  latatude FLOAT NOT NULL,
  longitude FLOAT NOT NULL,
  email TEXT NOT NULL,
  phone VARCHAR(16) NOT NULL,
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
  video_link TEXT NOT NULL,
  image_link TEXT NOT NULL,
  image_link_2 TEXT NOT NULL,
  thumbnail_link TEXT NOT NULL,
  gallery TEXT[] NOT NULL,
  featured_hotel BOOLEAN NOT NULL,
  partner_vendor_id integer references partner_vendor(id) NOT NULL,
  continent integer references continent(id) NOT NULL,
  country integer references country(id) NOT NULL,
  region integer references region(id) NOT NULL,
  city integer references city(id) NOT NULL,
  latatude FLOAT NOT NULL,
  longitude FLOAT NOT NULL,
  email TEXT NOT NULL,
  phone VARCHAR(16) NOT NULL,
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
  video_link TEXT NOT NULL,
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
  video_link TEXT NOT NULL,
  image_link TEXT NOT NULL,
  image_link_2 TEXT NOT NULL,
  thumbnail_link TEXT NOT NULL,
  gallery TEXT[] NOT NULL,
  featured_activity BOOLEAN NOT NULL,
  partner_vendor_id integer references partner_vendor(id) NOT NULL,
  continent integer references continent(id) NOT NULL,
  country integer references country(id) NOT NULL,
  region integer references region(id) NOT NULL,
  city integer references city(id) NOT NULL,
  latatude FLOAT NOT NULL,
  longitude FLOAT NOT NULL,
  email TEXT NOT NULL,
  phone VARCHAR(16) NOT NULL,
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
  video_link TEXT NOT NULL,
  image_link TEXT NOT NULL,
  image_link_2 TEXT NOT NULL,
  thumbnail_link TEXT NOT NULL,
  gallery TEXT[] NOT NULL,
  featured_event BOOLEAN NOT NULL,
  ticket_link TEXT NOT NULL,
  partner_vendor_id integer references partner_vendor(id) NOT NULL,
  continent integer references continent(id) NOT NULL,
  country integer references country(id) NOT NULL,
  region integer references region(id) NOT NULL,
  city integer references city(id) NOT NULL,
  latatude FLOAT NOT NULL,
  longitude FLOAT NOT NULL,
  email TEXT NOT NULL,
  phone VARCHAR(16) NOT NULL,
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
insert into blog (title, content, publish_date, category_id) values ('how to die', 'just jump', '2023-08-05 12:34:56', 1);

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
INSERT INTO continent (name, slug, description_long, description_short, image_link, thumbnail_link, special_offer_image_link, gallery)
VALUES ('North America', 'north-america', 'Long description of North America', 'Short description of North America', 'image_north_america.jpg', 'thumbnail_north_america.jpg', 'special_offer_north_america.jpg', ARRAY['image1.jpg', 'image2.jpg']);
INSERT INTO continent (name, slug, description_long, description_short, image_link, video_link, gallery, tags)
VALUES ('South America', 'south-america', 'Long description of South America', 'Short description of South America', 'image_south_america.jpg', 'video_south_america.mp4', ARRAY['image1.jpg', 'image2.jpg'], 'adventure, history');
INSERT INTO continent (name, slug, description_long, description_short, image_link, thumbnail_link, special_offer_image_link, tags)
VALUES ('Australia', 'australia', 'Long description of Australia', 'Short description of Australia', 'image_australia.jpg', 'thumbnail_australia.jpg', 'special_offer_australia.jpg', 'beach, surfing');
INSERT INTO continent (name, slug, image_link, thumbnail_link, special_offer_image_link, video_link, gallery, tags)
VALUES ('Antarctica', 'antarctica', 'image_antarctica.jpg', 'thumbnail_antarctica.jpg', 'special_offer_antarctica.jpg', 'video_antarctica.mp4', ARRAY['image1.jpg', 'image2.jpg'], 'expedition, wildlife');
INSERT INTO continent (name, slug, description_long, description_short, image_link, thumbnail_link, special_offer_image_link, video_link, gallery, tags)
VALUES ('North Pole', 'north-pole', 'Long description of North Pole', 'Short description of North Pole', 'image_north_pole.jpg', 'thumbnail_north_pole.jpg', 'special_offer_north_pole.jpg', 'video_north_pole.mp4', ARRAY['image1.jpg', 'image2.jpg'], 'arctic, polar bears');
INSERT INTO continent (name, slug, description_long, description_short, image_link, thumbnail_link, special_offer_image_link, video_link, gallery)
VALUES ('Greenland', 'greenland', 'Long description of Greenland', 'Short description of Greenland', 'image_greenland.jpg', 'thumbnail_greenland.jpg', 'special_offer_greenland.jpg', 'video_greenland.mp4', ARRAY['image1.jpg', 'image2.jpg']);
INSERT INTO continent (name, slug, description_long, description_short, image_link, thumbnail_link, special_offer_image_link, video_link, gallery, tags)
VALUES ('Oceania', 'oceania', 'Long description of Oceania', 'Short description of Oceania', 'image_oceania.jpg', 'thumbnail_oceania.jpg', 'special_offer_oceania.jpg', 'video_oceania.mp4', ARRAY['image1.jpg', 'image2.jpg'], 'islands, diving');
INSERT INTO continent (name, slug, description_long, description_short, image_link, thumbnail_link, video_link, gallery, tags)
VALUES ('Pangaea', 'pangaea', 'Long description of Pangaea', 'Short description of Pangaea', 'image_pangaea.jpg', 'thumbnail_pangaea.jpg', 'video_pangaea.mp4', ARRAY['image1.jpg', 'image2.jpg'], 'prehistoric');
INSERT INTO continent (name, slug, description_long, description_short, image_link, thumbnail_link, special_offer_image_link, gallery)
VALUES ('Eurasia', 'eurasia', 'Long description of Eurasia', 'Short description of Eurasia', 'image_eurasia.jpg', 'thumbnail_eurasia.jpg', 'special_offer_eurasia.jpg', ARRAY['image1.jpg', 'image2.jpg']);
INSERT INTO continent (name, slug, description_long, description_short, image_link, video_link, gallery, tags)
VALUES ('Atlantis', 'atlantis', 'Long description of Atlantis', 'Short description of Atlantis', 'image_atlantis.jpg', 'video_atlantis.mp4', ARRAY['image1.jpg', 'image2.jpg'], 'mythical');
INSERT INTO continent (name, slug, description_long, description_short, image_link, thumbnail_link, special_offer_image_link, tags)
VALUES ('Lemuria', 'lemuria', 'Long description of Lemuria', 'Short description of Lemuria', 'image_lemuria.jpg', 'thumbnail_lemuria.jpg', 'special_offer_lemuria.jpg', 'lost civilization');
INSERT INTO continent (name, slug, image_link, thumbnail_link, special_offer_image_link, video_link, gallery, tags)
VALUES ('Mu', 'mu', 'image_mu.jpg', 'thumbnail_mu.jpg', 'special_offer_mu.jpg', 'video_mu.mp4', ARRAY['image1.jpg', 'image2.jpg'], 'mythical, ancient');
INSERT INTO continent (name, slug, description_long, description_short, image_link, thumbnail_link, special_offer_image_link, video_link, gallery, tags)
VALUES ('Laurasia', 'laurasia', 'Long description of Laurasia', 'Short description of Laurasia', 'image_laurasia.jpg', 'thumbnail_laurasia.jpg', 'special_offer_laurasia.jpg', 'video_laurasia.mp4', ARRAY['image1.jpg', 'image2.jpg'], 'supercontinent');
INSERT INTO continent (name, slug, description_long, description_short, image_link, thumbnail_link, special_offer_image_link, video_link, gallery)
VALUES ('Gondwana', 'gondwana', 'Long description of Gondwana', 'Short description of Gondwana', 'image_gondwana.jpg', 'thumbnail_gondwana.jpg', 'special_offer_gondwana.jpg', 'video_gondwana.mp4', ARRAY['image1.jpg', 'image2.jpg']);
INSERT INTO continent (name, slug, description_long, description_short, image_link, thumbnail_link, special_offer_image_link, video_link, gallery, tags)
VALUES ('Pangaea Ultima', 'pangaea-ultima', 'Long description of Pangaea Ultima', 'Short description of Pangaea Ultima', 'image_pangaea_ultima.jpg', 'thumbnail_pangaea_ultima.jpg', 'special_offer_pangaea_ultima.jpg', 'video_pangaea_ultima.mp4', ARRAY['image1.jpg', 'image2.jpg'], 'future, science fiction');
INSERT INTO continent (name, slug, description_long, description_short, image_link, thumbnail_link, video_link, gallery, tags)
VALUES ('Lemuria II', 'lemuria-ii', 'Long description of Lemuria II', 'Short description of Lemuria II', 'image_lemuria_ii.jpg', 'thumbnail_lemuria_ii.jpg', 'video_lemuria_ii.mp4', ARRAY['image1.jpg', 'image2.jpg'], 'lost civilization');
INSERT INTO continent (name, slug, description_long, description_short, image_link, thumbnail_link, special_offer_image_link, gallery)
VALUES ('Mu II', 'mu-ii', 'Long description of Mu II', 'Short description of Mu II', 'image_mu_ii.jpg', 'thumbnail_mu_ii.jpg', 'special_offer_mu_ii.jpg', ARRAY['image1.jpg', 'image2.jpg']);
INSERT INTO continent (name, slug, description_long, description_short, image_link, video_link, gallery, tags)
VALUES ('Atlantis II', 'atlantis-ii', 'Long description of Atlantis II', 'Short description of Atlantis II', 'image_atlantis_ii.jpg', 'video_atlantis_ii.mp4', ARRAY['image1.jpg', 'image2.jpg'], 'mythical');
INSERT INTO continent (name, slug, description_long, description_short, image_link, thumbnail_link, special_offer_image_link, tags)
VALUES ('Lost Continent', 'lost-continent', 'Long description of Lost Continent', 'Short description of Lost Continent', 'image_lost_continent.jpg', 'thumbnail_lost_continent.jpg', 'special_offer_lost_continent.jpg', 'mythical, ancient');
INSERT INTO continent (name, slug, image_link, thumbnail_link, special_offer_image_link, video_link, gallery, tags)
VALUES ('Hidden Land', 'hidden-land', 'image_hidden_land.jpg', 'thumbnail_hidden_land.jpg', 'special_offer_hidden_land.jpg', 'video_hidden_land.mp4', ARRAY['image1.jpg', 'image2.jpg'], 'mythical');
INSERT INTO continent (name, slug, description_long, description_short, image_link, thumbnail_link, special_offer_image_link, video_link, gallery, tags)
VALUES ('Superland', 'superland', 'Long description of Superland', 'Short description of Superland', 'image_superland.jpg', 'thumbnail_superland.jpg', 'special_offer_superland.jpg', 'video_superland.mp4', ARRAY['image1.jpg', 'image2.jpg'], 'fantasy');
INSERT INTO continent (name, slug, description_long, description_short, image_link, thumbnail_link, special_offer_image_link, video_link, gallery)
VALUES ('Mystery Land', 'mystery-land', 'Long description of Mystery Land', 'Short description of Mystery Land', 'image_mystery_land.jpg', 'thumbnail_mystery_land.jpg', 'special_offer_mystery_land.jpg', 'video_mystery_land.mp4', ARRAY['image1.jpg', 'image2.jpg']);
INSERT INTO continent (name, slug, description_long, description_short, image_link, thumbnail_link, special_offer_image_link, video_link, gallery, tags)
VALUES ('Wonderland', 'wonderland', 'Long description of Wonderland', 'Short description of Wonderland', 'image_wonderland.jpg', 'thumbnail_wonderland.jpg', 'special_offer_wonderland.jpg', 'video_wonderland.mp4', ARRAY['image1.jpg', 'image2.jpg'], 'fantasy, fairy tales');
INSERT INTO continent (name, slug, description_long, description_short, image_link, thumbnail_link, video_link, gallery, tags)
VALUES ('Mythica', 'mythica', 'Long description of Mythica', 'Short description of Mythica', 'image_mythica.jpg', 'thumbnail_mythica.jpg', 'video_mythica.mp4', ARRAY['image1.jpg', 'image2.jpg'], 'mythical');
INSERT INTO continent (name, slug, description_long, description_short, image_link, thumbnail_link, special_offer_image_link, gallery)
VALUES ('Fabled Land', 'fabled-land', 'Long description of Fabled Land', 'Short description of Fabled Land', 'image_fabled_land.jpg', 'thumbnail_fabled_land.jpg', 'special_offer_fabled_land.jpg', ARRAY['image1.jpg', 'image2.jpg']);
INSERT INTO continent (name, slug, description_long, description_short, image_link, video_link, gallery, tags)
VALUES ('Legendary Continent', 'legendary-continent', 'Long description of Legendary Continent', 'Short description of Legendary Continent', 'image_legendary_continent.jpg', 'video_legendary_continent.mp4', ARRAY['image1.jpg', 'image2.jpg'], 'mythical, legendary');
INSERT INTO continent (name, slug, description_long, description_short, image_link, thumbnail_link, special_offer_image_link, tags)
VALUES ('Enchanted Land', 'enchanted-land', 'Long description of Enchanted Land', 'Short description of Enchanted Land', 'image_enchanted_land.jpg', 'thumbnail_enchanted_land.jpg', 'special_offer_enchanted_land.jpg', 'fantasy, magic');

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
