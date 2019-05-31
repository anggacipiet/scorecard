-- Your SQL goes here
CREATE TABLE users
(
  ID int(11) NOT NULL
  AUTO_INCREMENT,
  IMEI         varchar
  (255) NOT NULL,
  NAME         varchar
  (255) NOT NULL,
  PASSWORD     varchar
  (255),
  PRIMARY KEY
  (ID),
  UNIQUE
  (IMEI)
);