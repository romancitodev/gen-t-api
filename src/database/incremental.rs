use bson::doc;
use mongodb::{options::FindOneAndUpdateOptions, Collection};
use serde::{Deserialize, Serialize};

/// Represents a Collection.
#[derive(Debug, Deserialize, Serialize)]
pub struct AutoIncrement {
    pub collection: String,
    pub seq: i32,
}

impl AutoIncrement {
    pub async fn get_next_id(
        &mut self,
        collection: Collection<AutoIncrement>,
    ) -> Result<i32, String> {
        let filter = doc! { "collection": &self.collection };
        let update = doc! { "$inc": { "seq" : 1 } };
        let options = FindOneAndUpdateOptions::builder()
            .return_document(mongodb::options::ReturnDocument::After)
            .upsert(true)
            .build();
        let id = collection
            .find_one_and_update(filter, update, options)
            .await;
        match id {
            Ok(Some(doc)) => {
                self.seq = doc.seq;
                Ok(self.seq)
            }
            Ok(None) => Err("Autoincrement not founded".to_string()),
            Err(mongo) => Err(format!("MongoDB error: {mongo}")),
        }
    }
}
