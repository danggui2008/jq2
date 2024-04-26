use rbatis::rbatis_codegen::IntoSql;
use rbatis::{
    executor::Executor,
    plugin::page::{Page, PageRequest},
    rbdc::db::ExecResult,
    Error,
};

use crate::{dao::DB, model::User, response::result};
impl_select!(User {});
impl_insert!(User {});
impl_delete!(User {delete_by_id(user_id :i64) => "`where user_id = #{user_id}`"});
impl_update!(User {update_by_id(user_id :i64) => "`where user_id = #{user_id}`"});
impl_select!(User {find_by_id(user_id :i64) => "`where user_id = #{user_id} limit 1`"});
impl_select!(User {find_by_login_name(login_name :&str) => "`where login_name = #{login_name} limit 1`"});
impl_select!(User {find_by_login_name_password(login_name :&str,password :&str) => "`where login_name = #{login_name} and password_md5 = #{password} limit 1`"});
impl_select_page!(User {list_page() => "` order by create_time`"});

#[py_sql(
    "`update user set locked_flag = #{locked_flae}  where is_deleted =0`
        if !ids.is_empty():
        ` and user_id in`
         ${ids.sql()}"
)]
async fn update_locked_inner(
    rb: &dyn Executor,
    ids: &[i64],
    locked_flae: i8,
) -> std::result::Result<ExecResult, Error> {
    impled!()
}
pub struct UserDao;

impl UserDao {
    pub fn new() -> Self {
        Self {}
    }
}

impl UserDao {
    pub async fn create(&self, user: &User) -> result::Result<bool> {
        let result = User::insert(DB.get_rb_ref(), user).await?;
        Ok(result.rows_affected > 0)
    }

    pub async fn find_by_id(&self, user_id: i64) -> result::Result<User> {
        let mut result = User::find_by_id(DB.get_rb_ref(), user_id).await?;
        if result.len() > 0 {
            Ok(result.pop().unwrap())
        } else {
            Err("该用户不存在".into())
        }
    }
    pub async fn find_by_login_name(&self, login_name: &str) -> result::Result<Vec<User>> {
        Ok(User::find_by_login_name(DB.get_rb_ref(), login_name).await?)
    }
    pub async fn find_by_login_name_password(
        &self,
        login_name: &str,
        password: &str,
    ) -> result::Result<Vec<User>> {
        Ok(User::find_by_login_name_password(DB.get_rb_ref(), login_name, password).await?)
    }
    pub async fn update(&self, user: &User) -> result::Result<bool> {
        let result = User::update_by_id(DB.get_rb_ref(), user, user.user_id).await?;
        Ok(result.rows_affected > 0)
    }
    pub async fn list_page(&self, page_no: u64, page_size: u64) -> result::Result<Page<User>> {
        let result =
            User::list_page(DB.get_rb_ref(), &PageRequest::new(page_no, page_size)).await?;
        Ok(result)
    }

    pub async fn update_locked(&self, ids: &[i64], locked_flae: i8) -> result::Result<bool> {
        let result = update_locked_inner(DB.get_rb_ref(), ids, locked_flae).await?;
        Ok(result.rows_affected == (ids.len() as u64))
    }

    pub async fn list(&self) -> result::Result<Vec<User>> {
        Ok(User::select_all(DB.get_rb_ref()).await?)
    }
}
