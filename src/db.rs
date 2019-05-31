extern crate mysql;
extern crate r2d2;
extern crate r2d2_mysql;
// use actix::{SyncContext};

use r2d2_mysql::mysql::{Opts, OptsBuilder};
use r2d2_mysql::pool::MysqlConnectionManager;

pub type Pool = r2d2::Pool<MysqlConnectionManager>;

// pub struct DbExecutor(pub Pool);

// impl Actor for DbExecutor {
//     type context = SyncContext<Self>;
// }

// pub fn init_pool(db_url: &str) -> Arc<Pool> {
//     let opts = Opts::from_url(db_url).unwrap();
//     let builder = OptsBuilder::from_opts(opts);
//     let manager = MysqlConnectionManager::new(builder);
//     Arc::new(r2d2::Pool::builder().max_size(15).build(manager).unwrap())
// }

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
