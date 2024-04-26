use actix_web::{
    get, post, put,
    web::{Data, Json, Path},
};
use rbatis::rbdc::DateTime;

use crate::{
    common::constant::INTRODUCE_SIGN,
    middleware::UserIdentity,
    model::{ReqEditUser, ReqLockUser, ReqLogin, ReqRegister, User},
    response::{self, result::Response},
    service::UserService,
    util,
};

// 用户注册
#[post("/register")]
pub async fn register(us: Data<UserService>, Json(data): Json<ReqRegister>) -> Response {
    let user = User {
        user_id: util::gen_random_num(15) as i64,
        login_name: data.login_name.clone(),
        nick_name: data.login_name.clone(),
        password_md5: util::md5(data.password.as_str()),
        introduce_sign: INTRODUCE_SIGN.to_string(),
        create_time: DateTime::now(),
        is_deleted: 0,
        locked_flag: 0,
    };
    let user_service = us.get_ref();
    let data = user_service.register(user).await?;
    response::Response::success(data)
}
#[get("/list")]
pub async fn list(us: Data<UserService>) -> Response {
    let user_service = us.get_ref();
    let users = user_service.list().await?;
    response::Response::success(users)
}

#[put["{locked_flag}"]]
pub async fn lock_user(
    us: Data<UserService>,
    Json(req_lock_user): Json<ReqLockUser>,
    locked_flag: Path<i8>,
) -> Response {
    let user_service = us.get_ref();
    let lock_user = user_service
        .lock_users(req_lock_user.user_ids, locked_flag.into_inner())
        .await?;
    response::Response::success(lock_user)
}

#[post("/login")]
pub async fn login(us: Data<UserService>, Json(req_login): Json<ReqLogin>) -> Response {
    let user_service = us.get_ref();
    let token = user_service
        .login(&req_login.login_name, &req_login.password_md5)
        .await?;
    response::Response::success(token)
}
#[post("/logout")]
pub async fn logout(us: Data<UserService>, identity: UserIdentity) -> Response {
    let user_service = us.get_ref();
    let data = user_service.login_out(&identity).await?;
    response::Response::success(data)
}

// 修改用户信息
#[put("/info")]
pub async fn edit_info(
    us: Data<UserService>,
    Json(edit_user): Json<ReqEditUser>,
    mut identity: UserIdentity,
) -> Response {
    let user_service = us.get_ref();

    let data = user_service
        .modify_user(&mut identity.user, edit_user)
        .await?;
    response::Response::success(data)
}
