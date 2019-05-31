table! {
    SC_CALCULATE (CAL_ID) {
        CAL_ID -> Integer,
        PACK_ID -> Integer,
        COST_BASIC -> Integer,
        COST_ADDON -> Integer,
        TOTAL -> Integer,
        CREATED_DATE -> Nullable<Datetime>,
    }
}

table! {
    SC_CALLBACK (CALLBACK_ID) {
        CALLBACK_ID -> Integer,
        SCORE_ID -> Integer,
        MAT_ID -> Integer,
        MAP_ID -> Integer,
        SOURCE -> Nullable<Char>,
        CREATED_DATE -> Nullable<Datetime>,
    }
}

table! {
    SC_CUSTOMER (ID, CUSTOMER_ID) {
        ID -> Integer,
        CUSTOMER_ID -> Bigint,
        CUSTOMER_NAME -> Varchar,
        ADDRESS -> Text,
        MOBILE_PHONE -> Nullable<Varchar>,
        HOME_PHONE -> Nullable<Varchar>,
        EXTRA_PHONE -> Nullable<Varchar>,
        WHATSAPP -> Nullable<Varchar>,
        GENDER -> Nullable<Char>,
        EMAIL -> Nullable<Varchar>,
        FOTO -> Nullable<Varchar>,
        BRAND -> Nullable<Integer>,
        PROSPECT_TYPE -> Nullable<Integer>,
        CUSTOMER_CLASS -> Nullable<Integer>,
        HW_STATUS -> Nullable<Integer>,
        HOUSE_STATUS -> Nullable<Integer>,
        PROMO_ID -> Nullable<Integer>,
        PRODUCT_ID -> Nullable<Integer>,
        BF -> Nullable<Integer>,
        CREATED_DATE -> Nullable<Datetime>,
        UPDATED_DATE -> Nullable<Datetime>,
    }
}

table! {
    SC_ELC (EC_ID, TB_ID, TDB_ID) {
        EC_ID -> Integer,
        TB_ID -> Integer,
        TDB_ID -> Integer,
        DESCR -> Varchar,
        STATUS -> Char,
        CREATED_DATE -> Datetime,
    }
}

table! {
    SC_MAPPING_PRODUCT (MAP_ID) {
        MAP_ID -> Integer,
        PRODUCT_ID -> Integer,
        PRODUCT_NAME -> Varchar,
        BILL_FREQ -> Integer,
        PROMO_ID -> Integer,
        PROMO_CODE -> Varchar,
        PROMO_DESCR -> Text,
        STATUS -> Nullable<Char>,
        CREATED_DATE -> Nullable<Datetime>,
    }
}

table! {
    SC_MATRIX (MAT_ID) {
        MAT_ID -> Integer,
        PROD_SLS -> Integer,
        BF_PROD_SLS -> Integer,
        ADDON_SLS -> Nullable<Char>,
        MIN_BF_ADDON_SLS -> Nullable<Integer>,
        SCORE -> Integer,
        REGION -> Varchar,
        PROD_TS -> Integer,
        BF_PROD_TS -> Integer,
        ADDON_TS -> Nullable<Char>,
        BF_ADDON_TS -> Nullable<Integer>,
        STATUS -> Nullable<Char>,
        CREATED_DATE -> Nullable<Datetime>,
    }
}

table! {
    SC_PACKAGES (PACK_ID) {
        PACK_ID -> Integer,
        CALLBACK_ID -> Integer,
        CREATED_DATE -> Nullable<Datetime>,
    }
}

table! {
    SC_RESULT_DETAIL (ID, CUSTOMER_ID) {
        ID -> Integer,
        CUSTOMER_ID -> Integer,
        CUSTOMER_NAME -> Nullable<Varchar>,
        ADDRESS -> Nullable<Text>,
        MOBILE_PHONE -> Nullable<Varchar>,
        HOME_PHONE -> Nullable<Varchar>,
        EXTRA_PHONE -> Nullable<Varchar>,
        WHATSAPP -> Nullable<Varchar>,
        GENDER -> Nullable<Char>,
        EMAIL -> Nullable<Varchar>,
        CREATED_DATE -> Nullable<Datetime>,
    }
}

