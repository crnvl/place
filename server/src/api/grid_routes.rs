use actix_web::{
    get, post,
    web::{Data, Json},
};
use mongodb::bson::doc;

use crate::{models::point::Point, mongo_db::MongoRepo};

#[get("/grid")]
async fn get_grid(db: Data<MongoRepo>) -> Json<Vec<Point>> {
    Json(db.get_all_points().await)
}

#[post("/grid/point")]
async fn post_grid(db: Data<MongoRepo>, body: Json<Point>) -> Json<String> {
    let existing = db
        .points
        .find_one(doc! {"x": body.x, "y": body.y}, None)
        .await
        .unwrap();

    let point = Point {
        x: body.x,
        y: body.y,
        color: body.color,
    };

    if existing.is_some() {
        db.update_point(point).await;
        return Json("Point updated".to_string());
    }

    db.create_point(point).await;
    Json("Point created".to_string())
}
