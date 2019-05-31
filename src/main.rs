#![allow(unused_imports, dead_code, non_snake_case)]
extern crate actix;
extern crate dotenv;
extern crate futures;
extern crate r2d2_mysql;
#[macro_use]
extern crate serde_derive;
extern crate env_logger;
extern crate listenfd;
extern crate num_cpus;
extern crate serde_json;

// Exports Addr.
//use actix::prelude::*;
use actix_web::{
    http::header, middleware, middleware::cors::Cors, web, App, error, Error, HttpResponse, HttpServer,
};
use actix_files::Files;
use actix_multipart::{Field, Multipart, MultipartError};
use dotenv::dotenv;
use futures::{
    future::{err, Either},
    {Future, Stream},
};
use rand::{
    distributions::Alphanumeric,
    {thread_rng, Rng},
};
use listenfd::ListenFd;
use r2d2_mysql::{mysql::from_row, mysql::params};
use serde_json::json;

use std::io::Write;
use std::{env, fs::File, iter, path::PathBuf};


mod db;
mod model;

fn sc_tb(db: web::Data<db::Pool>) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        let mut conn = db.get().unwrap();
        conn.prep_exec("SELECT TB_ID, DESCR FROM SC_TB", ())
            .map(|result| {
                result
                    .map(|x| x.unwrap())
                    .map(|row| {
                        let (tb_id, descr) = from_row(row);
                        model::ScTB { tb_id, descr }
                    })
                    .collect::<Vec<model::ScTB>>()
            })
    })
    .then(|res| match res {
        Ok(tb) => Ok(HttpResponse::Ok().json(json!({
            "code": 200,
            "status": true,
            "data": tb,
        }))),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
}

fn sc_tdb(
    id: web::Path<i32>,
    db: web::Data<db::Pool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        let mut conn = db.get().unwrap();
        conn.prep_exec(
            "SELECT TDB_ID, DESCR FROM SC_TDB where TB_ID = :id",
            params! {"id" => id.into_inner()},
        )
        .map(|result| {
            result
                .map(|x| x.unwrap())
                .map(|row| {
                    let (tdb_id, descr) = from_row(row);
                    model::ScTDB { tdb_id, descr }
                })
                .collect::<Vec<model::ScTDB>>()
        })
    })
    .then(|res| match res {
        Ok(tdb) => Ok(HttpResponse::Ok().json(json!({
            "code": 200,
            "status": true,
            "data": tdb,
        }))),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
}

fn sc_td(
    id: web::Path<i32>,
    db: web::Data<db::Pool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        let mut conn = db.get().unwrap();
        conn.prep_exec(
            "SELECT TD_ID, DESCR FROM SC_TD where TB_ID = :id",
            params! {"id" => id.into_inner()},
        )
        .map(|result| {
            result
                .map(|x| x.unwrap())
                .map(|row| {
                    let (td_id, descr) = from_row(row);
                    model::ScTD { td_id, descr }
                })
                .collect::<Vec<model::ScTD>>()
        })
    })
    .then(|res| match res {
        Ok(td) => Ok(HttpResponse::Ok().json(json!({
            "code": 200,
            "status": true,
            "data": td,
        }))),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
}

fn sc_elc(
    //(tb_id, tdb_id): (web::Path<i32>, web::Path<i32>),
    info: web::Path<(i32, i32)>,
    db: web::Data<db::Pool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        let mut conn = db.get().unwrap();
        let id = info.into_inner();
        conn.prep_exec(
            "SELECT EC_ID, DESCR FROM SC_ELC where TB_ID = :tb_id and TDB_ID = :tdb_id",
            params! {"tb_id" => id.0, "tdb_id" => id.1},
        )
        .map(|result| {
            result
                .map(|x| x.unwrap())
                .map(|row| {
                    let (ec_id, descr) = from_row(row);
                    model::ScELC { ec_id, descr }
                })
                .collect::<Vec<model::ScELC>>()
        })
    })
    .then(|res| match res {
        Ok(elc) => Ok(HttpResponse::Ok().json(json!({
            "code": 200,
            "status": true,
            "data": elc,
        }))),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
}

fn sc_list(
    id: web::Path<i32>,
    db: web::Data<db::Pool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        let mut conn = db.get().unwrap();
        conn.prep_exec(
            "SELECT WO_ID, CUSTOMER_ID, PROSPECT_NBR, ASSIGN_TO, SERVICES_ID, SERVICES_DESCR, SERVICES_CATEGORY, DESCR, SCHEDULE_DATE, REGION, LATITUDE, LONGITUDE, CREATED_DATE FROM SC_WORKORDER WHERE ASSIGN_TO = :id ORDER BY CREATED_DATE DESC",
            params! {"id" => id.into_inner()},
        )
        .map(|result| {
            result
                .map(|x| x.unwrap())
                .map(|mut row| {
                    /*let (
                        wo_id,
                        customer_id,
                        prospect_nbr,
                        employee_id,
                        service_id,
                        service_descr,
                        category,
                        descr,
                        schedule_date,
                        region,
                        latitude,
                        longitude,
                        created_date,
                    ) = from_row(row);*/
                    model::ScWorkorder {
                        wo_id: row.take("WO_ID").unwrap(),
                        customer_id: row.take("CUSTOMER_ID").unwrap(),
                        prospect_nbr: row.take("PROSPECT_NBR").unwrap(),
                        employee_id: row.take("ASSIGN_TO").unwrap(),
                        service_id: row.take("SERVICES_ID").unwrap(),
                        service_descr: row.take("SERVICES_DESCR").unwrap(),
                        category: row.take("SERVICES_CATEGORY").unwrap(),
                        descr: row.take("DESCR").unwrap(),
                        schedule_date: row.take("SCHEDULE_DATE").unwrap(),
                        region: row.take("REGION").unwrap(),
                        latitude: row.take("LATITUDE").unwrap(),
                        longitude: row.take("LONGITUDE").unwrap(),
                        created_date: row.take("CREATED_DATE").unwrap(),
                    }
                })
                .collect::<Vec<model::ScWorkorder>>()
        })
    })
    .then(|res| match res {
        Ok(sc) => Ok(HttpResponse::Ok().json(json!({
            "code": 200,
            "status": true,
            "data": sc,
        }))),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
}

fn sc_profile(
    id: web::Path<i32>,
    db: web::Data<db::Pool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        let mut conn = db.get().unwrap();
        conn.prep_exec(
            "SELECT CUSTOMER_ID, CUSTOMER_NAME, ADDRESS, MOBILE_PHONE, HOME_PHONE, EXTRA_PHONE, WHATSAPP, GENDER, EMAIL, FOTO FROM SC_CUSTOMER WHERE CUSTOMER_ID = :id",
            params! {"id" => id.into_inner()},
        )
        .map(|result| {
            result
                .map(|x| x.unwrap())
                .map(|mut row| {
                    model::ScCustomer {
                        customer_id : row.take("CUSTOMER_ID").unwrap(),
                        customer_name: row.take("CUSTOMER_NAME").unwrap(),
                        address: row.take("ADDRESS").unwrap(),
                        mobile_phone: row.take("MOBILE_PHONE").unwrap(),
                        home_phone: row.take("HOME_PHONE").unwrap(),
                        extra_phone: row.take("EXTRA_PHONE").unwrap(),
                        whatsapp: row.take("WHATSAPP").unwrap(),
                        gender: row.take("GENDER").unwrap(),
                        email: row.take("EMAIL").unwrap(),
                        foto: row.take("FOTO").unwrap(),
                    }
                })
                .collect::<Vec<model::ScCustomer>>()
        })
    })
    .then(|res| match res {
        Ok(prof) => Ok(HttpResponse::Ok().json(json!({
            "code": 200,
            "status": true,
            "data": prof,
        }))),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
}

pub fn sc_result(
    id: web::Path<i32>,
    req: web::Json<model::ScResult>,
    db: web::Data<db::Pool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        let mut conn = db.get().unwrap();
        println!("request: {:?}", req);
        let simulation = id.into_inner();
        println!("simulation: {}", simulation);
        let _ = conn
                .start_transaction(false, None, None)
                .and_then(|mut t| {
                    t.prep_exec("INSERT INTO SC_RESULT_NEW
                                       (CUSTOMER_ID, TB_ID, TDB_ID, TD_ID, EC_ID, EMPLOYEE_ID, LATITUDE, LONGITUDE)
                                   VALUES
                                       (:customer_id, :tb_id, :tdb_id, :td_id, :ec_id, :employee_id, :latitude, :longitude)",
                                    params!{
                                        "customer_id" => req.customer_id.clone(),
                                        "tb_id" => req.tb_id.clone(),
                                        "tdb_id" => req.tdb_id.clone(),
                                        "td_id" => req.td_id.clone(),
                                        "ec_id" => req.ec_id.clone(),
                                        "employee_id" => req.employee_id.clone(),
                                        "latitude" => &req.latitude.clone(),
                                        "longitude" => &req.longitude.clone(),
                                    })
                        .unwrap();
                    t.commit().is_ok();
                    Ok(())
                })
        .unwrap();
       
        match simulation {
            o =>  if o == 1 {
                conn.prep_exec("
                SELECT A.SC_ID, A.CUSTOMER_ID, C.SCORE, C.SEC, E.PRODUCT_ID, E.PRODUCT_NAME, E.PROMO_ID, E.PROMO_CODE, E.PROMO_DESCR FROM SC_RESULT_NEW A
                JOIN SC_RESULT_SCORE B ON A.SC_ID = B.SC_ID
                JOIN SC_SCORE C ON B.SEC_ID = C.SEC_ID
                JOIN SC_CALLBACK D ON B.SCORE_ID = D.SCORE_ID
                JOIN SC_MAPPING_PRODUCT E ON D.MAP_ID = E.MAP_ID WHERE A.CUSTOMER_ID = :id",  params!{"id" => req.customer_id.clone()})
                .map(|result| {
                    result
                        .map(|x| x.unwrap())
                        .map(|mut row| {
                            model::ScCallback {
                                sc_id: row.take("SC_ID").unwrap(),
                                customer_id: row.take("CUSTOMER_ID").unwrap(),
                                score: row.take("SCORE").unwrap(),
                                sec: row.take("SEC").unwrap(),
                                product_id: row.take("PRODUCT_ID").unwrap(),
                                product_name: row.take("PRODUCT_NAME").unwrap(),
                                promo_id: row.take("PROMO_ID").unwrap(),
                                promo_code: row.take("PROMO_CODE").unwrap(),
                                promo_descr: row.take("PROMO_DESCR").unwrap(),
                            }
                        })
                        .collect::<Vec<model::ScCallback>>()
                })
            }else{
                conn.prep_exec("
                SELECT A.SC_ID, A.CUSTOMER_ID, C.SCORE, C.SEC, E.PRODUCT_ID, E.PRODUCT_NAME, E.PROMO_ID, E.PROMO_CODE, E.PROMO_DESCR FROM SC_RESULT_NEW A
                JOIN SC_RESULT_SCORE B ON A.SC_ID = B.SC_ID
                JOIN SC_SCORE C ON B.SEC_ID = C.SEC_ID
                JOIN SC_CALLBACK D ON B.SCORE_ID = D.SCORE_ID
                JOIN SC_MAPPING_PRODUCT E ON D.MAP_ID = E.MAP_ID WHERE A.CUSTOMER_ID = :id",  params!{"id" => req.customer_id.clone()})
                .map(|result| {
                    result
                        .map(|x| x.unwrap())
                        .map(|mut row| {
                            model::ScCallback {
                                sc_id: row.take("SC_ID").unwrap(),
                                customer_id: row.take("CUSTOMER_ID").unwrap(),
                                score: row.take("SCORE").unwrap(),
                                sec: row.take("SEC").unwrap(),
                                product_id: row.take("PRODUCT_ID").unwrap(),
                                product_name: row.take("PRODUCT_NAME").unwrap(),
                                promo_id: row.take("PROMO_ID").unwrap(),
                                promo_code: row.take("PROMO_CODE").unwrap(),
                                promo_descr: row.take("PROMO_DESCR").unwrap(),
                            }
                        })
                        .collect::<Vec<model::ScCallback>>()
                })
            },
        }
    })
    .then(|res| match res {
        Ok(call) => Ok(HttpResponse::Ok().json(json!({
            "code": 200,
            "status": true,
            "data": call}))),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
}

pub fn sc_edited(
    id: web::Path<i32>,
    req: web::Json<model::ScCustomer>,
    db: web::Data<db::Pool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        let mut conn = db.get().unwrap();
        println!("request: {:?}", req);
        let is_edited = id.into_inner();
        println!("simulation: {}", is_edited);
        match is_edited {
            x => if x == 1 { 
                    let _ = conn
                        .start_transaction(false, None, None)
                        .and_then(|mut t| {
                            t.prep_exec("INSERT INTO SC_RESULT_DETAIL
                                                (CUSTOMER_ID, CUSTOMER_NAME, ADDRESS, MOBILE_PHONE, HOME_PHONE, EXTRA_PHONE, WHATSAPP, GENDER, EMAIL)
                                            VALUES
                                                (:customer_id, :customer_name, :address, :mobile_phone, :home_phone, :extra_phone, :whatsapp, :gender, :email)",
                                            params!{
                                                "customer_id" => &req.customer_id.clone(),
                                                "customer_name" => &req.customer_name.clone(),
                                                "address" => &req.address.clone(),
                                                "mobile_phone" => &req.mobile_phone.clone(),
                                                "home_phone" => &req.home_phone.clone(),
                                                "extra_phone" => &req.extra_phone.clone(),
                                                "whatsapp" => &req.whatsapp.clone(),
                                                "gender" => &req.gender.clone(),
                                                "email" => &req.email.clone(),
                                            })
                                .unwrap();
                            t.commit().is_ok();
                            Ok(())
                        })
                    .unwrap();
                }else{

                    let _ = conn
                        .start_transaction(false, None, None)
                        .and_then(|mut t| {
                            t.prep_exec("UPDATE SC_RESULT_DETAIL SET
                                                 CUSTOMER_NAME = :customer_name,
                                                ADDRESS = :address,
                                                MOBILE_PHONE = :mobile_phone,
                                                HOME_PHONE = :home_phone,
                                                EXTRA_PHONE = :extra_phone,
                                                WHATSAPP = :whatsapp,
                                                GENDER = :gender,  
                                                EMAIL = :email
                                                WHERE CUSTOMER_ID =:customer_id",
                                            params!{
                                                "customer_name" => &req.customer_name.clone(),
                                                "address" => &req.address.clone(),
                                                "mobile_phone" => &req.mobile_phone.clone(),
                                                "home_phone" => &req.home_phone.clone(),
                                                "extra_phone" => &req.extra_phone.clone(),
                                                "whatsapp" => &req.whatsapp.clone(),
                                                "gender" => &req.gender.clone(),
                                                "email" => &req.email.clone(),
                                                "customer_id" => &req.customer_id.clone(),
                                            })
                                .unwrap();
                            t.commit().is_ok();
                            Ok(())
                        })
                    .unwrap();
                }
        }
        conn.prep_exec(
            "SELECT CUSTOMER_ID, CUSTOMER_NAME, ADDRESS, MOBILE_PHONE, HOME_PHONE, EXTRA_PHONE, WHATSAPP, GENDER, EMAIL FROM SC_RESULT_DETAIL WHERE CUSTOMER_ID = :id",
            params! {"id" => &req.customer_id.clone()},
        )
        .map(|result| {
            result
                .map(|x| x.unwrap())
                .map(|mut row| {
                    model::ScDetail {
                        customer_id : row.take("CUSTOMER_ID").unwrap(),
                        customer_name: row.take("CUSTOMER_NAME").unwrap(),
                        address: row.take("ADDRESS").unwrap(),
                        mobile_phone: row.take("MOBILE_PHONE").unwrap(),
                        home_phone: row.take("HOME_PHONE").unwrap(),
                        extra_phone: row.take("EXTRA_PHONE").unwrap(),
                        whatsapp: row.take("WHATSAPP").unwrap(),
                        gender: row.take("GENDER").unwrap(),
                        email: row.take("EMAIL").unwrap(),
                    }
                })
                .collect::<Vec<model::ScDetail>>()
        })
    })
    .then(|res| match res {
        Ok(edit) => Ok(HttpResponse::Ok().json(json!({
            "code": 200,
            "status": true,
            "data": edit}))),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
}

/// Small utility function that generates random filenames and paths
fn get_filename() -> (String, String) {
    let mut rng = thread_rng();
    let file_name: String = iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .take(16)
        .collect::<String>()
        .as_str()
        .into();

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
    path.push(file_name.clone());
    (file_name, path.as_os_str().to_str().unwrap().to_owned())
}

/// Async IO file storage handler
pub fn save_file(field: Field) -> impl Future<Item = String, Error = Error> {
    let (name, path) = get_filename();

    let file = match File::create(path) {
        Ok(file) => file,
        Err(e) => return Either::A(err(error::ErrorInternalServerError(e))),
    };
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
            .map(move |(_, _)| name)
            .map_err(|e| {
                println!("save_file failed, {:?}", e);
                error::ErrorInternalServerError(e)
            }),
    )
}

/// Handle multi-part stream forms
pub fn sc_upload(multipart: Multipart) -> impl Future<Item = HttpResponse, Error = Error> {
    multipart
        .map_err(error::ErrorInternalServerError)
        .map(|field| save_file(field).into_stream())
        .flatten()
        .collect()
        .map(|file| HttpResponse::Ok().json(json!({
            "code": 200,
            "status": true,
            "data": file})))
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
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = db::init_pool(&database_url);

    // let addr: Addr<DbExecutor> = SyncArbiter::start(4, move|| db::DbExecutor(pool.clone()));
    HttpServer::new(move || {
        // App::with_state(AppState{db: addr.clone()})
        App::new()
            .data(pool.clone())
            .wrap(
                Cors::new()
                    .allowed_origin("*")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::ORIGIN, header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .wrap(middleware::DefaultHeaders::new().header("SC-Version", "1.0.0"))
            //.wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/api/v1.0.0")
                    .service(web::resource("").to(|| "New-ScoreCard Version 1.0.0."))
                    .service(web::resource("/sc-tb").route(web::get().to_async(sc_tb)))
                    .service(web::resource("/sc-tdb/{id}").route(web::get().to_async(sc_tdb)))
                    .service(web::resource("/sc-td/{id}").route(web::get().to_async(sc_td)))
                    .service(web::resource("/sc-list/{id}").route(web::get().to_async(sc_list)))
                    .service(
                        web::resource("/sc-profile/{id}").route(web::get().to_async(sc_profile)),
                    )
                    .service(
                        web::resource("/sc-elc/{tb_id}/{tdb_id}")
                            .route(web::get().to_async(sc_elc)),
                    )
                    .service(
                        web::resource("/sc-edited/{id}")
                            .route(web::post().to_async(sc_edited)),
                    )
                    .service(
                        web::resource("/sc-result/{id}")
                            .route(web::post().to_async(sc_result)),
                    )
                    .service(
                        web::resource("/sc-upload")
                            .route(web::post().to_async(sc_upload)),
                    )

            )
            .default_service(web::route().to(|| HttpResponse::NotFound()))
    })
    .bind("127.0.0.1:8080")?
    .run()

    //let _ = sys.run();
}
