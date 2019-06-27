#![allow(unused_imports, dead_code, non_snake_case, unused_variables)]
#[macro_use]
extern crate actix;
extern crate actix_web;
extern crate dotenv;
extern crate futures;
extern crate failure;
extern crate diesel;
extern crate mysql;
extern crate r2d2;
extern crate r2d2_mysql;
extern crate serde_derive;
extern crate env_logger;
extern crate listenfd;
extern crate num_cpus;
extern crate serde_json;
extern crate jsonwebtoken as jwt;
extern crate csrf_token;
extern crate log;
extern crate lazy_static;
#[macro_use]
extern crate dotenv_codegen;


// Exports Addr.
//use actix::prelude::*;
use actix_web::{
    http::header, middleware, 
    middleware::identity::{CookieIdentityPolicy, IdentityService, Identity},
    middleware::cors::Cors, web, App, error, Error, HttpResponse, HttpServer,
};
use actix_multipart::{Field, Multipart, MultipartError};
use csrf_token::CsrfTokenGenerator;
use chrono::Duration;
use chrono::prelude::{Date, Datelike, Local, Utc};
use actix_files::Files;
use dotenv::dotenv;
use futures::{
    future::{err, ok, result, Either},
    {Future, Stream, IntoFuture},
};
use rand::{
    distributions::Alphanumeric,
    {thread_rng, Rng},
};
use listenfd::ListenFd;
use r2d2_mysql::{mysql::from_row, mysql::params, mysql::from_value};
use serde_json::{value::RawValue, value::Value, json, to_string, Map};
use log::{info, trace, warn, debug};
use std::io::Write;
use std::{env, fs::File, fs::create_dir_all, iter, path::PathBuf};
use std::str::FromStr;
mod db;
mod model;
mod auth;
mod token;
mod errors;
mod client;
mod failures;

use crate::token::{create_token, verify_token, decode_token};
use crate::client::ScClient;

fn sc_tb(req: web::Json<model::Request>, db: web::Data<db::Pool>) 
-> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        let mut conn = db.get().unwrap();
        match db::TrxLogs(&mut conn, &req.into_inner()) {
            Ok(_) => {
                match db::get_tb(&mut conn) {
                    Ok(ok) => Ok(ok),
                    Err(e) => Err(e),
                }
            },
            Err(e) => Err(e),
        }
    })
    .then(|res| match res {
        Ok(tb) => Ok(HttpResponse::Ok().json(json!({
            "message": "success get master tb".to_string(),
            "status": true,
            "data": tb,
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "message":"failed get master tb".to_string(),
            "status":false,
            "data": e.to_string()}))),
    })
}

fn sc_tdb(
    id: web::Path<i32>,
    req: web::Json<model::Request>,
    db: web::Data<db::Pool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        let mut conn = db.get().unwrap();
        match db::TrxLogs(&mut conn, &req.into_inner()) {
            Ok(_) => {
                match db::get_tdb(&mut conn, id.into_inner()) {
                    Ok(ok) => Ok(ok),
                    Err(e) => Err(e),
                }
            },
            Err(e) => Err(e),
        }
    })
    .then(|res| match res {
        Ok(tdb) => Ok(HttpResponse::Ok().json(json!({
            "message": "success get master tdb".to_string(),
            "status": true,
            "data": tdb,
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "message":"failed get master tdb".to_string(),
            "status":false,
            "data": e.to_string()}))),
    })
}

fn sc_td(
    id: web::Path<i32>,
    req: web::Json<model::Request>,
    db: web::Data<db::Pool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        let mut conn = db.get().unwrap();
        match db::TrxLogs(&mut conn, &req.into_inner()) {
            Ok(_) => {
                match db::get_td(&mut conn, id.into_inner()) {
                    Ok(ok) => Ok(ok),
                    Err(e) => Err(e),
                }
            },
            Err(e) => Err(e),
        }
    })
    .then(|res| match res {
        Ok(td) => Ok(HttpResponse::Ok().json(json!({
            "message": "success get master td".to_string(),
            "status": true,
            "data": td,
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "message":"failed get master td".to_string(),
            "status":false,
            "data": e.to_string()}))),
    })
}

