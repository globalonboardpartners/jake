CREATE database exploro;

-- formally known as "Region"
CREATE TABLE continent (
  id SERIAL PRIMARY KEY,
  name varchar(50),
  slug varchar(50),
  description_long varchar(100),
  description_short varchar(200),
  image_link TEXT,
  thumbnail_link TEXT,
  special_offer_image_link,
  video_link TEXT,
  gallery TEXT[],
  tags TEXT[],
  created TIMESTAMP DEFAULT NOW(),
);

ALTER TABLE continent
ADD CONSTRAINT fk_continent_references_tag
FOREIGN KEY (tags)
REFERENCES tag (id);

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
)

ALTER TABLE country
ADD CONSTRAINT fk_country_references_tag
FOREIGN KEY (tags)
REFERENCES tag (id);

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
)

ALTER TABLE region
ADD CONSTRAINT fk_region_references_tag
FOREIGN KEY (tags)
REFERENCES tag (id);

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
)

ALTER TABLE city
ADD CONSTRAINT fk_city_references_tag
FOREIGN KEY (tags)
REFERENCES tag (id);

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
  featured_restaurant BOOLEAN,
  parter_vender_id refrences parter_vender(id),
  continent refrences continent(id),
  country refrences country(id),
  region refrences region(id),
  city refrences city(id),
  latatude FLOAT,
  longitude FLOAT,
  email TEXT,
  phone VARCHAR(16),
  website_link TEXT
  tags TEXT[],
  created TIMESTAMP DEFAULT NOW(),
)

ALTER TABLE partner_vendor
ADD CONSTRAINT fk_partner_vendor_references_tag
FOREIGN KEY (tags)
REFERENCES tag (id);

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
  featured_restaurant BOOLEAN,
  parter_vender_id refrences parter_vender(id),
  continent refrences continent(id),
  country refrences country(id),
  region refrences region(id),
  city refrences city(id),
  latatude FLOAT,
  longitude FLOAT,
  email TEXT,
  phone VARCHAR(16),
  website_link TEXT
  tags TEXT[],
  created TIMESTAMP DEFAULT NOW(),
)

ALTER TABLE restaurant
ADD CONSTRAINT fk_restaurant_references_tag
FOREIGN KEY (tags)
REFERENCES tag (id);

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
  featured_restaurant BOOLEAN,
  parter_vender_id refrences parter_vender(id),
  continent refrences continent(id),
  country refrences country(id),
  region refrences region(id),
  city refrences city(id),
  latatude FLOAT,
  longitude FLOAT,
  email TEXT,
  phone VARCHAR(16),
  website_link TEXT
  tags TEXT[],
  created TIMESTAMP DEFAULT NOW(),
)

ALTER TABLE hotel
ADD CONSTRAINT fk_hotel_references_tag
FOREIGN KEY (tags)
REFERENCES tag (id);

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
  featured_restaurant BOOLEAN,
  parter_vender_id refrences parter_vender(id),
  continent refrences continent(id),
  country refrences country(id),
  region refrences region(id),
  city refrences city(id),
  latatude FLOAT,
  longitude FLOAT,
  email TEXT,
  phone VARCHAR(16),
  website_link TEXT
  tags TEXT[],
  created TIMESTAMP DEFAULT NOW(),
)

ALTER TABLE activity
ADD CONSTRAINT fk_activity_references_tag
FOREIGN KEY (tags)
REFERENCES tag (id);

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
  featured_restaurant BOOLEAN,
  parter_vender_id refrences parter_vender(id),
  continent refrences continent(id),
  country refrences country(id),
  region refrences region(id),
  city refrences city(id),
  latatude FLOAT,
  longitude FLOAT,
  email TEXT,
  phone VARCHAR(16),
  website_link TEXT
  tags integer[],
  created TIMESTAMP DEFAULT NOW(),
)

ALTER TABLE event
ADD CONSTRAINT fk_event_references_tag
FOREIGN KEY (tags)
REFERENCES tag (id);

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
  city refrences city(id),
  country refrences country(id),
  region refrences region(id),
  ticket_link TEXT
  gallery TEXT[],
  tags TEXT[],
  created TIMESTAMP DEFAULT NOW(),
)

ALTER TABLE event_details
ADD CONSTRAINT fk_event_details_references_tag
FOREIGN KEY (tags)
REFERENCES tag (id);

CREATE TABLE tag (
  id SERIAL PRIMARY KEY,
  name VARCHAR(20),
)
