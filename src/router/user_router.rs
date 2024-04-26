use actix_web::{web, Scope};

use crate::controller::activity_controller;


pub(crate) struct UserRouter{}

impl UserRouter {

    //用户模块路由
    pub fn get_router()->Scope{
        web::scope("/activity")
        .service(activity_controller::list)
        .service(activity_controller::new)
    }
    
}