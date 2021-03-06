-- Your SQL goes here
CREATE TABLE SC_PACKAGES(
    PACK_ID INT(11) NOT NULL PRIMARY KEY AUTO_INCREMENT,
    CALLBACK_ID INT
(11) NOT NULL,
    CREATED_DATE DATETIME,
    KEY SC_PACK_1
(CALLBACK_ID),
     CONSTRAINT SC_PACKFK_1 FOREIGN KEY
(CALLBACK_ID) REFERENCES SC_CALLBACK
(CALLBACK_ID)
)ENGINE = InnoDB DEFAULT CHARSET = utf8;


CREATE TABLE SC_CALCULATE(
    CAL_ID INT(11) NOT NULL PRIMARY KEY AUTO_INCREMENT,
    PACK_ID INT
(11) NOT NULL,
    COST_BASIC INT
(11) NOT NULL,
    COST_ADDON INT
(11) NOT NULL,
    TOTAL INT
(11) NOT NULL,
    CREATED_DATE DATETIME,
    KEY SC_CALC_1
(PACK_ID),
     CONSTRAINT SC_CALCFK_1 FOREIGN KEY
(PACK_ID) REFERENCES SC_PACKAGES
(PACK_ID)
)ENGINE = InnoDB DEFAULT CHARSET = utf8;