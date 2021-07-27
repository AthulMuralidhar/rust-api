use actix_web::web::{Json, Path};
use actix_web::HttpResponse;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Debug, Deserialize, Serialize)]
pub struct Response<T> {
    pub results: Vec<T>
}

pub type Tweets = Response<Tweet>;

pub const APPLICATION_JSON: &str = "application/json";

#[derive(Debug, Deserialize, Serialize)]
pub struct Tweet {
    id: String,
    created_at: DateTime<Utc>,
    message: String,
}

impl Tweet {
    pub fn new(message: String) -> Self{
        Self{
            id: Uuid::new_v4().to_string(),
            created_at: Utc::now(),
            message
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TweetRequest {
    message: Option<String>
}

impl TweetRequest {
    pub fn to_tweet(&self) -> Option<Tweet> {
        match &self.message{
            Some(message) => Some(Tweet::new(message.to_string())),
            None => None
        }
    }
}


#[get("/tweets")]
pub async fn list() -> HttpResponse {
    let tweets = Tweets { results: vec![]};

    HttpResponse::Ok().content_type(APPLICATION_JSON).json(tweets)
}

#[post("/tweets")]
pub async fn create(tweet: Json<TweetRequest>) -> HttpResponse {
    HttpResponse::Created().content_type(APPLICATION_JSON).json(tweet.to_tweet())
}

#[delete("/tweets/{id}")]
pub async fn delete(_path: Path<String>) -> HttpResponse {
    HttpResponse::NoContent().content_type(APPLICATION_JSON).await.unwrap()
}