fn sc_elc(
    //(tb_id, tdb_id): (web::Path<i32>, web::Path<i32>),
    info: web::Path<(i32, i32)>,
    req: web::Json<model::Request>,
    db: web::Data<db::Pool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        let id = info.into_inner();
        let mut conn = db.get().unwrap();
        match db::TrxLogs(&mut conn, &req.into_inner()) {
            Ok(_) => {
                match db::get_elc(&mut conn, id.0, id.1) {
                    Ok(ok) => Ok(ok),
                    Err(e) => Err(e),
                }
            },
            Err(e) => Err(e),
        }
    })
    .then(|res| match res {
        Ok(elc) => Ok(HttpResponse::Ok().json(json!({
            "message": "success get master elc".to_string(),
            "status": true,
            "data": elc,
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "message":"failed get master elc".to_string(),
            "status":false,
            "data": e.to_string()}))),
    })
}

fn sc_list(
    id: web::Path<i32>,
    req: web::Json<model::Request>,
    db: web::Data<db::Pool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        let mut conn = db.get().unwrap();
        match db::TrxLogs(&mut conn, &req.into_inner()) {
            Ok(_) => {
                match db::get_list(&mut conn, id.into_inner()) {
                    Ok(ok) => Ok(ok),
                    Err(e) => Err(e),
                }
            },
            Err(e) => Err(e),
        }
    })
    .then(|res| match res {
        Ok(sc) => Ok(HttpResponse::Ok().json(json!({
            "message": "success get list sc".to_string(),
            "status": true,
            "data": sc,
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "message":"failed get list sc".to_string(),
            "status":false,
            "data": e.to_string()}))),
    })
}

fn sc_profile(
    id: web::Path<i32>,
    req: web::Json<model::Request>,
    db: web::Data<db::Pool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        let mut conn = db.get().unwrap();
        match db::TrxLogs(&mut conn, &req.into_inner()) {
            Ok(_) => {
                match db::get_profile(&mut conn, id.into_inner()) {
                    Ok(ok) => Ok(ok),
                    Err(e) => Err(e),
                }
            },
            Err(e) => Err(e),
        }
    })
    .then(|res| match res {
        Ok(prof) => Ok(HttpResponse::Ok().json(json!({
            "message": "success get detail profile".to_string(),
            "status": true,
            "data": prof,
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "message": "failed get detail profile".to_string(),
            "status":false,
            "data": e.to_string()}))),
    })
}

