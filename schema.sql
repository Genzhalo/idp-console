CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS users (
  id                VARCHAR(36) NOT NULL PRIMARY KEY DEFAULT uuid_generate_v4(),
  email             VARCHAR(64) NOT NULL UNIQUE,
  password_alg      VARCHAR(8) NOT NULL,
  password_hash     VARCHAR(255) NOT NULL,
  created_at        timestamp NOT NULL DEFAULT NOW()
);


CREATE TABLE IF NOT EXISTS user_tokens (
  id                SERIAL PRIMARY KEY,
  user_id           VARCHAR(36) NOT NULL,
  token             VARCHAR NOT NULL,
  type              VARCHAR(16) NOT NULL,

  CONSTRAINT fk_user_tokens
    FOREIGN KEY(user_id) 
      REFERENCES users(id)
        ON DELETE CASCADE
);


CREATE INDEX IF NOT EXISTS idx_user_token_token ON user_tokens (token);
CREATE INDEX IF NOT EXISTS idx_user_token_type ON user_tokens (type);


CREATE TABLE IF NOT EXISTS respondents (
  id                VARCHAR(36) NOT NULL PRIMARY KEY DEFAULT uuid_generate_v4(),
  passport_id       VARCHAR(64) NOT NULL UNIQUE,
  idp_code          VARCHAR(64),
  first_name        VARCHAR(64) NOT NULL,
  last_name         VARCHAR(64) NOT NULL,
  phone             VARCHAR(16) NOT NULL,
  region            VARCHAR(64) NOT NULL,
  children          SMALLINT NOT NULL DEFAULT 0,
  created_at        timestamp NOT NULL DEFAULT NOW()
);


CREATE TABLE IF NOT EXISTS forms (
  id                    VARCHAR(36) NOT NULL PRIMARY KEY DEFAULT uuid_generate_v4(),
  name                  VARCHAR(64) NOT NULL,
  form_limit            INT NOT NULL,
  status                VARCHAR(16) NOT NULL,
  scheduled_start_date  timestamp NOT NULL,
  scheduled_end_date    timestamp NOT NULL,
  created_at            timestamp NOT NULL DEFAULT NOW(),
  time_frame_duration   INT NOT NULL DEFAULT 0,
  exclude_form_ids      text[]
);


CREATE TABLE IF NOT EXISTS submissions (
  id                VARCHAR(36) NOT NULL PRIMARY KEY DEFAULT uuid_generate_v4(),
  form_id           VARCHAR(36) NOT NULL,
  respondent_id     VARCHAR(36) NOT NULL,
  sub_order         INT NOT NULL,
  arrival_date      timestamp NOT NULL,
  status            VARCHAR(16) NOT NULL,
  created_at        timestamp NOT NULL DEFAULT NOW(),
  
  CONSTRAINT fk_form
    FOREIGN KEY(form_id) 
      REFERENCES forms(id)
        ON DELETE CASCADE,

  CONSTRAINT fk_respondent
    FOREIGN KEY(respondent_id) 
      REFERENCES respondents(id)
        ON DELETE CASCADE
);
