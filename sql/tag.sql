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
  created TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
  edited TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TRIGGER trigger_row_edit_update_timestamp_for_tag
BEFORE UPDATE ON tag
FOR EACH ROW
EXECUTE FUNCTION update_updated_on();

CREATE TABLE table_row_tags (
    assoc_table_row_id INTEGER,
    assoc_table assoc_table,
    tag_id INTEGER REFERENCES tag(id),
    PRIMARY KEY (assoc_table_row_id, assoc_table, tag_id)
);

INSERT INTO tag (name, assoc_table)
VALUES ('SummerFest', 'event');

INSERT INTO tag (name, assoc_table)
VALUES ('TechTalk', 'blog');

INSERT INTO tag (name, assoc_table)
VALUES ('Paris', 'city');

INSERT INTO tag (name, assoc_table)
VALUES ('HikingGear', 'product_feature');

INSERT INTO tag (name, assoc_table)
VALUES ('OceanView', 'hotel');
