BEGIN;

ALTER TABLE user_credentials DROP CONSTRAINT user_credentials_value_key;

ALTER TABLE user_credentials RENAME value TO login;

CREATE UNIQUE INDEX user_credentials_login_trim_lower_unique ON user_credentials (LOWER(TRIM(login)));

ALTER TABLE user_credentials ADD CONSTRAINT user_credentials_login_not_empty_check CHECK (TRIM(login) != '');

COMMIT;
