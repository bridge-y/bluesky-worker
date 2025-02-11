use html2text::{from_read_with_decorator, render::RichDecorator};
use regex::Regex;
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
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

#[derive(Serialize)]
struct FacetsMain {
    index: ByteSlice,
    features: Vec<FeatureItem>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ByteSlice {
    byte_start: i32,
    byte_end: i32,
}

#[derive(Serialize)]
#[serde(tag = "$type")]
enum FeatureItem {
    #[serde(rename = "app.bsky.richtext.facet#mention")]
    Mention(Mention),
    #[serde(rename = "app.bsky.richtext.facet#link")]
    Link(Link),
    #[serde(rename = "app.bsky.richtext.facet#tag")]
    Tag(Tag),
}

#[derive(Serialize)]
struct Mention {
    did: String,
}

#[derive(Serialize)]
struct Link {
    uri: String,
}

#[derive(Serialize)]
struct Tag {
    tag: String,
}

fn is_html(input: &str) -> bool {
    // simple regex to check html format
    let re = Regex::new(r"<[^>]*>").unwrap();
    re.is_match(input)
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

            let Content { mut text } = match req.json().await {
                Ok(val) => val,
                Err(_) => return Response::error("Bad reqest", 400),
            };

            // extract plain text
            if is_html(&text) {
                console_log!("This is html!");
                text = from_read_with_decorator(text.as_bytes(), 500, RichDecorator::new())
                    .expect("fail to get text.");
            }
            console_log!("{text}");

            // expected plain text
            let factes = make_facets(&text);

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
            create_record(&access_jwt, &base_url, &handle, &text, factes).await
        })
        .run(req, env)
        .await
}

async fn create_record(
    token: &str,
    base_url: &str,
    handle: &str,
    text: &str,
    facets: Vec<FacetsMain>,
) -> Result<Response> {
    let url = format!("{base_url}/xrpc/com.atproto.repo.createRecord");
    let payload = json!({
        "repo": handle,
        "collection": "app.bsky.feed.post",
        "record": {
            "text": text.to_string().trim(),
            "facets": Some(facets),
            "createdAt": format!("{:?}", chrono::Utc::now()),
        },
    });

    let client = Client::new();
    let result = client
        .post(url)
        .json(&payload)
        .bearer_auth(token)
        .send()
        .await;

    match result {
        Ok(response) => match response.status() {
            StatusCode::OK => Response::ok(""),
            _ => Response::error(
                response
                    .status()
                    .canonical_reason()
                    .expect("fail to get reason phrase")
                    .to_string(),
                response.status().as_u16(),
            ),
        },
        Err(_) => Response::error("Internal Server Error", 500),
    }
}

fn make_facets(text: &str) -> Vec<FacetsMain> {
    // url
    let url_pattern = r"https?://\S+";
    let url_regex = Regex::new(url_pattern).unwrap();

    let mut facets: Vec<FacetsMain> = vec![];
    for mat in url_regex.find_iter(text) {
        let (start, end) = (mat.start() as i32, mat.end() as i32);
        let matched_text = mat.as_str();

        facets.push(FacetsMain {
            index: ByteSlice {
                byte_start: start,
                byte_end: end,
            },
            features: vec![FeatureItem::Link(Link {
                uri: matched_text.to_string(),
            })],
        })
    }

    // tag
    let tag_pattern = r"#\S+";
    let tag_regex = Regex::new(tag_pattern).unwrap();

    for mat in tag_regex.find_iter(text) {
        let (start, end) = (mat.start() as i32, mat.end() as i32);
        let matched_text = mat.as_str();

        facets.push(FacetsMain {
            index: ByteSlice {
                byte_start: start,
                byte_end: end,
            },
            features: vec![FeatureItem::Tag(Tag {
                tag: matched_text
                    .to_string()
                    .strip_prefix('#')
                    .expect("no prefix")
                    .to_string(),
            })],
        })
    }

    facets
}
