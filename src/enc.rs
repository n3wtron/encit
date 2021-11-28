use josekit::jwe::{JweHeader, RSA_OAEP};
use josekit::jws::{JwsHeader, RS256};
use josekit::jwt::JwtPayload;
use josekit::{jwt, Value};
use log::debug;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use std::string::String;

use crate::config::{EncItConfig, EncItFriend, EncItIdentity};
use crate::errors::EncItError;

#[derive(Debug, Serialize, Deserialize)]
pub struct EncItMessage {
    sender: String,
    receiver: String,
    subject: Option<String>,
    payload: String,
    verified: bool,
}

#[allow(dead_code)]
impl EncItMessage {
    pub fn sender(&self) -> &str {
        &self.sender
    }
    pub fn receiver(&self) -> &str {
        &self.receiver
    }
    pub fn subject(&self) -> &Option<String> {
        &self.subject
    }
    pub fn payload(&self) -> &str {
        &self.payload
    }
    pub fn verified(&self) -> bool {
        self.verified
    }
}

pub fn encrypt(
    cfg: Rc<dyn EncItConfig>,
    identity: &str,
    friend: &str,
    subject: Option<&str>,
    message: &str,
) -> Result<String, EncItError> {
    let identity = cfg
        .identity(identity)
        .ok_or_else(|| EncItError::IdentityNotFound(identity.to_string()))?;
    let friend = cfg
        .friend(friend)
        .ok_or_else(|| EncItError::FriendNotFound(friend.to_string()))?;

    let jws = create_jws(message, identity)?;
    debug!("jws:{}", &jws);
    let jwe = create_jwe(subject, &jws, identity, friend)?;
    debug!("jwe:{}", &jwe);
    Ok(jwe)
}

fn create_jwe(
    subject: Option<&str>,
    message: &str,
    identity: &EncItIdentity,
    friend: &EncItFriend,
) -> Result<String, EncItError> {
    let friend_pub_key = friend.public_key().pem()?;
    let identity_pub_key = identity.private_key().public_key_pem_hex()?;
    let mut jwe_header = JweHeader::new();
    jwe_header.set_token_type("JWT");
    jwe_header.set_content_encryption("A128CBC-HS256");
    if let Some(subject) = subject {
        jwe_header.set_subject(subject);
    }
    jwe_header.set_claim("rcp", Some(Value::from(friend.public_key().hex_pem()?)))?;
    let mut payload = JwtPayload::new();
    payload.set_issuer(identity_pub_key);
    payload.set_claim("message", Some(Value::String(message.to_string())))?;
    let encrypter = RSA_OAEP.encrypter_from_pem(friend_pub_key)?;
    jwt::encode_with_encrypter(&payload, &jwe_header, &encrypter).map_err(|e| e.into())
}

fn create_jws(message: &str, identity: &EncItIdentity) -> Result<String, EncItError> {
    let identity_priv_key = identity.private_key().pem()?;
    debug!(
        "signing with private key:{}",
        String::from_utf8(identity_priv_key.clone()).unwrap()
    );
    let mut jws_header = JwsHeader::new();
    jws_header.set_token_type("JWT");

    let mut payload = JwtPayload::new();
    payload.set_claim("message", Some(message.into()))?;

    let signer = RS256.signer_from_pem(identity_priv_key)?;
    jwt::encode_with_signer(&payload, &jws_header, &signer).map_err(|e| e.into())
}

pub fn decrypt(
    cfg: Rc<dyn EncItConfig>,
    jwe: &str,
    identity: Option<&str>,
) -> Result<EncItMessage, EncItError> {
    let identity = if let Some(identity_name) = identity {
        cfg.identity(identity_name)
    } else {
        let header = jwt::decode_header(jwe)?;
        let receiver_public_key = header.claim("rcp").unwrap().as_str().unwrap();
        cfg.identity_by_public_key_hex(receiver_public_key)
    }
    .ok_or_else(|| EncItError::IdentityNotFound(String::new()))?;
    debug!("Identity found:{}", identity.name());

    let (payload, header) = extract_jwe(jwe, identity)?;

    let friend = payload
        .issuer()
        .and_then(|friend_pub_key_hex| cfg.friend_by_public_key_hex(friend_pub_key_hex))
        .ok_or_else(|| {
            EncItError::FriendNotFound(
                "cannot find a friend that match with the message public key".to_string(),
            )
        })?;

    let (verified, message) = extract_jws(payload.claim("message").unwrap().as_str(), friend)?;

    Ok(EncItMessage {
        sender: friend.name().to_string(),
        receiver: identity.name().to_string(),
        subject: header.subject().map(|s| s.to_string()),
        payload: message,
        verified,
    })
}

