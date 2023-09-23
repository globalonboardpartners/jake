use crate::data_types::structs::Auth;
use crate::db;
use crate::utils::handle_sql_error;
use actix_web::http::StatusCode;
use actix_web::web::Json;
use actix_web::{web, http, post, HttpRequest, HttpResponse};
use sqlx::Row;


use actix_session::{storage::CookieSessionStore, Session, SessionMiddleware};
use actix_web::{
    cookie::{Key, SameSite},
    error::InternalError,
    middleware, Error, Responder,
};
use serde::{Deserialize, Serialize};
use std::result::Result as StdResult;

use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

#[post("/auth")]
async fn login(session: Session, req: HttpRequest, auth: Json<Auth>) -> HttpResponse {


    #[derive(Serialize, Deserialize)]
    struct Credentials {
        username: String,
        password: String,
    }

    
    #[derive(Serialize, Deserialize)]
    struct User {
        id: i64,
        username: String,
        password: String,
    }

    
    impl User {
        fn authenticate(credentials: Credentials) -> StdResult<Self, String> {
            // to do: figure out why I keep getting hacked      /s
            if &credentials.password != "hunter2" {
                return Err(String::from("Unauthorized"));
            }
    
            Ok(User {
                id: 42,
                username: credentials.username,
                password: credentials.password,
            })
        }
    }

    
    pub fn validate_session(session: &Session) -> StdResult<i64, &str> {
        let user_id: Option<i64> = session.get("user_id").unwrap_or(None);
    
        match user_id {
            Some(id) => {
                // keep the user's session alive
                session.renew();
                Ok(id)
            }
            None => Err("Unauthorized")
        }
    }

    
    fn log_me_in(
        credentials: Credentials,
        session: Session,
    ) -> StdResult<String, String> {
        // let credentials = credentials.into_inner();
    
        match User::authenticate(credentials) {
            Ok(user) => session.insert("user_id", user.id).unwrap(),
            Err(err) => return Err(String::from("error in log me in")),
        };
    
        Ok(String::from("Welcome!"))
    }
    

    /// some protected resource
    fn secret(session: Session) -> StdResult<String, String> {
        // only allow access to this resource if the user has an active session
        validate_session(&session)?;
    
        Ok(String::from("secret revealed"))
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////

    let my_user = User { id: 1, username: String::from("harry"), password: String::from("hunter2") };
    let my_creds = Credentials { username: String::from("harry"), password: String::from("hunter2") };

    // log_me_in(my_creds, session.clone());

    // secret(session);
    let lmi = log_me_in(my_creds, session.clone());
    let secr = secret(session);

    dbg!(lmi);
    dbg!(secr);

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    let password = b"hunter42"; // Bad password; don't actually use!
    let salt = SaltString::generate(&mut OsRng);
    
    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();
    
    // Hash password to PHC string ($argon2id$v=19$...)
    let password_hash = argon2.hash_password(password, &salt).unwrap().to_string();
    dbg!(&password_hash);
    
    // Verify password against PHC string.
    //
    // NOTE: hash params from `parsed_hash` are used instead of what is configured in the
    // `Argon2` instance.
    let parsed_hash = PasswordHash::new(&password_hash).unwrap();
    dbg!(parsed_hash);
    match db::connect(req).await {
        Ok(pg) => {
            let returned = sqlx::query(
                "
                    SELECT EXISTS (
                        SELECT 1
                        FROM auth
                        WHERE username = $1 AND password = $2
                        LIMIT 1
                    );
                "
            )
            .bind(&auth.username)
            .bind(&auth.password)
            .fetch_one(&pg)
            .await;

            match returned {
                Ok(record) => {
                    HttpResponse::Created()
                    .status(StatusCode::CREATED)
                    .content_type("application/json")
                    .body(
                        serde_json::to_string(&Json(record.get::<bool, usize>(0)))
                            .unwrap_or_else(|e| format!("JSON serialization error: {}", e)),
                    )},

                Err(e) => {
                    handle_sql_error(e)
                }
            }
        }
        Err(e) => HttpResponse::InternalServerError()
            .status(http::StatusCode::INTERNAL_SERVER_ERROR)
            .content_type("application/json")
            .body(e.message),
    }
}

