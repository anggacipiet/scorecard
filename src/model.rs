use chrono::{NaiveDateTime, Utc};
use derive_new::new;
use serde::{Deserialize, Serialize};
use serde_json::{value::RawValue, Value};
use std::collections::HashMap;


#[derive(Serialize, Deserialize)]
pub struct Token {
    pub iat: i64,
    pub exp: i64,
    pub user: String,
    pub uid: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize, new)]
pub struct User {
    #[serde(skip_deserializing)]
    #[new(default)]
    pub employee_id: i32,
    #[new(default)]
    pub username: String,
    #[serde(skip_serializing)]
    #[new(default)]
    pub password: String,
    #[new(default)]
    pub token: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Application {
    pub application_id: String,
    pub application_name: String,
    pub version_code: String,
    pub version_name: String,
    pub database_name: String,
    pub database_version: String,
    pub device_name: String,
    pub os: String,
    pub imei: String,
    pub ip_address: String,
    //#[serde(skip_deserializing)]
    //pub request: Box<RawValue>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Process {
    pub user_login: String,
    pub nik: String,
    pub employee_id: String,
    pub employee_name: String,
    pub latlng: String,
    pub time_latlng: String,
    pub process_name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Request {
    pub application: Application,
    pub process: Process,
    //#[serde(flatten)]
    //pub data: Data,
    pub data: Box<RawValue>,
    //pub data: HashMap<String, Value>,
}

/*
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Data {
    User(User),
    ScDetail(ScDetail),
    ScResult(ScResult),
}

impl Data {

    pub fn as_user(&self) -> Option<&User> {
        match *self {
            Data::User(ref u) => Some(u),
            _ => None,
        }
    }

    pub fn as_detail(&self) -> Option<&ScDetail> {
        match *self {
            Data::ScDetail(ref d) => Some(d),
            _ => None,
        }
    }

    pub fn as_result(&self) -> Option<&ScResult> {
        match *self {
            Data::ScResult(ref r) => Some(r),
            _ => None,
        }
    }
}
*/

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

#[derive(Serialize, Deserialize, Debug, Clone)]
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
