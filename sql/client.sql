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

INSERT INTO client (name, slug, created, title, description_short, logo, image_link, description_long, quote, quote_author, quote_author_position, number_of_employees, industry, website_link, features_used, featured) VALUES ('American Dream', 'american-dream', '2023-09-07T05:15:36+00:00', 'Tickemaster uses Exploro platform to grow their business to over 10,000 users.', 'Add a brief description of the company here to provide some context about the customer.', 'https://uploads-ssl.webflow.com/641881bc97e4e960fe5d8ec7/6421094930d27c159f1f1a69_Ticketmaster-Logo-Azure_without_R.png', 'https://uploads-ssl.webflow.com/641881bc97e4e960fe5d8ec7/6421094e5993ba657728e3f6_Magecart-Ticketmaster.jpg', $$<h3>The Problem</h3><p>Erat eleifend lacus mattis at porttitor at mauris vel pharetra. Consequat, dictum et magna augue. Risus maecenas morbi ante scelerisque consequat. Id mi porttitor dui platea mauris. Accumsan, consequat feugiat at vitae laoreet pulvinar volutpat. Eros, mi tincidunt lorem donec aenean. Facilisis velit eget morbi urna, in sed viverra sagittis dolor. Ultrices sem mattis eget faucibus tortor, cras. Ipsum ac volutpat, nibh odio penatibus rhoncus, massa, leo tincidunt. Auctor diam massa a, elementum quis libero eleifend mauris porttitor.</p><p>Gravida pretium et, vestibulum ultrices est faucibus ullamcorper. Enim, egestas nulla vitae risus facilisis senectus maecenas bibendum.</p><h3>The Solution</h3><p>Dapibus massa id duis commodo mauris non lacus porttitor sed. Nibh vitae, tellus eros nibh eget laoreet non. Pellentesque augue non nec at netus ut. Tincidunt nullam semper eros neque. Iaculis dictum in non ac massa. Eget sem massa aliquam sed. Hac pulvinar donec morbi eu senectus accumsan.</p><p>Feugiat aliquam elementum suspendisse eget in. Blandit ac tellus, etiam nunc. Vitae venenatis mi imperdiet arcu netus scelerisque amet nulla. Posuere dui ac pulvinar diam. Nisl senectus viverra ultricies convallis odio egestas. Odio pellentesque erat.</p><ul><li>Tincidunt nullam semper eros neque. Iaculis dictum in non ac massa. Eget sem massa aliquam sed.</li><li>Sit justo, gravida egestas tincidunt lorem curabitur lacus.</li><li>Varius diam consectetur pellentesque commodo. Volutpat gravida bibendum sit nunc. Et velit eu a, amet in ac, feugiat eget.</li></ul>$$, '“Without Exploro we would still be managing things the bad old way: wasting time, missing deadlines and finger-pointing. No more! Exploro has saved our business.”', 'Brian Henderson', 'Director of Sales', '30+ Employees', 'Online Retail', 'https://webflow.com', 'multi-language-support; exclusive-offers; easy-booking', FALSE);
INSERT INTO client (name, slug, created, title, description_short, logo, image_link, description_long, quote, quote_author, quote_author_position, number_of_employees, industry, website_link, features_used, featured) VALUES ('Caesars Palace', 'caesars-palace', '2023-09-07T05:15:36+00:00', 'Caesars Palace increased their sales performance by 35% in their first month.', 'Add a brief description of the company here to provide some context about the customer.', 'https://uploads-ssl.webflow.com/641881bc97e4e960fe5d8ec7/64210820e502091d0e1d5029__caesars_palace_las_vegas.svg', 'https://uploads-ssl.webflow.com/641881bc97e4e960fe5d8ec7/6421083c5993ba4c2128d0c0_welcome-to-caesars-palace.jpg', $$<h3>The Problem</h3><p>Erat eleifend lacus mattis at porttitor at mauris vel pharetra. Consequat, dictum et magna augue. Risus maecenas morbi ante scelerisque consequat. Id mi porttitor dui platea mauris. Accumsan, consequat feugiat at vitae laoreet pulvinar volutpat. Eros, mi tincidunt lorem donec aenean. Facilisis velit eget morbi urna, in sed viverra sagittis dolor. Ultrices sem mattis eget faucibus tortor, cras. Ipsum ac volutpat, nibh odio penatibus rhoncus, massa, leo tincidunt. Auctor diam massa a, elementum quis libero eleifend mauris porttitor.</p><p>Gravida pretium et, vestibulum ultrices est faucibus ullamcorper. Enim, egestas nulla vitae risus facilisis senectus maecenas bibendum.</p><h3>The Solution</h3><p>Dapibus massa id duis commodo mauris non lacus porttitor sed. Nibh vitae, tellus eros nibh eget laoreet non. Pellentesque augue non nec at netus ut. Tincidunt nullam semper eros neque. Iaculis dictum in non ac massa. Eget sem massa aliquam sed. Hac pulvinar donec morbi eu senectus accumsan.</p><p>Feugiat aliquam elementum suspendisse eget in. Blandit ac tellus, etiam nunc. Vitae venenatis mi imperdiet arcu netus scelerisque amet nulla. Posuere dui ac pulvinar diam. Nisl senectus viverra ultricies convallis odio egestas. Odio pellentesque erat.</p><ul><li>Tincidunt nullam semper eros neque. Iaculis dictum in non ac massa. Eget sem massa aliquam sed.</li><li>Sit justo, gravida egestas tincidunt lorem curabitur lacus.</li><li>Varius diam consectetur pellentesque commodo. Volutpat gravida bibendum sit nunc. Et velit eu a, amet in ac, feugiat eget.</li></ul>$$, '“Without Exploro we would still be managing things the bad old way: wasting time, missing deadlines and finger-pointing. No more! Exploro has saved our business.”', 'Brian Henderson', 'Director of Sales', '30,000+ Employees', 'Hotel Chain', 'https://webflow.com', 'multi-language-support; exclusive-offers; easy-booking', TRUE);
INSERT INTO client (name, slug, created, title, description_short, logo, image_link, description_long, quote, quote_author, quote_author_position, number_of_employees, industry, website_link, features_used, featured) VALUES ('Emirates Airlines', 'emirates-airlines', '2023-09-07T05:15:36+00:00', 'Emirates Airline leveraged Exploro to help the city of Dubai', 'Add a brief description of the company here to provide some context about the customer.', 'https://uploads-ssl.webflow.com/641881bc97e4e960fe5d8ec7/642108dde50209a7971d6080_566px-Emirates_logo.svg.png', 'https://uploads-ssl.webflow.com/641881bc97e4e960fe5d8ec7/642108f27e674a2b24ecde25_Emirates-A380-Premium-Economy-Cabin.webp', $$<h3>The Problem</h3><p>Erat eleifend lacus mattis at porttitor at mauris vel pharetra. Consequat, dictum et magna augue. Risus maecenas morbi ante scelerisque consequat. Id mi porttitor dui platea mauris. Accumsan, consequat feugiat at vitae laoreet pulvinar volutpat. Eros, mi tincidunt lorem donec aenean. Facilisis velit eget morbi urna, in sed viverra sagittis dolor. Ultrices sem mattis eget faucibus tortor, cras. Ipsum ac volutpat, nibh odio penatibus rhoncus, massa, leo tincidunt. Auctor diam massa a, elementum quis libero eleifend mauris porttitor.</p><p>Gravida pretium et, vestibulum ultrices est faucibus ullamcorper. Enim, egestas nulla vitae risus facilisis senectus maecenas bibendum.</p><h3>The Solution</h3><p>Dapibus massa id duis commodo mauris non lacus porttitor sed. Nibh vitae, tellus eros nibh eget laoreet non. Pellentesque augue non nec at netus ut. Tincidunt nullam semper eros neque. Iaculis dictum in non ac massa. Eget sem massa aliquam sed. Hac pulvinar donec morbi eu senectus accumsan.</p><p>Feugiat aliquam elementum suspendisse eget in. Blandit ac tellus, etiam nunc. Vitae venenatis mi imperdiet arcu netus scelerisque amet nulla. Posuere dui ac pulvinar diam. Nisl senectus viverra ultricies convallis odio egestas. Odio pellentesque erat.</p><ul><li>Tincidunt nullam semper eros neque. Iaculis dictum in non ac massa. Eget sem massa aliquam sed.</li><li>Sit justo, gravida egestas tincidunt lorem curabitur lacus.</li><li>Varius diam consectetur pellentesque commodo. Volutpat gravida bibendum sit nunc. Et velit eu a, amet in ac, feugiat eget.</li></ul>$$, '“Without Exploro we would still be managing things the bad old way: wasting time, missing deadlines and finger-pointing. No more! Exploro has saved our business.”', 'Brian Henderson', 'Director of Sales', '30+ Employees', 'Online Retail', 'https://webflow.com', 'multi-language-support; exclusive-offers; easy-booking', FALSE);
INSERT INTO client (name, slug, created, title, description_short, logo, image_link, description_long, quote, quote_author, quote_author_position, number_of_employees, industry, website_link, features_used, featured) VALUES ('MGM Grand', 'mgm-grand', '2023-09-07T05:15:36+00:00', 'MGM Grand leveraged Exploro to help launch their product and grow their base.', 'Add a brief description of the company here to provide some context about the customer.', 'https://uploads-ssl.webflow.com/641881bc97e4e960fe5d8ec7/64210868938268b0187fd2f7_images.png', 'https://uploads-ssl.webflow.com/641881bc97e4e960fe5d8ec7/6421087b927bd512db2fb2a2_mgm-grand.jpg', $$<h3>The Problem</h3><p>Erat eleifend lacus mattis at porttitor at mauris vel pharetra. Consequat, dictum et magna augue. Risus maecenas morbi ante scelerisque consequat. Id mi porttitor dui platea mauris. Accumsan, consequat feugiat at vitae laoreet pulvinar volutpat. Eros, mi tincidunt lorem donec aenean. Facilisis velit eget morbi urna, in sed viverra sagittis dolor. Ultrices sem mattis eget faucibus tortor, cras. Ipsum ac volutpat, nibh odio penatibus rhoncus, massa, leo tincidunt. Auctor diam massa a, elementum quis libero eleifend mauris porttitor.</p><p>Gravida pretium et, vestibulum ultrices est faucibus ullamcorper. Enim, egestas nulla vitae risus facilisis senectus maecenas bibendum.</p><h3>The Solution</h3><p>Dapibus massa id duis commodo mauris non lacus porttitor sed. Nibh vitae, tellus eros nibh eget laoreet non. Pellentesque augue non nec at netus ut. Tincidunt nullam semper eros neque. Iaculis dictum in non ac massa. Eget sem massa aliquam sed. Hac pulvinar donec morbi eu senectus accumsan.</p><p>Feugiat aliquam elementum suspendisse eget in. Blandit ac tellus, etiam nunc. Vitae venenatis mi imperdiet arcu netus scelerisque amet nulla. Posuere dui ac pulvinar diam. Nisl senectus viverra ultricies convallis odio egestas. Odio pellentesque erat.</p><ul><li>Tincidunt nullam semper eros neque. Iaculis dictum in non ac massa. Eget sem massa aliquam sed.</li><li>Sit justo, gravida egestas tincidunt lorem curabitur lacus.</li><li>Varius diam consectetur pellentesque commodo. Volutpat gravida bibendum sit nunc. Et velit eu a, amet in ac, feugiat eget.</li></ul>$$, '“Without Exploro we would still be managing things the bad old way: wasting time, missing deadlines and finger-pointing. No more! Exploro has saved our business.”', 'Brian Henderson', 'Director of Sales', '30,000+ Employees', 'Online Retail', 'https://webflow.com', 'multi-language-support; exclusive-offers; easy-booking', FALSE);
INSERT INTO client (name, slug, created, title, description_short, logo, image_link, description_long, quote, quote_author, quote_author_position, number_of_employees, industry, website_link, features_used, featured) VALUES ('Spirit Airlines', 'spirit-airlines', '2023-09-07T05:15:36+00:00', 'Spirit shortened it’s lead time on customer orders and made big bucks.', 'Add a brief description of the company here to provide some context about the customer.', 'https://uploads-ssl.webflow.com/641881bc97e4e960fe5d8ec7/642108a6f708deee07efbafe_1200px-Spirit_Airlines_logo.svg.png', 'https://uploads-ssl.webflow.com/641881bc97e4e960fe5d8ec7/642108bd885ad95cf1189e6b_Spirit-Airlines-A321-Big-Front-Seat-12.webp', $$<h3>The Problem</h3><p>Erat eleifend lacus mattis at porttitor at mauris vel pharetra. Consequat, dictum et magna augue. Risus maecenas morbi ante scelerisque consequat. Id mi porttitor dui platea mauris. Accumsan, consequat feugiat at vitae laoreet pulvinar volutpat. Eros, mi tincidunt lorem donec aenean. Facilisis velit eget morbi urna, in sed viverra sagittis dolor. Ultrices sem mattis eget faucibus tortor, cras. Ipsum ac volutpat, nibh odio penatibus rhoncus, massa, leo tincidunt. Auctor diam massa a, elementum quis libero eleifend mauris porttitor.</p><p>Gravida pretium et, vestibulum ultrices est faucibus ullamcorper. Enim, egestas nulla vitae risus facilisis senectus maecenas bibendum.</p><h3>The Solution</h3><p>Dapibus massa id duis commodo mauris non lacus porttitor sed. Nibh vitae, tellus eros nibh eget laoreet non. Pellentesque augue non nec at netus ut. Tincidunt nullam semper eros neque. Iaculis dictum in non ac massa. Eget sem massa aliquam sed. Hac pulvinar donec morbi eu senectus accumsan.</p><p>Feugiat aliquam elementum suspendisse eget in. Blandit ac tellus, etiam nunc. Vitae venenatis mi imperdiet arcu netus scelerisque amet nulla. Posuere dui ac pulvinar diam. Nisl senectus viverra ultricies convallis odio egestas. Odio pellentesque erat.</p><ul><li>Tincidunt nullam semper eros neque. Iaculis dictum in non ac massa. Eget sem massa aliquam sed.</li><li>Sit justo, gravida egestas tincidunt lorem curabitur lacus.</li><li>Varius diam consectetur pellentesque commodo. Volutpat gravida bibendum sit nunc. Et velit eu a, amet in ac, feugiat eget.</li></ul>$$, '“Without Exploro we would still be managing things the bad old way: wasting time, missing deadlines and finger-pointing. No more! Exploro has saved our business.”', 'Brian Henderson', 'Director of Sales', '3000+ Employees', 'Online Retail', 'https://webflow.com', 'multi-language-support; exclusive-offers; easy-booking', FALSE);