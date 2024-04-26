#[macro_use]
extern crate rbatis;

//配制模块：负责配制解析
pub(crate) mod config;

//server模块
pub(crate) mod server;
pub use server::Server;

//路由模块：路由注册，负责路由与controller关系配置
mod router;

//controller模块,相当于spring中的controller
pub(crate) mod controller;

//服务模块
pub(crate) mod service;

//模型：vo,po,dto等
pub(crate) mod model;

//dao模块
pub(crate) mod dao;

//中间件模块
pub(crate) mod middleware;

//响应封装
pub(crate) mod response;

//error,常量等
pub(crate) mod common;
//util
pub(crate) mod util;
