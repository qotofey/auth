BEGIN;

ALTER TABLE user_credentials DROP CONSTRAINT user_credentials_login_not_empty_check;

DROP INDEX user_credentials_login_trim_lower_unique;

ALTER TABLE user_credentials RENAME login TO value;

ALTER TABLE user_credentials ADD CONSTRAINT user_credentials_value_key UNIQUE (value);

COMMIT;
