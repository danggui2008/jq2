use rbatis::rbdc::DateTime;
use serde::{Deserialize, Serialize};


#[derive(Debug,Clone,Deserialize,Serialize)]
pub struct User {
    pub user_id: i64,
    pub nick_name: String,
    pub login_name: String,
    pub password_md5: String,
    pub introduce_sign: String,
    pub is_deleted: i8,
    pub locked_flag: i8,
    pub create_time: DateTime,
}
#[derive(Debug,Deserialize,Serialize)]
pub struct ReqRegister {
    pub login_name: String,
    pub password: String,
}

#[derive(Debug,Deserialize,Serialize)]
pub struct ReqEditUser {
    pub nick_name: String,
    pub introduce_sign: String,
    pub password_md5: Option<String>,
}

#[derive(Debug,Deserialize,Serialize)]
pub struct ReqLockUser {
    pub user_ids: Vec<i64>,
}


#[derive(Debug,Deserialize)]
pub struct ReqLogin {
    pub login_name: String,
    pub password_md5: String,
}


