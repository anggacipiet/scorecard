
use crate::model::{
    Request, ScCallback, ScCustomer, ScDetail, ScELC, ScResult, ScTB, ScTD, ScTDB, ScWorkorder,
    Token, User,
};
use chrono::Local;
use log::{debug, info, trace, warn};
use mysql::{from_row, from_row_opt, from_value, params, Error};
use r2d2_mysql::mysql::{Opts, OptsBuilder};
use r2d2_mysql::pool::MysqlConnectionManager;
use serde_json::from_str;


pub type Conn = r2d2::PooledConnection<MysqlConnectionManager>;
pub type Pool = r2d2::Pool<MysqlConnectionManager>;

pub fn init_pool(db_url: &str) -> Pool {
    let opts = Opts::from_url(db_url).unwrap();
    let builder = OptsBuilder::from_opts(opts);
    let manager = MysqlConnectionManager::new(builder);
    r2d2::Pool::builder()
        .max_size(15)
        .min_idle(Some(0))
        .build(manager)
        .unwrap()
}

pub fn TrxLogs(conn: &mut Conn, req: &Request) -> Result<(), Error> {
    let _ = conn
                .start_transaction(false, None, None)
                .and_then(|mut t| {
                t.prep_exec("INSERT INTO SC_APP_ACCESS_LOG
                                        (EMPLOYEE_ID, NIK, USER_LOGIN, EMPLOYEE_NAME, APPLICATION_ID, 
                                        APPLICATION_NAME, VERSION_CODE, VERSION_NAME, OS,
                                        DEVICES, IMEI, IP_ADDRESS, DB_VERSION, DB_NAME, LATLNG, TIME_LNG,
                                        DATA, PROCESS)
                                VALUES
                                        (:employee_id, :nik, :user_login, :employee_name, :application_id,
                                        :application_name, :version_code, :version_name, :os,
                                        :devices, :imei, :ip_address, :db_ver, :db_name, :latlng, :timelng,
                                        :data, :process)",
                                params!{
                                        "employee_id" => &req.process.employee_id.clone(),
                                        "nik" => &req.process.nik.clone(),
                                        "user_login" => &req.process.user_login.clone(),
                                        "employee_name" => &req.process.employee_name.clone(),
                                        "application_id" => &req.application.application_id.clone(),
                                        "application_name" => &req.application.application_name.clone(),
                                        "version_code" => &req.application.version_code.clone(),
                                        "version_name" => &req.application.version_name.clone(),
                                        "os" => &req.application.os.clone(),
                                        "devices" => &req.application.device_name.clone(),
                                        "imei" => &req.application.imei.clone(),
                                        "ip_address" => &req.application.ip_address.clone(),
                                        "db_ver" => &req.application.database_version.clone(),
                                        "db_name" => &req.application.database_name.clone(),
                                        "latlng" => &req.process.latlng.clone(),
                                        "timelng" => &req.process.time_latlng.clone(),
                                        "data" =>  &req.data.get(),
                                        "process" => &req.process.process_name.clone(),
                                })
                        .unwrap();
                        t.commit().is_ok();
                Ok(())
                })
        .unwrap();
    Ok(())
}

pub fn get_login(conn: &mut Conn, username: &str, password: &str) -> Result<Option<User>, Error> {
    let oke = conn.prep_exec(format!("select IMEI, NAME ,PASSWORD, null as TOKEN from users where NAME=:username and PASSWORD=:pass limit 1"),
            params!("username" => username, "pass" => password))
                .map(|r| r.map(|x| x.unwrap())
                    .map(|row| {
                        let (employee_id, username, password, token) = from_row(row);
                        User{
                            employee_id,
                            username,
                            password,
                            token,
                        }
                    }).into_iter().next())?;
    match oke {
        Some(u) => Ok(Some(u)),
        None => return Ok(None),
    }
}

