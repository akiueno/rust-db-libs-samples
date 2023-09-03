-- Add up migration script here
CREATE SCHEMA sqlx;

CREATE TABLE sqlx.product_category (
  id BIGSERIAL NOT NULL,
  name varchar(20) NOT NULL DEFAULT '',
  created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  CONSTRAINT product_category_pk PRIMARY KEY (id)
);

CREATE TABLE sqlx.product (
  id BIGSERIAL NOT NULL,
  name varchar(20) NOT NULL DEFAULT '',
  price integer NOT NULL DEFAULT 0,
  category_id BIGINT NOT NULL,
  created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  CONSTRAINT product_pk PRIMARY KEY (id),
  CONSTRAINT product_category_fk FOREIGN KEY(category_id)
    REFERENCES sqlx.product_category(id)
    ON UPDATE NO ACTION ON DELETE NO ACTION
);
