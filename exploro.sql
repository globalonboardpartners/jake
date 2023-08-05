CREATE database exploro;

CREATE OR REPLACE FUNCTION update_updated_on()
RETURNS TRIGGER AS $$
BEGIN
  NEW.edited = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- formally known as "Region"
CREATE TABLE continent (
  id SERIAL PRIMARY KEY,
  name varchar(50),
  slug varchar(50),
  description_long varchar(100),
  description_short varchar(200),
  image_link TEXT,
  thumbnail_link TEXT,
  special_offer_image_link TEXT,
  video_link TEXT,
  gallery TEXT[],
  tags TEXT[],
  created TIMESTAMP DEFAULT NOW(),
  edited TIMESTAMP DEFAULT NOW()
);

CREATE TRIGGER trigger_row_edit_update_timestamp_for_continent
BEFORE UPDATE ON continent
FOR EACH ROW
EXECUTE FUNCTION update_updated_on();

CREATE TABLE country (
  id SERIAL PRIMARY KEY,
  name varchar(50),
  slug varchar(50),
  description_long varchar(100),
  description_short varchar(200),
  image_link TEXT,
  thumbnail_link TEXT,
  special_offer_image_link TEXT,
  video_link TEXT,
  gallery TEXT[],
  tags TEXT[],
  created TIMESTAMP DEFAULT NOW(),
  edited TIMESTAMP DEFAULT NOW()
);

CREATE TRIGGER trigger_row_edit_update_timestamp_for_country
BEFORE UPDATE ON country
FOR EACH ROW
EXECUTE FUNCTION update_updated_on();

CREATE TABLE region (
  id SERIAL PRIMARY KEY,
  name varchar(50),
  slug varchar(50),
  description_long varchar(100),
  description_short varchar(200),
  image_link TEXT,
  thumbnail_link TEXT,
  special_offer_image_link TEXT,
  video_link TEXT,
  gallery TEXT[],
  tags TEXT[],
  created TIMESTAMP DEFAULT NOW(),
  edited TIMESTAMP DEFAULT NOW()
);

CREATE TRIGGER trigger_row_edit_update_timestamp_for_region
BEFORE UPDATE ON country
FOR EACH ROW
EXECUTE FUNCTION update_updated_on();

CREATE TABLE city (
  id SERIAL PRIMARY KEY,
  name varchar(50),
  slug varchar(50),
  description_long varchar(100),
  description_short varchar(200),
  image_link TEXT,
  thumbnail_link TEXT,
  special_offer_image_link TEXT,
  video_link TEXT,
  gallery TEXT[],
  featured_city BOOLEAN,
  tags TEXT[],
  created TIMESTAMP DEFAULT NOW(),
  edited TIMESTAMP DEFAULT NOW()
);

CREATE TRIGGER trigger_row_edit_update_timestamp_for_city
BEFORE UPDATE ON city
FOR EACH ROW
EXECUTE FUNCTION update_updated_on();

-- formally known as "business"
CREATE TABLE partner_vendor (
  id SERIAL PRIMARY KEY,
  name varchar(50),
  slug varchar(50),
  description_short varchar(100),
  description_long varchar(200),
  video_link TEXT,
  image_link TEXT,
  image_link_2 TEXT,
  thumbnail_link TEXT,
  gallery TEXT[],
  featured_partner_vendor BOOLEAN,
  continent integer references continent(id),
  country integer references country(id),
  region integer references region(id),
  city integer references city(id),
  latatude FLOAT,
  longitude FLOAT,
  email TEXT,
  phone VARCHAR(16),
  website_link TEXT,
  tags TEXT[],
  created TIMESTAMP DEFAULT NOW(),
  edited TIMESTAMP DEFAULT NOW()
);

CREATE TRIGGER trigger_row_edit_update_timestamp_for_partner_vendor
BEFORE UPDATE ON partner_vendor
FOR EACH ROW
EXECUTE FUNCTION update_updated_on();
-- I want to change the name of the "collection_id" and "item_id" to be something significantly more descriptive
-- restaurant table has significant differences from the csv joe gave me
CREATE TABLE restaurant (
  id SERIAL PRIMARY KEY,
  name varchar(50),
  slug varchar(50),
  description_short varchar(100),
  description_long varchar(200),
  video_link TEXT,
  image_link TEXT,
  image_link_2 TEXT,
  thumbnail_link TEXT,
  gallery TEXT[],
  menu_gallery TEXT[],
  featured_restaurant BOOLEAN,
  partner_vendor_id integer references partner_vendor(id),
  continent integer references continent(id),
  country integer references country(id),
  region integer references region(id),
  city integer references city(id),
  latatude FLOAT,
  longitude FLOAT,
  email TEXT,
  phone VARCHAR(16),
  website_link TEXT,
  tags TEXT[],
  created TIMESTAMP DEFAULT NOW(),
  edited TIMESTAMP DEFAULT NOW()
);

