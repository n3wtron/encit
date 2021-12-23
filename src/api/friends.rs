use crate::api::server::WebServer;
use actix_web::{web, HttpResponse, Responder};

impl WebServer {
    pub async fn get_friends(&self) -> impl Responder {
        let friends = self.enc_it().get_config().friends().to_vec();
        web::Json(friends)
    }

    pub async fn get_friend(&self, path: web::Path<String>) -> impl Responder {
        if let Some(friend) = self
            .enc_it()
            .get_config()
            .friend(path.into_inner().as_str())
        {
            HttpResponse::Ok().json(friend)
        } else {
            HttpResponse::NotFound().body("Friend not found")
        }
    }
}
