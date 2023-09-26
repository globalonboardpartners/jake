CREATE TABLE blog_category (
  id SERIAL PRIMARY KEY,
  category VARCHAR(50) NOT NULL,
  slug VARCHAR(75) NOT NULL,
  created TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
  edited TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);

INSERT INTO blog_category (category, slug) VALUES ('Airline News', 'airline-news');
INSERT INTO blog_category (category, slug) VALUES ('Budget Travel', 'budget-travel');
INSERT INTO blog_category (category, slug) VALUES ('Customer Stories', 'customer-stories');
INSERT INTO blog_category (category, slug) VALUES ('Travel Destinations', 'travel-destinations');
INSERT INTO blog_category (category, slug) VALUES ('Travel Tips', 'travel-tips');
