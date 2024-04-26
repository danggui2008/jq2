use once_cell::sync::Lazy;
use rbatis::rbatis::RBatis;

use crate::config::Config;

pub static DB: Lazy<Dao> = Lazy::new(|| {
    let dao = Dao::new();
    dao
});
pub struct Dao {
    rb: RBatis,
}

impl Dao {
    fn init_db() -> RBatis {
        let rb = RBatis::new();
        let config  = Config::default();
        // ------------choose database driver------------
        rb.init(
            rbdc_mysql::driver::MysqlDriver {},
            &config.db.url(),
        )
        .unwrap();
        rb
    }

    pub(crate) fn new() -> Self {
        Self {
            rb: Self::init_db(),
        }
    }

    pub(crate) fn get_rb_ref(&self) -> &RBatis {
        &self.rb
    }
}
