----------------------------------------
--             WARNING!               --
--  if you change the table schema,   --
--  you will have to also change the  --
--               code!                --
----------------------------------------
-- create table api key
CREATE TABLE api_key (
  id SERIAL PRIMARY KEY,
  key TEXT NOT NULL UNIQUE,
  security_level SMALLINT NOT NULL,
  name TEXT NOT NULL UNIQUE,
  created TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
  edited TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);

-- Create Team Members Table
CREATE TABLE employee (
  id SERIAL PRIMARY KEY,
  name VARCHAR(50) NOT NULL,
  slug VARCHAR(75) NOT NULL,
  position VARCHAR(50) NOT NULL,
  bio TEXT NOT NULL,
  image_url VARCHAR(255) NOT NULL,
  twitter_link VARCHAR(16),
  linkedin_link VARCHAR(70),
  email VARCHAR(100),
  created TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
  edited TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TYPE status AS ENUM ('Active', 'Inactive', 'Suspended');

CREATE TABLE test_enum (
  id SERIAL PRIMARY KEY,
  name TEXT,
  enum_col status NOT NULL DEFAULT 'Active'
);

insert into test_enum (name)  values ('test');

CREATE TABLE auth (
  id SERIAL PRIMARY KEY,
  email VARCHAR(255) UNIQUE NOT NULL,
  username VARCHAR(50) UNIQUE NOT NULL,
  password VARCHAR(128) NOT NULL,
  salt VARCHAR(32) NOT NULL,
  api_key UUID NOT NULL UNIQUE,
  token UUID UNIQUE,
  security_level SMALLINT NOT NULL,
  employee_id INT REFERENCES employee(id),
  status status DEFAULT 'Active' NOT NULL,
  last_login TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
  failed_login_attempts INT DEFAULT 0,
  created TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
  edited TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);

-- Create Blog Categories Table
CREATE TABLE blog_category (
  id SERIAL PRIMARY KEY,
  category VARCHAR(50) NOT NULL,
  slug VARCHAR(75) NOT NULL,
  created TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
  edited TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);

-- Create Blog Posts Table
CREATE TABLE blog (
  id SERIAL PRIMARY KEY,
  title VARCHAR(100) NOT NULL,
  slug VARCHAR(200) NOT NULL,
  category_id INT NOT NULL REFERENCES blog_category(id),
  content TEXT NOT NULL,
  image_link TEXT NOT NULL,
  thumbnail_link TEXT NOT NULL,
  featured BOOLEAN NOT NULL,
  created TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
  edited TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);

-- Create Job Listings Table
CREATE TABLE job_listing (
  id SERIAL PRIMARY KEY,
  title VARCHAR(100) NOT NULL,
  slug VARCHAR(150) NOT NULL,
  description TEXT NOT NULL,
  location VARCHAR(100) NOT NULL,
  employment_basis VARCHAR(30) NOT NULL,
  created TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
  edited TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);

-- Create Product Features Table
CREATE TABLE product_feature (
  id SERIAL PRIMARY KEY,
  title VARCHAR(100) NOT NULL,
  slug VARCHAR(150) NOT NULL,
  description_short TEXT NOT NULL,
  description_long TEXT NOT NULL,
  image_link TEXT NOT NULL,
  video_link TEXT NOT NULL,
  icon TEXT NOT NULL,
  quote TEXT NOT NULL,
  quote_author VARCHAR(100) NOT NULL,
  quote_author_position VARCHAR(100) NOT NULL,
  order_number INT NOT NULL,
  created TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
  edited TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TABLE client (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL,
  slug TEXT NOT NULL,
  title TEXT NOT NULL,
  description_short TEXT,
  description_long TEXT,
  logo TEXT,
  image_link TEXT,
  quote TEXT,
  quote_author TEXT,
  quote_author_position TEXT,
  number_of_employees TEXT,
  industry TEXT,
  website_link TEXT,
  features_used TEXT,
  featured BOOLEAN,
  created TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
  edited TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
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
  created TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
  edited TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
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
  created TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
  edited TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
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
  created TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
  edited TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
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
  featured_city FLOAT NOT NULL,
  tags TEXT,
  created TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
  edited TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
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
  featured_partner_vendor FLOAT NOT NULL,
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
  featured_hotel FLOAT NOT NULL,
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
  created TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
  edited TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
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

-- inserts for api_key
INSERT INTO api_key (key, security_level, name) VALUES ('e8eafe91-781e-4d5b-a615-807540518b24', 1, 'admin');

-- inserts for employee
INSERT INTO employee(
  name, slug, position, bio, image_url, twitter_link, linkedin_link, email
) VALUES (
  'Ben Fuller',
  'ben-fuller',
  'Vice President, Airline Partnerships; Americas',
  'Colleen is a seasoned marketing expert with more than 20 years of field experience guiding integrated direct-to-consumer and B2B campaigns with an acute focus on return. She has been instrumental in leading teams through the process of launching, highlighted by incredibly successful launches with My Pillow Pets and Little Giant Ladder.',
  'https://uploads-ssl.webflow.com/641881bc97e4e960fe5d8ec7/645928e2f315e403b97cd551_Ben_Cropped.jpg',
  NULL,
  'https://www.linkedin.com/in/benfuller15/',
  NULL
);

INSERT INTO employee(
  name, slug, position, bio, image_url, twitter_link, linkedin_link, email
) VALUES (
  'Colleen Ferrier',
  'colleen-ferrier',
  'Chief Executive Officer',
  'A seasoned leader with more than 20 years of field experience guiding integrated, direct-to-consumer and B2B brands with an acute focus on growth. She has been instrumental in leading teams through the incredibly successful launches of My Pillow Pets and Little Giant Ladder.',
  'https://uploads-ssl.webflow.com/641881bc97e4e960fe5d8ec7/6459268c6e76ab76c8788e87_Colleen%20Ferrier.jpg',
  NULL,
  'https://www.linkedin.com/in/colleen-ferrier/',
  NULL
);

INSERT INTO employee(
  name, slug, position, bio, image_url, twitter_link, linkedin_link, email
) VALUES (
  'Dean Hemstreet',
  'dean-hemstreet',
  'Chief Technology Officer',
  'Dean brings expertise in high volume and high transaction systems in the banking industry, internet of things, high volume eCommerce sites and mobile applications. Dean has previously held positions at SmartThings, Samsung, Omniture, and Adobe.',
  'https://uploads-ssl.webflow.com/641881bc97e4e960fe5d8ec7/64592ed2c70cb1c32b036157_Dean_cropped.jpg',
  NULL,
  'https://www.linkedin.com/in/deanhemstreet/',
  NULL
);

INSERT INTO employee(
  name, slug, position, bio, image_url, twitter_link, linkedin_link, email
) VALUES (
  'Emilio Chacon',
  'emilio-chacon',
  'Vice President, Airline Partnerships: Europe',
  'Emilio is an expert in driving change in airlines, generating diversified revenues through innovative products / technologies, negotiate impactful outcomes, and implement solutions early and under budget. He currently resides in Barcelona with his family.',
  'https://uploads-ssl.webflow.com/641881bc97e4e960fe5d8ec7/64592adb01db8ba749debfd8_Emilio%20Chacon.jpg',
  NULL,
  'https://www.linkedin.com/in/chacon/',
  NULL
);

INSERT INTO employee(
  name, slug, position, bio, image_url, twitter_link, linkedin_link, email
) VALUES (
  'Martin Harlow',
  'martin-harlow',
  'Chief Growth Officer',
  'An industry recognized expert on distribution strategy and connectivity in the lodging industry’s digital transformation. He has been part of pioneering companies that led the charge with some of the best-known brands across the hospitality industry.',
  'https://uploads-ssl.webflow.com/641881bc97e4e960fe5d8ec7/6459287ec70cb1474bfa0b33_Martin_Cropped.jpg',
  NULL,
  'https://www.linkedin.com/in/martin-harlow-12317319/',
  NULL
);

INSERT INTO employee(
  name, slug, position, bio, image_url, twitter_link, linkedin_link, email
) VALUES (
  'Tim Hackleman',
  'tim-hackleman',
  'Director of Marketing and Creative Services',
  'Tim has over 25 years of experience in branding and marketing and holds two degrees in Graphic Design. His career took flight working for such brands as Gulfstream Aerospace®, West Marine® and Cabela’s®. He is a perfect blend of data and design.',
  'https://uploads-ssl.webflow.com/641881bc97e4e960fe5d8ec7/64592c241ab9bf73eaafd9f0_Tim_Cropped.jpg',
  NULL,
  'http://www.linkedin.com/in/hackleman',
  NULL
);

-- insert for auth
INSERT INTO auth (
  email, 
  username, 
  password, 
  salt, 
  api_key, 
  token,
  security_level, 
  employee_id, 
  status, 
  failed_login_attempts
)
VALUES (
  'john.doe@example.com',  -- email
  'john_doe',             -- username
  'hashed_password_here', -- password
  'salt_here',            -- salt
  '123e4567-e89b-12d3-a456-426614174000', -- api_key as UUID
  NULL,
  1,                      -- security_level
  1,                     -- employee_id
  'Active',               -- status
  0                      -- failed_login_attempts
);

-- insert blog_category
INSERT INTO blog_category (category, slug) VALUES ('Airline News', 'airline-news');
INSERT INTO blog_category (category, slug) VALUES ('Budget Travel', 'budget-travel');
INSERT INTO blog_category (category, slug) VALUES ('Customer Stories', 'customer-stories');
INSERT INTO blog_category (category, slug) VALUES ('Travel Destinations', 'travel-destinations');
INSERT INTO blog_category (category, slug) VALUES ('Travel Tips', 'travel-tips');

-- inserts for blog
INSERT INTO blog (title, slug, category_id, content, image_link, thumbnail_link, featured, created)
VALUES (
	'A Guide to Solo Travel: How to Stay Safe and Make the Most of Your Adventure', 
	'a-guide-to-solo-travel-how-to-stay-safe-and-make-the-most-of-your-adventure', 
	1,
	$$<h3>Large Heading</h3><p>Imperdiet convallis urna, ut donec nullam tincidunt. Laoreet lacus, convallis urna luctus proin amet pulvinar. Morbi odio habitant cras pulvinar massa. Nam quam magnis neque, sit sed vulputate vulputate. In congue cras elementum vulputate ut sollicitudin felis. Dui justo rutrum tellus tortor. Ullamcorper nibh quisque ut felis diam. At nibh vitae egestas lorem metus nibh iaculis tristique. At sit sit sodales amet diam nunc, nibh eros justo. Neque, leo lacinia massa enim amet. Vel ornare scelerisque neque est pellentesque maecenas feugiat. Nunc luctus nibh risus scelerisque justo, cursus donec. Euismod enim mi condimentum nec suspendisse vitae, cras quis.</p><p>Porttitor ut mollis etiam sit id neque. Eget interdum nisl vehicula tristique vestibulum, vitae risus, porta. Egestas pretium nunc, tincidunt sollicitudin. A magnis tincidunt sed adipiscing viverra. Viverra at arcu nulla elit hendrerit hendrerit proin. Feugiat eget eu bibendum nunc semper risus urna, ut. Cras purus velit bibendum varius amet leo. Nibh ornare orci, tellus massa ut quis arcu praesent. Congue vestibulum risus tincidunt tellus sem tempus euismod. Suspendisse id venenatis est aliquet vitae, id eu pellentesque lectus. Enim scelerisque tristique aenean ipsum a pellentesque mauris lectus. Maecenas sed cum dui convallis. Gravida purus id velit mi libero, neque, tortor faucibus vulputate. At nullam mauris aliquet viverra imperdiet nibh.</p><figure class=\"w-richtext-figure-type-image w-richtext-align-center\" style=\"max-width:420px\" data-rt-type=\"image\" data-rt-align=\"center\" data-rt-max-width=\"420px\"><div><img src=\"https://uploads-ssl.webflow.com/641881bc97e4e960fe5d8ec7/641881bc97e4e9f6095d8ede_portrait-10.jpg\" loading=\"lazy\" width=\"auto\" height=\"auto\"></div><figcaption>Caption for image here.</figcaption></figure><p>Sapien facilisis quis consectetur dictum lacinia mauris viverra rhoncus. Dui penatibus pulvinar lorem vestibulum et nulla. Quisque id euismod metus dui. Sed fringilla nunc integer blandit. Ligula rhoncus felis faucibus sed nec, neque. Ac urna vel blandit ut id aliquam id quis. Sit condimentum morbi justo varius egestas mollis sapien. Nisl senectus justo tellus tempus enim, viverra. Tortor orci massa amet lorem leo sed pulvinar mauris parturient. Viverra sed vestibulum justo, posuere felis tellus euismod sagittis. Aliquam sit eget tristique tellus pellentesque adipiscing et cursus. Aliquam felis condimentum est mauris id porttitor nunc enim.</p><p>Malesuada facilisis vestibulum placerat lectus pulvinar elit eu. Risus lectus turpis in sagittis turpis mattis erat diam. Lectus at turpis nulla tincidunt nisl a. A pellentesque venenatis.</p><blockquote>“Some large quote text here from the body of the article.”</blockquote><h4>Smaller Heading</h4><p>Dapibus massa id duis commodo mauris non lacus porttitor sed. Nibh vitae, tellus eros nibh eget laoreet non. Pellentesque augue non nec at netus ut. Tincidunt nullam semper eros neque. Iaculis dictum in non ac massa. Eget sem massa aliquam sed. Hac pulvinar donec morbi eu senectus accumsan.</p><p>Feugiat aliquam elementum suspendisse eget in. Blandit ac tellus, etiam nunc. Vitae venenatis mi imperdiet arcu netus scelerisque amet nulla. Posuere dui ac pulvinar diam. Nisl senectus viverra ultricies convallis odio egestas. Odio pellentesque erat.</p><ul><li>Tincidunt nullam semper eros neque. Iaculis dictum in non ac massa. Eget sem massa aliquam sed.</li><li>Sit justo, gravida egestas tincidunt lorem curabitur lacus.</li><li>Varius diam consectetur pellentesque commodo. Volutpat gravida bibendum sit nunc. Et velit eu a, amet in ac, feugiat eget.</li></ul>$$,
	'https://uploads-ssl.webflow.com/641881bc97e4e960fe5d8ec7/642106eb4cf41ff63c370c3a_26.png',
	'https://uploads-ssl.webflow.com/641881bc97e4e960fe5d8ec7/642106eb4cf41ff63c370c3a_26.png',
	TRUE,
	'2021-08-16 03:35:17 UTC'
);

INSERT INTO blog (title, slug, category_id, content, image_link, thumbnail_link, featured, created)
VALUES (
  'Exploring the Worlds Most Beautiful Beaches: A Beach Lovers Paradise',
  'exploring-the-worlds-most-beautiful-beaches-a-beach-lovers-paradise',
  2,
	$$<h3>Large Heading</h3><p>Imperdiet convallis urna, ut donec nullam tincidunt. Laoreet lacus, convallis urna luctus proin amet pulvinar. Morbi odio habitant cras pulvinar massa. Nam quam magnis neque, sit sed vulputate vulputate. In congue cras elementum vulputate ut sollicitudin felis. Dui justo rutrum tellus tortor. Ullamcorper nibh quisque ut felis diam. At nibh vitae egestas lorem metus nibh iaculis tristique. At sit sit sodales amet diam nunc, nibh eros justo. Neque, leo lacinia massa enim amet. Vel ornare scelerisque neque est pellentesque maecenas feugiat. Nunc luctus nibh risus scelerisque justo, cursus donec. Euismod enim mi condimentum nec suspendisse vitae, cras quis.</p><p>Porttitor ut mollis etiam sit id neque. Eget interdum nisl vehicula tristique vestibulum, vitae risus, porta. Egestas pretium nunc, tincidunt sollicitudin. A magnis tincidunt sed adipiscing viverra. Viverra at arcu nulla elit hendrerit hendrerit proin. Feugiat eget eu bibendum nunc semper risus urna, ut. Cras purus velit bibendum varius amet leo. Nibh ornare orci, tellus massa ut quis arcu praesent. Congue vestibulum risus tincidunt tellus sem tempus euismod. Suspendisse id venenatis est aliquet vitae, id eu pellentesque lectus. Enim scelerisque tristique aenean ipsum a pellentesque mauris lectus. Maecenas sed cum dui convallis. Gravida purus id velit mi libero, neque, tortor faucibus vulputate. At nullam mauris aliquet viverra imperdiet nibh.</p><figure class=\"w-richtext-figure-type-image w-richtext-align-center\" style=\"max-width:420px\" data-rt-type=\"image\" data-rt-align=\"center\" data-rt-max-width=\"420px\"><div><img src=\"https://uploads-ssl.webflow.com/641881bc97e4e960fe5d8ec7/641881bc97e4e9f6095d8ede_portrait-10.jpg\" loading=\"lazy\" width=\"auto\" height=\"auto\"></div><figcaption>Caption for image here.</figcaption></figure><p>Sapien facilisis quis consectetur dictum lacinia mauris viverra rhoncus. Dui penatibus pulvinar lorem vestibulum et nulla. Quisque id euismod metus dui. Sed fringilla nunc integer blandit. Ligula rhoncus felis faucibus sed nec, neque. Ac urna vel blandit ut id aliquam id quis. Sit condimentum morbi justo varius egestas mollis sapien. Nisl senectus justo tellus tempus enim, viverra. Tortor orci massa amet lorem leo sed pulvinar mauris parturient. Viverra sed vestibulum justo, posuere felis tellus euismod sagittis. Aliquam sit eget tristique tellus pellentesque adipiscing et cursus. Aliquam felis condimentum est mauris id porttitor nunc enim.</p><p>Malesuada facilisis vestibulum placerat lectus pulvinar elit eu. Risus lectus turpis in sagittis turpis mattis erat diam. Lectus at turpis nulla tincidunt nisl a. A pellentesque venenatis.</p><blockquote>“Some large quote text here from the body of the article.”</blockquote><h4>Smaller Heading</h4><p>Dapibus massa id duis commodo mauris non lacus porttitor sed. Nibh vitae, tellus eros nibh eget laoreet non. Pellentesque augue non nec at netus ut. Tincidunt nullam semper eros neque. Iaculis dictum in non ac massa. Eget sem massa aliquam sed. Hac pulvinar donec morbi eu senectus accumsan.</p><p>Feugiat aliquam elementum suspendisse eget in. Blandit ac tellus, etiam nunc. Vitae venenatis mi imperdiet arcu netus scelerisque amet nulla. Posuere dui ac pulvinar diam. Nisl senectus viverra ultricies convallis odio egestas. Odio pellentesque erat.</p><ul><li>Tincidunt nullam semper eros neque. Iaculis dictum in non ac massa. Eget sem massa aliquam sed.</li><li>Sit justo, gravida egestas tincidunt lorem curabitur lacus.</li><li>Varius diam consectetur pellentesque commodo. Volutpat gravida bibendum sit nunc. Et velit eu a, amet in ac, feugiat eget.</li></ul>$$,
	'https://uploads-ssl.webflow.com/641881bc97e4e960fe5d8ec7/642106eb4cf41ff63c370c3a_26.png',
	'https://uploads-ssl.webflow.com/641881bc97e4e960fe5d8ec7/642106eb4cf41ff63c370c3a_26.png',
	TRUE,
	'2021-08-16 03:35:17 UTC'
);

INSERT INTO blog (title, slug, category_id, content, image_link, thumbnail_link, featured, created)
VALUES (
  'How to Travel on a Budget: Tips and Tricks for Saving Money on Your Next Trip',
  'how-to-travel-on-a-budget-tips-and-tricks-for-saving-money-on-your-next-trip',
  2,
	$$<h3>Large Heading</h3><p>Imperdiet convallis urna, ut donec nullam tincidunt. Laoreet lacus, convallis urna luctus proin amet pulvinar. Morbi odio habitant cras pulvinar massa. Nam quam magnis neque, sit sed vulputate vulputate. In congue cras elementum vulputate ut sollicitudin felis. Dui justo rutrum tellus tortor. Ullamcorper nibh quisque ut felis diam. At nibh vitae egestas lorem metus nibh iaculis tristique. At sit sit sodales amet diam nunc, nibh eros justo. Neque, leo lacinia massa enim amet. Vel ornare scelerisque neque est pellentesque maecenas feugiat. Nunc luctus nibh risus scelerisque justo, cursus donec. Euismod enim mi condimentum nec suspendisse vitae, cras quis.</p><p>Porttitor ut mollis etiam sit id neque. Eget interdum nisl vehicula tristique vestibulum, vitae risus, porta. Egestas pretium nunc, tincidunt sollicitudin. A magnis tincidunt sed adipiscing viverra. Viverra at arcu nulla elit hendrerit hendrerit proin. Feugiat eget eu bibendum nunc semper risus urna, ut. Cras purus velit bibendum varius amet leo. Nibh ornare orci, tellus massa ut quis arcu praesent. Congue vestibulum risus tincidunt tellus sem tempus euismod. Suspendisse id venenatis est aliquet vitae, id eu pellentesque lectus. Enim scelerisque tristique aenean ipsum a pellentesque mauris lectus. Maecenas sed cum dui convallis. Gravida purus id velit mi libero, neque, tortor faucibus vulputate. At nullam mauris aliquet viverra imperdiet nibh.</p><figure class=\"w-richtext-figure-type-image w-richtext-align-center\" style=\"max-width:420px\" data-rt-type=\"image\" data-rt-align=\"center\" data-rt-max-width=\"420px\"><div><img src=\"https://uploads-ssl.webflow.com/641881bc97e4e960fe5d8ec7/641881bc97e4e9f6095d8ede_portrait-10.jpg\" loading=\"lazy\" width=\"auto\" height=\"auto\"></div><figcaption>Caption for image here.</figcaption></figure><p>Sapien facilisis quis consectetur dictum lacinia mauris viverra rhoncus. Dui penatibus pulvinar lorem vestibulum et nulla. Quisque id euismod metus dui. Sed fringilla nunc integer blandit. Ligula rhoncus felis faucibus sed nec, neque. Ac urna vel blandit ut id aliquam id quis. Sit condimentum morbi justo varius egestas mollis sapien. Nisl senectus justo tellus tempus enim, viverra. Tortor orci massa amet lorem leo sed pulvinar mauris parturient. Viverra sed vestibulum justo, posuere felis tellus euismod sagittis. Aliquam sit eget tristique tellus pellentesque adipiscing et cursus. Aliquam felis condimentum est mauris id porttitor nunc enim.</p><p>Malesuada facilisis vestibulum placerat lectus pulvinar elit eu. Risus lectus turpis in sagittis turpis mattis erat diam. Lectus at turpis nulla tincidunt nisl a. A pellentesque venenatis.</p><blockquote>“Some large quote text here from the body of the article.”</blockquote><h4>Smaller Heading</h4><p>Dapibus massa id duis commodo mauris non lacus porttitor sed. Nibh vitae, tellus eros nibh eget laoreet non. Pellentesque augue non nec at netus ut. Tincidunt nullam semper eros neque. Iaculis dictum in non ac massa. Eget sem massa aliquam sed. Hac pulvinar donec morbi eu senectus accumsan.</p><p>Feugiat aliquam elementum suspendisse eget in. Blandit ac tellus, etiam nunc. Vitae venenatis mi imperdiet arcu netus scelerisque amet nulla. Posuere dui ac pulvinar diam. Nisl senectus viverra ultricies convallis odio egestas. Odio pellentesque erat.</p><ul><li>Tincidunt nullam semper eros neque. Iaculis dictum in non ac massa. Eget sem massa aliquam sed.</li><li>Sit justo, gravida egestas tincidunt lorem curabitur lacus.</li><li>Varius diam consectetur pellentesque commodo. Volutpat gravida bibendum sit nunc. Et velit eu a, amet in ac, feugiat eget.</li></ul>$$,
	'https://uploads-ssl.webflow.com/641881bc97e4e960fe5d8ec7/642106eb4cf41ff63c370c3a_26.png',
	'https://uploads-ssl.webflow.com/641881bc97e4e960fe5d8ec7/642106eb4cf41ff63c370c3a_26.png',
	TRUE,
	'2021-08-16 03:35:17 UTC'
);

INSERT INTO blog (title, slug, category_id, content, image_link, thumbnail_link, featured, created)
VALUES (
  'Island Hopping in the Caribbean: A Guide to the Best Islands and Activities',
  'island-hopping-in-the-caribbean-a-guide-to-the-best-islands-and-activities',
  2,
	$$<h3>Large Heading</h3><p>Imperdiet convallis urna, ut donec nullam tincidunt. Laoreet lacus, convallis urna luctus proin amet pulvinar. Morbi odio habitant cras pulvinar massa. Nam quam magnis neque, sit sed vulputate vulputate. In congue cras elementum vulputate ut sollicitudin felis. Dui justo rutrum tellus tortor. Ullamcorper nibh quisque ut felis diam. At nibh vitae egestas lorem metus nibh iaculis tristique. At sit sit sodales amet diam nunc, nibh eros justo. Neque, leo lacinia massa enim amet. Vel ornare scelerisque neque est pellentesque maecenas feugiat. Nunc luctus nibh risus scelerisque justo, cursus donec. Euismod enim mi condimentum nec suspendisse vitae, cras quis.</p><p>Porttitor ut mollis etiam sit id neque. Eget interdum nisl vehicula tristique vestibulum, vitae risus, porta. Egestas pretium nunc, tincidunt sollicitudin. A magnis tincidunt sed adipiscing viverra. Viverra at arcu nulla elit hendrerit hendrerit proin. Feugiat eget eu bibendum nunc semper risus urna, ut. Cras purus velit bibendum varius amet leo. Nibh ornare orci, tellus massa ut quis arcu praesent. Congue vestibulum risus tincidunt tellus sem tempus euismod. Suspendisse id venenatis est aliquet vitae, id eu pellentesque lectus. Enim scelerisque tristique aenean ipsum a pellentesque mauris lectus. Maecenas sed cum dui convallis. Gravida purus id velit mi libero, neque, tortor faucibus vulputate. At nullam mauris aliquet viverra imperdiet nibh.</p><figure class=\"w-richtext-figure-type-image w-richtext-align-center\" style=\"max-width:420px\" data-rt-type=\"image\" data-rt-align=\"center\" data-rt-max-width=\"420px\"><div><img src=\"https://uploads-ssl.webflow.com/641881bc97e4e960fe5d8ec7/641881bc97e4e9f6095d8ede_portrait-10.jpg\" loading=\"lazy\" width=\"auto\" height=\"auto\"></div><figcaption>Caption for image here.</figcaption></figure><p>Sapien facilisis quis consectetur dictum lacinia mauris viverra rhoncus. Dui penatibus pulvinar lorem vestibulum et nulla. Quisque id euismod metus dui. Sed fringilla nunc integer blandit. Ligula rhoncus felis faucibus sed nec, neque. Ac urna vel blandit ut id aliquam id quis. Sit condimentum morbi justo varius egestas mollis sapien. Nisl senectus justo tellus tempus enim, viverra. Tortor orci massa amet lorem leo sed pulvinar mauris parturient. Viverra sed vestibulum justo, posuere felis tellus euismod sagittis. Aliquam sit eget tristique tellus pellentesque adipiscing et cursus. Aliquam felis condimentum est mauris id porttitor nunc enim.</p><p>Malesuada facilisis vestibulum placerat lectus pulvinar elit eu. Risus lectus turpis in sagittis turpis mattis erat diam. Lectus at turpis nulla tincidunt nisl a. A pellentesque venenatis.</p><blockquote>“Some large quote text here from the body of the article.”</blockquote><h4>Smaller Heading</h4><p>Dapibus massa id duis commodo mauris non lacus porttitor sed. Nibh vitae, tellus eros nibh eget laoreet non. Pellentesque augue non nec at netus ut. Tincidunt nullam semper eros neque. Iaculis dictum in non ac massa. Eget sem massa aliquam sed. Hac pulvinar donec morbi eu senectus accumsan.</p><p>Feugiat aliquam elementum suspendisse eget in. Blandit ac tellus, etiam nunc. Vitae venenatis mi imperdiet arcu netus scelerisque amet nulla. Posuere dui ac pulvinar diam. Nisl senectus viverra ultricies convallis odio egestas. Odio pellentesque erat.</p><ul><li>Tincidunt nullam semper eros neque. Iaculis dictum in non ac massa. Eget sem massa aliquam sed.</li><li>Sit justo, gravida egestas tincidunt lorem curabitur lacus.</li><li>Varius diam consectetur pellentesque commodo. Volutpat gravida bibendum sit nunc. Et velit eu a, amet in ac, feugiat eget.</li></ul>$$,
	'https://uploads-ssl.webflow.com/641881bc97e4e960fe5d8ec7/642106eb4cf41ff63c370c3a_26.png',
	'https://uploads-ssl.webflow.com/641881bc97e4e960fe5d8ec7/642106eb4cf41ff63c370c3a_26.png',
	FALSE,
	'2021-08-16 03:35:17 UTC'
);

INSERT INTO blog (title, slug, category_id, content, image_link, thumbnail_link, featured, created)
VALUES (
  'The Best Local Cuisine to Try in Asia: A Foodies Guide',
  'the-best-local-cuisine-to-try-in-asia-a-foodies-guide',
  2,
	$$<h3>Large Heading</h3><p>Imperdiet convallis urna, ut donec nullam tincidunt. Laoreet lacus, convallis urna luctus proin amet pulvinar. Morbi odio habitant cras pulvinar massa. Nam quam magnis neque, sit sed vulputate vulputate. In congue cras elementum vulputate ut sollicitudin felis. Dui justo rutrum tellus tortor. Ullamcorper nibh quisque ut felis diam. At nibh vitae egestas lorem metus nibh iaculis tristique. At sit sit sodales amet diam nunc, nibh eros justo. Neque, leo lacinia massa enim amet. Vel ornare scelerisque neque est pellentesque maecenas feugiat. Nunc luctus nibh risus scelerisque justo, cursus donec. Euismod enim mi condimentum nec suspendisse vitae, cras quis.</p><p>Porttitor ut mollis etiam sit id neque. Eget interdum nisl vehicula tristique vestibulum, vitae risus, porta. Egestas pretium nunc, tincidunt sollicitudin. A magnis tincidunt sed adipiscing viverra. Viverra at arcu nulla elit hendrerit hendrerit proin. Feugiat eget eu bibendum nunc semper risus urna, ut. Cras purus velit bibendum varius amet leo. Nibh ornare orci, tellus massa ut quis arcu praesent. Congue vestibulum risus tincidunt tellus sem tempus euismod. Suspendisse id venenatis est aliquet vitae, id eu pellentesque lectus. Enim scelerisque tristique aenean ipsum a pellentesque mauris lectus. Maecenas sed cum dui convallis. Gravida purus id velit mi libero, neque, tortor faucibus vulputate. At nullam mauris aliquet viverra imperdiet nibh.</p><figure class=\"w-richtext-figure-type-image w-richtext-align-center\" style=\"max-width:420px\" data-rt-type=\"image\" data-rt-align=\"center\" data-rt-max-width=\"420px\"><div><img src=\"https://uploads-ssl.webflow.com/641881bc97e4e960fe5d8ec7/641881bc97e4e9f6095d8ede_portrait-10.jpg\" loading=\"lazy\" width=\"auto\" height=\"auto\"></div><figcaption>Caption for image here.</figcaption></figure><p>Sapien facilisis quis consectetur dictum lacinia mauris viverra rhoncus. Dui penatibus pulvinar lorem vestibulum et nulla. Quisque id euismod metus dui. Sed fringilla nunc integer blandit. Ligula rhoncus felis faucibus sed nec, neque. Ac urna vel blandit ut id aliquam id quis. Sit condimentum morbi justo varius egestas mollis sapien. Nisl senectus justo tellus tempus enim, viverra. Tortor orci massa amet lorem leo sed pulvinar mauris parturient. Viverra sed vestibulum justo, posuere felis tellus euismod sagittis. Aliquam sit eget tristique tellus pellentesque adipiscing et cursus. Aliquam felis condimentum est mauris id porttitor nunc enim.</p><p>Malesuada facilisis vestibulum placerat lectus pulvinar elit eu. Risus lectus turpis in sagittis turpis mattis erat diam. Lectus at turpis nulla tincidunt nisl a. A pellentesque venenatis.</p><blockquote>“Some large quote text here from the body of the article.”</blockquote><h4>Smaller Heading</h4><p>Dapibus massa id duis commodo mauris non lacus porttitor sed. Nibh vitae, tellus eros nibh eget laoreet non. Pellentesque augue non nec at netus ut. Tincidunt nullam semper eros neque. Iaculis dictum in non ac massa. Eget sem massa aliquam sed. Hac pulvinar donec morbi eu senectus accumsan.</p><p>Feugiat aliquam elementum suspendisse eget in. Blandit ac tellus, etiam nunc. Vitae venenatis mi imperdiet arcu netus scelerisque amet nulla. Posuere dui ac pulvinar diam. Nisl senectus viverra ultricies convallis odio egestas. Odio pellentesque erat.</p><ul><li>Tincidunt nullam semper eros neque. Iaculis dictum in non ac massa. Eget sem massa aliquam sed.</li><li>Sit justo, gravida egestas tincidunt lorem curabitur lacus.</li><li>Varius diam consectetur pellentesque commodo. Volutpat gravida bibendum sit nunc. Et velit eu a, amet in ac, feugiat eget.</li></ul>$$,
	'https://uploads-ssl.webflow.com/641881bc97e4e960fe5d8ec7/642106eb4cf41ff63c370c3a_26.png',
	'https://uploads-ssl.webflow.com/641881bc97e4e960fe5d8ec7/642106eb4cf41ff63c370c3a_26.png',
	FALSE,
	'2021-08-16 03:35:17 UTC'
);

INSERT INTO blog (title, slug, category_id, content, image_link, thumbnail_link, featured, created)
VALUES (
  'The Rise of Sustainable Tourism: How to Travel Responsibly',
  'the-rise-of-sustainable-tourism-how-to-travel-responsibly',
  2,
	$$<h3>Large Heading</h3><p>Imperdiet convallis urna, ut donec nullam tincidunt. Laoreet lacus, convallis urna luctus proin amet pulvinar. Morbi odio habitant cras pulvinar massa. Nam quam magnis neque, sit sed vulputate vulputate. In congue cras elementum vulputate ut sollicitudin felis. Dui justo rutrum tellus tortor. Ullamcorper nibh quisque ut felis diam. At nibh vitae egestas lorem metus nibh iaculis tristique. At sit sit sodales amet diam nunc, nibh eros justo. Neque, leo lacinia massa enim amet. Vel ornare scelerisque neque est pellentesque maecenas feugiat. Nunc luctus nibh risus scelerisque justo, cursus donec. Euismod enim mi condimentum nec suspendisse vitae, cras quis.</p><p>Porttitor ut mollis etiam sit id neque. Eget interdum nisl vehicula tristique vestibulum, vitae risus, porta. Egestas pretium nunc, tincidunt sollicitudin. A magnis tincidunt sed adipiscing viverra. Viverra at arcu nulla elit hendrerit hendrerit proin. Feugiat eget eu bibendum nunc semper risus urna, ut. Cras purus velit bibendum varius amet leo. Nibh ornare orci, tellus massa ut quis arcu praesent. Congue vestibulum risus tincidunt tellus sem tempus euismod. Suspendisse id venenatis est aliquet vitae, id eu pellentesque lectus. Enim scelerisque tristique aenean ipsum a pellentesque mauris lectus. Maecenas sed cum dui convallis. Gravida purus id velit mi libero, neque, tortor faucibus vulputate. At nullam mauris aliquet viverra imperdiet nibh.</p><figure class=\"w-richtext-figure-type-image w-richtext-align-center\" style=\"max-width:420px\" data-rt-type=\"image\" data-rt-align=\"center\" data-rt-max-width=\"420px\"><div><img src=\"https://uploads-ssl.webflow.com/641881bc97e4e960fe5d8ec7/641881bc97e4e9f6095d8ede_portrait-10.jpg\" loading=\"lazy\" width=\"auto\" height=\"auto\"></div><figcaption>Caption for image here.</figcaption></figure><p>Sapien facilisis quis consectetur dictum lacinia mauris viverra rhoncus. Dui penatibus pulvinar lorem vestibulum et nulla. Quisque id euismod metus dui. Sed fringilla nunc integer blandit. Ligula rhoncus felis faucibus sed nec, neque. Ac urna vel blandit ut id aliquam id quis. Sit condimentum morbi justo varius egestas mollis sapien. Nisl senectus justo tellus tempus enim, viverra. Tortor orci massa amet lorem leo sed pulvinar mauris parturient. Viverra sed vestibulum justo, posuere felis tellus euismod sagittis. Aliquam sit eget tristique tellus pellentesque adipiscing et cursus. Aliquam felis condimentum est mauris id porttitor nunc enim.</p><p>Malesuada facilisis vestibulum placerat lectus pulvinar elit eu. Risus lectus turpis in sagittis turpis mattis erat diam. Lectus at turpis nulla tincidunt nisl a. A pellentesque venenatis.</p><blockquote>“Some large quote text here from the body of the article.”</blockquote><h4>Smaller Heading</h4><p>Dapibus massa id duis commodo mauris non lacus porttitor sed. Nibh vitae, tellus eros nibh eget laoreet non. Pellentesque augue non nec at netus ut. Tincidunt nullam semper eros neque. Iaculis dictum in non ac massa. Eget sem massa aliquam sed. Hac pulvinar donec morbi eu senectus accumsan.</p><p>Feugiat aliquam elementum suspendisse eget in. Blandit ac tellus, etiam nunc. Vitae venenatis mi imperdiet arcu netus scelerisque amet nulla. Posuere dui ac pulvinar diam. Nisl senectus viverra ultricies convallis odio egestas. Odio pellentesque erat.</p><ul><li>Tincidunt nullam semper eros neque. Iaculis dictum in non ac massa. Eget sem massa aliquam sed.</li><li>Sit justo, gravida egestas tincidunt lorem curabitur lacus.</li><li>Varius diam consectetur pellentesque commodo. Volutpat gravida bibendum sit nunc. Et velit eu a, amet in ac, feugiat eget.</li></ul>$$,
	'https://uploads-ssl.webflow.com/641881bc97e4e960fe5d8ec7/642106eb4cf41ff63c370c3a_26.png',
	'https://uploads-ssl.webflow.com/641881bc97e4e960fe5d8ec7/642106eb4cf41ff63c370c3a_26.png',
	FALSE,
	'2021-08-16 03:35:17 UTC'
);

INSERT INTO blog (title, slug, category_id, content, image_link, thumbnail_link, featured, created)
VALUES (
  'Top 10 Must-Visit Destinations in Europe This Summer',
  'top-10-must-visit-destinations-in-europe-this-summer',
  3,
	$$<h3>Large Heading</h3><p>Imperdiet convallis urna, ut donec nullam tincidunt. Laoreet lacus, convallis urna luctus proin amet pulvinar. Morbi odio habitant cras pulvinar massa. Nam quam magnis neque, sit sed vulputate vulputate. In congue cras elementum vulputate ut sollicitudin felis. Dui justo rutrum tellus tortor. Ullamcorper nibh quisque ut felis diam. At nibh vitae egestas lorem metus nibh iaculis tristique. At sit sit sodales amet diam nunc, nibh eros justo. Neque, leo lacinia massa enim amet. Vel ornare scelerisque neque est pellentesque maecenas feugiat. Nunc luctus nibh risus scelerisque justo, cursus donec. Euismod enim mi condimentum nec suspendisse vitae, cras quis.</p><p>Porttitor ut mollis etiam sit id neque. Eget interdum nisl vehicula tristique vestibulum, vitae risus, porta. Egestas pretium nunc, tincidunt sollicitudin. A magnis tincidunt sed adipiscing viverra. Viverra at arcu nulla elit hendrerit hendrerit proin. Feugiat eget eu bibendum nunc semper risus urna, ut. Cras purus velit bibendum varius amet leo. Nibh ornare orci, tellus massa ut quis arcu praesent. Congue vestibulum risus tincidunt tellus sem tempus euismod. Suspendisse id venenatis est aliquet vitae, id eu pellentesque lectus. Enim scelerisque tristique aenean ipsum a pellentesque mauris lectus. Maecenas sed cum dui convallis. Gravida purus id velit mi libero, neque, tortor faucibus vulputate. At nullam mauris aliquet viverra imperdiet nibh.</p><figure class=\"w-richtext-figure-type-image w-richtext-align-center\" style=\"max-width:420px\" data-rt-type=\"image\" data-rt-align=\"center\" data-rt-max-width=\"420px\"><div><img src=\"https://uploads-ssl.webflow.com/641881bc97e4e960fe5d8ec7/641881bc97e4e9f6095d8ede_portrait-10.jpg\" loading=\"lazy\" width=\"auto\" height=\"auto\"></div><figcaption>Caption for image here.</figcaption></figure><p>Sapien facilisis quis consectetur dictum lacinia mauris viverra rhoncus. Dui penatibus pulvinar lorem vestibulum et nulla. Quisque id euismod metus dui. Sed fringilla nunc integer blandit. Ligula rhoncus felis faucibus sed nec, neque. Ac urna vel blandit ut id aliquam id quis. Sit condimentum morbi justo varius egestas mollis sapien. Nisl senectus justo tellus tempus enim, viverra. Tortor orci massa amet lorem leo sed pulvinar mauris parturient. Viverra sed vestibulum justo, posuere felis tellus euismod sagittis. Aliquam sit eget tristique tellus pellentesque adipiscing et cursus. Aliquam felis condimentum est mauris id porttitor nunc enim.</p><p>Malesuada facilisis vestibulum placerat lectus pulvinar elit eu. Risus lectus turpis in sagittis turpis mattis erat diam. Lectus at turpis nulla tincidunt nisl a. A pellentesque venenatis.</p><blockquote>“Some large quote text here from the body of the article.”</blockquote><h4>Smaller Heading</h4><p>Dapibus massa id duis commodo mauris non lacus porttitor sed. Nibh vitae, tellus eros nibh eget laoreet non. Pellentesque augue non nec at netus ut. Tincidunt nullam semper eros neque. Iaculis dictum in non ac massa. Eget sem massa aliquam sed. Hac pulvinar donec morbi eu senectus accumsan.</p><p>Feugiat aliquam elementum suspendisse eget in. Blandit ac tellus, etiam nunc. Vitae venenatis mi imperdiet arcu netus scelerisque amet nulla. Posuere dui ac pulvinar diam. Nisl senectus viverra ultricies convallis odio egestas. Odio pellentesque erat.</p><ul><li>Tincidunt nullam semper eros neque. Iaculis dictum in non ac massa. Eget sem massa aliquam sed.</li><li>Sit justo, gravida egestas tincidunt lorem curabitur lacus.</li><li>Varius diam consectetur pellentesque commodo. Volutpat gravida bibendum sit nunc. Et velit eu a, amet in ac, feugiat eget.</li></ul>$$,
	'https://uploads-ssl.webflow.com/641881bc97e4e960fe5d8ec7/642106eb4cf41ff63c370c3a_26.png',
	'https://uploads-ssl.webflow.com/641881bc97e4e960fe5d8ec7/642106eb4cf41ff63c370c3a_26.png',
	FALSE,
	'2021-08-16 03:35:17 UTC'
);

-- insert job listing:
INSERT INTO job_listing (title, slug, description, location, employment_basis, created) VALUES ('CRM Specialist', 'crm-specialist', $$<h4>About the position</h4><p>Erat eleifend lacus mattis at porttitor at mauris vel pharetra. Consequat, dictum et magna augue. Risus maecenas morbi ante scelerisque consequat. Id mi porttitor dui platea mauris. Accumsan, consequat feugiat at vitae laoreet pulvinar volutpat. Eros, mi tincidunt lorem donec aenean. Facilisis velit eget morbi urna, in sed viverra sagittis dolor. Ultrices sem mattis eget faucibus tortor, cras. Ipsum ac volutpat, nibh odio penatibus rhoncus, massa, leo tincidunt. Auctor diam massa a, elementum quis libero eleifend mauris porttitor.</p><p>Gravida pretium et, vestibulum ultrices est faucibus ullamcorper. Enim, egestas nulla vitae risus facilisis senectus maecenas bibendum.</p><h4>Main responsibilities</h4><p>Dapibus massa id duis commodo mauris non lacus porttitor sed. Nibh vitae, tellus eros nibh eget laoreet non. Pellentesque augue non nec at netus ut. Tincidunt nullam semper eros neque. Iaculis dictum in non ac massa. Eget sem massa aliquam sed. Hac pulvinar donec morbi eu senectus accumsan.</p><p>Feugiat aliquam elementum suspendisse eget in. Blandit ac tellus, etiam nunc. Vitae venenatis mi imperdiet arcu netus scelerisque amet nulla. Posuere dui ac pulvinar diam. Nisl senectus viverra ultricies convallis odio egestas. Odio pellentesque erat.</p><ul><li>Tincidunt nullam semper eros neque. Iaculis dictum in non ac massa. Eget sem massa aliquam sed.</li><li>Sit justo, gravida egestas tincidunt lorem curabitur lacus.</li><li>Varius diam consectetur pellentesque commodo. Volutpat gravida bibendum sit nunc. Et velit eu a, amet in ac, feugiat eget.</li></ul>$$, 'Remote', 'Full Time', '2023-03-20T21:06:25.000Z');
INSERT INTO job_listing (title, slug, description, location, employment_basis, created) VALUES ('Director of Lead Generation', 'director-of-lead-generation', $$<h4>About the position</h4><p>Erat eleifend lacus mattis at porttitor at mauris vel pharetra. Consequat, dictum et magna augue. Risus maecenas morbi ante scelerisque consequat. Id mi porttitor dui platea mauris. Accumsan, consequat feugiat at vitae laoreet pulvinar volutpat. Eros, mi tincidunt lorem donec aenean. Facilisis velit eget morbi urna, in sed viverra sagittis dolor. Ultrices sem mattis eget faucibus tortor, cras. Ipsum ac volutpat, nibh odio penatibus rhoncus, massa, leo tincidunt. Auctor diam massa a, elementum quis libero eleifend mauris porttitor.</p><p>Gravida pretium et, vestibulum ultrices est faucibus ullamcorper. Enim, egestas nulla vitae risus facilisis senectus maecenas bibendum.</p><h4>Main responsibilities</h4><p>Dapibus massa id duis commodo mauris non lacus porttitor sed. Nibh vitae, tellus eros nibh eget laoreet non. Pellentesque augue non nec at netus ut. Tincidunt nullam semper eros neque. Iaculis dictum in non ac massa. Eget sem massa aliquam sed. Hac pulvinar donec morbi eu senectus accumsan.</p><p>Feugiat aliquam elementum suspendisse eget in. Blandit ac tellus, etiam nunc. Vitae venenatis mi imperdiet arcu netus scelerisque amet nulla. Posuere dui ac pulvinar diam. Nisl senectus viverra ultricies convallis odio egestas. Odio pellentesque erat.</p><ul><li>Tincidunt nullam semper eros neque. Iaculis dictum in non ac massa. Eget sem massa aliquam sed.</li><li>Sit justo, gravida egestas tincidunt lorem curabitur lacus.</li><li>Varius diam consectetur pellentesque commodo. Volutpat gravida bibendum sit nunc. Et velit eu a, amet in ac, feugiat eget.</li></ul>$$, 'Remote', 'Full Time', '2023-03-20T21:06:25.000Z');
INSERT INTO job_listing (title, slug, description, location, employment_basis, created) VALUES ('Mid-Market Account Executive', 'mid-market-account-executive', $$<h4>About the position</h4><p>Erat eleifend lacus mattis at porttitor at mauris vel pharetra. Consequat, dictum et magna augue. Risus maecenas morbi ante scelerisque consequat. Id mi porttitor dui platea mauris. Accumsan, consequat feugiat at vitae laoreet pulvinar volutpat. Eros, mi tincidunt lorem donec aenean. Facilisis velit eget morbi urna, in sed viverra sagittis dolor. Ultrices sem mattis eget faucibus tortor, cras. Ipsum ac volutpat, nibh odio penatibus rhoncus, massa, leo tincidunt. Auctor diam massa a, elementum quis libero eleifend mauris porttitor.</p><p>Gravida pretium et, vestibulum ultrices est faucibus ullamcorper. Enim, egestas nulla vitae risus facilisis senectus maecenas bibendum.</p><h4>Main responsibilities</h4><p>Dapibus massa id duis commodo mauris non lacus porttitor sed. Nibh vitae, tellus eros nibh eget laoreet non. Pellentesque augue non nec at netus ut. Tincidunt nullam semper eros neque. Iaculis dictum in non ac massa. Eget sem massa aliquam sed. Hac pulvinar donec morbi eu senectus accumsan.</p><p>Feugiat aliquam elementum suspendisse eget in. Blandit ac tellus, etiam nunc. Vitae venenatis mi imperdiet arcu netus scelerisque amet nulla. Posuere dui ac pulvinar diam. Nisl senectus viverra ultricies convallis odio egestas. Odio pellentesque erat.</p><ul><li>Tincidunt nullam semper eros neque. Iaculis dictum in non ac massa. Eget sem massa aliquam sed.</li><li>Sit justo, gravida egestas tincidunt lorem curabitur lacus.</li><li>Varius diam consectetur pellentesque commodo. Volutpat gravida bibendum sit nunc. Et velit eu a, amet in ac, feugiat eget.</li></ul>$$, 'Remote', 'Full Time', '2023-03-20T21:06:25.000Z');
INSERT INTO job_listing (title, slug, description, location, employment_basis, created) VALUES ('Senior Account Executive', 'senior-account-executive', $$<h4>About the position</h4><p>Erat eleifend lacus mattis at porttitor at mauris vel pharetra. Consequat, dictum et magna augue. Risus maecenas morbi ante scelerisque consequat. Id mi porttitor dui platea mauris. Accumsan, consequat feugiat at vitae laoreet pulvinar volutpat. Eros, mi tincidunt lorem donec aenean. Facilisis velit eget morbi urna, in sed viverra sagittis dolor. Ultrices sem mattis eget faucibus tortor, cras. Ipsum ac volutpat, nibh odio penatibus rhoncus, massa, leo tincidunt. Auctor diam massa a, elementum quis libero eleifend mauris porttitor.</p><p>Gravida pretium et, vestibulum ultrices est faucibus ullamcorper. Enim, egestas nulla vitae risus facilisis senectus maecenas bibendum.</p><h4>Main responsibilities</h4><p>Dapibus massa id duis commodo mauris non lacus porttitor sed. Nibh vitae, tellus eros nibh eget laoreet non. Pellentesque augue non nec at netus ut. Tincidunt nullam semper eros neque. Iaculis dictum in non ac massa. Eget sem massa aliquam sed. Hac pulvinar donec morbi eu senectus accumsan.</p><p>Feugiat aliquam elementum suspendisse eget in. Blandit ac tellus, etiam nunc. Vitae venenatis mi imperdiet arcu netus scelerisque amet nulla. Posuere dui ac pulvinar diam. Nisl senectus viverra ultricies convallis odio egestas. Odio pellentesque erat.</p><ul><li>Tincidunt nullam semper eros neque. Iaculis dictum in non ac massa. Eget sem massa aliquam sed.</li><li>Sit justo, gravida egestas tincidunt lorem curabitur lacus.</li><li>Varius diam consectetur pellentesque commodo. Volutpat gravida bibendum sit nunc. Et velit eu a, amet in ac, feugiat eget.</li></ul>$$, 'Remote', 'Full Time', '2023-03-20T21:06:25.000Z');
INSERT INTO job_listing (title, slug, description, location, employment_basis, created) VALUES ('Senior Product Software Engineer', 'senior-product-software-engineer', $$<h4>About the position</h4><p>Erat eleifend lacus mattis at porttitor at mauris vel pharetra. Consequat, dictum et magna augue. Risus maecenas morbi ante scelerisque consequat. Id mi porttitor dui platea mauris. Accumsan, consequat feugiat at vitae laoreet pulvinar volutpat. Eros, mi tincidunt lorem donec aenean. Facilisis velit eget morbi urna, in sed viverra sagittis dolor. Ultrices sem mattis eget faucibus tortor, cras. Ipsum ac volutpat, nibh odio penatibus rhoncus, massa, leo tincidunt. Auctor diam massa a, elementum quis libero eleifend mauris porttitor.</p><p>Gravida pretium et, vestibulum ultrices est faucibus ullamcorper. Enim, egestas nulla vitae risus facilisis senectus maecenas bibendum.</p><h4>Main responsibilities</h4><p>Dapibus massa id duis commodo mauris non lacus porttitor sed. Nibh vitae, tellus eros nibh eget laoreet non. Pellentesque augue non nec at netus ut. Tincidunt nullam semper eros neque. Iaculis dictum in non ac massa. Eget sem massa aliquam sed. Hac pulvinar donec morbi eu senectus accumsan.</p><p>Feugiat aliquam elementum suspendisse eget in. Blandit ac tellus, etiam nunc. Vitae venenatis mi imperdiet arcu netus scelerisque amet nulla. Posuere dui ac pulvinar diam. Nisl senectus viverra ultricies convallis odio egestas. Odio pellentesque erat.</p><ul><li>Tincidunt nullam semper eros neque. Iaculis dictum in non ac massa. Eget sem massa aliquam sed.</li><li>Sit justo, gravida egestas tincidunt lorem curabitur lacus.</li><li>Varius diam consectetur pellentesque commodo. Volutpat gravida bibendum sit nunc. Et velit eu a, amet in ac, feugiat eget.</li></ul>$$, 'Remote', 'Full Time', '2023-03-20T21:06:25.000Z');
INSERT INTO job_listing (title, slug, description, location, employment_basis, created) VALUES ('Strategic Partnership Manager', 'strategic-partnership-manager', $$<h4>About the position</h4><p>Erat eleifend lacus mattis at porttitor at mauris vel pharetra. Consequat, dictum et magna augue. Risus maecenas morbi ante scelerisque consequat. Id mi porttitor dui platea mauris. Accumsan, consequat feugiat at vitae laoreet pulvinar volutpat. Eros, mi tincidunt lorem donec aenean. Facilisis velit eget morbi urna, in sed viverra sagittis dolor. Ultrices sem mattis eget faucibus tortor, cras. Ipsum ac volutpat, nibh odio penatibus rhoncus, massa, leo tincidunt. Auctor diam massa a, elementum quis libero eleifend mauris porttitor.</p><p>Gravida pretium et, vestibulum ultrices est faucibus ullamcorper. Enim, egestas nulla vitae risus facilisis senectus maecenas bibendum.</p><h4>Main responsibilities</h4><p>Dapibus massa id duis commodo mauris non lacus porttitor sed. Nibh vitae, tellus eros nibh eget laoreet non. Pellentesque augue non nec at netus ut. Tincidunt nullam semper eros neque. Iaculis dictum in non ac massa. Eget sem massa aliquam sed. Hac pulvinar donec morbi eu senectus accumsan.</p><p>Feugiat aliquam elementum suspendisse eget in. Blandit ac tellus, etiam nunc. Vitae venenatis mi imperdiet arcu netus scelerisque amet nulla. Posuere dui ac pulvinar diam. Nisl senectus viverra ultricies convallis odio egestas. Odio pellentesque erat.</p><ul><li>Tincidunt nullam semper eros neque. Iaculis dictum in non ac massa. Eget sem massa aliquam sed.</li><li>Sit justo, gravida egestas tincidunt lorem curabitur lacus.</li><li>Varius diam consectetur pellentesque commodo. Volutpat gravida bibendum sit nunc. Et velit eu a, amet in ac, feugiat eget.</li></ul>$$, 'Remote', 'Full Time', '2023-03-20T21:06:25.000Z');

-- insert product feature:
INSERT INTO product_feature (title, slug, description_short, description_long, image_link, video_link, icon, quote, quote_author, quote_author_position, order_number) VALUES ('Easy Booking', 'easy-booking', 'Passengers can easily book activities and services directly through the app, making travel planning hassle-free and convenient.', $$<h2>Main Heading</h2><p>Erat eleifend lacus mattis at porttitor at mauris vel pharetra. Consequat, dictum et magna augue. Risus maecenas morbi ante scelerisque consequat. Id mi porttitor dui platea mauris. Accumsan, consequat feugiat at vitae laoreet pulvinar volutpat. Eros, mi tincidunt lorem donec aenean. Facilisis velit eget morbi urna, in sed viverra sagittis dolor. Ultrices sem mattis eget faucibus tortor, cras. Ipsum ac volutpat, nibh odio penatibus rhoncus, massa, leo tincidunt. Auctor diam massa a, elementum quis libero eleifend mauris porttitor.</p><p>Gravida pretium et, vestibulum ultrices est faucibus ullamcorper. Enim, egestas nulla vitae risus facilisis senectus maecenas bibendum.</p><h3>Secondary Heading</h3><p>Dapibus massa id duis commodo mauris non lacus porttitor sed. Nibh vitae, tellus eros nibh eget laoreet non. Pellentesque augue non nec at netus ut. Tincidunt nullam semper eros neque. Iaculis dictum in non ac massa. Eget sem massa aliquam sed. Hac pulvinar donec morbi eu senectus accumsan.</p><p>Feugiat aliquam elementum suspendisse eget in. Blandit ac tellus, etiam nunc. Vitae venenatis mi imperdiet arcu netus scelerisque amet nulla. Posuere dui ac pulvinar diam. Nisl senectus viverra ultricies convallis odio egestas. Odio pellentesque erat.</p><ul><li>Tincidunt nullam semper eros neque. Iaculis dictum in non ac massa. Eget sem massa aliquam sed.</li><li>Sit justo, gravida egestas tincidunt lorem curabitur lacus.</li><li>Varius diam consectetur pellentesque commodo. Volutpat gravida bibendum sit nunc. Et velit eu a, amet in ac, feugiat eget.</li></ul>$$, 'https://uploads-ssl.webflow.com/641881bc97e4e960fe5d8ec7/642101b6f2602a3442f885c3_13.png', 'https://www.youtube.com/watch?v=Ojiv9Smi4XE', '', $$<p id=\"\">“Exploro gave us the confidence we needed to take our brand to the next level. At first we thought, should we do it? Then we took the plunge and <strong id=\"\">now we’re never looking back.</strong>”</p>$$, 'Kelly P.', 'Business Owner', 3);
INSERT INTO product_feature (title, slug, description_short, description_long, image_link, video_link, icon, quote, quote_author, quote_author_position, order_number) VALUES ('Exclusive Offers', 'exclusive-offers', 'We work with our partner brands to provide passengers with exclusive offers and discounts on activities, dining, and accommodations.', $$<h2>Main Heading</h2><p>Erat eleifend lacus mattis at porttitor at mauris vel pharetra. Consequat, dictum et magna augue. Risus maecenas morbi ante scelerisque consequat. Id mi porttitor dui platea mauris. Accumsan, consequat feugiat at vitae laoreet pulvinar volutpat. Eros, mi tincidunt lorem donec aenean. Facilisis velit eget morbi urna, in sed viverra sagittis dolor. Ultrices sem mattis eget faucibus tortor, cras. Ipsum ac volutpat, nibh odio penatibus rhoncus, massa, leo tincidunt. Auctor diam massa a, elementum quis libero eleifend mauris porttitor.</p><p>Gravida pretium et, vestibulum ultrices est faucibus ullamcorper. Enim, egestas nulla vitae risus facilisis senectus maecenas bibendum.</p><h3>Secondary Heading</h3><p>Dapibus massa id duis commodo mauris non lacus porttitor sed. Nibh vitae, tellus eros nibh eget laoreet non. Pellentesque augue non nec at netus ut. Tincidunt nullam semper eros neque. Iaculis dictum in non ac massa. Eget sem massa aliquam sed. Hac pulvinar donec morbi eu senectus accumsan.</p><p>Feugiat aliquam elementum suspendisse eget in. Blandit ac tellus, etiam nunc. Vitae venenatis mi imperdiet arcu netus scelerisque amet nulla. Posuere dui ac pulvinar diam. Nisl senectus viverra ultricies convallis odio egestas. Odio pellentesque erat.</p><ul><li>Tincidunt nullam semper eros neque. Iaculis dictum in non ac massa. Eget sem massa aliquam sed.</li><li>Sit justo, gravida egestas tincidunt lorem curabitur lacus.</li><li>Varius diam consectetur pellentesque commodo. Volutpat gravida bibendum sit nunc. Et velit eu a, amet in ac, feugiat eget.</li></ul>$$, 'https://uploads-ssl.webflow.com/641881bc97e4e960fe5d8ec7/642101a07a77c62c61ba2b24_12.png', 'https://www.youtube.com/watch?v=Ojiv9Smi4XE', 'https://uploads-ssl.webflow.com/641881bc97e4e960fe5d8ec7/641881bc97e4e940e05d8ed3_icon-shopping-bag.svg', $$<p id=\"\">“Exploro gave us the confidence we needed to take our brand to the next level. At first we thought, should we do it? Then we took the plunge and <strong id=\"\">now we’re never looking back.</strong>”</p>$$, 'Kelly P.', 'Business Owner', 4);
INSERT INTO product_feature (title, slug, description_short, description_long, image_link, video_link, icon, quote, quote_author, quote_author_position, order_number) VALUES ('In-Flight Entertainment', 'in-flight-entertainment', 'Passengers can access the app through the in-flight entertainment system, providing them with a convenient and engaging way to plan their travel itinerary.', $$<h2>Main Heading</h2><p>Erat eleifend lacus mattis at porttitor at mauris vel pharetra. Consequat, dictum et magna augue. Risus maecenas morbi ante scelerisque consequat. Id mi porttitor dui platea mauris. Accumsan, consequat feugiat at vitae laoreet pulvinar volutpat. Eros, mi tincidunt lorem donec aenean. Facilisis velit eget morbi urna, in sed viverra sagittis dolor. Ultrices sem mattis eget faucibus tortor, cras. Ipsum ac volutpat, nibh odio penatibus rhoncus, massa, leo tincidunt. Auctor diam massa a, elementum quis libero eleifend mauris porttitor.</p><p>Gravida pretium et, vestibulum ultrices est faucibus ullamcorper. Enim, egestas nulla vitae risus facilisis senectus maecenas bibendum.</p><h3>Secondary Heading</h3><p>Dapibus massa id duis commodo mauris non lacus porttitor sed. Nibh vitae, tellus eros nibh eget laoreet non. Pellentesque augue non nec at netus ut. Tincidunt nullam semper eros neque. Iaculis dictum in non ac massa. Eget sem massa aliquam sed. Hac pulvinar donec morbi eu senectus accumsan.</p><p>Feugiat aliquam elementum suspendisse eget in. Blandit ac tellus, etiam nunc. Vitae venenatis mi imperdiet arcu netus scelerisque amet nulla. Posuere dui ac pulvinar diam. Nisl senectus viverra ultricies convallis odio egestas. Odio pellentesque erat.</p><ul><li>Tincidunt nullam semper eros neque. Iaculis dictum in non ac massa. Eget sem massa aliquam sed.</li><li>Sit justo, gravida egestas tincidunt lorem curabitur lacus.</li><li>Varius diam consectetur pellentesque commodo. Volutpat gravida bibendum sit nunc. Et velit eu a, amet in ac, feugiat eget.</li></ul>$$, 'https://uploads-ssl.webflow.com/641881bc97e4e960fe5d8ec7/642101caa2bd1e1a4158ff5c_14.png', 'https://www.youtube.com/watch?v=Ojiv9Smi4XE', '', $$<p id=\"\">“Exploro gave us the confidence we needed to take our brand to the next level. At first we thought, should we do it? Then we took the plunge and <strong id=\"\">now we’re never looking back.</strong>”</p>$$, 'Kelly P.', 'Business Owner', 1);
INSERT INTO product_feature (title, slug, description_short, description_long, image_link, video_link, icon, quote, quote_author, quote_author_position, order_number) VALUES ('Multi-Language Support', 'multi-language-support', 'Our app supports multiple languages, making it accessible to a diverse range of passengers.', $$<h2>Main Heading</h2><p>Erat eleifend lacus mattis at porttitor at mauris vel pharetra. Consequat, dictum et magna augue. Risus maecenas morbi ante scelerisque consequat. Id mi porttitor dui platea mauris. Accumsan, consequat feugiat at vitae laoreet pulvinar volutpat. Eros, mi tincidunt lorem donec aenean. Facilisis velit eget morbi urna, in sed viverra sagittis dolor. Ultrices sem mattis eget faucibus tortor, cras. Ipsum ac volutpat, nibh odio penatibus rhoncus, massa, leo tincidunt. Auctor diam massa a, elementum quis libero eleifend mauris porttitor.</p><p>Gravida pretium et, vestibulum ultrices est faucibus ullamcorper. Enim, egestas nulla vitae risus facilisis senectus maecenas bibendum.</p><h3>Secondary Heading</h3><p>Dapibus massa id duis commodo mauris non lacus porttitor sed. Nibh vitae, tellus eros nibh eget laoreet non. Pellentesque augue non nec at netus ut. Tincidunt nullam semper eros neque. Iaculis dictum in non ac massa. Eget sem massa aliquam sed. Hac pulvinar donec morbi eu senectus accumsan.</p><p>Feugiat aliquam elementum suspendisse eget in. Blandit ac tellus, etiam nunc. Vitae venenatis mi imperdiet arcu netus scelerisque amet nulla. Posuere dui ac pulvinar diam. Nisl senectus viverra ultricies convallis odio egestas. Odio pellentesque erat.</p><ul><li>Tincidunt nullam semper eros neque. Iaculis dictum in non ac massa. Eget sem massa aliquam sed.</li><li>Sit justo, gravida egestas tincidunt lorem curabitur lacus.</li><li>Varius diam consectetur pellentesque commodo. Volutpat gravida bibendum sit nunc. Et velit eu a, amet in ac, feugiat eget.</li></ul>$$, 'https://uploads-ssl.webflow.com/641881bc97e4e960fe5d8ec7/642101d55993ba97fc283dd7_10.png', 'https://www.youtube.com/watch?v=Ojiv9Smi4XE', '', $$<p id=\"\">“Exploro gave us the confidence we needed to take our brand to the next level. At first we thought, should we do it? Then we took the plunge and <strong id=\"\">now we’re never looking back.</strong>”</p>$$, 'Kelly P.', 'Business Owner', 2);

-- insert client:
INSERT INTO client (name, slug, created, title, description_short, logo, image_link, description_long, quote, quote_author, quote_author_position, number_of_employees, industry, website_link, features_used, featured) VALUES ('American Dream', 'american-dream', '2023-09-07T05:15:36+00:00', 'Tickemaster uses Exploro platform to grow their business to over 10,000 users.', 'Add a brief description of the company here to provide some context about the customer.', 'https://uploads-ssl.webflow.com/641881bc97e4e960fe5d8ec7/6421094930d27c159f1f1a69_Ticketmaster-Logo-Azure_without_R.png', 'https://uploads-ssl.webflow.com/641881bc97e4e960fe5d8ec7/6421094e5993ba657728e3f6_Magecart-Ticketmaster.jpg', $$<h3>The Problem</h3><p>Erat eleifend lacus mattis at porttitor at mauris vel pharetra. Consequat, dictum et magna augue. Risus maecenas morbi ante scelerisque consequat. Id mi porttitor dui platea mauris. Accumsan, consequat feugiat at vitae laoreet pulvinar volutpat. Eros, mi tincidunt lorem donec aenean. Facilisis velit eget morbi urna, in sed viverra sagittis dolor. Ultrices sem mattis eget faucibus tortor, cras. Ipsum ac volutpat, nibh odio penatibus rhoncus, massa, leo tincidunt. Auctor diam massa a, elementum quis libero eleifend mauris porttitor.</p><p>Gravida pretium et, vestibulum ultrices est faucibus ullamcorper. Enim, egestas nulla vitae risus facilisis senectus maecenas bibendum.</p><h3>The Solution</h3><p>Dapibus massa id duis commodo mauris non lacus porttitor sed. Nibh vitae, tellus eros nibh eget laoreet non. Pellentesque augue non nec at netus ut. Tincidunt nullam semper eros neque. Iaculis dictum in non ac massa. Eget sem massa aliquam sed. Hac pulvinar donec morbi eu senectus accumsan.</p><p>Feugiat aliquam elementum suspendisse eget in. Blandit ac tellus, etiam nunc. Vitae venenatis mi imperdiet arcu netus scelerisque amet nulla. Posuere dui ac pulvinar diam. Nisl senectus viverra ultricies convallis odio egestas. Odio pellentesque erat.</p><ul><li>Tincidunt nullam semper eros neque. Iaculis dictum in non ac massa. Eget sem massa aliquam sed.</li><li>Sit justo, gravida egestas tincidunt lorem curabitur lacus.</li><li>Varius diam consectetur pellentesque commodo. Volutpat gravida bibendum sit nunc. Et velit eu a, amet in ac, feugiat eget.</li></ul>$$, '“Without Exploro we would still be managing things the bad old way: wasting time, missing deadlines and finger-pointing. No more! Exploro has saved our business.”', 'Brian Henderson', 'Director of Sales', '30+ Employees', 'Online Retail', 'https://webflow.com', 'multi-language-support; exclusive-offers; easy-booking', FALSE);
INSERT INTO client (name, slug, created, title, description_short, logo, image_link, description_long, quote, quote_author, quote_author_position, number_of_employees, industry, website_link, features_used, featured) VALUES ('Caesars Palace', 'caesars-palace', '2023-09-07T05:15:36+00:00', 'Caesars Palace increased their sales performance by 35% in their first month.', 'Add a brief description of the company here to provide some context about the customer.', 'https://uploads-ssl.webflow.com/641881bc97e4e960fe5d8ec7/64210820e502091d0e1d5029__caesars_palace_las_vegas.svg', 'https://uploads-ssl.webflow.com/641881bc97e4e960fe5d8ec7/6421083c5993ba4c2128d0c0_welcome-to-caesars-palace.jpg', $$<h3>The Problem</h3><p>Erat eleifend lacus mattis at porttitor at mauris vel pharetra. Consequat, dictum et magna augue. Risus maecenas morbi ante scelerisque consequat. Id mi porttitor dui platea mauris. Accumsan, consequat feugiat at vitae laoreet pulvinar volutpat. Eros, mi tincidunt lorem donec aenean. Facilisis velit eget morbi urna, in sed viverra sagittis dolor. Ultrices sem mattis eget faucibus tortor, cras. Ipsum ac volutpat, nibh odio penatibus rhoncus, massa, leo tincidunt. Auctor diam massa a, elementum quis libero eleifend mauris porttitor.</p><p>Gravida pretium et, vestibulum ultrices est faucibus ullamcorper. Enim, egestas nulla vitae risus facilisis senectus maecenas bibendum.</p><h3>The Solution</h3><p>Dapibus massa id duis commodo mauris non lacus porttitor sed. Nibh vitae, tellus eros nibh eget laoreet non. Pellentesque augue non nec at netus ut. Tincidunt nullam semper eros neque. Iaculis dictum in non ac massa. Eget sem massa aliquam sed. Hac pulvinar donec morbi eu senectus accumsan.</p><p>Feugiat aliquam elementum suspendisse eget in. Blandit ac tellus, etiam nunc. Vitae venenatis mi imperdiet arcu netus scelerisque amet nulla. Posuere dui ac pulvinar diam. Nisl senectus viverra ultricies convallis odio egestas. Odio pellentesque erat.</p><ul><li>Tincidunt nullam semper eros neque. Iaculis dictum in non ac massa. Eget sem massa aliquam sed.</li><li>Sit justo, gravida egestas tincidunt lorem curabitur lacus.</li><li>Varius diam consectetur pellentesque commodo. Volutpat gravida bibendum sit nunc. Et velit eu a, amet in ac, feugiat eget.</li></ul>$$, '“Without Exploro we would still be managing things the bad old way: wasting time, missing deadlines and finger-pointing. No more! Exploro has saved our business.”', 'Brian Henderson', 'Director of Sales', '30,000+ Employees', 'Hotel Chain', 'https://webflow.com', 'multi-language-support; exclusive-offers; easy-booking', TRUE);
INSERT INTO client (name, slug, created, title, description_short, logo, image_link, description_long, quote, quote_author, quote_author_position, number_of_employees, industry, website_link, features_used, featured) VALUES ('Emirates Airlines', 'emirates-airlines', '2023-09-07T05:15:36+00:00', 'Emirates Airline leveraged Exploro to help the city of Dubai', 'Add a brief description of the company here to provide some context about the customer.', 'https://uploads-ssl.webflow.com/641881bc97e4e960fe5d8ec7/642108dde50209a7971d6080_566px-Emirates_logo.svg.png', 'https://uploads-ssl.webflow.com/641881bc97e4e960fe5d8ec7/642108f27e674a2b24ecde25_Emirates-A380-Premium-Economy-Cabin.webp', $$<h3>The Problem</h3><p>Erat eleifend lacus mattis at porttitor at mauris vel pharetra. Consequat, dictum et magna augue. Risus maecenas morbi ante scelerisque consequat. Id mi porttitor dui platea mauris. Accumsan, consequat feugiat at vitae laoreet pulvinar volutpat. Eros, mi tincidunt lorem donec aenean. Facilisis velit eget morbi urna, in sed viverra sagittis dolor. Ultrices sem mattis eget faucibus tortor, cras. Ipsum ac volutpat, nibh odio penatibus rhoncus, massa, leo tincidunt. Auctor diam massa a, elementum quis libero eleifend mauris porttitor.</p><p>Gravida pretium et, vestibulum ultrices est faucibus ullamcorper. Enim, egestas nulla vitae risus facilisis senectus maecenas bibendum.</p><h3>The Solution</h3><p>Dapibus massa id duis commodo mauris non lacus porttitor sed. Nibh vitae, tellus eros nibh eget laoreet non. Pellentesque augue non nec at netus ut. Tincidunt nullam semper eros neque. Iaculis dictum in non ac massa. Eget sem massa aliquam sed. Hac pulvinar donec morbi eu senectus accumsan.</p><p>Feugiat aliquam elementum suspendisse eget in. Blandit ac tellus, etiam nunc. Vitae venenatis mi imperdiet arcu netus scelerisque amet nulla. Posuere dui ac pulvinar diam. Nisl senectus viverra ultricies convallis odio egestas. Odio pellentesque erat.</p><ul><li>Tincidunt nullam semper eros neque. Iaculis dictum in non ac massa. Eget sem massa aliquam sed.</li><li>Sit justo, gravida egestas tincidunt lorem curabitur lacus.</li><li>Varius diam consectetur pellentesque commodo. Volutpat gravida bibendum sit nunc. Et velit eu a, amet in ac, feugiat eget.</li></ul>$$, '“Without Exploro we would still be managing things the bad old way: wasting time, missing deadlines and finger-pointing. No more! Exploro has saved our business.”', 'Brian Henderson', 'Director of Sales', '30+ Employees', 'Online Retail', 'https://webflow.com', 'multi-language-support; exclusive-offers; easy-booking', FALSE);
INSERT INTO client (name, slug, created, title, description_short, logo, image_link, description_long, quote, quote_author, quote_author_position, number_of_employees, industry, website_link, features_used, featured) VALUES ('MGM Grand', 'mgm-grand', '2023-09-07T05:15:36+00:00', 'MGM Grand leveraged Exploro to help launch their product and grow their base.', 'Add a brief description of the company here to provide some context about the customer.', 'https://uploads-ssl.webflow.com/641881bc97e4e960fe5d8ec7/64210868938268b0187fd2f7_images.png', 'https://uploads-ssl.webflow.com/641881bc97e4e960fe5d8ec7/6421087b927bd512db2fb2a2_mgm-grand.jpg', $$<h3>The Problem</h3><p>Erat eleifend lacus mattis at porttitor at mauris vel pharetra. Consequat, dictum et magna augue. Risus maecenas morbi ante scelerisque consequat. Id mi porttitor dui platea mauris. Accumsan, consequat feugiat at vitae laoreet pulvinar volutpat. Eros, mi tincidunt lorem donec aenean. Facilisis velit eget morbi urna, in sed viverra sagittis dolor. Ultrices sem mattis eget faucibus tortor, cras. Ipsum ac volutpat, nibh odio penatibus rhoncus, massa, leo tincidunt. Auctor diam massa a, elementum quis libero eleifend mauris porttitor.</p><p>Gravida pretium et, vestibulum ultrices est faucibus ullamcorper. Enim, egestas nulla vitae risus facilisis senectus maecenas bibendum.</p><h3>The Solution</h3><p>Dapibus massa id duis commodo mauris non lacus porttitor sed. Nibh vitae, tellus eros nibh eget laoreet non. Pellentesque augue non nec at netus ut. Tincidunt nullam semper eros neque. Iaculis dictum in non ac massa. Eget sem massa aliquam sed. Hac pulvinar donec morbi eu senectus accumsan.</p><p>Feugiat aliquam elementum suspendisse eget in. Blandit ac tellus, etiam nunc. Vitae venenatis mi imperdiet arcu netus scelerisque amet nulla. Posuere dui ac pulvinar diam. Nisl senectus viverra ultricies convallis odio egestas. Odio pellentesque erat.</p><ul><li>Tincidunt nullam semper eros neque. Iaculis dictum in non ac massa. Eget sem massa aliquam sed.</li><li>Sit justo, gravida egestas tincidunt lorem curabitur lacus.</li><li>Varius diam consectetur pellentesque commodo. Volutpat gravida bibendum sit nunc. Et velit eu a, amet in ac, feugiat eget.</li></ul>$$, '“Without Exploro we would still be managing things the bad old way: wasting time, missing deadlines and finger-pointing. No more! Exploro has saved our business.”', 'Brian Henderson', 'Director of Sales', '30,000+ Employees', 'Online Retail', 'https://webflow.com', 'multi-language-support; exclusive-offers; easy-booking', FALSE);
INSERT INTO client (name, slug, created, title, description_short, logo, image_link, description_long, quote, quote_author, quote_author_position, number_of_employees, industry, website_link, features_used, featured) VALUES ('Spirit Airlines', 'spirit-airlines', '2023-09-07T05:15:36+00:00', 'Spirit shortened it’s lead time on customer orders and made big bucks.', 'Add a brief description of the company here to provide some context about the customer.', 'https://uploads-ssl.webflow.com/641881bc97e4e960fe5d8ec7/642108a6f708deee07efbafe_1200px-Spirit_Airlines_logo.svg.png', 'https://uploads-ssl.webflow.com/641881bc97e4e960fe5d8ec7/642108bd885ad95cf1189e6b_Spirit-Airlines-A321-Big-Front-Seat-12.webp', $$<h3>The Problem</h3><p>Erat eleifend lacus mattis at porttitor at mauris vel pharetra. Consequat, dictum et magna augue. Risus maecenas morbi ante scelerisque consequat. Id mi porttitor dui platea mauris. Accumsan, consequat feugiat at vitae laoreet pulvinar volutpat. Eros, mi tincidunt lorem donec aenean. Facilisis velit eget morbi urna, in sed viverra sagittis dolor. Ultrices sem mattis eget faucibus tortor, cras. Ipsum ac volutpat, nibh odio penatibus rhoncus, massa, leo tincidunt. Auctor diam massa a, elementum quis libero eleifend mauris porttitor.</p><p>Gravida pretium et, vestibulum ultrices est faucibus ullamcorper. Enim, egestas nulla vitae risus facilisis senectus maecenas bibendum.</p><h3>The Solution</h3><p>Dapibus massa id duis commodo mauris non lacus porttitor sed. Nibh vitae, tellus eros nibh eget laoreet non. Pellentesque augue non nec at netus ut. Tincidunt nullam semper eros neque. Iaculis dictum in non ac massa. Eget sem massa aliquam sed. Hac pulvinar donec morbi eu senectus accumsan.</p><p>Feugiat aliquam elementum suspendisse eget in. Blandit ac tellus, etiam nunc. Vitae venenatis mi imperdiet arcu netus scelerisque amet nulla. Posuere dui ac pulvinar diam. Nisl senectus viverra ultricies convallis odio egestas. Odio pellentesque erat.</p><ul><li>Tincidunt nullam semper eros neque. Iaculis dictum in non ac massa. Eget sem massa aliquam sed.</li><li>Sit justo, gravida egestas tincidunt lorem curabitur lacus.</li><li>Varius diam consectetur pellentesque commodo. Volutpat gravida bibendum sit nunc. Et velit eu a, amet in ac, feugiat eget.</li></ul>$$, '“Without Exploro we would still be managing things the bad old way: wasting time, missing deadlines and finger-pointing. No more! Exploro has saved our business.”', 'Brian Henderson', 'Director of Sales', '3000+ Employees', 'Online Retail', 'https://webflow.com', 'multi-language-support; exclusive-offers; easy-booking', FALSE);

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

-- partner_vendor
INSERT INTO partner_vendor (
  name,
  slug,
  description_short,
  description_long,
  video_link,
  image_link,
  image_link_2,
  thumbnail_link,
  gallery,
  featured_partner_vendor,
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
  'VendorName',                     -- name
  'vendor-name',                    -- slug
  'This is a short description.',   -- description_short
  'This is a longer description.',  -- description_long
  'http://www.example.com/video',   -- video_link
  'http://www.example.com/image1',  -- image_link
  'http://www.example.com/image2',  -- image_link_2
  'http://www.example.com/thumb',   -- thumbnail_link
  '{"img1", "img2", "img3"}',       -- gallery
  4.5,                             -- featured_partner_vendor
  1,                               -- continent
  2,                               -- country
  3,                               -- region
  1,                               -- city
  40.7128,                         -- latitude
  -74.0060,                        -- longitude
  'email@example.com',             -- email
  '+1-123-456-7890',               -- phone
  '123 Example St, City, Country', -- address
  'http://www.example.com',        -- website_link
  'tag1,tag2,tag3'                 -- tags
);

INSERT INTO partner_vendor (
  name,
  slug,
  description_short,
  description_long,
  video_link,
  image_link,
  image_link_2,
  thumbnail_link,
  gallery,
  featured_partner_vendor,
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
  'Chuck E. Cheese''s',                     -- name
  'chuck-e-cheeses',                        -- slug
  'Family entertainment center.',           -- description_short
  'A place where a kid can be a kid.',      -- description_long
  'http://www.example.com/chuckecheese',    -- video_link
  'http://www.example.com/image_chuck1',    -- image_link
  'http://www.example.com/image_chuck2',    -- image_link_2
  'http://www.example.com/thumb_chuck',     -- thumbnail_link
  '{"gallery1", "gallery2", "gallery3"}',   -- gallery
  4.5,                                      -- featured_partner_vendor
  1,                                        -- continent
  1,                                        -- country
  1,                                        -- region
  1,                                        -- city
  40.7128,                                  -- latitude
  -74.0060,                                 -- longitude
  'contact@chuckecheese.com',               -- email
  '+1-123-456-7890',                        -- phone
  '123 Fun St, Anytown, USA',                -- address
  'http://www.chuckecheese.com',            -- website_link
  'kids,fun,games,pizza'                    -- tags
);

INSERT INTO partner_vendor (
  name,
  slug,
  description_short,
  description_long,
  video_link,
  image_link,
  image_link_2,
  thumbnail_link,
  gallery,
  featured_partner_vendor,
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
  'Boondocks',                                  -- name
  'boondocks',                                  -- slug
  'Outdoor adventure services',                 -- description_short
  'Boondocks offers a range of outdoor activities, from kayaking to hiking.',  -- description_long
  'http://www.boondocks.com/video',             -- video_link
  'http://www.boondocks.com/image1',            -- image_link
  'http://www.boondocks.com/image2',            -- image_link_2
  'http://www.boondocks.com/thumbnail',         -- thumbnail_link
  '{"gallery_img1", "gallery_img2", "gallery_img3"}',  -- gallery
  5.0,                                         -- featured_partner_vendor
  1,                                           -- continent (reference)
  1,                                         -- country (reference)
  1,                                         -- region (reference)
  1,                                         -- city (reference)
  40.7128,                                     -- latitude
  -74.0060,                                    -- longitude
  'contact@boondocks.com',                     -- email
  '+1-123-456-7890',                           -- phone
  '123 Boondocks St, Adventure City, USA',     -- address
  'http://www.boondocks.com',                  -- website_link
  'outdoor,adventure,kayaking'                 -- tags
);

-- restaurant
INSERT INTO restaurant (name, slug, description_short, description_long, video_link, image_link, image_link_2, thumbnail_link, gallery, menu_gallery, featured_restaurant, partner_vendor, continent, country, region, city, latitude, longitude, email, phone, website_link, tags)
VALUES ('Sample Restaurant 1', 'sample-restaurant-1', 'Short description for Sample Restaurant 1', 'Long description for Sample Restaurant 1', 'https://www.youtube.com/watch?v=sample_video', 'https://www.example.com/images/sample1.jpg', 'https://www.example.com/images/sample1_2.jpg', 'https://www.example.com/images/sample1_thumb.jpg', ARRAY['https://www.example.com/gallery/sample1_1.jpg', 'https://www.example.com/gallery/sample1_2.jpg'], ARRAY['https://www.example.com/menu/sample1_1.jpg', 'https://www.example.com/menu/sample1_2.jpg'], 1.0, 1, 1, 1, 1, 1, 40.7128, -74.0060, 'info@sample1.com', '+123456789', 'https://www.sample1.com', 'tag1, tag2, tag3');
INSERT INTO restaurant (name, slug, description_short, description_long, video_link, image_link, image_link_2, thumbnail_link, gallery, menu_gallery, featured_restaurant, partner_vendor, continent, country, region, city, latitude, longitude, email, phone, website_link, tags)
VALUES ('Sample Restaurant 2', 'sample-restaurant-2', 'Short description for Sample Restaurant 2', 'Long description for Sample Restaurant 2', 'https://www.youtube.com/watch?v=sample_video_2', 'https://www.example.com/images/sample2.jpg', 'https://www.example.com/images/sample2_2.jpg', 'https://www.example.com/images/sample2_thumb.jpg', ARRAY['https://www.example.com/gallery/sample2_1.jpg', 'https://www.example.com/gallery/sample2_2.jpg'], ARRAY['https://www.example.com/menu/sample2_1.jpg', 'https://www.example.com/menu/sample2_2.jpg'], 1.0, 1, 1, 1, 1, 1, 34.0522, -118.2437, 'info@sample2.com', '+987654321', 'https://www.sample2.com', 'tag4, tag5');
INSERT INTO restaurant (name, slug, description_short, description_long, video_link, image_link, image_link_2, thumbnail_link, gallery, menu_gallery, featured_restaurant, partner_vendor, continent, country, region, city, latitude, longitude, email, phone, website_link, tags)
VALUES ('Sample Restaurant 3', 'sample-restaurant-3', 'Short description for Sample Restaurant 3', 'Long description for Sample Restaurant 3', '', '', '', '', ARRAY['https://www.example.com/gallery/sample3_1.jpg', 'https://www.example.com/gallery/sample3_2.jpg'], ARRAY['https://www.example.com/menu/sample3_1.jpg', 'https://www.example.com/menu/sample3_2.jpg'], 1.0, 1, 1, 1, 1, 1, 51.5074, -0.1278, 'info@sample3.com', '+444555666', 'https://www.sample3.com', 'tag6, tag7, tag8');

-- hotel
INSERT INTO hotel (name, slug, description_short, description_long, video_link, image_link, image_link_2, thumbnail_link, gallery, featured_hotel, partner_vendor, continent, country, region, city, latitude, longitude, email, phone, website_link)
VALUES ('Hotel ABC', 'hotel-abc', 'A luxury hotel', 'Hotel ABC is a luxurious hotel located...', 'https://www.youtube.com/hotel_abc_video', 'https://example.com/images/hotel_abc.jpg', 'https://example.com/images/hotel_abc_2.jpg', 'https://example.com/images/hotel_abc_thumbnail.jpg', '{"https://example.com/images/hotel_abc_gallery_1.jpg", "https://example.com/images/hotel_abc_gallery_2.jpg"}', 1.0, 1, 1, 1, 1, 1, 40.7128, -74.0060, 'info@hotelabc.com', '+1-123-456-7890', 'https://www.hotelabc.com');
INSERT INTO hotel (name, slug, description_short, description_long, video_link, image_link, image_link_2, thumbnail_link, gallery, featured_hotel, partner_vendor, continent, country, region, city, latitude, longitude, email, phone, website_link, tags)
VALUES ('Hotel XYZ', 'hotel-xyz', 'A cozy hotel', 'Hotel XYZ offers a cozy atmosphere...', 'https://www.youtube.com/hotel_xyz_video', 'https://example.com/images/hotel_xyz.jpg', 'https://example.com/images/hotel_xyz_2.jpg', 'https://example.com/images/hotel_xyz_thumbnail.jpg', '{"https://example.com/images/hotel_xyz_gallery_1.jpg", "https://example.com/images/hotel_xyz_gallery_2.jpg", "https://example.com/images/hotel_xyz_gallery_3.jpg"}', 1.0, 1, 1, 1, 1, 1, 34.0522, -118.2437, 'info@hotelxyz.com', '+1-987-654-3210', 'https://www.hotelxyz.com', '{"luxury", "cozy", "view"}');
INSERT INTO hotel (name, slug, description_short, description_long, video_link, image_link, image_link_2, thumbnail_link, gallery, featured_hotel, partner_vendor, continent, country, region, city, latitude, longitude, email, phone, website_link)
VALUES ('Seaside Resort', 'seaside-resort', 'Beachfront resort', 'Seaside Resort is a beachfront resort...', 'https://www.youtube.com/seaside_resort_video', 'https://example.com/images/seaside_resort.jpg', 'https://example.com/images/seaside_resort_2.jpg', 'https://example.com/images/seaside_resort_thumbnail.jpg', '{"https://example.com/images/seaside_resort_gallery_1.jpg", "https://example.com/images/seaside_resort_gallery_2.jpg", "https://example.com/images/seaside_resort_gallery_3.jpg"}', 1.0, 1, 1, 1, 1, 1, 25.7617, -80.1918, 'info@seasideresort.com', '+1-555-123-4567', 'https://www.seasideresort.com');

-- hotel_room
INSERT INTO hotel_room (name, hotel_id, description_short, description_long, video_link, image_link, image_link_2, thumbnail_link, gallery, tags)
VALUES ('Standard Room', 1, 'Cozy standard room', 'This room offers a comfortable stay with all basic amenities.', 'https://www.example.com/room_video', 'https://www.example.com/room_image.jpg', 'https://www.example.com/room_image2.jpg', 'https://www.example.com/room_thumbnail.jpg', ARRAY['https://www.example.com/room_gallery1.jpg', 'https://www.example.com/room_gallery2.jpg'], 'Standard, Cozy, Basic');
INSERT INTO hotel_room (name, hotel_id, description_short, description_long, video_link, image_link, image_link_2, thumbnail_link, gallery, tags)
VALUES ('Luxury Suite', 1, 'Spacious luxury suite', 'Indulge in luxury with this spacious suite, featuring modern amenities and breathtaking views.', 'https://www.example.com/suite_video', 'https://www.example.com/suite_image.jpg', 'https://www.example.com/suite_image2.jpg', 'https://www.example.com/suite_thumbnail.jpg', ARRAY['https://www.example.com/suite_gallery1.jpg', 'https://www.example.com/suite_gallery2.jpg', 'https://www.example.com/suite_gallery3.jpg'], 'Luxury, Suite, Spacious, View, Modern');
INSERT INTO hotel_room (name, hotel_id, description_short, description_long, video_link, image_link, image_link_2, thumbnail_link, gallery, tags)
VALUES ('Family Room', 1, 'Perfect for families', 'This family room is designed to accommodate families comfortably, ensuring a pleasant stay for everyone.', 'https://www.example.com/family_video', 'https://www.example.com/family_image.jpg', 'https://www.example.com/family_image2.jpg', 'https://www.example.com/family_thumbnail.jpg', ARRAY[]::TEXT[], 'Family, Spacious, Comfortable');

-- activity
INSERT INTO activity (name, slug, description_short, description_long, video_link, image_link, image_link_2, thumbnail_link, gallery, featured_activity, partner_vendor, continent, country, region, city, latitude, longitude, email, phone, website_link)
VALUES ('Hiking Adventure', 'hiking-adventure', 'Explore nature on foot', 'Join us for an exciting hiking adventure through scenic trails.', 'https://www.youtube.com/watch?v=xyz', 'https://example.com/images/hiking.jpg', 'https://example.com/images/hiking_2.jpg', 'https://example.com/images/hiking_thumb.jpg', ARRAY['https://example.com/gallery/image1.jpg', 'https://example.com/gallery/image2.jpg'], 1.0, 1, 1, 1, 1, 1, 123.456, -78.901, 'contact@example.com', '+1234567890', 'https://www.example.com');
INSERT INTO activity (name, slug, description_short, description_long, video_link, image_link, image_link_2, thumbnail_link, gallery, featured_activity, partner_vendor, continent, country, region, city, latitude, longitude, email, phone, website_link, tags)
VALUES ('Cultural Tour', 'cultural-tour', 'Experience rich cultural heritage', 'Immerse yourself in the local culture and traditions on this guided tour.', 'https://www.youtube.com/watch?v=abc', 'https://example.com/images/cultural.jpg', 'https://example.com/images/cultural_2.jpg', 'https://example.com/images/cultural_thumb.jpg', ARRAY['https://example.com/gallery/image3.jpg', 'https://example.com/gallery/image4.jpg'], 1.0, 1, 1, 1, 1, 1, 12.345, -67.890, 'info@example.com', '+0987654321', 'https://www.tourcompany.com', 'culture, heritage, guided tour');
INSERT INTO activity (name, slug, description_short, description_long, video_link, image_link, image_link_2, thumbnail_link, gallery, featured_activity, partner_vendor, continent, country, region, city, latitude, longitude, email, phone, website_link, tags)
VALUES ('Snorkeling Adventure', 'snorkeling-adventure', 'Discover marine life underwater', 'Explore the beautiful marine life with our snorkeling adventure.', 'https://www.youtube.com/watch?v=pqr', 'https://example.com/images/snorkeling.jpg', 'https://example.com/images/snorkeling_2.jpg', 'https://example.com/images/snorkeling_thumb.jpg', ARRAY['https://example.com/gallery/image5.jpg', 'https://example.com/gallery/image6.jpg'], 1.0, 1, 1, 1, 1, 1, 23.456, -45.678, 'contact@snorkelers.com', '+9876543210', 'https://www.snorkelers.com', 'snorkeling, marine life, water sports');

-- event
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