pub fn sc_result(
    id:  web::Path<i32>,
    req: web::Json<model::Request>, 
    db: web::Data<db::Pool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        let mut conn = db.get().unwrap();
        println!("request: {:?}", req);
        let simulation: Option<i32> = Some(id.into_inner());
        println!("simulation: {:?}", simulation);
        let data = req.data.get();
        let result = serde_json::from_str::<model::ScResult>(&data).unwrap();
        println!("data: {:?} request: {:?}", data, result);
        
        if let Some(Some(1)) = Some(simulation) {
            match db::TrxLogs(&mut conn, &req.into_inner()) {
                Ok(_) => {
                    match db::TrxResult(&mut conn, &result) {
                        Ok(_) => {
                            match db::getCallback(&mut conn, &result.customer_id, &result.tb_id, &result.tdb_id,
                            &result.td_id, &result.ec_id) {
                                Ok(cb) => Ok(cb),
                                Err(e) => Err(e),
                            }
                        },
                        Err(e) => Err(e),
                    }
                },
                Err(e) => Err(e),
            }       
        }else{
            println!("tb_id: {:?} tdb_id: {:?} td_id: {:?} ec_id: {:?}", result.tb_id, result.tdb_id, result.td_id, result.ec_id);
            match db::getSimulation(&mut conn, &result.customer_id, 
                    &result.tb_id, &result.tdb_id,
                    &result.td_id, &result.ec_id) {
                Ok(sm) => Ok(sm),
                Err(e) => Err(e),
            }
        }
    })
    .then(|res| match res {
        Ok(call) => Ok(HttpResponse::Ok().json(json!({
            "message":"send data result success".to_string(),
            "status": true,
            "data": call,
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "message":"send data result failed".to_string(),
            "status":false,
            "data": e.to_string()}))),
    })
}
/*
pub fn sc_edited(
    id: web::Path<i32>,
    req: web::Json<model::Request>,
    db: web::Data<db::Pool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        let mut conn = db.get().unwrap();
        println!("request: {:?}", req);
        let is_edited: Option<i32> = Some(id.into_inner());
        println!("edited: {:?}", is_edited);
        let data = req.data.get();
        let result = serde_json::from_str::<model::ScDetail>(&data).unwrap();
        println!("data: {:?} request: {:?}", data, result);
        match db::TrxLogs(&mut conn, &req.into_inner()) {
            Ok(_) => {
                match db::TrxDetail(&mut conn, &result,is_edited) {
                    Ok(_) => {
                        match db::getDetail(&mut conn, &result.customer_id) {
                            Ok(ok) => Ok(ok),
                            Err(e) => Err(e),
                        }
                    },
                    Err(e) => Err(e),
                }
            },
            Err(e) => Err(e),
        }  
    })
    .then(|res| match res {
        Ok(edit) => Ok(HttpResponse::Ok().json(json!({
            "message": "send data customer success".to_string(),
            "status": true,
            "data": edit,
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "message": "send data customer failed".to_string(),
            "status":false,
            "data": e.to_string()}))),
    })
}
*/

pub fn sc_edited(
    req: web::Json<model::Request>,
    db: web::Data<db::Pool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        let mut conn = db.get().unwrap();
        println!("request: {:?}", req);
        let data = req.data.get();
        let result = serde_json::from_str::<model::ScDetail>(&data).unwrap();
        println!("data: {:?} request: {:?}", data, result);
        match db::TrxLogs(&mut conn, &req.into_inner()) {
            Ok(_) => {
                match db::TrxDetail(&mut conn, &result) {
                    Ok(_) => {
                        match db::getDetail(&mut conn, &result.customer_id) {
                            Ok(ok) => Ok(ok),
                            Err(e) => Err(e),
                        }
                    },
                    Err(e) => Err(e),
                }
            },
            Err(e) => Err(e),
        }  
    })
    .then(|res| match res {
        Ok(edit) => Ok(HttpResponse::Ok().json(json!({
            "message": "send data customer success".to_string(),
            "status": true,
            "data": edit,
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "message": "send data customer failed".to_string(),
            "status":false,
            "data": e.to_string()}))),
    })
}

pub fn sc_reason(
    req: web::Json<model::Request>,
    db: web::Data<db::Pool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        let mut conn = db.get().unwrap();
        println!("request: {:?}", req);
        let data = req.data.get();
        let result = serde_json::from_str::<model::ScReason>(&data).unwrap();
        println!("data: {:?} request: {:?}", data, result);
        match db::TrxLogs(&mut conn, &req.into_inner()) {
            Ok(_) => {
                match db::TrxReason(&mut conn, &result) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e),
                }
            },
            Err(e) => Err(e),
        }  
    })
    .then(|res| match res {
        Ok(edit) => Ok(HttpResponse::Ok().json(json!({
            "message": "send data customer success".to_string(),
            "status": true,
            "data": "save reason success",
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "message": "send data customer failed".to_string(),
            "status":false,
            "data": e.to_string()}))),
    })
}

