use chrono::{NaiveDateTime, Utc};

#[derive(Serialize, Debug)]
pub struct ScTB {
    pub tb_id: i32,
    pub descr: String,
}

#[derive(Serialize, Debug)]
pub struct ScTDB {
    pub tdb_id: i32,
    pub descr: String,
}

#[derive(Serialize, Debug)]
pub struct ScTD {
    pub td_id: i32,
    pub descr: String,
}

#[derive(Serialize, Debug)]
pub struct ScELC {
    pub ec_id: i32,
    pub descr: String,
}

#[derive(Serialize, Debug)]
pub struct ScWorkorder {
    pub wo_id: i64,
    pub customer_id: i64,
    pub prospect_nbr: String,
    pub employee_id: i32,
    pub service_id: i32,
    pub service_descr: String,
    pub category: String,
    pub descr: String,
    pub schedule_date: NaiveDateTime,
    pub region: String,
    pub latitude: String,
    pub longitude: String,
    pub created_date: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScCustomer {
    pub customer_id: i64,
    pub customer_name: String,
    pub address: String,
    pub mobile_phone: String,
    pub home_phone: String,
    pub extra_phone: String,
    pub whatsapp: String,
    pub gender: String,
    pub email: String,
    pub foto: String,
    //pub created_date: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScDetail {
    pub customer_id: i64,
    pub customer_name: String,
    pub address: String,
    pub mobile_phone: String,
    pub home_phone: String,
    pub extra_phone: String,
    pub whatsapp: String,
    pub gender: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug,  Clone)]
pub struct ScResult {
    pub customer_id: i64,
    pub tb_id: i32,
    pub tdb_id: i32,
    pub td_id: i32,
    pub ec_id: i32,
    pub employee_id: i32,
    pub latitude: String,
    pub longitude: String,
}

#[derive(Serialize, Debug)]
pub struct ScCallback {
    pub sc_id: i32,
    pub customer_id: i64,
    pub score: i32,
    pub sec: String,
    pub product_id: i32,
    pub product_name: String,
    pub promo_id: i32,
    pub promo_code: String,
    pub promo_descr: String,
}
