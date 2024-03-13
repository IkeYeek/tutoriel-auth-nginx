extern crate core;

use actix_web::{get, App, HttpResponse, HttpServer, HttpRequest, post, web};
use actix_web::cookie::Cookie;
use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    authed: bool,
    exp: i64
}

fn jwt_is_valid(jwt_cookie: &Cookie) -> bool {
    match decode::<Claims>(jwt_cookie.value(), &DecodingKey::from_secret("348?z&de7#FLTD75aXn$tzE7!PpscBKAhL8AGxjY".as_ref()), &Validation::default()) {
        Ok(decoded_token) => {
            decoded_token.claims.exp >= chrono::Utc::now().timestamp()
        },
        Err(e) => {
            println!("{e:?}");
            false
        },
    }
}

fn create_jwt() -> String {
    let claims = Claims {
        authed: true,
        exp: chrono::Utc::now().timestamp() + 3600 * 24 * 7,
    };
    encode(&Header::default(), &claims, &EncodingKey::from_secret("348?z&de7#FLTD75aXn$tzE7!PpscBKAhL8AGxjY".as_ref())).unwrap()
}

fn form() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(
        r#"<form action="/auth" method="post"><label for="password">Password:</label><input type="password" name="password" placeholder="password" /><input type="submit" value="auth" /></form>"#
    )
}

#[get("/")]
async fn test(req: HttpRequest) -> HttpResponse {
    let jwt_cookie = req.cookie("jwt");
    if let Some(jwt_cookie) = jwt_cookie {
        if jwt_is_valid(&jwt_cookie) {
            HttpResponse::Ok().body("you are authorized.")
        } else {
            let mut res = form();
            res.del_cookie("jwt");
            res
        }
    } else {
        form()
    }
}

#[get("/auth")]
async fn auth_get(req: HttpRequest) -> HttpResponse {
    let jwt_cookie = req.cookie("jwt");
    if let Some(jwt_cookie) = jwt_cookie {
        if jwt_is_valid(&jwt_cookie) {
            return HttpResponse::Ok().body("Ok.")
        } else {
            let mut res = HttpResponse::Forbidden().finish();
            res.del_cookie("jwt");  // Si le cookie existe mais n'est pas valide on le supprime
            return res;
        }
    }
    HttpResponse::Forbidden().finish()
}


#[derive(Deserialize, Debug)]
struct FormData {
    password: String,
}
#[post("/auth")]
async fn auth_post(req: HttpRequest, form: web::Form<FormData>) -> HttpResponse {
    //E5korCa56JAQBc?r!h!N&8LHyJ9#fd#jje#o5fKR  --- je le laisse en clair pour l'exemple
    if bcrypt::hash_with_salt(&form.password, 12, [0, 3, 6, 4, 3, 5, 6, 7, 4, 3, 1, 2, 5, 6, 7, 4]).unwrap().to_string() == "$2y$12$..KE/.KD/eaC.uCA/OWF/.EwJK/cWXtuIzU1xHN0AuAxfZm6Xv12C" {
        let jwt = create_jwt();
        let jwt_cookie = Cookie::build("jwt", jwt).domain(".localhost.dummy").finish();
        let mut res = HttpResponse::Ok().body("authed.");
        match res.add_cookie(&jwt_cookie) {
            Ok(_) => {
                res
            }
            Err(_) => {
                HttpResponse::InternalServerError().finish()
            }
        }
    } else {
        HttpResponse::Forbidden().finish()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(
        || App::new().service(test).service(auth_get).service(auth_post)
    ).bind(("0.0.0.0", 8080))?.run().await
}