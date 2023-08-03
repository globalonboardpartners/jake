-- Create Team Members Table
CREATE TABLE team_members (
  id SERIAL PRIMARY KEY,
  name VARCHAR(50) NOT NULL,
  position VARCHAR(50) NOT NULL,
  bio TEXT NOT NULL,
  image_url VARCHAR(255) NOT NULL
);
-- Create Blog Categories Table
CREATE TABLE blog_categories (
  id SERIAL PRIMARY KEY,
  name VARCHAR(50) NOT NULL
);
-- Create Blog Posts Table
CREATE TABLE blog_posts (
  id SERIAL PRIMARY KEY,
  title VARCHAR(100) NOT NULL,
  content TEXT NOT NULL,
  published_at TIMESTAMP NOT NULL,
  category_id INT NOT NULL REFERENCES blog_categories(id)
);
-- Create Job Listings Table
CREATE TABLE job_listings (
  id SERIAL PRIMARY KEY,
  title VARCHAR(100) NOT NULL,
  description TEXT NOT NULL,
  posted_at TIMESTAMP NOT NULL
);
-- Create Product Features Table
CREATE TABLE product_features (
  id SERIAL PRIMARY KEY,
  title VARCHAR(100) NOT NULL,
  description TEXT NOT NULL
);
-- Insert Data into Team Members Table
-- INSERT INTO team_members (name, position, bio, image_url)
-- VALUES
--   (‘John Doe’, ‘CEO’, ‘John is the CEO of our company.’, ‘https://example.com/images/john.jpg’),
--   (‘Jane Smith’, ‘CTO’, ‘Jane is the Chief Technology Officer.’, ‘https://example.com/images/jane.jpg’),
--   (‘Mike Johnson’, ‘Senior Developer’, ‘Mike is a senior developer on our team.’, ‘https://example.com/images/mike.jpg’);
-- -- Insert Data into Blog Categories Table
-- INSERT INTO blog_categories (name)
-- VALUES
--   (‘Technology’),
--   (‘Business’),
--   (‘Marketing’),
--   (‘Product Updates’);
-- -- Insert Data into Blog Posts Table
-- INSERT INTO blog_posts (title, content, published_at, category_id)
-- VALUES
--   (‘Introduction to AI’, ‘In this post, we introduce the basics of artificial intelligence...’, ‘2023-07-15 10:00:00’, 1),
--   (‘Tips for Effective Marketing’, ‘Learn some valuable marketing tips to grow your business...’, ‘2023-07-20 09:30:00’, 3),
--   (‘New Product Launch’, ‘We are excited to announce the launch of our new product...’, ‘2023-07-25 14:15:00’, 4);
-- -- Insert Data into Job Listings Table
-- INSERT INTO job_listings (title, description, posted_at)
-- VALUES
--   (‘Software Engineer’, ‘We are looking for a skilled software engineer to join our team...’, ‘2023-07-10 08:00:00’),
--   (‘Marketing Specialist’, ‘We need a marketing specialist to help us promote our products...’, ‘2023-07-12 10:30:00’),
--   (‘Data Analyst’, ‘Join our data team and help us analyze and interpret data...’, ‘2023-07-18 11:45:00’);
-- -- Insert Data into Product Features Table
-- INSERT INTO product_features (title, description)
-- VALUES
--   (‘Real-time Analytics’, ‘Get instant insights with our real-time analytics feature.’),
--   (‘Intuitive Interface’, ‘Our product comes with an easy-to-use and intuitive interface.’),
--   (‘Advanced Security’, ‘We prioritize the security of your data with advanced encryption.’);
