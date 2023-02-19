use std::env;

use futures::TryStreamExt;
use mongodb::{Client, Collection, bson::doc};

use crate::models::point::Point;

#[derive(Clone)]
pub struct MongoRepo {
    pub points: Collection<Point>,
}

impl MongoRepo {
    pub async fn init() -> Self {
        dotenv::dotenv().ok();
        let uri = match env::var("MONGO_URI") {
            Ok(val) => val,
            Err(_) => format!("Could not find MONGO_URI in environment"),
        };

        let client = Client::with_uri_str(&uri).await.unwrap();
        let db = client.database("place");
        let points = db.collection("points");
        MongoRepo { points }
    }

    pub async fn get_all_points(&self) -> Vec<Point> {
        self.points
            .find(None, None)
            .await
            .unwrap()
            .try_collect::<Vec<Point>>()
            .await
            .unwrap()
    }

    pub async fn create_point(&self, point: Point) -> mongodb::results::InsertOneResult {
        self.points.insert_one(point, None).await.unwrap()
    }

    pub async fn update_point(&self, point: Point) -> mongodb::results::UpdateResult {
        self.points
            .update_one(
                doc! {"x": point.x, "y": point.y},
                doc! {"$set": {"color": point.color}},
                None,
            )
            .await
            .unwrap()
    }

    pub async fn create_or_update_point(&self, point: Point) {
        let result = self.points.find_one(doc! {"x": point.x, "y": point.y}, None).await.unwrap();

        if result.is_none() {
            self.points.insert_one(point, None).await.unwrap();
        }else {
            self.points
                .update_one(
                    doc! {"x": point.x, "y": point.y},
                    doc! {"$set": {"color": point.color}},
                    None,
                )
                .await
                .unwrap();
        }
    }
}