pub fn sc_calculate(
    req: web::Json<model::Request>, 
    db: web::Data<db::Pool>
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        let mut conn = db.get().unwrap();
        let data = req.data.get();
        let result = serde_json::from_str::<model::ScCallback>(&data).unwrap();
        info!("data: {} id: {}", data, result.sc_id);
        let id: i32 = FromStr::from_str(&result.sc_id.to_string()).unwrap();
        let customer_id: i64 = FromStr::from_str(&result.customer_id.to_string()).unwrap();
        println!("id : {:?}, {:?}", id, customer_id);
        //match db::TrxLogs(&mut conn, &req.into_inner()) {
        //   Ok(_) => {
                match ScClient::Calculate(&mut ScClient::default(), &mut conn, &customer_id, &id) {
                    Ok(ok) => Ok(ok),
                    Err(e) => Err(e),
                }
        //    },
        //    Err(e) => Err(e),
        //}
    })
    .then(|res| match res {
        Ok(calc) =>  Ok(HttpResponse::Ok().json(json!({
                "message":"calculate success".to_string(),
                "status": true,
                "data": calc,
            }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
                "message":"calculate failed".to_string(),
                "status":false,
                "data": e.to_string()}))),
    })
}

pub fn sc_login(
    req: web::Json<model::Request>, 
    //id: Identity, 
    db: web::Data<db::Pool>
    //generator: web::Data<CsrfTokenGenerator>
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        let mut conn = db.get().unwrap();
        let data = req.data.get();
        let u = serde_json::from_str::<model::User>(&data).unwrap();
        println!("data: {} id: {}", data, u.username);

        match db::get_login(&mut conn, &u.username, &u.password) {
            Ok(Some(user)) => {
                println!("user : {:#?}", user);
                let token = create_token(user.clone());
                //id.remember(token);
                let id: i32 = FromStr::from_str(&user.employee_id.to_string()).unwrap();
                info!("token: {}", token);
                match db::TrxToken(&mut conn, &id, &token) {
                    Ok(_) => {
                        match db::TrxLogs(&mut conn, &req.into_inner()) {
                            Ok(_) => {
                                Ok(vec![model::User{employee_id: user.employee_id, 
                                                username: user.username, 
                                                password: user.password, 
                                                nik: user.nik,
                                                sfl_code: user.sfl_code,
                                                employee_name: user.employee_name,
                                                email: user.email,
                                                user_type: user.user_type,
                                                role_name: user.role_name,
                                                brand_code: user.brand_code, 
                                                branch_name: user.branch_name, 
                                                region_name: user.region_name,
                                                application_id: user.application_id,
                                                avatar: user.avatar,
                                                token: Some(token) }])                   
                            },
                            Err(e) => Err(e.to_string()),
                        }
                    },
                    Err(e) => Err(e.to_string()),
                }
            },
            Ok(None) => Ok(serde_json::from_value(json!(vec![{}])).unwrap()),
            Err(e) => Err(e.to_string()),
        }
    })
    .then(|res| match res {
        Ok(log) =>  Ok(HttpResponse::Ok()
                //.header("X-CSRF-TOKEN", hex::encode(generator.generate()))
                .json(json!({
                "message": "login success".to_string(),
                "status": true,
                "data":log,
            }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "message":"login failed".to_string(),
            "status":false,
            "data": e.to_string()}))),
    })
}

pub fn sc_logout(
    req: web::Json<model::Request>, 
    db: web::Data<db::Pool>
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        let mut conn = db.get().unwrap();
        let id: i32 = FromStr::from_str(&req.user.employee_id.to_string()).unwrap();
        println!("id : {:?}", id);
        match db::TrxLogout(&mut conn, &id) {
            Ok(_) => {
                match db::TrxLogs(&mut conn, &req.into_inner()) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e),
                }
            },
            Err(e) => Err(e),
        }
    })
    .then(|res| match res {
        Ok(_) =>  Ok(HttpResponse::Ok().json(json!({
            "message":"logout success".to_string(),
            "status": true,
            "data": "logout success".to_string(),
            }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "message":"logout failed".to_string(),
            "status":false,
            "data": e.to_string()}))),
    })
}