pub fn TrxToken(conn: &mut Conn, employee_id: &i32, token: &str) -> Result<(), Error> {
    let _ = conn
        .start_transaction(false, None, None)
        .and_then(|mut t| {
            t.prep_exec(
                "INSERT INTO SC_TOKEN
                                (EMPLOYEE_ID, TOKEN, LAST_LOGIN)
                                VALUES
                                (:employee_id, :token, now())",
                params! {
                        "employee_id" => employee_id,
                        "token" => token,
                },
            )
            .unwrap();
            t.commit().is_ok();
            Ok(())
        })
        .unwrap();
    Ok(())
}

pub fn TrxLogout(conn: &mut Conn, employee_id: &i32) -> Result<(), Error> {
    let _ = conn
        .start_transaction(false, None, None)
        .and_then(|mut t| {
            t.prep_exec(
                "UPDATE SC_TOKEN SET LAST_LOGOUT = now() WHERE EMPLOYEE_ID = :employee_id and LAST_LOGOUT IS NULL",
                params! {
                        "employee_id" => employee_id,
                },
            )
            .unwrap();
            t.commit().is_ok();
            Ok(())
        })
        .unwrap();
    Ok(())
}

pub fn get_token(conn: &mut Conn, req: &Token) -> Result<bool, Error> {
    let token = conn
        .prep_exec(
            "select TOKEN from SC_TOKEN where EMPLOYEE_ID=:employee_id AND LAST_LOGIN = CURRENT_DATE() LIMIT 1",
            params! {"employee_id" =>&req.uid},
        )
        .map(|result| {
            result
                .map(|x| x.unwrap())
                .map(|_| true)
                .into_iter()
                .next()
                .unwrap_or_else(|| false)
        });
    match token {
        Ok(ok) => Ok(ok),
        Err(e) => Err(e),
    }

}

pub fn get_tb(conn: &mut Conn) -> Result<Vec<ScTB>, Error> {
    let tb = conn
        .prep_exec("SELECT TB_ID, DESCR FROM SC_TB WHERE STATUS = 'Y'", ())
        .map(|result| {
            result
                .map(|x| x.unwrap())
                .map(|row| {
                    let (tb_id, descr) = from_row(row);
                    ScTB { tb_id, descr }
                })
                .collect()
        });
    match tb {
        Ok(ok) => Ok(ok),
        Err(e) => Err(e),
    }
}

pub fn get_tdb(conn: &mut Conn, id: i32) -> Result<Vec<ScTDB>, Error> {
    let tdb = conn
        .prep_exec(
            "SELECT TDB_ID, DESCR FROM SC_TDB where TB_ID = :id AND STATUS = 'Y'",
            params! {"id" => id},
        )
        .map(|result| {
            result
                .map(|x| x.unwrap())
                .map(|row| {
                    let (tdb_id, descr) = from_row(row);
                    ScTDB { tdb_id, descr }
                })
                .collect()
        });
    match tdb {
        Ok(ok) => Ok(ok),
        Err(e) => Err(e),
    }
}

pub fn get_td(conn: &mut Conn, id: i32) -> Result<Vec<ScTD>, Error> {
    let td = conn
        .prep_exec(
            "SELECT TD_ID, DESCR FROM SC_TD where TB_ID = :id AND STATUS ='Y'",
            params! {"id" => id},
        )
        .map(|result| {
            result
                .map(|x| x.unwrap())
                .map(|row| {
                    let (td_id, descr) = from_row(row);
                    ScTD { td_id, descr }
                })
                .collect()
        });
    match td {
        Ok(ok) => Ok(ok),
        Err(e) => Err(e),
    }
}

pub fn get_elc(conn: &mut Conn, tb_id: i32, tdb_id: i32) -> Result<Vec<ScELC>, Error> {
    let td = conn.prep_exec(
            "SELECT EC_ID, DESCR FROM SC_ELC where TB_ID = :tb_id and TDB_ID = :tdb_id AND STATUS = 'Y'",
            params! {"tb_id" => tb_id, "tdb_id" => tdb_id},
        )
        .map(|result| {
            result
                .map(|x| x.unwrap())
                .map(|row| {
                    let (ec_id, descr) = from_row(row);
                    ScELC { ec_id, descr }
                })
                .collect()
        });
    match td {
        Ok(ok) => Ok(ok),
        Err(e) => Err(e),
    }
}

