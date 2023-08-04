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
  created TIMESTAMP DEFAULT NOW(),
)

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
  created TIMESTAMP DEFAULT NOW(),
)

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
  created TIMESTAMP DEFAULT NOW(),
)

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
  created TIMESTAMP DEFAULT NOW(),
)

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
  created TIMESTAMP DEFAULT NOW(),
)

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
  created TIMESTAMP DEFAULT NOW(),
)

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
  created TIMESTAMP DEFAULT NOW(),
)

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
  created TIMESTAMP DEFAULT NOW(),
)

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
  created TIMESTAMP DEFAULT NOW(),
)

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
  created TIMESTAMP DEFAULT NOW(),
)
