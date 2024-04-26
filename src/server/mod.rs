use actix_web::{
    web::{self, Data},
    App, HttpServer,
};

use crate::{
    middleware::UserAuth,
    router,
    service::{ActivityService, UserService},
};

pub struct Server;

impl Server {
    pub fn new() -> Self {
        Self {}
    }
}
impl Server {
    pub async fn start(&self) -> Result<(), std::io::Error> {
        fast_log::init(
            fast_log::Config::new()
                .console()
                .level(log::LevelFilter::Debug),
        )
        .expect("rbatis init fail");

        HttpServer::new(move || {
            App::new()
                .app_data(Data::new(UserService::new()))
                .app_data(Data::new(ActivityService::new()))
                .service(
                    web::scope("/api")
                        .wrap(UserAuth)
                        .configure(router::register_routes),
                )
        })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
    }
}
