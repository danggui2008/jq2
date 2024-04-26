use crate::{dao::db, model::Activity, response::result::Result};

impl_insert!(Activity {});
impl_select!(Activity {});

pub struct ActivityDao;

impl ActivityDao {
    pub fn new() -> Self {
        Self {}
    }
    pub async fn new_activity(&self, activity: &Activity) -> Result<bool> {
        let result = Activity::insert(db::DB.get_rb_ref(), activity).await?;
        Ok(result.rows_affected > 0)
    }

    pub async fn list(&self) -> Result<Vec<Activity>> {
        Ok(Activity::select_all(db::DB.get_rb_ref()).await?)
    }
}
