use actix_cors::Cors;
use std::sync::Arc;

use actix_web::rt::System;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

use crate::enc::EncIt;
use crate::EncItError;

#[cfg(feature = "ui")]
include!(concat!(env!("OUT_DIR"), "/generated.rs"));

pub struct WebServer {
    host: String,
    enc_it: Arc<dyn EncIt>,
}

impl WebServer {
    pub fn new(host: &str, enc_it: Arc<dyn EncIt>) -> Self {
        WebServer {
            host: host.to_string(),
            enc_it,
        }
    }

    async fn health() -> impl Responder {
        HttpResponse::Ok().body("alive")
    }

    pub async fn start(&'static self) -> Result<(), EncItError> {
        let sys = System::new("encit-web-service");
        HttpServer::new(|| {
            #[cfg(feature = "ui")]
            let generated = generate();
            let cors = Cors::default()
                .allow_any_origin()
                .allow_any_header()
                .allow_any_method()
                .max_age(3600);

            let mut app = App::new()
                .wrap(cors)
                .data(web::JsonConfig::default().limit(1024 * 1024 * 50))
                .route("/health", web::get().to(Self::health))
                .route("/v1/identities", web::get().to(|| self.get_identities()))
                .route(
                    "/v1/identities/{identity}",
                    web::get().to(|path| self.get_identity(path)),
                )
                .route("/v1/friends", web::get().to(|| self.get_friends()))
                .route(
                    "/v1/friends/{friend}",
                    web::get().to(|path| self.get_friend(path)),
                )
                .route("/v1/encrypt", web::post().to(|body| self.encrypt(body)))
                .route("/v1/decrypt", web::post().to(|body| self.decrypt(body)));
            #[cfg(feature = "ui")]
            {
                app = app.service(
                    actix_web_static_files::ResourceFiles::new("/", generated)
                        .resolve_not_found_to_root(),
                );
            }
            app
        })
        .bind(&self.host)
        .map_err(|e| EncItError::WebError(e.to_string()))?
        .run();

        sys.run().map_err(|e| EncItError::WebError(e.to_string()))
    }

    pub fn enc_it(&self) -> Arc<dyn EncIt> {
        self.enc_it.clone()
    }
}
