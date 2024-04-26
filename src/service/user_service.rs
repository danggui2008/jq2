use std::time::Duration;

use rbatis::plugin::page::Page;
use rbatis::rbdc::DateTime;

use crate::{
    common::constant, dao::{UserDao, UserTokenDao}, middleware::UserIdentity, model::{ReqEditUser, User, UserToken}, response::result::Result, util
};

pub struct UserService {
    dao: UserDao,
}

impl UserService {
    pub fn new() -> Self {
        Self {
            dao: UserDao::new(),
        }
    }
}

impl UserService {
    pub async fn register(&self, user: User) -> Result<bool> {
        let users = self.dao.find_by_login_name(&user.login_name).await?;
        if users.len() > 0 {
            return Err("用户名已存在".into());
        }
        self.dao.create(&user).await
    }
    pub async fn login(&self, login_name: &str, password_md5: &str) -> Result<String> {
        let mut user = self
            .dao
            .find_by_login_name_password(login_name, password_md5)
            .await?;
        if user.len() > 0 {
            let user = user.pop().unwrap();
            if user.locked_flag == constant::LOCKED {
                return Err("用户已经禁止登录".into());
            }
            let token = util::generate_token(user.user_id);
            let user_token_dao = UserTokenDao::new();
            let mut user_token = user_token_dao.find_by_id(user.user_id).await?;
            if user_token.len() > 0 {
                let mut user_token = user_token.pop().unwrap();
                user_token.update_time = DateTime::now();
                user_token.expire_time = DateTime::now();
                user_token_dao.update(&user_token).await?;
            } else {
                let use_token = UserToken {
                    user_id: user.user_id,
                    token: token.clone(),
                    update_time: DateTime::now(),
                    expire_time: DateTime::now().add(Duration::from_secs(constant::DAY_SECS)),
                };
                user_token_dao.create(&use_token).await?;
            }
            Ok(token)
        } else {
            return Err("登录失败,请确定用户名/密码是否正确".into());
        }
    }

    pub async fn  login_out(&self,identity:&UserIdentity)->Result<bool>{
        let user_token_dao  = UserTokenDao::new();
        user_token_dao.delete(identity.user.user_id).await
    }

    pub async fn modify_user(&self, user: &mut User, data: ReqEditUser) -> Result<bool> {
        user.nick_name = data.nick_name;
        user.introduce_sign = data.introduce_sign;
        if let Some(password_md5) = data.password_md5 {
            if !password_md5.is_empty() {
                user.password_md5 = password_md5;
            }
        }
        self.dao.update(user).await
    }

    pub async fn list_page(&self, page_no: u64, page_size: u64) -> Result<Page<User>> {
        self.dao.list_page(page_no, page_size).await
    }

    pub async fn list(&self) -> Result<Vec<User>> {
        self.dao.list().await
    }

    pub async fn lock_users(&self, user_ids: Vec<i64>, locked_flae: i8) -> Result<bool> {
        self.dao
            .update_locked(user_ids.as_slice(), locked_flae)
            .await
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::util::md5_string;

    #[test]
    fn test_register() {
        let service = UserService::new();
        let password_md5 = md5_string("12345678".to_string());
        let user = User {
            user_id: 123,
            nick_name: "cyxy".to_string(),
            login_name: "cyxy".to_string(),
            password_md5: password_md5,
            introduce_sign: "kft".to_string(),
            is_deleted: 0,
            locked_flag: 0,
            create_time: DateTime::now(),
        };
        let result = tokio_test::block_on(service.register(user));
        assert!(result.is_ok())
    }

    #[test]
    fn test_login() {
        let login_name = "cyxy";
        let password = &crate::util::md5_string("12345678".to_string());
        let service = UserService::new();
        let result = tokio_test::block_on(service.login(login_name, password)).unwrap();
        println!("token:{}", &result);
        assert!(!result.is_empty())
    }

    #[test]
    fn test_modify_user() {
        let user_id: i64 = 123;
        let nick_name = "cyxy2";
        let introduce_sign = "kfr";
        let password_md5 = &crate::util::md5_string("12345678".to_string());
        let service = UserService::new();
        let mut user = tokio_test::block_on(service.dao.find_by_id(user_id)).unwrap();
        let data = ReqEditUser {
            nick_name: nick_name.to_string(),
            introduce_sign: introduce_sign.to_string(),
            password_md5: Some(password_md5.to_string()),
        };
        let result = tokio_test::block_on(service.modify_user(&mut user, data));
        assert!(result.is_ok())
    }

    #[test]
    fn test_list_page() {
        let page_no: u64 = 1;
        let page_size: u64 = 3;
        let service = UserService::new();
        let result = tokio_test::block_on(service.dao.list_page(page_no, page_size)).unwrap();
        assert_eq!(1, result.records.len())
    }

    #[test]
    fn test_user_lock() {
        let mut ids: Vec<i64> = Vec::new();
        ids.push(123);
        let service = UserService::new();
        let result = tokio_test::block_on(service.lock_users(ids, 1)).unwrap();
        assert!(result)
    }
}
