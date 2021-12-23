use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::api::server::WebServer;
use crate::enc::MessageType;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EncryptRequest {
    identity: String,
    friend: String,
    subject: Option<String>,
    message: String,
    message_type: String,
}

#[derive(Deserialize, Serialize)]
pub struct DecryptRequest {
    identity: Option<String>,
    message: String,
}

impl WebServer {
    pub async fn encrypt(&self, body: web::Json<EncryptRequest>) -> impl Responder {
        let subject = body.subject.as_deref();
        let message_type = MessageType::from_str(body.message_type.as_str());
        if message_type.is_err() {
            return HttpResponse::BadRequest().body("Invalid message type");
        }

        let message_type =
            MessageType::from_str(&body.message_type).unwrap_or(MessageType::Unknown);
        match self.enc_it().encrypt(
            &body.identity,
            &body.friend,
            subject,
            message_type,
            &body.message,
        ) {
            Ok(message) => HttpResponse::Ok().content_type("text/plain").body(message),
            Err(err) => HttpResponse::InternalServerError().body(format!("{}", err)),
        }
    }

    pub async fn decrypt(&self, body: web::Json<DecryptRequest>) -> impl Responder {
        match self
            .enc_it()
            .decrypt(&body.message, body.identity.as_deref())
        {
            Ok(message) => HttpResponse::Ok().json(message),
            Err(err) => HttpResponse::InternalServerError().body(format!("{}", err)),
        }
    }
}
