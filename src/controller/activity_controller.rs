use actix_web::{
    get, post,
    web::{Data, Json},
};

use crate::{
    model::Activity,
    response::{self, result::Response},
    service::ActivityService,
};

#[post("/new")]
pub async fn new(us: Data<ActivityService>, Json(activity): Json<Activity>) -> Response {
    let activity_service = us.get_ref();
    let data = activity_service.new_activity(&activity).await?;
    response::Response::success(data)
}

#[get("/list")]
pub async fn list(us: Data<ActivityService>) -> Response {
    let activity_service = us.get_ref();
    let activities = activity_service.get_all().await?;
    response::Response::success(activities)
}
