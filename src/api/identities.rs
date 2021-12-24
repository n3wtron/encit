use crate::api::server::WebServer;
use crate::config::EncItIdentity;
use crate::enc::EncItImpl;
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct NewIdentityRequest {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IdentityResponse {
    name: String,
    public_key: String,
}

impl From<&EncItIdentity> for IdentityResponse {
    fn from(identity: &EncItIdentity) -> Self {
        IdentityResponse {
            name: identity.name().to_string(),
            public_key: String::from_utf8(identity.private_key().public_key_pem().unwrap())
                .unwrap(),
        }
    }
}

impl WebServer {
    pub async fn get_identities(&self) -> impl Responder {
        let identities: Vec<IdentityResponse> = self
            .enc_it()
            .lock()
            .unwrap()
            .get_config()
            .identities()
            .iter()
            .map(|id| id.into())
            .collect();
        web::Json(identities)
    }

    pub async fn get_identity(&self, path: web::Path<String>) -> impl Responder {
        if let Some(identity) = self
            .enc_it()
            .lock()
            .unwrap()
            .get_config()
            .identity(path.into_inner().as_str())
        {
            HttpResponse::Ok().json(IdentityResponse::from(identity))
        } else {
            HttpResponse::NotFound().body("Identity not found")
        }
    }

    pub async fn new_identity(&self, body: web::Json<NewIdentityRequest>) -> impl Responder {
        let enc_it = self.enc_it();
        let mut enc_it = enc_it.lock().expect("lock error");
        let new_config_res = enc_it.get_config().new_identity(body.name.as_str());
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
