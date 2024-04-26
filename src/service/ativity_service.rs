use crate::{dao::ActivityDao, model::Activity, response::result::Result};

pub struct ActivityService {
    dao: ActivityDao,
}

impl ActivityService {
    pub fn new() -> Self {
        Self {
            dao: ActivityDao::new(),
        }
    }
}

impl ActivityService {
    pub async fn new_activity(&self, activity: &Activity) -> Result<bool> {
        let result = self.dao.new_activity(activity).await?;
        //do something
        Ok(result)
    }

    pub async fn get_all(&self) -> Result<Vec<Activity>> {
        let result = self.dao.list().await?;
        //do something
        Ok(result)
    }
}
