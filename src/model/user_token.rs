use rbatis::rbdc::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct UserToken {
    pub user_id: i64,
    pub token: String,
    pub update_time: DateTime,
    pub expire_time: DateTime,
}

impl  UserToken{
    pub fn expire_time_check(&self)->bool{
        self.expire_time < DateTime::now()
    }
}
