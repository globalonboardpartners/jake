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
  featured_event FLOAT NOT NULL,
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

CREATE TABLE event_details (
  id SERIAL PRIMARY KEY,
  name VARCHAR(50) NOT NULL,
  slug VARCHAR(50) NOT NULL,
  event_date DATE NOT NULL,
  event_artist_slug VARCHAR(100) NOT NULL,
  venue_name VARCHAR(100) NOT NULL,
  event_time TIME NOT NULL,
  continent integer references continent(id) NOT NULL,
  country integer references country(id) NOT NULL,
  region integer references region(id) NOT NULL,
  city integer references city(id) NOT NULL,
  ticket_link TEXT NOT NULL,
  gallery TEXT[] NOT NULL,
  tags TEXT,
  created TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
  edited TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TRIGGER trigger_row_edit_update_timestamp_for_event
BEFORE UPDATE ON event
FOR EACH ROW
EXECUTE FUNCTION update_updated_on();

CREATE TRIGGER trigger_row_edit_update_timestamp_for_event_details
BEFORE UPDATE ON event_details
FOR EACH ROW
EXECUTE FUNCTION update_updated_on();

INSERT INTO event (
  name, slug, description_short, description_long, 
  video_link, image_link, image_link_2, thumbnail_link,
  gallery, featured_event, partner_vendor, continent, 
  country, region, city, latitude, longitude, email,
  phone, address, website_link, tags
)
VALUES (
  'Music Festival 2023', 'music-festival-2023', 'A music festival like no other.', 
  'Come experience the most exhilarating music festival of the year.', 
  'https://example.com/video', 'https://example.com/image1', 'https://example.com/image2',
  'https://example.com/thumbnail', ARRAY['img1', 'img2', 'img3'], 
  1.0, 1, 1, 1, 1, 1, 40.7128, -74.0060, 
  'info@example.com', '+11234567890', '1234 Broadway, New York', 
  'https://example.com', 'music,festival'
);

INSERT INTO event (
  name, slug, description_short, description_long, 
  video_link, image_link, image_link_2, thumbnail_link,
  gallery, featured_event, partner_vendor, continent, 
  country, region, city, latitude, longitude, email,
  phone, address, website_link, tags
)
VALUES (
  'Burning Man', 'burning-man', 'An annual gathering in the desert.', 
  'Burning Man is an annual gathering that takes place at Black Rock City.', 
  'https://burningman.com/video', 'https://burningman.com/image1', 'https://burningman.com/image2',
  'https://burningman.com/thumbnail', ARRAY['img1', 'img2', 'img3'], 
  1.0, 1, 1, 1, 1, 1, 40.7860, -119.2069, 
  'info@burningman.com', '+11234567890', 'Black Rock Desert, Nevada', 
  'https://burningman.com', 'art,festival'
);

INSERT INTO event (
  name, slug, description_short, description_long,
  video_link, image_link, image_link_2, thumbnail_link,
  gallery, featured_event, partner_vendor, continent,
  country, region, city, latitude, longitude, email,
  phone, address, website_link, tags
)
VALUES (
  'Lollapalooza', 'lollapalooza', 'Iconic Music Festival.', 
  'Lollapalooza is an annual music festival featuring popular alternative rock, heavy metal, punk rock, hip hop, and electronic music bands and artists.',
  'https://example.com/lollapalooza-video', 'https://example.com/lollapalooza-image1', 'https://example.com/lollapalooza-image2',
  'https://example.com/lollapalooza-thumbnail', ARRAY['lolla1', 'lolla2', 'lolla3'],
  1.0, 1, 1, 1, 1, 1, 41.8781, -87.6298,
  'lolla@example.com', '+11234567890', 'Grant Park, Chicago, IL',
  'https://www.lollapalooza.com/', 'lollapalooza,music,festival'
);

-- event details
INSERT INTO event_details (
  name,
  slug,
  event_date,
  event_artist_slug,
  venue_name,
  event_time,
  continent,
  country,
  region,
  city,
  ticket_link,
  gallery,
  tags
)
VALUES (
  'Music Festival 2023',
  'music-festival-2023',
  '2023-10-01',
  'various-artists',
  'Central Park',
  '18:00:00',
  1,  -- assuming '1' refers to an existing continent ID
  1,  -- assuming '1' refers to an existing country ID
  1,  -- assuming '1' refers to an existing region ID
  1,  -- assuming '1' refers to an existing city ID
  'https://ticket-link.com/event/1',
  ARRAY['image1.jpg', 'image2.jpg'],
  'music,festival'
);