table! {
    SC_RESULT_NEW (SC_ID, CUSTOMER_ID) {
        SC_ID -> Integer,
        CUSTOMER_ID -> Bigint,
        TB_ID -> Integer,
        TDB_ID -> Integer,
        TD_ID -> Nullable<Integer>,
        EC_ID -> Integer,
        EMPLOYEE_ID -> Integer,
        LATITUDE -> Varchar,
        LONGITUDE -> Varchar,
        CREATED_DATE -> Nullable<Datetime>,
    }
}

table! {
    SC_RESULT_SCORE (SCORE_ID) {
        SCORE_ID -> Integer,
        SEC_ID -> Integer,
        SC_ID -> Integer,
        CREATED_DATE -> Nullable<Datetime>,
    }
}

table! {
    SC_SCORE (SEC_ID) {
        SEC_ID -> Integer,
        TB_ID -> Integer,
        TDB_ID -> Integer,
        TD_ID -> Nullable<Integer>,
        EC_ID -> Integer,
        SCORE -> Integer,
        SEC -> Varchar,
        STATUS -> Nullable<Char>,
        CREATED_DATE -> Nullable<Datetime>,
    }
}

table! {
    SC_TB (TB_ID) {
        TB_ID -> Integer,
        DESCR -> Varchar,
        STATUS -> Char,
        CREATED_DATE -> Datetime,
    }
}

table! {
    SC_TD (TD_ID) {
        TD_ID -> Integer,
        TB_ID -> Integer,
        DESCR -> Varchar,
        STATUS -> Char,
        CREATED_DATE -> Datetime,
    }
}

table! {
    SC_TDB (TDB_ID) {
        TDB_ID -> Integer,
        TB_ID -> Integer,
        DESCR -> Varchar,
        STATUS -> Char,
        CREATED_DATE -> Datetime,
    }
}

table! {
    SC_WORKORDER (ID, WO_ID, CUSTOMER_ID, PROSPECT_NBR) {
        ID -> Integer,
        WO_ID -> Bigint,
        CUSTOMER_ID -> Bigint,
        PROSPECT_NBR -> Varchar,
        ASSIGN_TO -> Integer,
        SERVICES_ID -> Integer,
        SERVICES_DESCR -> Varchar,
        SERVICES_CATEGORY -> Varchar,
        DESCR -> Nullable<Text>,
        SCHEDULE_DATE -> Datetime,
        REGION -> Varchar,
        LATITUDE -> Nullable<Varchar>,
        LONGITUDE -> Nullable<Varchar>,
        CREATED_DATE -> Nullable<Datetime>,
        UPDATED_DATE -> Nullable<Datetime>,
    }
}

table! {
    users (ID) {
        ID -> Integer,
        IMEI -> Varchar,
        NAME -> Varchar,
        PASSWORD -> Nullable<Varchar>,
    }
}

joinable!(SC_CALCULATE -> SC_PACKAGES (PACK_ID));
joinable!(SC_CALLBACK -> SC_MAPPING_PRODUCT (MAP_ID));
joinable!(SC_CALLBACK -> SC_MATRIX (MAT_ID));
joinable!(SC_CALLBACK -> SC_RESULT_SCORE (SCORE_ID));
joinable!(SC_ELC -> SC_TB (TB_ID));
joinable!(SC_ELC -> SC_TDB (TDB_ID));
joinable!(SC_PACKAGES -> SC_CALLBACK (CALLBACK_ID));
joinable!(SC_RESULT_NEW -> SC_TB (TB_ID));
joinable!(SC_RESULT_NEW -> SC_TD (TD_ID));
joinable!(SC_RESULT_NEW -> SC_TDB (TDB_ID));
joinable!(SC_RESULT_SCORE -> SC_SCORE (SEC_ID));
joinable!(SC_SCORE -> SC_TB (TB_ID));
joinable!(SC_SCORE -> SC_TD (TD_ID));
joinable!(SC_SCORE -> SC_TDB (TDB_ID));
joinable!(SC_TD -> SC_TB (TB_ID));
joinable!(SC_TDB -> SC_TB (TB_ID));

allow_tables_to_appear_in_same_query!(
    SC_CALCULATE,
    SC_CALLBACK,
    SC_CUSTOMER,
    SC_ELC,
    SC_MAPPING_PRODUCT,
    SC_MATRIX,
    SC_PACKAGES,
    SC_RESULT_DETAIL,
    SC_RESULT_NEW,
    SC_RESULT_SCORE,
    SC_SCORE,
    SC_TB,
    SC_TD,
    SC_TDB,
    SC_WORKORDER,
    users,
);
