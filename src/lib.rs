// use atrium_api::client::AtpServiceClient;
// use atrium_api::com::atproto::server::create_session::Input;
// use atrium_xrpc::client::reqwest::ReqwestClient;
// use std::sync::Arc;
// use bisky::atproto::{Client, ClientBuilder, UserSession};
// use bisky::bluesky::Bluesky;
// use bisky::lexicon::app::bsky::feed::Post;
// use url::Url as OrgUrl;
use chrono;
use reqwest::{Client, StatusCode};
use serde::Deserialize;
use serde_json::json;
use worker::*;

// https://atproto.com/lexicons/com-atproto-server#comatprotoservercreatesession
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Session {
    did: String,
    handle: String,
    email: String,
    access_jwt: String,
    refresh_jwt: String,
}

#[derive(Deserialize)]
struct Content {
    text: String,
}

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let sec_val = env.secret("REQUEST_PATH")?.to_string();
    let req_path = format!("/{}", sec_val);

    let router = Router::new();

    router
        .post_async(&req_path, |mut req, ctx| async move {
            let username = ctx.secret("FULL_USERNAME")?.to_string();
            let password = ctx.secret("PASSWORD")?.to_string();
            let base_url = "https://bsky.social";
            //
            // let client =
            //     AtpServiceClient::new(Arc::new(ReqwestClient::new("https://bsky.social".into())));
            // let result = client
            //     .com
            //     .atproto
            //     .server
            //     .create_session(Input {
            //         identifier: username.clone().into(),
            //         password: password.clone().into(),
            //     })
            //     .await;

            // let mut client = ClientBuilder::default().session(None).build().unwrap();
            // client.login(&url, &username, &password).await;
            // let mut bsky = Bluesky::new(client);
            // let result = bsky
            //     .me()
            //     .unwrap()
            //     .post(Post {
            //         text: "test".into(),
            //         created_at: chrono::UTC::now(),
            //     })
            //     .await
            //     .unwrap();
            let Content { text } = match req.json().await {
                Ok(val) => val,
                Err(_) => return Response::error("Bad reqest", 400),
            };

            let client = Client::new();

            // https://atproto.com/lexicons/com-atproto-server#comatprotoservercreatesession
            let payload = json!({
                "identifier": username,
                "password": password
            });

            let result = client
                .post(format!("{base_url}/xrpc/com.atproto.server.createSession"))
                .json(&payload)
                .send()
                .await;

            let res = match result {
                Ok(val) => val,
                Err(_) => return Response::error("Bad Gateway", 502),
            };

            let Session {
                did: _,
                handle,
                email: _,
                access_jwt,
                refresh_jwt: _,
            } = match res.json().await {
                Ok(val) => val,
                Err(_) => return Response::error("Bad Gateway", 502),
            };

            // https://atproto.com/lexicons/com-atproto-repo#comatprotorepocreaterecord
            create_record(&access_jwt, &base_url, &handle, &text).await
        })
        .run(req, env)
        .await
}

async fn create_record(token: &str, base_url: &str, handle: &str, text: &str) -> Result<Response> {
    let url = format!("{base_url}/xrpc/com.atproto.repo.createRecord");
    let payload = json!({
        "repo": handle,
        "collection": "app.bsky.feed.post",
        "record": {
            "text": text,
            "createdAt": format!("{:?}", chrono::Utc::now()),
        },
    });
    // Response::ok(format!("{:?}", payload))

    let client = Client::new();
    let result = client
        .post(url)
        .json(&payload)
        .bearer_auth(&token)
        .send()
        .await;

    match result {
        Ok(response) => match response.status() {
            StatusCode::OK => Response::ok(""),
            _ => Response::error("Bad Request", 400),
        },
        Err(_) => Response::error("Internal Server Error", 500),
    }
}
