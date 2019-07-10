use crate::model::{Response, ScBasic, ScCalculate, ScPackages};
use failure;
use failure::Error;
use lazy_static;
use log::{debug, error, info, warn};
use reqwest::{
    header::{HeaderMap, ACCEPT, AUTHORIZATION, CONTENT_TYPE, USER_AGENT},
    Client as HttpClient, Method, StatusCode, Url,
};
use serde::de::Deserialize;
use serde_json::{json, map::Map, Value};

use crate::db::{getCalculate, get_profile, push_ppg, Conn, ConnSFA, TrxCalculate, TrxUpdTrex};
use crate::errors::AppError;
use crate::failures::CustomError;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Debug;
use std::hash::Hash;
use std::io::Read;
use std::string::{String, ToString};
use std::time::Duration;


pub static BASE_URL: &str = "http://192.168.177.209/valsys";
//"http://192.168.177.221:8080/api"

pub struct ScClient {
    client: HttpClient,
    server: Url,
}

/// Describes API errors
#[derive(Debug)]
pub enum ApiError {
    Unauthorized,
    RateLimited(Option<usize>),
    Other(u16),
}
impl failure::Fail for ApiError {}
impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Calculation API Valsys error")
    }
}
impl From<&reqwest::Response> for ApiError {
    fn from(response: &reqwest::Response) -> Self {
        match response.status() {
            StatusCode::UNAUTHORIZED => ApiError::Unauthorized,
            StatusCode::TOO_MANY_REQUESTS => {
                if let Ok(duration) = response.headers()[reqwest::header::RETRY_AFTER].to_str() {
                    ApiError::RateLimited(duration.parse::<usize>().ok())
                } else {
                    ApiError::RateLimited(None)
                }
            }
            status => ApiError::Other(status.as_u16()),
        }
    }
}

impl ScClient {

    pub fn send_result(&mut self) -> Result<Response, failure::Error> {
        let req = r#"
        {
            "application": {
            "device_name": "Redmi Note 4G",
            "os": "Kitkat 4.4",
            "imei": "1234567890",
            "ip_address": "192.168.168.221",
            "application_name": "SCORECARD for UAT",
            "application_id": "id.mncvision.scorecard",
            "version_name": "1.0.0",
            "version_code": "1",
            "database_name": "1",
            "database_version": "mncvision_scorecard.db"
            },
            "process": {
            "user_login": "myusufsfa",
            "nik": "9595190207",
            "employee_id": "12345",
            "employee_name": "M MAULANA YUSUF",
            "latlng": "1234, -1234",
            "time_latlng": "2019-05-14 21:07:41",
            "process_name": "sc list wo"
            },
            "data": {"customer_id":500280534,"tb_id":1,"tdb_id":2,"td_id":1,"ec_id":2,"employee_id":50585,"latitude":"0.0","longitude":"0.0"}
        }"#;
        let v: Value = serde_json::from_str(req)?;
        println!("request: {:?}", v);
        let o = self.post("/v1.0.0/sc-result/2", &v);
        println!("response: {:?}", o);
        match o {
            Ok(ok) => {
                let resp = serde_json::from_str::<Response>(&ok)?;
                println!("bind: {:?}", resp);
                Ok(resp)
            }
            Err(e) => Err(e),
        }

    }

