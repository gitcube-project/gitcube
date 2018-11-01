CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  user_name VARCHAR(25) NOT NULL,
  user_email VARCHAR(50) NOT NULL,
  user_password VARCHAR(255) NOT NULL
)