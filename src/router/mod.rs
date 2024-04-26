mod router;
pub(crate) use router::register_routes;

mod user_router;
pub(crate) use user_router::UserRouter;

mod activity_router;
pub(crate) use activity_router::ActivityRouter;
