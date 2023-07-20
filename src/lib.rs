// use atrium_api::client::AtpServiceClient;
// use atrium_api::com::atproto::server::create_session::Input;
// use atrium_xrpc::client::reqwest::ReqwestClient;
// use std::sync::Arc;
// use bisky::atproto::{Client, ClientBuilder, UserSession};
// use bisky::bluesky::Bluesky;
// use bisky::lexicon::app::bsky::feed::Post;
// use url::Url as OrgUrl;
// use chrono;
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use worker::*;

// https://atproto.com/lexicons/com-atproto-server#comatprotoservercreatesession
#[derive(Deserialize)]
struct Session {
    did: String,
    handle: String,
    email: String,
    accessJwt: String,
    refreshJwt: String,
}

#[event(fetch)]
async fn main(req: Request, env: Env, ctx: Context) -> Result<Response> {
    let sec_val = env.secret("REQUEST_PATH")?.to_string();
    let req_path = format!("/{}", sec_val);

    let router = Router::new();

    router
        .post_async(&req_path, |mut req, ctx| async move {
            let username = ctx.secret("FULL_USERNAME")?.to_string();
            let password = ctx.secret("PASSWORD")?.to_string();
            let url = "https://bsky.social";
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

            let client = Client::new();
            let payload = json!({
                "identifier": username,
                "password": password
            });

            let result = client
                .post(format!("{url}/xrpc/com.atproto.server.createSession"))
                .json(&payload)
                .send()
                .await;

            let res = match result {
                Ok(val) => val,
                Err(_) => return Response::error("Bad Gateway", 502),
            };

            let Session {
                did,
                handle,
                email,
                accessJwt,
                refreshJwt,
            } = match res.json().await {
                Ok(val) => val,
                Err(_) => return Response::error("Bad Gateway", 502),
            };
            Response::ok(accessJwt)
        })
        .run(req, env)
        .await
}
