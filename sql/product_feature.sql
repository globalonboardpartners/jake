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

INSERT INTO product_feature (title, slug, description_short, description_long, image_link, video_link, icon, quote, quote_author, quote_author_position, order_number) VALUES ('Easy Booking', 'easy-booking', 'Passengers can easily book activities and services directly through the app, making travel planning hassle-free and convenient.', $$<h2>Main Heading</h2><p>Erat eleifend lacus mattis at porttitor at mauris vel pharetra. Consequat, dictum et magna augue. Risus maecenas morbi ante scelerisque consequat. Id mi porttitor dui platea mauris. Accumsan, consequat feugiat at vitae laoreet pulvinar volutpat. Eros, mi tincidunt lorem donec aenean. Facilisis velit eget morbi urna, in sed viverra sagittis dolor. Ultrices sem mattis eget faucibus tortor, cras. Ipsum ac volutpat, nibh odio penatibus rhoncus, massa, leo tincidunt. Auctor diam massa a, elementum quis libero eleifend mauris porttitor.</p><p>Gravida pretium et, vestibulum ultrices est faucibus ullamcorper. Enim, egestas nulla vitae risus facilisis senectus maecenas bibendum.</p><h3>Secondary Heading</h3><p>Dapibus massa id duis commodo mauris non lacus porttitor sed. Nibh vitae, tellus eros nibh eget laoreet non. Pellentesque augue non nec at netus ut. Tincidunt nullam semper eros neque. Iaculis dictum in non ac massa. Eget sem massa aliquam sed. Hac pulvinar donec morbi eu senectus accumsan.</p><p>Feugiat aliquam elementum suspendisse eget in. Blandit ac tellus, etiam nunc. Vitae venenatis mi imperdiet arcu netus scelerisque amet nulla. Posuere dui ac pulvinar diam. Nisl senectus viverra ultricies convallis odio egestas. Odio pellentesque erat.</p><ul><li>Tincidunt nullam semper eros neque. Iaculis dictum in non ac massa. Eget sem massa aliquam sed.</li><li>Sit justo, gravida egestas tincidunt lorem curabitur lacus.</li><li>Varius diam consectetur pellentesque commodo. Volutpat gravida bibendum sit nunc. Et velit eu a, amet in ac, feugiat eget.</li></ul>$$, 'https://uploads-ssl.webflow.com/641881bc97e4e960fe5d8ec7/642101b6f2602a3442f885c3_13.png', 'https://www.youtube.com/watch?v=Ojiv9Smi4XE', '', $$<p id=\"\">“Exploro gave us the confidence we needed to take our brand to the next level. At first we thought, should we do it? Then we took the plunge and <strong id=\"\">now we’re never looking back.</strong>”</p>$$, 'Kelly P.', 'Business Owner', 3);
INSERT INTO product_feature (title, slug, description_short, description_long, image_link, video_link, icon, quote, quote_author, quote_author_position, order_number) VALUES ('Exclusive Offers', 'exclusive-offers', 'We work with our partner brands to provide passengers with exclusive offers and discounts on activities, dining, and accommodations.', $$<h2>Main Heading</h2><p>Erat eleifend lacus mattis at porttitor at mauris vel pharetra. Consequat, dictum et magna augue. Risus maecenas morbi ante scelerisque consequat. Id mi porttitor dui platea mauris. Accumsan, consequat feugiat at vitae laoreet pulvinar volutpat. Eros, mi tincidunt lorem donec aenean. Facilisis velit eget morbi urna, in sed viverra sagittis dolor. Ultrices sem mattis eget faucibus tortor, cras. Ipsum ac volutpat, nibh odio penatibus rhoncus, massa, leo tincidunt. Auctor diam massa a, elementum quis libero eleifend mauris porttitor.</p><p>Gravida pretium et, vestibulum ultrices est faucibus ullamcorper. Enim, egestas nulla vitae risus facilisis senectus maecenas bibendum.</p><h3>Secondary Heading</h3><p>Dapibus massa id duis commodo mauris non lacus porttitor sed. Nibh vitae, tellus eros nibh eget laoreet non. Pellentesque augue non nec at netus ut. Tincidunt nullam semper eros neque. Iaculis dictum in non ac massa. Eget sem massa aliquam sed. Hac pulvinar donec morbi eu senectus accumsan.</p><p>Feugiat aliquam elementum suspendisse eget in. Blandit ac tellus, etiam nunc. Vitae venenatis mi imperdiet arcu netus scelerisque amet nulla. Posuere dui ac pulvinar diam. Nisl senectus viverra ultricies convallis odio egestas. Odio pellentesque erat.</p><ul><li>Tincidunt nullam semper eros neque. Iaculis dictum in non ac massa. Eget sem massa aliquam sed.</li><li>Sit justo, gravida egestas tincidunt lorem curabitur lacus.</li><li>Varius diam consectetur pellentesque commodo. Volutpat gravida bibendum sit nunc. Et velit eu a, amet in ac, feugiat eget.</li></ul>$$, 'https://uploads-ssl.webflow.com/641881bc97e4e960fe5d8ec7/642101a07a77c62c61ba2b24_12.png', 'https://www.youtube.com/watch?v=Ojiv9Smi4XE', 'https://uploads-ssl.webflow.com/641881bc97e4e960fe5d8ec7/641881bc97e4e940e05d8ed3_icon-shopping-bag.svg', $$<p id=\"\">“Exploro gave us the confidence we needed to take our brand to the next level. At first we thought, should we do it? Then we took the plunge and <strong id=\"\">now we’re never looking back.</strong>”</p>$$, 'Kelly P.', 'Business Owner', 4);
INSERT INTO product_feature (title, slug, description_short, description_long, image_link, video_link, icon, quote, quote_author, quote_author_position, order_number) VALUES ('In-Flight Entertainment', 'in-flight-entertainment', 'Passengers can access the app through the in-flight entertainment system, providing them with a convenient and engaging way to plan their travel itinerary.', $$<h2>Main Heading</h2><p>Erat eleifend lacus mattis at porttitor at mauris vel pharetra. Consequat, dictum et magna augue. Risus maecenas morbi ante scelerisque consequat. Id mi porttitor dui platea mauris. Accumsan, consequat feugiat at vitae laoreet pulvinar volutpat. Eros, mi tincidunt lorem donec aenean. Facilisis velit eget morbi urna, in sed viverra sagittis dolor. Ultrices sem mattis eget faucibus tortor, cras. Ipsum ac volutpat, nibh odio penatibus rhoncus, massa, leo tincidunt. Auctor diam massa a, elementum quis libero eleifend mauris porttitor.</p><p>Gravida pretium et, vestibulum ultrices est faucibus ullamcorper. Enim, egestas nulla vitae risus facilisis senectus maecenas bibendum.</p><h3>Secondary Heading</h3><p>Dapibus massa id duis commodo mauris non lacus porttitor sed. Nibh vitae, tellus eros nibh eget laoreet non. Pellentesque augue non nec at netus ut. Tincidunt nullam semper eros neque. Iaculis dictum in non ac massa. Eget sem massa aliquam sed. Hac pulvinar donec morbi eu senectus accumsan.</p><p>Feugiat aliquam elementum suspendisse eget in. Blandit ac tellus, etiam nunc. Vitae venenatis mi imperdiet arcu netus scelerisque amet nulla. Posuere dui ac pulvinar diam. Nisl senectus viverra ultricies convallis odio egestas. Odio pellentesque erat.</p><ul><li>Tincidunt nullam semper eros neque. Iaculis dictum in non ac massa. Eget sem massa aliquam sed.</li><li>Sit justo, gravida egestas tincidunt lorem curabitur lacus.</li><li>Varius diam consectetur pellentesque commodo. Volutpat gravida bibendum sit nunc. Et velit eu a, amet in ac, feugiat eget.</li></ul>$$, 'https://uploads-ssl.webflow.com/641881bc97e4e960fe5d8ec7/642101caa2bd1e1a4158ff5c_14.png', 'https://www.youtube.com/watch?v=Ojiv9Smi4XE', '', $$<p id=\"\">“Exploro gave us the confidence we needed to take our brand to the next level. At first we thought, should we do it? Then we took the plunge and <strong id=\"\">now we’re never looking back.</strong>”</p>$$, 'Kelly P.', 'Business Owner', 1);
INSERT INTO product_feature (title, slug, description_short, description_long, image_link, video_link, icon, quote, quote_author, quote_author_position, order_number) VALUES ('Multi-Language Support', 'multi-language-support', 'Our app supports multiple languages, making it accessible to a diverse range of passengers.', $$<h2>Main Heading</h2><p>Erat eleifend lacus mattis at porttitor at mauris vel pharetra. Consequat, dictum et magna augue. Risus maecenas morbi ante scelerisque consequat. Id mi porttitor dui platea mauris. Accumsan, consequat feugiat at vitae laoreet pulvinar volutpat. Eros, mi tincidunt lorem donec aenean. Facilisis velit eget morbi urna, in sed viverra sagittis dolor. Ultrices sem mattis eget faucibus tortor, cras. Ipsum ac volutpat, nibh odio penatibus rhoncus, massa, leo tincidunt. Auctor diam massa a, elementum quis libero eleifend mauris porttitor.</p><p>Gravida pretium et, vestibulum ultrices est faucibus ullamcorper. Enim, egestas nulla vitae risus facilisis senectus maecenas bibendum.</p><h3>Secondary Heading</h3><p>Dapibus massa id duis commodo mauris non lacus porttitor sed. Nibh vitae, tellus eros nibh eget laoreet non. Pellentesque augue non nec at netus ut. Tincidunt nullam semper eros neque. Iaculis dictum in non ac massa. Eget sem massa aliquam sed. Hac pulvinar donec morbi eu senectus accumsan.</p><p>Feugiat aliquam elementum suspendisse eget in. Blandit ac tellus, etiam nunc. Vitae venenatis mi imperdiet arcu netus scelerisque amet nulla. Posuere dui ac pulvinar diam. Nisl senectus viverra ultricies convallis odio egestas. Odio pellentesque erat.</p><ul><li>Tincidunt nullam semper eros neque. Iaculis dictum in non ac massa. Eget sem massa aliquam sed.</li><li>Sit justo, gravida egestas tincidunt lorem curabitur lacus.</li><li>Varius diam consectetur pellentesque commodo. Volutpat gravida bibendum sit nunc. Et velit eu a, amet in ac, feugiat eget.</li></ul>$$, 'https://uploads-ssl.webflow.com/641881bc97e4e960fe5d8ec7/642101d55993ba97fc283dd7_10.png', 'https://www.youtube.com/watch?v=Ojiv9Smi4XE', '', $$<p id=\"\">“Exploro gave us the confidence we needed to take our brand to the next level. At first we thought, should we do it? Then we took the plunge and <strong id=\"\">now we’re never looking back.</strong>”</p>$$, 'Kelly P.', 'Business Owner', 2);
