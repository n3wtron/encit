use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::api::server::WebServer;
use crate::enc::EncItImpl;
use crate::EncItPEM;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum KeyFormat {
    Pem,
    Hex,
    Base64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddFriendRequest {
    name: String,
    key_format: KeyFormat,
    public_key: String,
}

impl WebServer {
    pub async fn get_friends(&self) -> impl Responder {
        let friends = self
            .enc_it()
            .lock()
            .unwrap()
            .get_config()
            .friends()
            .to_vec();
        web::Json(friends)
    }

    pub async fn get_friend(&self, path: web::Path<String>) -> impl Responder {
        if let Some(friend) = self
            .enc_it()
            .lock()
            .unwrap()
            .get_config()
            .friend(path.into_inner().as_str())
        {
            HttpResponse::Ok().json(friend)
        } else {
            HttpResponse::NotFound().body("Friend not found")
        }
    }

    pub async fn add_friend(&self, body: web::Json<AddFriendRequest>) -> impl Responder {
        let enc_it = self.enc_it();
        let mut enc_it = enc_it.lock().expect("lock error");
        let public_key = match body.key_format {
            KeyFormat::Pem => EncItPEM::Pem(body.public_key.clone()),
            KeyFormat::Hex => EncItPEM::Hex(body.public_key.clone()),
            KeyFormat::Base64 => EncItPEM::Base64(body.public_key.clone()),
        };

        let new_config_res = enc_it
            .get_config()
            .add_friend(body.name.as_str(), &public_key);
        if let Err(err) = &new_config_res {
            return HttpResponse::InternalServerError().body(format!("{}", err));
        }
        let new_config = new_config_res.unwrap();
        if let Err(err) = &new_config.save() {
            return HttpResponse::InternalServerError().body(format!("{}", err));
        }
        *enc_it = Box::new(EncItImpl::new(new_config));
        HttpResponse::Created().finish()
    }
}
