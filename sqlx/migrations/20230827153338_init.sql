CREATE SCHEMA sqlx;

CREATE TABLE sqlx.product_category (
  id BIGSERIAL NOT NULL,
  name varchar(20) not null default '',
  CONSTRAINT product_category_pk PRIMARY KEY (id)
);

CREATE TABLE sqlx.product (
  id BIGSERIAL NOT NULL,
  name varchar(20) not  null default '',
  price integer not null default 0,
  category_id BIGINT NOT NULL,
  CONSTRAINT product_pk PRIMARY KEY (id),
  CONSTRAINT product_category_fk FOREIGN KEY(category_id)
    REFERENCES sqlx.product_category(id)
    ON UPDATE NO ACTION ON DELETE NO ACTION
);
