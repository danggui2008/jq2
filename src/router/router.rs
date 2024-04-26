use actix_web::web::{self, ServiceConfig};

use super::{ActivityRouter, UserRouter};

//总路由注册
pub(crate) fn register_routes(sc: &mut ServiceConfig) {
    sc.service(
        web::scope("/v1")
            .service(
              UserRouter::get_router(),
            )
            .service(
                ActivityRouter::get_router(),
            ),
    );
}
