use std::{
    future::{ready, Ready},
    rc::Rc,
};

use actix_web::{
    body::BoxBody, dev::{self, Service, ServiceRequest, ServiceResponse, Transform}, error::ErrorInternalServerError, http::StatusCode, web::Payload, Error, FromRequest, HttpMessage, ResponseError
};
use futures_util::future::LocalBoxFuture;
use serde::Serialize;

use crate::{
    common::{constant, error::ApplicationError, runtime::CRUNTIME},
    dao::{UserDao, UserTokenDao},
    model::User,
};

const USER_IGNORE_ROUTES: [&str; 2] = [
    "/api/v1/user/login",
    "/api/v1/user/register",
];

#[derive(Debug, Clone,Serialize)]
pub struct UserIdentity {
    pub user: User,
}

impl FromRequest for UserIdentity {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;
    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        ready(match req.extensions().get::<UserIdentity>() {
            Some(identity) => Ok(identity.clone()),
            None => Err(ErrorInternalServerError("not identity")),
        })
    }
}

pub struct UserAuth;

impl<S> Transform<S, ServiceRequest> for UserAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Transform = UserAuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;
    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(UserAuthMiddleware {
            svc: Rc::new(service),
        }))
    }
}

pub struct UserAuthMiddleware<S> {
    svc: Rc<S>,
}

impl<S> Service<ServiceRequest> for UserAuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;

    type Error = Error;

    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(svc);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let error_response =
            |req: ServiceRequest, status: StatusCode, message: String| -> Self::Future {
                Box::pin(async move {
                    let error = ApplicationError { status, message };
                    Ok(req.error_response(error))
                })
            };
        let next = |req: ServiceRequest| -> Self::Future {
            let future = self.svc.call(req);
            Box::pin(async move { future.await })
        };

        for ignore_route in USER_IGNORE_ROUTES {
            if req.path().starts_with(ignore_route) {
                return next(req);
            }
        }

        if let Some(token) = req.headers().get("token") {
            if !token.is_empty() {
                let token = token.to_str();
                if let Ok(token) = token {
                    let token = token.to_string();
                    //这里不应该这样，暂时先这样
                    let user_token = futures::executor::block_on(async move {
                        CRUNTIME
                            .spawn(async move {
                                let user_token_dao = UserTokenDao::new();
                                user_token_dao.find_by_token(&token).await
                            })
                            .await
                            .unwrap()
                    });
                    if let Ok(mut user_token) = user_token {
                        if !user_token.is_empty() {
                            let user_token = user_token.pop().unwrap();
                            if user_token.expire_time_check() {
                                return error_response(
                                    req,
                                    StatusCode::RANGE_NOT_SATISFIABLE,
                                    "认证已过期，请重新登录".into(),
                                );
                            }
                            let user = futures::executor::block_on(async move {
                                CRUNTIME
                                    .spawn(async move {
                                        let user_dao = UserDao::new();
                                        user_dao.find_by_id(user_token.user_id).await
                                    })
                                    .await
                                    .unwrap()
                            });
                            if let Ok(user) = user {
                                println!("user_flag{}",user.locked_flag);
                                if user.locked_flag == constant::LOCKED {
                                    return error_response(
                                        req,
                                        StatusCode::RANGE_NOT_SATISFIABLE,
                                        "用户已被禁用".into(),
                                    );
                                }
                                //如果要支持角色权限方式可以在这里进行扩展：
                                //1)根据用户查询当前用户拥有哪权限（权限信息中有url信息）
                                //2)req.path()与权限进行匹配，再决定是

                                req.extensions_mut().insert(UserIdentity { user });
                                return next(req);
                            }
                        }
                    }
                }
            }
        }
        error_response(req, StatusCode::RANGE_NOT_SATISFIABLE, "未登录".into())
    }
}
