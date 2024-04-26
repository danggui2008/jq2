use crate::{dao::DB, model::UserToken, response::result::Result};
pub struct UserTokenDao;

impl_insert!(UserToken {});
impl_delete!(UserToken {delete_by_id(user_id :i64) => "`where user_id = #{user_id}`"});
impl_update!(UserToken {update_by_id(user_id :i64) => "`where user_id = #{user_id}`"});
impl_select!(UserToken {find_by_id(user_id :i64) => "`where user_id = #{user_id} limit 1`"});
impl_select!(UserToken {find_by_token(token :&str) => "`where token = #{token} limit 1`"});

impl UserTokenDao {
    pub fn new() -> Self {
        Self {}
    }
}

impl UserTokenDao {
    pub async fn find_by_id(&self, user_id: i64) -> Result<Vec<UserToken>> {
        Ok(UserToken::find_by_id(DB.get_rb_ref(), user_id).await?)
    }
    pub async fn find_by_token(&self, token: &str) -> Result<Vec<UserToken>> {
        Ok(UserToken::find_by_token(DB.get_rb_ref(), token).await?)
    }

    pub async fn create(&self, user_token: &UserToken) -> Result<bool> {
        let result = UserToken::insert(DB.get_rb_ref(), user_token).await?;
        Ok(result.rows_affected > 0)
    }
    pub async fn update(&self, user_token: &UserToken) -> Result<bool> {
        let result: rbatis::rbdc::db::ExecResult =
            UserToken::update_by_id(DB.get_rb_ref(), user_token, user_token.user_id).await?;
        Ok(result.rows_affected > 0)
    }

    pub async fn delete(&self, user_id: i64) -> Result<bool> {
        let result = UserToken::delete_by_id(DB.get_rb_ref(), user_id).await?;
        Ok(result.rows_affected > 0)
    }
}
