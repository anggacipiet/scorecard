use chrono::{NaiveDateTime, Utc};
use derive_new::new;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{value::RawValue, Value};
use std::collections::HashMap;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub iat: i64,
    pub exp: i64,
    pub user: String,
    pub uid: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    #[serde(skip_deserializing)]
    pub employee_id: i32,
    #[serde(rename(serialize = "user_login"))]
    pub username: String,
    #[serde(skip_serializing)]
    pub password: String,
    #[serde(skip_deserializing)]
    pub nik: String,
    #[serde(skip_deserializing)]
    pub sfl_code: String,
    #[serde(skip_deserializing)]
    pub employee_name: String,
    #[serde(skip_deserializing)]
    pub email: String,
    #[serde(skip_deserializing)]
    pub user_type: String,
    #[serde(skip_deserializing)]
    pub role_name: String,
    #[serde(skip_deserializing)]
    pub brand_code: String,
    #[serde(skip_deserializing)]
    pub branch_name: String,
    #[serde(skip_deserializing)]
    pub region_name: String,
    #[serde(skip_deserializing)]
    pub avatar: String,
    #[serde(skip_deserializing)]
    pub application_id: String,
    #[serde(skip_deserializing)]
    pub token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Process {
    pub user_login: String,
    pub nik: String,
    pub employee_id: String,
    pub employee_name: String,
    pub latlng: String,
    pub date_latlng: String,
    #[serde(skip_deserializing)]
    pub process_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    pub action_package: String,
    pub application: Application,
    pub user: Process,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScTB {
    pub tb_id: i32,
    pub descr: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScTDB {
    pub tdb_id: i32,
    pub descr: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScTD {
    pub td_id: i32,
    pub descr: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScELC {
    pub ec_id: i32,
    pub descr: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub status: i32,
    pub status_descr: String,
    pub packages : Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScProfile {
    pub old: Vec<ScCustomer>,
    pub new: Vec<ScDetail>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScCustomer {
    pub customer_id: i64,
    pub customer_name: String,
    pub customer_name_update: Option<String>,
    pub customer_name_update_check: bool,
    pub address: String,
    pub address_update: Option<String>,
    pub address_update_check: bool,
    pub mobile_phone: String,
    pub mobile_phone_update: Option<String>,
    pub mobile_phone_update_check: bool,
    pub home_phone: String,
    pub home_phone_update: Option<String>,
    pub home_phone_update_check: bool,
    pub extra_phone: String,
    pub extra_phone_update: Option<String>,
    pub extra_phone_update_check: bool,
    pub whatsapp: String,
    pub whatsapp_update: Option<String>,
    pub whatsapp_update_check: bool,
    pub gender: String,
    pub gender_update: Option<String>,
    pub gender_update_check: bool,
    pub email: String,
    pub email_update: Option<String>,
    pub email_update_check: bool,
    pub foto: Option<String>,
    pub foto_update: Option<bool>,
    pub foto_update_check: bool,
    //pub created_date: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub foto: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScResult {
    pub wo_id : i64,
    pub customer_id: i64,
    pub tb_id: i32,
    pub tdb_id: i32,
    pub td_id: i32,
    pub ec_id: i32,
    pub employee_id: i32,
    pub latitude: String,
    pub longitude: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScCallback {
    pub id: i32,
    pub sc_id: i32,
    #[serde(skip_deserializing)]
    pub customer_id: i64,
    #[serde(skip_deserializing)]
    pub score: i32,
    #[serde(skip_deserializing)]
    pub sec: String,
    #[serde(skip_deserializing)]
    pub product_id: i32,
    #[serde(skip_deserializing)]
    pub product_name: String,
    #[serde(skip_deserializing)]
    pub promo_id: i32,
    #[serde(skip_deserializing)]
    pub promo_code: String,
    #[serde(skip_deserializing)]
    pub promo_descr: String,
    #[serde(skip_deserializing)]
    pub bill_freq: i32,
    #[serde(skip_deserializing)]
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScPackages {
    pub customer_id: i32,
    pub brand_id: i32,
    //note string
    pub promotion_id: i32,
    pub prospect_type: i32,
    //note string
    pub hardware_status: i32,
    pub customer_class: i32,
    pub house_status: i32,
    pub first_payment: i32,
    pub internet_package_router: Option<i32>,
    pub internet_package_addon: Option<i32>,
    //pub package: Vec<Box<ScBasic>>,
    //pub package: HashMap<String, Value>,
    pub package: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScBasic {
    pub billing_freq: Option<i32>,
    pub billing_freq_qty: Option<String>,
    pub package: Option<i32>,
    pub package_type: Option<String>,
    pub product_id: Option<i32>,
    pub product_name: Option<String>,
    pub hardware_product_id: Option<i32>,
    pub hardware_charge: Option<i32>,
    //pub list_addon: Vec<Box<ScAddon>>,
    //pub list_addon: HashMap<String, Value>,
    pub list_addon: Value,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScAddon {
    pub billing_freq: Option<String>,
    pub billing_freq_qty: Option<String>,
    pub package: Option<String>,
    pub package_type: Option<String>,
    pub product_id: Option<String>,
    pub product_name: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ScCalculate {
    pub ESTIMATED_INSTALLATION: String,
    pub ESTIMATED_COST_PACKAGE: i32,
    pub ESTIMATED_ALACARTE: i32,
    pub ESTIMATED_PROMO: i32,
    pub COST_PACKAGE: i32,
    pub COST_ALACARTE: Option<i32>,
    pub COST_INTERNET_ROUTER: i32,
    pub COST_INTERNET_ADDON: i32,
    pub BELI_PUTUS_CHARGE: Option<i32>,
    pub DECODER_HD_CHARGE: i32,
    pub COST_HD_CHARGE: i32,
    pub TOTAL_ESTIMATED_COSTS: i64,
    pub DETAIL_BASIC_PACKAGE: Vec<Value>,
    //pub DETAIL_ALACARTE: Option<Vec<Value>>,
    //pub DETAIL_INTERNET_ADDON: Option<Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Response {
    pub data: Box<RawValue>,
    pub message: String,
    pub status: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FileUpload {
    pub wo_id: i64,
    pub file_name: String,
    pub file_size: i64,
    pub file_path: String,
    pub file_type: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ScReason {
    pub id: i32,
    pub sc_id: i32,
    pub reason_id: i32,
    pub descr: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ScSimulation {
    pub id: i32,
    pub sc_id: i32,
    pub product_id: i32,
}