// Async IO file storage handler
pub fn save_file(id: &String, field: Field) -> impl Future<Item = model::FileUpload, Error = Error> {
   
    let raw = match field.content_disposition() {
        Some(e) => e.parameters,
        None => {
            return Either::A(err(error::ErrorInternalServerError(
                "no valid parameters",
            )))
        }
    };
    println!("raw : {:?}", raw);
  
    let name = match field.content_disposition().unwrap().get_name() {
        Some(key) => key.to_string(),
        None => return Either::A(err(error::ErrorInternalServerError("Couldn't read the key.")))
    };
    println!("content-dispostition key : {:?}", name);

    let file_name = match field.content_disposition().unwrap().get_filename() {
        Some(filename) => filename.replace(' ', "_").to_string(),
        None => return Either::A(err(error::ErrorInternalServerError("Couldn't read the filename.")))
    };
    println!("content-dispostition file : {:?}", file_name);

    let days: Date<Local> = Local::today();
    let years: String = format!("{}", days.year());
    let month: String = format!("{}", days.month());
    let upload_file = format!("{}", &id.clone());
    let files = format!("{}", &file_name.clone());

    let mut path = PathBuf::new();
    path.push(
        env::var("STORAGE").unwrap_or(
            env::current_dir()
                .expect("Failed to get current directory!")
                .to_str()
                .unwrap()
                .into(),
        ),
    );
    path.push(&years);
    path.push(&month);
    path.push(&upload_file);
    println!("path : {:?}", path);

    match create_dir_all(&path) {
        Ok(_) => {},
        Err(e) => return Either::A(err(error::ErrorInternalServerError(e))),
    }
   
    let mut upload_file_path = path.clone();
    println!("dir file upload : {:?}", upload_file_path);
    upload_file_path.push(&files);

    let file_path = format!("{}{}", "http://192.168.177.187/sfafile".to_string(), &upload_file_path.display());

    let wo_id: i64 = FromStr::from_str(&id.to_string()).unwrap();
    
    let file = match File::create(&upload_file_path) {
        Ok(file) => file,
        Err(e) => return Either::A(err(error::ErrorInternalServerError(e))),
    };
    println!("file : {:?}", file);

    Either::B(
        field
            .fold((file, 0i64), move |(mut file, mut acc), bytes| {
                web::block(move || {
                    file.write_all(bytes.as_ref()).map_err(|e| {
                        println!("file.write_all failed: {:?}", e);
                        MultipartError::Payload(error::PayloadError::Io(e))
                    })?;
                    acc += bytes.len() as i64;
                    Ok((file, acc))
                })
                .map_err(|e: error::BlockingError<MultipartError>| match e {
                    error::BlockingError::Error(e) => e,
                    error::BlockingError::Canceled => MultipartError::Incomplete,
                })
            })
            .map(move |(_, acc)|model::FileUpload
                    {wo_id: wo_id, file_name: file_name, 
                    file_size: acc, file_path: file_path.replace("\\","/").to_string(), file_type: name}
            )
            .map_err(|e| {
                println!("save_file failed, {:?}", e);
                error::ErrorInternalServerError(e)
            }),
    )
}

/// Handle multi-part stream forms
pub fn sc_upload(id: web::Path<String>, multipart: Multipart, db: web::Data<db::Pool>) -> impl Future<Item = HttpResponse, Error = Error> {   
    let mut conn = db.get().unwrap();  
    multipart
        .map_err(error::ErrorInternalServerError)
        .map(move |field| save_file(&id.clone(),field).into_stream())
        .flatten()
        .collect()
        .map(move |file| {
            for(i, files) in file.iter().enumerate() {
                println!("{}: key={} value={}", i, files.wo_id, files.file_name);
                let x = model::FileUpload{wo_id: files.wo_id, file_name: files.file_name.to_string(), file_size: files.file_size,
                    file_path: files.file_path.to_string(), file_type: files.file_type.to_string()};
                match db::TrxFile(&mut conn, &x.clone()) {
                    Ok(_) => {},
                    Err(_) =>{},
                };
            };
            
            HttpResponse::Ok().json(json!({
            "message":"upload success".to_string(),
            "status": true,
            "data": file}))
        })
        .map_err(|e| {
            println!("failed: {}", e);
            e
        })
}

