CREATE TYPE assoc_table AS ENUM (
  'activity',
  'blog',
  'city',
  'client',
  'continent',
  'country',
  'event',
  'hotel',
  'job_listing',
  'partner_vendor',
  'product_feature',
  'region',
  'restaurant'
);

CREATE TABLE tag (
  id SERIAL PRIMARY KEY,
  name VARCHAR(255) UNIQUE NOT NULL,
  assoc_table assoc_table NOT NULL,
  description TEXT,
  created TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
  edited TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TRIGGER trigger_row_edit_update_timestamp_for_tag
BEFORE UPDATE ON tag
FOR EACH ROW
EXECUTE FUNCTION update_updated_on();

CREATE TABLE entity_tag (
    assoc_table_id INTEGER,
    assoc_table assoc_table,
    tag_id INTEGER REFERENCES tag(id),
    PRIMARY KEY (assoc_table_id, assoc_table, tag_id)
);

-- CREATE TABLE activity_tags (
--     activity_id INTEGER REFERENCES activity(id),
--     tag_id INTEGER REFERENCES tag(id),
--     PRIMARY KEY (activity_id, tag_id)
-- );

-- CREATE TABLE blog_tags (
--     blog_id INTEGER REFERENCES blog(id),
--     tag_id INTEGER REFERENCES tag(id),
--     PRIMARY KEY (blog_id, tag_id)
-- );

-- CREATE TABLE city_tags (
--     city_id INTEGER REFERENCES city(id),
--     tag_id INTEGER REFERENCES tag(id),
--     PRIMARY KEY (city_id, tag_id)
-- );

-- CREATE TABLE client_tags (
--     client_id INTEGER REFERENCES client(id),
--     tag_id INTEGER REFERENCES tag(id),
--     PRIMARY KEY (client_id, tag_id)
-- );

-- CREATE TABLE continent_tags (
--     continent_id INTEGER REFERENCES continent(id),
--     tag_id INTEGER REFERENCES tag(id),
--     PRIMARY KEY (continent_id, tag_id)
-- );

-- CREATE TABLE country_tags (
--     country_id INTEGER REFERENCES country(id),
--     tag_id INTEGER REFERENCES tag(id),
--     PRIMARY KEY (country_id, tag_id)
-- );

-- CREATE TABLE event_tags (
--     event_id INTEGER REFERENCES event(id),
--     tag_id INTEGER REFERENCES tag(id),
--     PRIMARY KEY (event_id, tag_id)
-- );

-- CREATE TABLE hotel_tags (
--     hotel_id INTEGER REFERENCES hotel(id),
--     tag_id INTEGER REFERENCES tag(id),
--     PRIMARY KEY (hotel_id, tag_id)
-- );

-- CREATE TABLE job_listing_tags (
--     job_listing_id INTEGER REFERENCES job_listing(id),
--     tag_id INTEGER REFERENCES tag(id),
--     PRIMARY KEY (job_listing_id, tag_id)
-- );

-- CREATE TABLE partner_vendor_tags (
--     partner_vendor_id INTEGER REFERENCES partner_vendor(id),
--     tag_id INTEGER REFERENCES tag(id),
--     PRIMARY KEY (partner_vendor_id, tag_id)
-- );

-- CREATE TABLE product_feature_tags (
--     product_feature_id INTEGER REFERENCES product_feature(id),
--     tag_id INTEGER REFERENCES tag(id),
--     PRIMARY KEY (product_feature_id, tag_id)
-- );

-- CREATE TABLE region_tags (
--     region_id INTEGER REFERENCES region(id),
--     tag_id INTEGER REFERENCES tag(id),
--     PRIMARY KEY (region_id, tag_id)
-- );

-- CREATE TABLE restaurant_tags (
--     restaurant_id INTEGER REFERENCES restaurant(id),
--     tag_id INTEGER REFERENCES tag(id),
--     PRIMARY KEY (restaurant_id, tag_id)
-- );

INSERT INTO tag (name, assoc_table, description)
VALUES ('SummerFest', 'event', 'An annual summer festival.');

INSERT INTO tag (name, assoc_table, description)
VALUES ('TechTalk', 'blog', 'A blog about technology.');

INSERT INTO tag (name, assoc_table, description)
VALUES ('Paris', 'city', 'The capital city of France.');

INSERT INTO tag (name, assoc_table, description)
VALUES ('HikingGear', 'product_feature', 'Features of hiking gear.');

INSERT INTO tag (name, assoc_table, description)
VALUES ('OceanView', 'hotel', 'Hotels with ocean views.');
