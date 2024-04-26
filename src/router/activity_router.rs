use actix_web::{web, Scope};

use crate::controller::user_controller;


pub(crate)struct ActivityRouter{}

impl ActivityRouter{
    //活动模块路由
    pub fn get_router()->Scope{
        web::scope("/user")
        .service(user_controller::register)
        .service(user_controller::login)
        .service(user_controller::logout)
        .service(user_controller::list)
        .service(user_controller::edit_info)
        .service(user_controller::lock_user)
    }
}