CREATE TRIGGER trigger_row_edit_update_timestamp_for_partner_restaurant
BEFORE UPDATE ON restaurant
FOR EACH ROW
EXECUTE FUNCTION update_updated_on();

CREATE TABLE hotel (
  id SERIAL PRIMARY KEY,
  name varchar(50),
  slug varchar(50),
  description_short varchar(100),
  description_long varchar(200),
  video_link TEXT,
  image_link TEXT,
  image_link_2 TEXT,
  thumbnail_link TEXT,
  gallery TEXT[],
  featured_hotel BOOLEAN,
  partner_vendor_id integer references partner_vendor(id),
  continent integer references continent(id),
  country integer references country(id),
  region integer references region(id),
  city integer references city(id),
  latatude FLOAT,
  longitude FLOAT,
  email TEXT,
  phone VARCHAR(16),
  website_link TEXT,
  tags TEXT[],
  created TIMESTAMP DEFAULT NOW(),
  edited TIMESTAMP DEFAULT NOW()
);

CREATE TRIGGER trigger_row_edit_update_timestamp_for_partner_hotel
BEFORE UPDATE ON hotel
FOR EACH ROW
EXECUTE FUNCTION update_updated_on();

CREATE TABLE hotel_room (
  id SERIAL PRIMARY KEY,
  name varchar(50),
  hotel_id integer references hotel(id),
  description_short varchar(100),
  description_long varchar(200),
  video_link TEXT,
  image_link TEXT,
  image_link_2 TEXT,
  thumbnail_link TEXT,
  gallery TEXT[],
  tags TEXT[],
  created TIMESTAMP DEFAULT NOW(),
  edited TIMESTAMP DEFAULT NOW()
);

CREATE TRIGGER trigger_row_edit_update_timestamp_for_partner_hotel_room
BEFORE UPDATE ON hotel_room
FOR EACH ROW
EXECUTE FUNCTION update_updated_on();

CREATE TABLE activity (
  id SERIAL PRIMARY KEY,
  name varchar(50),
  slug varchar(50),
  description_short varchar(100),
  description_long varchar(200),
  video_link TEXT,
  image_link TEXT,
  image_link_2 TEXT,
  thumbnail_link TEXT,
  gallery TEXT[],
  featured_activity BOOLEAN,
  partner_vendor_id integer references partner_vendor(id),
  continent integer references continent(id),
  country integer references country(id),
  region integer references region(id),
  city integer references city(id),
  latatude FLOAT,
  longitude FLOAT,
  email TEXT,
  phone VARCHAR(16),
  website_link TEXT,
  tags TEXT[],
  created TIMESTAMP DEFAULT NOW(),
  edited TIMESTAMP DEFAULT NOW()
);

CREATE TRIGGER trigger_row_edit_update_timestamp_for_activity
BEFORE UPDATE ON activity
FOR EACH ROW
EXECUTE FUNCTION update_updated_on();

CREATE TABLE event (
  id SERIAL PRIMARY KEY,
  name varchar(50),
  slug varchar(50),
  description_short varchar(100),
  description_long varchar(200),
  video_link TEXT,
  image_link TEXT,
  image_link_2 TEXT,
  thumbnail_link TEXT,
  gallery TEXT[],
  featured_event BOOLEAN,
  ticket_link TEXT,
  partner_vendor_id integer references partner_vendor(id),
  continent integer references continent(id),
  country integer references country(id),
  region integer references region(id),
  city integer references city(id),
  latatude FLOAT,
  longitude FLOAT,
  email TEXT,
  phone VARCHAR(16),
  website_link TEXT,
  tags TEXT[],
  created TIMESTAMP DEFAULT NOW(),
  edited TIMESTAMP DEFAULT NOW()
);

CREATE TRIGGER trigger_row_edit_update_timestamp_for_event
BEFORE UPDATE ON event
FOR EACH ROW
EXECUTE FUNCTION update_updated_on();

CREATE TABLE event_details (
  id SERIAL PRIMARY KEY,
  name VARCHAR(50),
  slug VARCHAR(50),
  collection_id uuid,
  item_id uuid,
  event_date DATE,
  event_artist_slug VARCHAR(100),
  venue_name VARCHAR(100),
  event_time TIME,
  city integer references city(id),
  country integer references country(id),
  region integer references region(id),
  ticket_link TEXT,
  gallery TEXT[],
  tags TEXT[],
  created TIMESTAMP DEFAULT NOW(),
  edited TIMESTAMP DEFAULT NOW()
);

CREATE TRIGGER trigger_row_edit_update_timestamp_for_event_details
BEFORE UPDATE ON event_details
FOR EACH ROW
EXECUTE FUNCTION update_updated_on();
