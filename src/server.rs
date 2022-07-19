use actix_web::{
    error::{ErrorBadRequest, ErrorInternalServerError},
    App, HttpResponse, HttpServer,
};
use futures::StreamExt;
use serde::{Deserialize, Serialize};

const MAX_SIZE: usize = 262_144; // max payload size is 256k

#[derive(Deserialize)]
struct Request {
    name: String,
}

#[derive(Serialize)]
struct Response {
    message: String,
    time: bson::DateTime,
}

async fn greet(mut payload: actix_web::web::Payload) -> actix_web::Result<HttpResponse> {
    let mut body = actix_web::web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(actix_web::error::ErrorBadRequest("body overflow"));
        }
        body.extend_from_slice(&chunk);
    }

    let req: Request =
        bson::from_slice(&body).map_err(|err| ErrorBadRequest(format!("invalid BSON: {}", err)))?;

    let resp = Response {
        message: format!("Hello, {}.", req.name),
        time: bson::DateTime::now(),
    };
    bson::to_vec(&resp)
        .map_err(|_| ErrorInternalServerError("internal server error"))
        .map(|resp| {
            HttpResponse::Ok()
                .content_type(crate::BSON_CONTENT_TYPE)
                .body(resp)
        })
}

pub async fn server(port: u16) -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().route(
            "/greet",
            actix_web::web::post()
                .guard(actix_web::guard::Header(
                    "content-type",
                    crate::BSON_CONTENT_TYPE,
                ))
                .to(greet),
        )
    })
    .bind(("localhost", port))?
    .run()
    .await
}