fn extract_jwe(jwe: &str, identity: &EncItIdentity) -> Result<(JwtPayload, JweHeader), EncItError> {
    let decrypter = RSA_OAEP.decrypter_from_pem(identity.private_key().pem()?)?;
    jwt::decode_with_decrypter(jwe, &decrypter).map_err(|e| e.into())
}

fn extract_jws(jws: Option<&str>, friend: &EncItFriend) -> Result<(bool, String), EncItError> {
    let jws = jws.ok_or_else(EncItError::EmptyMessage)?;
    debug!("extract jws :{}", jws);
    let friend_public_key = friend.public_key().pem()?;
    debug!(
        "verifying with friend public key:{}",
        String::from_utf8(friend_public_key.clone()).unwrap()
    );
    let verifier = RS256.verifier_from_pem(friend_public_key)?;
    jwt::decode_with_verifier(jws, &verifier)
        .map(|(payload, _)| {
            (
                true,
                payload
                    .claim("message")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .to_string(),
            )
        })
        .map_err(|e| e.into())
}

#[cfg(test)]
mod tests {
    use mockall::predicate::eq;
    use openssl::pkey::Private;
    use openssl::rsa::Rsa;

    use crate::config::{EncItFriend, EncItIdentity, EncItPEM, EncItPrivateKey, MockEncItConfig};

    use super::*;

    #[test]
    fn encrypt_decrypt_payload() -> Result<(), EncItError> {
        env_logger::init();
        let friend_name = "friend-1";
        let (priv_key, friend) = generate_friend(friend_name);
        let (_, identity) = generate_identity(friend_name, Some(priv_key));
        let friend = Box::leak(friend);
        let identity = Box::leak(identity);
        let identity_public_key = Box::leak(Box::new(
            identity.private_key().public_key_pem_hex().unwrap(),
        ));
        let mut cfg_mock = MockEncItConfig::new();
        cfg_mock
            .expect_friend()
            .with(eq(friend_name))
            .returning(|_f| Some(friend));
        cfg_mock
            .expect_identity()
            .with(eq(friend_name))
            .returning(|_f| Some(identity));
        cfg_mock
            .expect_identity_by_public_key_hex()
            .with(eq(identity_public_key.as_str()))
            .returning(|_f| Some(identity));
        cfg_mock
            .expect_friend_by_public_key_hex()
            .with(eq(identity_public_key.as_str()))
            .returning(|_f| Some(friend));

        let cfg: Rc<dyn EncItConfig> = Rc::new(cfg_mock);
        let plain_message = "hello";

        let enc_msg = encrypt(
            cfg.clone(),
            friend_name,
            friend_name,
            Some("subject"),
            plain_message,
        )?;

        let decrypted = decrypt(cfg, &enc_msg, None);
        //
        let message = decrypted?;
        assert_eq!(message.payload, plain_message);
        assert!(message.verified);
        assert_eq!(message.sender, identity.name());
        assert_eq!(message.receiver, friend.name());

        Ok(())
    }

    fn generate_friend(friend_name: &str) -> (Rsa<Private>, Box<EncItFriend>) {
        let friend_priv_key = Rsa::generate(2048).unwrap();
        let friend_pub_key = friend_priv_key.public_key_to_pem().unwrap();
        let friend_pub_key_b64 = base64::encode(friend_pub_key);
        let friend = Box::new(EncItFriend::new(
            friend_name.to_string(),
            EncItPEM::Base64(friend_pub_key_b64),
        ));
        (friend_priv_key, friend)
    }

    fn generate_identity(
        identity_name: &str,
        key: Option<Rsa<Private>>,
    ) -> (Rsa<Private>, Box<EncItIdentity>) {
        let identity_pair = key.unwrap_or_else(|| Rsa::generate(2048).unwrap());
        let identity_pub_key = identity_pair.private_key_to_pem().unwrap();
        let identity_pub_key_b64 = base64::encode(identity_pub_key);
        let identity_priv_key = EncItPrivateKey::new(EncItPEM::Base64(identity_pub_key_b64), None);
        let identity = Box::new(EncItIdentity::new(
            identity_name.to_string(),
            identity_priv_key,
        ));
        (identity_pair, identity)
    }
}
