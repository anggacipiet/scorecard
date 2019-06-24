
use crate::model::{
    Request, ScAddon, ScBasic, ScCallback, ScCustomer, ScDetail, ScELC, ScPackages, ScResult, ScTB,
    ScTD, ScTDB, ScWorkorder, Token, User, ScCalculate,
};
use chrono::Local;
use log::{debug, info, trace, warn};
use mysql::{from_row, from_row_opt, from_value, params, Error};
use r2d2_mysql::mysql::{Opts, OptsBuilder};
use r2d2_mysql::pool::MysqlConnectionManager;
use serde_json::{from_str, json, Value};
use std::collections::HashMap;

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
                                        "employee_id" => &req.user.employee_id.clone(),
                                        "nik" => &req.user.nik.clone(),
                                        "user_login" => &req.user.user_login.clone(),
                                        "employee_name" => &req.user.employee_name.clone(),
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
                                        "latlng" => &req.user.latlng.clone(),
                                        "timelng" => &req.user.date_latlng.clone(),
                                        "data" =>  &req.data.get(),
                                        "process" => &req.action_package.clone(),
                                })
                        .unwrap();
                        t.commit().is_ok();
                Ok(())
                })
        .unwrap();
    Ok(())
}

pub fn get_login(conn: &mut Conn, username: &str, password: &str) -> Result<Option<User>, Error> {
    let oke = conn.prep_exec("select employee_id, user_name, password, nik, sfl_code,
            full_name, email,  user_type, role_name, brand, branch, region_name,
            application_id, avatar, null as token 
            from V_SC_USER_LOGIN where user_name=:username and password=:pass limit 1",
            params! {"username" => &username, "pass" => &password},)
                .map(|r| r.map(|x| x.unwrap())
                    .map(|mut row| {
                        User{
                            employee_id: row.take("employee_id").unwrap(),
                            username: row.take("user_name").unwrap(),
                            password: row.take("password").unwrap(),
                            nik: row.take("nik").unwrap(),
                            sfl_code: row.take("sfl_code").unwrap(),
                            employee_name: row.take("full_name").unwrap(),
                            email: row.take("email").unwrap(),
                            user_type: row.take("user_type").unwrap(),
                            role_name: row.take("role_name").unwrap(),
                            brand_code: row.take("brand").unwrap(),
                            branch_name: row.take("branch").unwrap(), 
                            region_name: row.take("region_name").unwrap(),
                            application_id: row.take("application_id").unwrap(),
                            avatar: row.take("avatar").unwrap(),
                            token: row.take("token").unwrap(),
                        }
                    }).into_iter().next())?;
    match oke {
        Some(u) => Ok(Some(u)),
        _ => return Ok(None),
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
                                (:employee_id, :token, NOW())",
                params! {
                        "employee_id" => &employee_id,
                        "token" => &token,
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
                "UPDATE SC_TOKEN SET LAST_LOGOUT = NOW() WHERE EMPLOYEE_ID = :employee_id and LAST_LOGOUT IS NULL",
                params! {
                        "employee_id" => &employee_id,
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
            "SELECT TOKEN FROM SC_TOKEN WHERE EMPLOYEE_ID=:employee_id ORDER BY LAST_LOGIN DESC LIMIT 1",
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
            "SELECT CUSTOMER_ID, CUSTOMER_NAME, CUSTOMER_NAME_UPDATE, CUSTOMER_NAME_UPDATE_CHECK,
            ADDRESS, ADDRESS_UPDATE, ADDRESS_UPDATE_CHECK,
            MOBILE_PHONE, MOBILE_PHONE_UPDATE, MOBILE_PHONE_UPDATE_CHECK,
            HOME_PHONE, HOME_PHONE_UPDATE, HOME_PHONE_UPDATE_CHECK,
            EXTRA_PHONE, EXTRA_PHONE_UPDATE, EXTRA_PHONE_UPDATE_CHECK,
            WHATSAPP, WHATSAPP_UPDATE, WHATSAPP_UPDATE_CHECK,
            GENDER, GENDER_UPDATE, GENDER_UPDATE_CHECK,
            EMAIL, EMAIL_UPDATE, EMAIL_UPDATE_CHECK,
            FOTO, FOTO_UPDATE, FOTO_UPDATE_CHECK
            FROM SC_V_CUSTOMER WHERE CUSTOMER_ID = :id",
            params! {"id" => id},
        )
        .map(|result| {
            result
                .map(|x| x.unwrap())
                .map(|mut row| {
                    ScCustomer {
                        customer_id : row.take("CUSTOMER_ID").unwrap(),
                        customer_name: row.take("CUSTOMER_NAME").unwrap(),
                        customer_name_update: row.take("CUSTOMER_NAME_UPDATE").unwrap(),
                        customer_name_update_check: row.take("CUSTOMER_NAME_UPDATE_CHECK").unwrap(),
                        address: row.take("ADDRESS").unwrap(),
                        address_update: row.take("ADDRESS_UPDATE").unwrap(),
                        address_update_check: row.take("ADDRESS_UPDATE_CHECK").unwrap(),
                        mobile_phone: row.take("MOBILE_PHONE").unwrap(),
                        mobile_phone_update: row.take("MOBILE_PHONE_UPDATE").unwrap(),
                        mobile_phone_update_check: row.take("MOBILE_PHONE_UPDATE_CHECK").unwrap(),
                        home_phone: row.take("HOME_PHONE").unwrap(),
                        home_phone_update: row.take("HOME_PHONE_UPDATE").unwrap(),
                        home_phone_update_check: row.take("HOME_PHONE_UPDATE_CHECK").unwrap(),
                        extra_phone: row.take("EXTRA_PHONE").unwrap(),
                        extra_phone_update: row.take("EXTRA_PHONE_UPDATE").unwrap(),
                        extra_phone_update_check: row.take("EXTRA_PHONE_UPDATE_CHECK").unwrap(),
                        whatsapp: row.take("WHATSAPP").unwrap(),
                        whatsapp_update: row.take("WHATSAPP_UPDATE").unwrap(),
                        whatsapp_update_check: row.take("WHATSAPP_UPDATE_CHECK").unwrap(),
                        gender: row.take("GENDER").unwrap(),
                        gender_update: row.take("GENDER_UPDATE").unwrap(),
                        gender_update_check: row.take("GENDER_UPDATE_CHECK").unwrap(),
                        email: row.take("EMAIL").unwrap(),
                        email_update: row.take("EMAIL_UPDATE").unwrap(),
                        email_update_check: row.take("EMAIL_UPDATE_CHECK").unwrap(),
                        foto: row.take("FOTO").unwrap(),
                        foto_update: row.take("FOTO_UPDATE").unwrap(),
                        foto_update_check: row.take("FOTO_UPDATE_CHECK").unwrap(),
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
                                (CUSTOMER_ID, TB_ID, TDB_ID, TD_ID, EC_ID, EMPLOYEE_ID, LATITUDE, LONGITUDE, CREATED_DATE)
                            VALUES
                                (:customer_id, :tb_id, :tdb_id, :td_id, :ec_id, :employee_id, :latitude, :longitude, NOW())",
                            params!{
                                "customer_id" => &req.customer_id.clone(),
                                "tb_id" => &req.tb_id.clone(),
                                "tdb_id" => &req.tdb_id.clone(),
                                "td_id" => &req.td_id.clone(),
                                "ec_id" => &req.ec_id.clone(),
                                "employee_id" => &req.employee_id.clone(),
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
        SELECT A.SC_ID, A.CUSTOMER_ID, C.SCORE, C.SEC,
        CASE WHEN D.SOURCE = 'N' THEN E.PRODUCT_ID ELSE F.PRODUCT_ID END AS PRODUCT_ID,
        CASE WHEN D.SOURCE = 'N' THEN E.PRODUCT_NAME ELSE G.PRODUCT_NAME END AS PRODUCT_NAME, 
        CASE WHEN D.SOURCE = 'N' THEN E.PROMO_ID ELSE F.PROMO_ID END AS PROMO_ID,
        CASE WHEN D.SOURCE = 'N' THEN E.PROMO_CODE ELSE H.PROMOTION_CODE END AS PROMO_CODE, 
        CASE WHEN D.SOURCE = 'N' THEN E.PROMO_DESCR ELSE H.PROMOTION_DESC END AS PROMO_DESCR, 
        CASE WHEN D.SOURCE = 'N' THEN E.BILL_FREQ ELSE F.BF END AS BILL_FREQ,
        CASE WHEN D.SOURCE = 'N' THEN 'REJECT' ELSE 'APPROVE' END AS REASON
        FROM SC_RESULT_NEW A
        JOIN SC_RESULT_SCORE B ON A.SC_ID = B.SC_ID
        JOIN SC_SCORE C ON B.SEC_ID = C.SEC_ID
        JOIN SC_CALLBACK D ON B.SCORE_ID = D.SCORE_ID
        LEFT JOIN SC_MAPPING_PRODUCT E ON D.MAP_ID = E.MAP_ID
        LEFT JOIN SC_CUSTOMER F ON D.MAP_ID = F.PRODUCT_ID  AND D.MAT_ID = F.PROMO_ID
        LEFT JOIN valsys_prod.VAL_PRODUCT_PRICE2 G ON G.PRODUCT_ID = F.PRODUCT_ID
        LEFT JOIN valsys_prod.M_PROMOTION H ON F.PROMO_ID = H.PROMOTION_ID
        WHERE A.CUSTOMER_ID = :id",  params!{"id" => &customer_id})
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
                        bill_freq: row.take("BILL_FREQ").unwrap(),
                        reason: row.take("REASON").unwrap(),
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
        E.PRODUCT_NAME, E.PROMO_ID, E.PROMO_CODE, E.PROMO_DESCR, E.BILL_FREQ, '' AS REASON
        FROM SC_MATRIX A 
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
                    bill_freq: row.take("BILL_FREQ").unwrap(),
                    reason: row.take("REASON").unwrap(),
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
                                        (CUSTOMER_ID, CUSTOMER_NAME, ADDRESS, MOBILE_PHONE, HOME_PHONE, EXTRA_PHONE, WHATSAPP, GENDER, EMAIL, FOTO, CREATED_DATE)
                                    VALUES
                                        (:customer_id, :customer_name, :address, :mobile_phone, :home_phone, :extra_phone, :whatsapp, :gender, :email, :foto, NOW())",
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
                                        "foto" => &req.foto.clone(),
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
                                        EMAIL = :email,
                                        FOTO = :foto,
                                        CREATED_DATE = NOW()
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
                            "foto" => &req.foto.clone(),
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

pub fn getDetail(conn: &mut Conn, customer_id: &i64) -> Result<Vec<ScDetail>, Error> {
    let detail = conn.prep_exec(
        "SELECT CUSTOMER_ID, CUSTOMER_NAME, ADDRESS, MOBILE_PHONE, HOME_PHONE, EXTRA_PHONE, WHATSAPP, GENDER, EMAIL, FOTO FROM SC_RESULT_DETAIL WHERE CUSTOMER_ID = :id",
        params! {"id" => &customer_id},
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
                    foto: row.take("FOTO").unwrap(),
                }
            })
            .collect()
    });
    match detail {
        Ok(ok) => Ok(ok),
        Err(e) => Err(e),
    }
}

pub fn getCalculate(conn: &mut Conn, customer_id: &i64) -> Result<Option<ScPackages>, Error> {
    let packages = conn
        .prep_exec(
            "SELECT BRAND, PROMO_ID, PROSPECT_TYPE, HW_STATUS, CUSTOMER_CLASS, HOUSE_STATUS, 
        FIRST_PAYMENT, INET_ROUTER, INET_ADDON, PRODUCT FROM SC_V_CALCULATE WHERE CUSTOMER_ID = :id",
            params! {"id" => &customer_id},
        )
        .map(|r| {
            r.map(|x| x.unwrap())
                .map(|mut row| {  
                    ScPackages {
                        brand_id: row.take("BRAND").unwrap(),
                        promotion_id: row.take("PROMO_ID").unwrap(),
                        prospect_type: row.take("PROSPECT_TYPE").unwrap(),
                        hardware_status: row.take("HW_STATUS").unwrap(),
                        customer_class: row.take("CUSTOMER_CLASS").unwrap(),
                        house_status: row.take("HOUSE_STATUS").unwrap(),
                        first_payment: row.take("FIRST_PAYMENT").unwrap(),
                        internet_package_router: row.take("INET_ROUTER").unwrap(),
                        internet_package_addon: row.take("INET_ADDON").unwrap(),
                        package: row.take("PRODUCT").unwrap(),
                    }
            
                })
                .into_iter()
                .next()
        })?;
    match packages {
        Some(calculate) => Ok(Some(calculate)),
        _ => return Ok(None),
    }
}

pub fn TrxCalculate(conn: &mut Conn, req: &ScCalculate, callback_id: &i32) -> Result<(), Error> {
    let _ = conn
        .start_transaction(false, None, None)
        .and_then(|mut t| {
            t.prep_exec("INSERT INTO SC_CALCULATE
                                (CALLBACK_ID, COST_BASIC, COST_ADDON, COST_INET_ADDON, COST_INET_ROUTER,
                                 COST_HD_CHARGE, BP_CHARGE, DEC_HD_CHARGE, ESTIMATED_iNSTALLATION,
                                 ESTIMATED_PACKAGE, ESTIMATED_ADDON, ESTIMATED_PROMO,TOTAL_ESTIMATED_COST)
                            VALUES
                                (:callback_id, :cost_basic, :cost_addon, :cost_inet_addon, :cost_inet_router,
                                 :cost_hd_charge, :bp_charge, :dec_hd_charge, :estimated_installation,
                                 :estimated_package, :estimated_addon, :estimated_promo, :total_estimated_cost)",
                            params!{
                                "callback_id" => &callback_id,
                                "cost_basic" => &req.COST_PACKAGE.clone(),
                                "cost_addon" => &req.COST_ALACARTE.clone(),
                                "cost_inet_addon" => &req.COST_INTERNET_ADDON.clone(),
                                "cost_inet_router" => &req.COST_INTERNET_ROUTER.clone(),
                                "cost_hd_charge" => &req.COST_HD_CHARGE.clone(),
                                "bp_charge" => &req.BELI_PUTUS_CHARGE.clone(),
                                "dec_hd_charge" => &req.DECODER_HD_CHARGE.clone(),
                                "estimated_installation" => &req.ESTIMATED_INSTALLATION.clone(),
                                "estimated_package" => &req.ESTIMATED_COST_PACKAGE.clone(),
                                "estimated_addon" => &req.ESTIMATED_ALACARTE.clone(),
                                "estimated_promo" => &req.ESTIMATED_PROMO.clone(),
                                "total_estimated_cost" => &req.TOTAL_ESTIMATED_COSTS.clone(),
                            })
                .unwrap();
            t.commit().is_ok();
            Ok(())
        })
        .unwrap();
    Ok(())
}

pub fn push_ppg(conn: &mut Conn, customer_id: &i64, customer_name: &String, amount: &i64) -> Result<(), Error> {
    let paid = conn
        .prep_exec(
            "select customer_id from valsys_prod.CUST_INQUIRY where CUST_NEW=:customer_id AND PAID =0",
            params! {"customer_id" =>&customer_id},
        )
        .map(|result| {
            result
                .map(|x| x.unwrap())
                .map(|_| true)
                .into_iter()
                .next()
                .unwrap_or_else(|| false)
        });
    match Some(paid) {
        Some(ok) => {
            let _ = conn
            .start_transaction(false, None, None)
            .and_then(|mut t| {
                t.prep_exec(
                    "UPDATE valsys_prod.CUST_INQUIRY  SET AMMOUNT = :amount, NMPEL = :customer_name WHERE CUST_NEW=:customer_id",
                    params! {
                            "amount" => &amount,
                            "customer_name" => &customer_name,
                            "customer_id" => &customer_id,
                    },
                )
                .unwrap();
                t.commit().is_ok();
                Ok(())
            })
            .unwrap();
            Ok(())
        },
        None => {
            let _ = conn
            .start_transaction(false, None, None)
            .and_then(|mut t| {
                t.prep_exec(
                    "INSERT INTO valsys_prod.CUST_INQUIRY(CUST_OLD, CUST_NEW, NMPEL, AMMOUNT, PAID, SFROM, STO)
                      VALUES (0, :customer_id, :name, :amount, 0, '00010101000000', '00010101000000')",
                    params! {
                        "customer_id" => &customer_id,
                        "name" => &customer_name,
                        "amount" => &amount,
                    },
                )
                .unwrap();
                t.commit().is_ok();
                Ok(())
            })
            .unwrap();
            Ok(())
        },
    } 
}