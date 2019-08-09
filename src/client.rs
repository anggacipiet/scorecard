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

use crate::db::{getCalculate, getCalculate_Simulation, get_profile, push_ppg, Conn, ConnSFA, TrxCalculate, TrxUpdTrex};
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
            status => ApiError::Other(status.as_u16()),
        }
    }
}

impl ScClient {

    pub fn Calc_Simulation(
        &mut self,
        conn: &mut Conn,
        sc_id: &i32,
        cb_id: &i32,
        prod_id: &i32,
    ) -> Result<Vec<ScCalculate>, Error> {
        if let Some(prod_id) = Some(338) {
            match getCalculate_Simulation(conn, &sc_id, &cb_id) {
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
                            Ok(vec![resp])
                        }
                        Err(e) => Err(Error::from(CustomError::new(format!("{}", e).as_str()))),
                    }
                }
                Ok(None) => Ok(vec![ScCalculate::default()]), 
                Err(e) => Err(Error::from(CustomError::new(format!("{}", e).as_str()))),
            }
        }else{
            match getCalculate_Simulation(conn, &sc_id, &cb_id) {
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
                            Ok(vec![resp])
                        }
                        Err(e) => Err(Error::from(CustomError::new(format!("{}", e).as_str()))),
                    }
                }
                Ok(None) => Ok(vec![ScCalculate::default()]), 
                Err(e) => Err(Error::from(CustomError::new(format!("{}", e).as_str()))),
            }
        }
    }

    pub fn Calculate(
        &mut self,
        sfa: &mut ConnSFA,
        conn: &mut Conn,
        sc_id: &i32,
        cb_id: &i32,
    ) -> Result<Vec<ScCalculate>, Error> {

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
                                                // bugfix point 4 update product ke trex 
                                                match TrxUpdTrex(sfa, &oke.customer_id, &resp) {
                                                    Ok(_) => Ok(vec![resp]),
                                                    Err(e) => Err(Error::from(CustomError::new(
                                                        format!("{}", e).as_str(),
                                                    ))),
                                                }
                                                //Ok(vec![resp])
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
            // bugfix point 1 result reject tidak setuju struct default resp calculation
            Ok(None) => Ok(vec![ScCalculate::default()]), 
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