// pub struct AppState {
//     pub db: Addr<db::DbExecutor>
// }

fn main() -> std::io::Result<()> {
    // Grab the env vars.
    dotenv().ok();
    env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    //let sys = actix::System::new("scorecard");
    //let cpus = num_cpus::get();
    //let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database_url = dotenv!("DATABASE_URL");

    let pool = db::init_pool(&database_url);

    //let csrf_token_header = header::HeaderName::from_lowercase(b"x-csrf-token").unwrap();

    // let addr: Addr<DbExecutor> = SyncArbiter::start(4, move|| db::DbExecutor(pool.clone()));
    HttpServer::new(move || {
        // App::with_state(AppState{db: addr.clone()})
        App::new()
            .data(pool.clone())
            /*.data(
                CsrfTokenGenerator::new(
                    dotenv!("CSRF_TOKEN_KEY").as_bytes().to_vec(),
                    Duration::hours(1)
                )
            )
            .wrap(
                IdentityService::new(
                    CookieIdentityPolicy::new(dotenv!("SECRET_KEY").as_bytes())
                        .domain(dotenv!("MYSTOREDOMAIN"))
                        .name("scorecard")
                        .path("/")
                        .max_age(Duration::days(1).num_seconds())
                        .secure(dotenv!("COOKIE_SECURE").parse().unwrap())
                )
            )
            .wrap(
                Cors::new()
                    .allowed_origin("*")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::ORIGIN, header::CONTENT_TYPE, header::AUTHORIZATION, header::ACCEPT,  csrf_token_header.clone()])
                    .expose_headers(vec![csrf_token_header.clone()])
                    .max_age(3600),
            )*/
            .wrap(middleware::DefaultHeaders::new().header("SC-Version", "1.0.0"))
            //.wrap(middleware::Compress::default())
            .wrap(auth::CheckAuth)
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/api/v1.0.0")
                    .service(web::resource("").to(|| "New-ScoreCard Version 1.0.0."))
                    .service(web::resource("/sc-tb").route(web::post().to_async(sc_tb)))
                    .service(web::resource("/sc-tdb/{id}").route(web::post().to_async(sc_tdb)))
                    .service(web::resource("/sc-td/{id}").route(web::post().to_async(sc_td)))
                    .service(web::resource("/sc-list/{id}").route(web::post().to_async(sc_list)))
                    .service(
                        web::resource("/sc-profile/{id}").route(web::post().to_async(sc_profile)),
                    )
                    .service(
                        web::resource("/sc-elc/{tb_id}/{tdb_id}")
                            .route(web::post().to_async(sc_elc)),
                    )
                    /*.service(
                        web::resource("/sc-edited/{id}")
                            .route(web::post().to_async(sc_edited)),
                    )*/
                    .service(
                        web::resource("/sc-edited")
                            .route(web::post().to_async(sc_edited)),
                    )
                    .service(
                        web::resource("/sc-result/{id}")
                            .route(web::post().to_async(sc_result)),
                    )
                    .service(
                        web::resource("/sc-reason")
                            .route(web::post().to_async(sc_reason)),
                    )
                    .service(
                        web::resource("/sc-calculate")
                            .route(web::post().to_async(sc_calculate)),
                    )
                    .service(
                        web::resource("/sc-upload/{id}")
                            .route(web::post().to_async(sc_upload)),
                    )
                    .service(
                        web::resource("/sc-login")
                            .route(web::post().to_async(sc_login)),
                    )
                    .service(
                        web::resource("/sc-logout")
                            .route(web::post().to_async(sc_logout)),
                    )

            )
            .default_service(web::route().to(|| HttpResponse::NotFound()))
    })
    .bind("127.0.0.1:8080")?
    .run()

    //let _ = sys.run();
}
