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
