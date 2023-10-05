----------------------------------------
--             WARNING!               --
--  if you change the table schema,   --
--  you will have to also change the  --
--               code!                --
----------------------------------------

CREATE OR REPLACE FUNCTION update_updated_on()
RETURNS TRIGGER AS $$
BEGIN
  NEW.edited = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

--------------MUST BE FIRST IN THIS ORDER!-----------
\i /docker-entrypoint-initdb.d/sql/continent.sql
\i /docker-entrypoint-initdb.d/sql/country.sql
\i /docker-entrypoint-initdb.d/sql/region.sql
\i /docker-entrypoint-initdb.d/sql/city.sql
\i /docker-entrypoint-initdb.d/sql/partner_vendor.sql
-----------------------------------------------------

\i /docker-entrypoint-initdb.d/sql/api_key.sql
\i /docker-entrypoint-initdb.d/sql/employee.sql
\i /docker-entrypoint-initdb.d/sql/auth.sql
\i /docker-entrypoint-initdb.d/sql/blog_category.sql
\i /docker-entrypoint-initdb.d/sql/blog.sql
\i /docker-entrypoint-initdb.d/sql/client.sql
\i /docker-entrypoint-initdb.d/sql/event.sql
\i /docker-entrypoint-initdb.d/sql/hotel.sql
\i /docker-entrypoint-initdb.d/sql/job_listing.sql
\i /docker-entrypoint-initdb.d/sql/product_feature.sql
\i /docker-entrypoint-initdb.d/sql/restaurant.sql
\i /docker-entrypoint-initdb.d/sql/activity.sql
\i /docker-entrypoint-initdb.d/sql/tag.sql