pub fn get_list(conn: &mut Conn, id: i32) -> Result<Vec<ScWorkorder>, Error> {
    let sc_wo = conn.prep_exec(
            "SELECT WO_ID, CUSTOMER_ID, PROSPECT_NBR, ASSIGN_TO, SERVICES_ID, SERVICES_DESCR, SERVICES_CATEGORY, DESCR, SCHEDULE_DATE, REGION, LATITUDE, LONGITUDE, CREATED_DATE FROM SC_WORKORDER WHERE ASSIGN_TO = :id ORDER BY CREATED_DATE DESC",
            params! {"id" => id},
        )
        .map(|result| {
            result
                .map(|x| x.unwrap())
                .map(|mut row| {
                    ScWorkorder {
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
                .collect()
        });
    match sc_wo {
        Ok(ok) => Ok(ok),
        Err(e) => Err(e),
    }
}

pub fn get_profile(conn: &mut Conn, id: i32) -> Result<Vec<ScCustomer>, Error> {
    let sc_cust = conn.prep_exec(
            "SELECT CUSTOMER_ID, CUSTOMER_NAME, ADDRESS, MOBILE_PHONE, HOME_PHONE, EXTRA_PHONE, WHATSAPP, GENDER, EMAIL, FOTO FROM SC_CUSTOMER WHERE CUSTOMER_ID = :id",
            params! {"id" => id},
        )
        .map(|result| {
            result
                .map(|x| x.unwrap())
                .map(|mut row| {
                    ScCustomer {
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
                .collect()
        });
    match sc_cust {
        Ok(ok) => Ok(ok),
        Err(e) => Err(e),
    }
}

pub fn TrxResult(conn: &mut Conn, req: &ScResult) -> Result<(), Error> {
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
    Ok(())
}

pub fn getCallback(conn: &mut Conn, customer_id: &i64) -> Result<Vec<ScCallback>, Error> {
    let call = conn.prep_exec("
        SELECT A.SC_ID, A.CUSTOMER_ID, C.SCORE, C.SEC, E.PRODUCT_ID, E.PRODUCT_NAME, E.PROMO_ID, E.PROMO_CODE, E.PROMO_DESCR FROM SC_RESULT_NEW A
        JOIN SC_RESULT_SCORE B ON A.SC_ID = B.SC_ID
        JOIN SC_SCORE C ON B.SEC_ID = C.SEC_ID
        JOIN SC_CALLBACK D ON B.SCORE_ID = D.SCORE_ID
        JOIN SC_MAPPING_PRODUCT E ON D.MAP_ID = E.MAP_ID WHERE A.CUSTOMER_ID = :id",  params!{"id" => &customer_id})
        .map(|result| {
            result
                .map(|x| x.unwrap())
                .map(|mut row| {
                    ScCallback {
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
                .collect()
        });
    match call {
        Ok(ok) => Ok(ok),
        Err(e) => Err(e),
    }
}

pub fn getSimulation(
    conn: &mut Conn,
    customer_id: &i64,
    tb_id: &i32,
    tdb_id: &i32,
    td_id: &i32,
    elc_id: &i32,
) -> Result<Vec<ScCallback>, Error> {
    let sim = conn
        .prep_exec(
            "
        SELECT 0 AS SC_ID, D.CUSTOMER_ID, B.SCORE, B.SEC, A.PROD_TS AS PRODUCT_ID, 
        E.PRODUCT_NAME, E.PROMO_ID, E.PROMO_CODE, E.PROMO_DESCR FROM SC_MATRIX A 
        JOIN SC_SCORE B ON A.SCORE = B.SCORE JOIN SC_WORKORDER C
        ON A.REGION = C.REGION JOIN SC_CUSTOMER D
        ON A.PROD_SLS = D.PRODUCT_ID AND A.BF_PROD_SLS = D.BF
        JOIN SC_MAPPING_PRODUCT E
        ON A.PROD_TS = E.PRODUCT_ID AND A.BF_PROD_TS = E.BILL_FREQ WHERE C.CUSTOMER_ID = :id
        AND B.TB_ID = :tb_id AND B.TDB_ID = :tdb_id AND B.TD_ID = :td_id AND B.EC_ID = :ec_id ",
            params! {
                "id" => &customer_id,
                "tb_id" => &tb_id,
                "tdb_id" => &tdb_id,
                "td_id" => &td_id,
                "ec_id" => &elc_id,
            },
        )
        .map(|result| {
            result
                .map(|x| x.unwrap())
                .map(|mut row| ScCallback {
                    sc_id: row.take("SC_ID").unwrap(),
                    customer_id: row.take("CUSTOMER_ID").unwrap(),
                    score: row.take("SCORE").unwrap(),
                    sec: row.take("SEC").unwrap(),
                    product_id: row.take("PRODUCT_ID").unwrap(),
                    product_name: row.take("PRODUCT_NAME").unwrap(),
                    promo_id: row.take("PROMO_ID").unwrap(),
                    promo_code: row.take("PROMO_CODE").unwrap(),
                    promo_descr: row.take("PROMO_DESCR").unwrap(),
                })
                .collect()
        });
    match sim {
        Ok(ok) => Ok(ok),
        Err(e) => Err(e),
    }

}

pub fn TrxDetail(conn: &mut Conn, req: &ScDetail, opts: Option<i32>) -> Result<(), Error> {
    if let Some(o) = Some(opts) {
        if let Some(1) = o {
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
        } else {
            let _ = conn
                .start_transaction(false, None, None)
                .and_then(|mut t| {
                    t.prep_exec(
                        "UPDATE SC_RESULT_DETAIL SET
                                            CUSTOMER_NAME = :customer_name,
                                        ADDRESS = :address,
                                        MOBILE_PHONE = :mobile_phone,
                                        HOME_PHONE = :home_phone,
                                        EXTRA_PHONE = :extra_phone,
                                        WHATSAPP = :whatsapp,
                                        GENDER = :gender,  
                                        EMAIL = :email
                                        WHERE CUSTOMER_ID =:customer_id",
                        params! {
                            "customer_name" => &req.customer_name.clone(),
                            "address" => &req.address.clone(),
                            "mobile_phone" => &req.mobile_phone.clone(),
                            "home_phone" => &req.home_phone.clone(),
                            "extra_phone" => &req.extra_phone.clone(),
                            "whatsapp" => &req.whatsapp.clone(),
                            "gender" => &req.gender.clone(),
                            "email" => &req.email.clone(),
                            "customer_id" => &req.customer_id.clone(),
                        },
                    )
                    .unwrap();
                    t.commit().is_ok();
                    Ok(())
                })
                .unwrap();
        }
    }
    Ok(())
}

pub fn getDetail(conn: &mut Conn, employee_id: &i32) -> Result<Vec<ScDetail>, Error> {
    let detail = conn.prep_exec(
        "SELECT CUSTOMER_ID, CUSTOMER_NAME, ADDRESS, MOBILE_PHONE, HOME_PHONE, EXTRA_PHONE, WHATSAPP, GENDER, EMAIL FROM SC_RESULT_DETAIL WHERE CUSTOMER_ID = :id",
        params! {"id" => employee_id},
    )
    .map(|result| {
        result
            .map(|x| x.unwrap())
            .map(|mut row| {
                ScDetail {
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
            .collect()
    });
    match detail {
        Ok(ok) => Ok(ok),
        Err(e) => Err(e),
    }
}