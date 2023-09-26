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
  featured_city FLOAT NOT NULL,
  tags TEXT,
  created TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
  edited TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TRIGGER trigger_row_edit_update_timestamp_for_city
BEFORE UPDATE ON city
FOR EACH ROW
EXECUTE FUNCTION update_updated_on();

INSERT INTO city (
  name, 
  slug, 
  description_long, 
  description_short, 
  image_link, 
  thumbnail_link, 
  gallery, 
  featured_city
) VALUES (
  'MISSING DATA', 
  'missing-data', 
  'This is a placeholder for missing data.', 
  'Placeholder for missing data.', 
  'https://i.imgur.com/hfM1J8s.png', 
  'https://i.imgur.com/hfM1J8s.png', 
  ARRAY['https://i.imgur.com/hfM1J8s.png'], 
  0.0
);

INSERT INTO city (
  name,
  slug,
  description_long,
  description_short,
  image_link,
  thumbnail_link,
  special_offer_image_link,
  video_link,
  gallery,
  featured_city,
  tags,
  created,
  edited
)
VALUES (
  'San Francisco',
  'san-francisco',
  'San Francisco is known for its Golden Gate Bridge and tech scene.',
  'A city in Northern California, famous for tech industry and beautiful landmarks.',
  'https://example.com/image/san_francisco.jpg',
  'https://example.com/thumbnail/san_francisco_thumbnail.jpg',
  'https://example.com/special_offer/san_francisco_special.jpg',
  'https://example.com/video/san_francisco_video.mp4',
  ARRAY['img1.jpg', 'img2.jpg', 'img3.jpg'],
  1.0,
  'California,USA,Tech',
  NOW(),
  NOW()
);

INSERT INTO city (
  name, 
  slug, 
  description_long, 
  description_short, 
  image_link, 
  thumbnail_link, 
  special_offer_image_link, 
  video_link, 
  gallery, 
  featured_city, 
  tags
) 
VALUES (
  'New York', 
  'new-york', 
  'New York is known as the city that never sleeps.', 
  'NYC is a bustling metropolis.', 
  'https://example.com/image_link.jpg', 
  'https://example.com/thumbnail_link.jpg', 
  'https://example.com/special_offer_image_link.jpg', 
  'https://example.com/video_link.mp4', 
  ARRAY['img1.jpg', 'img2.jpg'], 
  1.0, 
  'tourist,metropolis'
);

INSERT INTO city (
  name,
  slug,
  description_long,
  description_short,
  image_link,
  thumbnail_link,
  special_offer_image_link,
  video_link,
  gallery,
  featured_city,
  tags
) VALUES (
  'Tokyo',
  'tokyo',
  'Tokyo is the capital city of Japan and one of the most populous cities in the world.',
  'Capital of Japan known for its modern architecture, culture, and technology.',
  'https://example.com/image/tokyo.jpg',
  'https://example.com/thumbnail/tokyo.jpg',
  'https://example.com/special_offer/tokyo.jpg',
  'https://example.com/video/tokyo.mp4',
  ARRAY['image1.jpg', 'image2.jpg', 'image3.jpg'],
  9.5,
  'capital,Japan,technology'
);
