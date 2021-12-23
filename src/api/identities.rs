use crate::api::server::WebServer;
use actix_web::{web, HttpResponse, Responder};

impl WebServer {
    pub async fn get_identities(&self) -> impl Responder {
        let identities = self.enc_it().get_config().identities().to_vec();
        web::Json(identities)
    }

    pub async fn get_identity(&self, path: web::Path<String>) -> impl Responder {
        if let Some(identity) = self
            .enc_it()
            .get_config()
            .identity(path.into_inner().as_str())
        {
            HttpResponse::Ok().json(identity)
        } else {
            HttpResponse::NotFound().body("Identity not found")
        }
    }
}
