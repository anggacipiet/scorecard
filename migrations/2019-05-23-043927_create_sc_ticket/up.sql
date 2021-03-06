-- Your SQL goes here
CREATE TABLE SC_WORKORDER
(
    ID INT
(11) NOT NULL
    AUTO_INCREMENT,
    WO_ID BIGINT
    (20) NOT NULL,
    CUSTOMER_ID BIGINT
    (20) NOT NULL,
    PROSPECT_NBR VARCHAR
    (20) NOT NULL,
    ASSIGN_TO INT
    (11) NOT NULL,
    SERVICES_ID INT
    (11) NOT NULL,
    SERVICES_DESCR VARCHAR
    (255) NOT NULL,
    SERVICES_CATEGORY VARCHAR
    (5) NOT NULL,
    DESCR TEXT NULL,
    SCHEDULE_DATE DATETIME NOT NULL,
    REGION VARCHAR
    (255) NOT NULL,
    LATITUDE VARCHAR
    (100) NULL,
    LONGITUDE VARCHAR
    (100) NULL,
    CREATED_DATE DATETIME,
    UPDATED_DATE DATETIME,
    INDEX IDX_1
    (SERVICES_ID),
    INDEX IDX_2
    (REGION),
    INDEX IDX_3
    (SERVICES_CATEGORY),
    INDEX IDX_4
    (SCHEDULE_DATE),
    INDEX IDX_5
    (ASSIGN_TO),
    UNIQUE
    (WO_ID),
    UNIQUE
    (CUSTOMER_ID),
    UNIQUE
    (PROSPECT_NBR),
    PRIMARY KEY
    (ID, WO_ID, CUSTOMER_ID, PROSPECT_NBR)
    
)ENGINE = InnoDB DEFAULT CHARSET = utf8;

    CREATE TABLE SC_CUSTOMER
    (
        ID INT(11) NOT NULL
        AUTO_INCREMENT,
    CUSTOMER_ID BIGINT
        (20) NOT NULL,
    CUSTOMER_NAME VARCHAR
        (255) NOT NULL,
    ADDRESS TEXT NOT NULL,
    MOBILE_PHONE VARCHAR
        (100) NULL,
    HOME_PHONE VARCHAR
        (100) NULL,
    EXTRA_PHONE VARCHAR
        (100) NULL,
    WHATSAPP VARCHAR
        (100) NULL,
    GENDER CHAR
        (1) NULL,
    EMAIL VARCHAR
        (100) NULL,
    FOTO VARCHAR
        (255) NULL,
    BRAND INT
        (11) NULL,
    PROSPECT_TYPE INT
        (11) NULL,
    CUSTOMER_CLASS INT
        (11) NULL,
    HW_STATUS INT
        (11) NULL,
    HOUSE_STATUS INT
        (11) NULL,
    PROMO_ID INT
        (11) NULL,
    PRODUCT_ID INT
        (11) NULL,
    BF INT
        (11) NULL,
    CREATED_DATE DATETIME,
    UPDATED_DATE DATETIME,
    KEY SC_CUS_1
        (CUSTOMER_ID),
    CONSTRAINT SC_FK_1 
    FOREIGN KEY
        (CUSTOMER_ID) REFERENCES SC_WORKORDER
        (CUSTOMER_ID),
    INDEX IDX_1
        (CUSTOMER_NAME),
    INDEX IDX_2
        (PROSPECT_TYPE),
    INDEX IDX_3
        (BRAND),
    INDEX IDX_4
        (HW_STATUS),
    INDEX IDX_5
        (PROMO_ID),
    INDEX IDX_6
        (PRODUCT_ID),
    INDEX IDX_7
        (BF),
    INDEX IDX_8
        (FOTO),
    INDEX IDX_9
        (CUSTOMER_CLASS),
    UNIQUE
        (WHATSAPP),
    PRIMARY KEY
        (ID,CUSTOMER_ID)
)ENGINE = InnoDB DEFAULT CHARSET = utf8;