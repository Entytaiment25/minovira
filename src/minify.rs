use axum::{
    body::{Body, Bytes},
    http::{Request, Response, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use http_body_util::BodyExt;
use minify_html::{Cfg, minify};

pub async fn html_minifier(
    req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let (parts, body) = next.run(req).await.into_parts();
    let response_bytes = response_buffer(body)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    let is_html = parts
        .headers
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .map(|ct| ct.contains("text/html"))
        .unwrap_or(false);

    let final_body = if is_html {
        let mut cfg = Cfg::new();
        cfg.allow_removing_spaces_between_attributes = true;
        cfg.minify_css = true;
        cfg.minify_js = true;
        cfg.remove_bangs = true;
        cfg.remove_processing_instructions = true;
        cfg.keep_comments = false;

        Bytes::from(minify(&response_bytes, &cfg))
    } else {
        response_bytes
    };

    let response = Response::from_parts(parts, Body::from(final_body));
    Ok(response)
}

async fn response_buffer<B>(body: B) -> Result<axum::body::Bytes, String>
where
    B: axum::body::HttpBody<Data = axum::body::Bytes>,
    B::Error: std::fmt::Display,
{
    let bytes = match body.collect().await {
        Ok(collected) => collected.to_bytes(),
        Err(err) => {
            return Err(format!("failed to read response body: {err}"));
        }
    };
    Ok(bytes)
}
