use serde::Deserialize;

#[derive(Deserialize)]
pub struct Response {
    pub message: String,
    pub time: bson::DateTime,
}

pub async fn make_request<U: reqwest::IntoUrl>(
    client: &reqwest::Client,
    name: &str,
    url: U,
) -> anyhow::Result<Response> {
    let req_body = bson::to_vec(&bson::doc! {
        "name": name,
    })?;
    let resp_body = client
        .post(url)
        .header("Content-Type", crate::BSON_CONTENT_TYPE)
        .body(req_body)
        .send()
        .await?
        .bytes()
        .await?;

    Ok(bson::from_slice(&resp_body)?)
}
