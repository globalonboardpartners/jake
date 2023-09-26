CREATE TYPE status AS ENUM ('Active', 'Inactive', 'Suspended');

CREATE TABLE auth (
  id SERIAL PRIMARY KEY,
  email VARCHAR(255) UNIQUE NOT NULL,
  username VARCHAR(50) UNIQUE NOT NULL,
  password VARCHAR(128) NOT NULL,
  security_level SMALLINT NOT NULL,
  employee_id INT REFERENCES employee(id),
  status status DEFAULT 'Active' NOT NULL,
  last_login TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
  failed_login_attempts INT DEFAULT 0,
  created TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
  edited TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);

INSERT INTO auth (
  email, 
  username, 
  password, 
  security_level, 
  employee_id, 
  status, 
  failed_login_attempts
)
VALUES (
  'john.doe@example.com',  -- email
  'john_doe',             -- username
  'hashed_password_here', -- password
  1,                      -- security_level
  1,                     -- employee_id
  'Active',               -- status
  0                      -- failed_login_attempts
);
