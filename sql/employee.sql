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