    pub fn Calculate(
        &mut self,
        conn: &mut Conn,
        sfa: &mut ConnSFA,
        sc_id: &i32,
        cb_id: &i32,
    ) -> Result<Vec<ScCalculate>, Error> {
        /*
        let req = r#"
        {
            "brand_id": 1,
            "promotion_id": "257",
            "prospect_type": 104,
            "hardware_status": "3",
            "customer_class": 1,
            "house_status": 1,
            "first_payment": 0,
            "internet_package_router": 199,
            "internet_package_addon": 243,
            "package": [{
                "billing_freq": "2",
                "billing_freq_qty": "3",
                "package": 1,
                "package_type": "basic",
                "product_id": "26",
                "product_name": "Venus",
                "hardware_product_id": 1,
                "hardware_charge": null,
                "list_addon": [
                {
                    "billing_freq": "2",
                    "billing_freq_qty": "3",
                    "package": 1,
                    "package_type": "addon",
                    "product_id": "21",
                    "product_name": "Cinema 3"
                },
                {
                    "billing_freq": "2",
                    "billing_freq_qty": "3",
                    "package": 1,
                    "package_type": "addon",
                    "product_id": "247",
                    "product_name": "Besmart"
                }]
            }]
        }"#;
        let v: Value = serde_json::from_str(req)?;
        info!("request: {:?}", v);
        //RESPONSE BY EMAIL
        //{ "ESTIMATED_INSTALLATION": "200000", "ESTIMATED_COST_PACKAGE": 2158800, "COST_PACKAGE": 2158800,
        //"ESTIMATED_ALACARTE": 1440000, "COST_ALACARTE": 1440000, "COST_INTERNET_ADDON": 2877600,
        // "COST_INTERNET_ROUTER": 0, "ESTIMATED_PROMO": 200000, "BELI_PUTUS_CHARGE": 0,
        //"DECODER_HD_CHARGE": 0, "COST_HD_CHARGE": 0, "TOTAL_ESTIMATED_COSTS": 6476400 }
        let o = self.post("/api/Calculation/calculateEstimation", &v);
        println!("response: {:?}", o);
        match o {
            Ok(ok) => {
                let resp = serde_json::from_str::<ScCalculate>(&ok).unwrap();
                println!("bind: {:?}", resp);
                Ok(resp)
            },
            Err(_) => Err(Error::from(CustomError::new("")))
        }
        */
        match getCalculate(conn, &sc_id, &cb_id) {
            Ok(Some(oke)) => {
                let req = json!({
                    "brand_id": oke.brand_id,
                    "promotion_id": oke.promotion_id,
                    "prospect_type": oke.prospect_type,
                    "hardware_status": oke.hardware_status,
                    "customer_class": oke.customer_class,
                    "house_status": oke.house_status,
                    "first_payment": oke.first_payment,
                    "internet_package_router": oke.internet_package_router,
                    "internet_package_addon": oke.internet_package_addon,
                    "package": oke.package
                });
                let v: Value = serde_json::from_value(req).unwrap();
                println!("request: {:?}", v);
                let o = self.post("/api/Calculation/calculateEstimation", &v);
                println!("response: {:?}", o);
                match o {
                    Ok(ok) => {
                        let resp = serde_json::from_str::<ScCalculate>(&ok).unwrap();
                        println!("bind: {:?}", resp);
                        match get_profile(conn, oke.customer_id) {
                            Ok(ooo) => {
                                println!("get name: {:?}", ooo[0].customer_name);
                                match TrxCalculate(
                                    conn,
                                    &resp,
                                    &sc_id,
                                    &cb_id,
                                    format!("{}", v).as_str(),
                                    &ok,
                                ) {
                                    Ok(_) => {
                                        match push_ppg(
                                            conn,
                                            &ooo[0].customer_id,
                                            &ooo[0].customer_name.to_uppercase(),
                                            &resp.TOTAL_ESTIMATED_COSTS,
                                        ) {
                                            Ok(_) => {
                                                /*match TrxUpdTrex(sfa, &oke.customer_id, &resp) {
                                                    Ok(_) => Ok(vec![resp]),
                                                    Err(e) => Err(Error::from(CustomError::new(
                                                        format!("{}", e).as_str(),
                                                    ))),
                                                }*/
                                                Ok(vec![resp])
                                            }
                                            Err(e) => Err(Error::from(CustomError::new(
                                                format!("{}", e).as_str(),
                                            ))),
                                        }
                                    }
                                    Err(e) => Err(Error::from(CustomError::new(
                                        format!("{}", e).as_str(),
                                    ))),
                                }
                            }
                            Err(e) => Err(Error::from(CustomError::new(format!("{}", e).as_str()))),
                        }
                    }
                    Err(e) => Err(Error::from(CustomError::new(format!("{}", e).as_str()))),
                }
            }
            Ok(None) => Ok(serde_json::from_value(json!(vec![{}])).unwrap()),
            Err(e) => Err(Error::from(CustomError::new(format!("{}", e).as_str()))),
        }
    }
}

impl ScClient {

    pub fn default() -> ScClient {
        let client = HttpClient::new();
        let server = BASE_URL.parse().unwrap();
        return ScClient {
            client: client,
            server: server,
        };
    }

    fn get_url(&self) -> &str {
        self.server.as_str()
    }

    fn get_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();

        headers.insert(USER_AGENT, "SC-CLIENT-v1.0.0".parse().unwrap());
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        headers.insert(ACCEPT, "application/json".parse().unwrap());
        //headers.insert(AUTHORIZATION, "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpYXQiOjE1NjAyMTE1NjMsImV4cCI6MTU2MDgxNjM2MywidXNlciI6InRlc3QiLCJ1aWQiOjEyMzQ1fQ.b7X0s-E9-egaId6Gp7CRcNBu7UZZ0_YeHj3kM5eTUOI".parse().unwrap());
        return headers;
    }

    fn send_request(
        &self,
        method: Method,
        path: &str,
        payload: Option<&Value>,
    ) -> Result<String, failure::Error> {

        let mut url: Cow<str> = path.into();
        if !url.starts_with("https") {
            url = [self.get_url(), &url].concat().into();
            println!("url :{:?}", url);
        }
        let mut resp = {
            let builder = self
                .client
                .request(method, &url.into_owned())
                .headers(self.get_headers());

            let builder = if let Some(json) = payload {
                builder.json(json)
            } else {
                builder
            };

            builder.send().unwrap()
        };

        let mut buf = String::new();
        resp.read_to_string(&mut buf)
            .expect("failed to read response");
        if resp.status().is_success() {
            Ok(buf)
        } else {
            Err(failure::Error::from(ApiError::from(&resp)))
        }
    }

    //Convert Map To String
    fn convert_map_to_string<K: Debug + Eq + Hash + ToString, V: Debug + ToString>(
        &self,
        map: &HashMap<K, V>,
    ) -> String {
        let mut string: String = String::new();
        for (key, value) in map.iter() {
            string.push_str(&key.to_string());
            string.push_str("=");
            string.push_str(&value.to_string());
            string.push_str("&");
        }
        string
    }

    //send get request
    fn get(
        &self,
        url: &str,
        params: &mut HashMap<String, String>,
    ) -> Result<String, failure::Error> {
        if !params.is_empty() {
            let param: String = self.convert_map_to_string(params);
            let mut url_with_params = url.to_owned();
            url_with_params.push('?');
            url_with_params.push_str(&param);
            self.send_request(Method::GET, &url_with_params, None)
        } else {
            self.send_request(Method::GET, url, None)
        }
    }

    //send post request
    fn post(&self, url: &str, payload: &Value) -> Result<String, failure::Error> {
        self.send_request(Method::POST, url, Some(payload))
    }

}