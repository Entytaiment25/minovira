use askama::Template;
use axum::{
    Router,
    extract::Path,
    http::{StatusCode, header},
    middleware,
    response::{Html, IntoResponse},
    routing::get,
};
use comrak::{ComrakOptions, markdown_to_html};
use std::collections::HashMap;
use tower_http::compression::CompressionLayer;
mod minify;
use minify::html_minifier;

#[derive(Template)]
#[template(path = "root.html")]
struct RootTemplate {
    title: &'static str,
    myownvalue: &'static str,
    testvalue: &'static str,
}

async fn root() -> impl IntoResponse {
    let root = RootTemplate {
        title: "title",
        myownvalue: "Hello from AXUM+ASKAMA+HTMX",
        testvalue: "meow",
    };
    (StatusCode::OK, Html(root.render().unwrap()))
}

async fn welcome() -> impl IntoResponse {
    let welcome = WelcomePartialTemplate {
        myownvalue: "Hello from AXUM+ASKAMA+HTMX",
        testvalue: "meow",
    };
    (StatusCode::OK, Html(welcome.render().unwrap()))
}

#[derive(Template)]
#[template(path = "note.html")]
struct NotePartialTemplate<'a> {
    content: &'a str,
    title: &'static str,
}

#[derive(Template)]
#[template(path = "welcome.html")]
struct WelcomePartialTemplate {
    myownvalue: &'static str,
    testvalue: &'static str,
}

struct MarkdownConfig {
    title: &'static str,
}

fn get_markdown_configs() -> HashMap<String, MarkdownConfig> {
    let mut configs = HashMap::new();

    configs.insert(
        "1".to_string(),
        MarkdownConfig {
            title: "Introduction",
        },
    );

    configs.insert("2".to_string(), MarkdownConfig { title: "Features" });

    configs.insert(
        "3".to_string(),
        MarkdownConfig {
            title: "Data Showcase",
        },
    );

    configs.insert(
        "4".to_string(),
        MarkdownConfig {
            title: "Getting Started",
        },
    );

    configs
}

macro_rules! create_template {
    ($name:ident, $path:expr, with_vars) => {
        #[derive(Template)]
        #[template(path = $path)]
        struct $name {
            stringvalue: &'static str,
            vec_strings: Vec<&'static str>,
        }
    };
    ($name:ident, $path:expr, no_vars) => {
        #[derive(Template)]
        #[template(path = $path)]
        struct $name;
    };
}

create_template!(IntroductionTemplate, "markdown/example.md", with_vars);
create_template!(FeaturesTemplate, "markdown/page2.md", no_vars);
create_template!(DataShowcaseTemplate, "markdown/page3.md", no_vars);
create_template!(GettingStartedTemplate, "markdown/page4.md", with_vars);

fn load_markdown_content(page_id: &str) -> Result<String, Box<dyn std::error::Error>> {
    match page_id {
        "1" => {
            let processor = IntroductionTemplate {
                stringvalue: "Hello from Rust variables!",
                vec_strings: vec!["Rust", "Askama", "Markdown", "Integration"],
            };
            Ok(processor.render()?)
        }
        "2" => {
            let processor = FeaturesTemplate;
            Ok(processor.render()?)
        }
        "3" => {
            let processor = DataShowcaseTemplate;
            Ok(processor.render()?)
        }
        "4" => {
            let processor = GettingStartedTemplate {
                stringvalue: "Dynamic content loading works!",
                vec_strings: vec!["Dynamic", "Loading", "Automatic", "Discovery"],
            };
            Ok(processor.render()?)
        }
        _ => {
            let processor = IntroductionTemplate {
                stringvalue: "Hello from Rust variables!",
                vec_strings: vec!["Rust", "Askama", "Markdown", "Integration"],
            };
            Ok(processor.render()?)
        }
    }
}

async fn note_page(Path(page): Path<String>) -> impl IntoResponse {
    let processed_md = load_markdown_content(&page)
        .unwrap_or_else(|_| "Error loading markdown content".to_string());

    let mut options = ComrakOptions::default();
    options.extension.strikethrough = true;
    options.extension.tagfilter = false;
    options.extension.table = true;
    options.extension.autolink = true;
    options.extension.tasklist = true;
    options.extension.superscript = true;
    options.extension.header_ids = Some("".to_string());
    options.extension.footnotes = true;
    options.extension.description_lists = true;
    options.parse.smart = true;
    options.render.unsafe_ = true;
    options.render.hardbreaks = false;

    let html = markdown_to_html(&processed_md, &options);

    let configs = get_markdown_configs();
    let page_title = configs
        .get(&page)
        .map(|config| config.title)
        .unwrap_or("Unknown Page");

    let tpl = NotePartialTemplate {
        content: &html,
        title: page_title,
    };
    (StatusCode::OK, Html(tpl.render().unwrap()))
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    tracing_subscriber::fmt().init();
    const PORT: &str = "8080";

    tracing::info!("router initialized, now listening on port {}", PORT);

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{PORT}"))
        .await
        .unwrap();

    let app = Router::new()
        .merge(
            Router::new()
                .route(
                    "/output.css",
                    get(|| async {
                        let css = include_str!("../static/output.css");
                        (StatusCode::OK, [(header::CONTENT_TYPE, "text/css")], css)
                    }),
                )
                .route(
                    "/htmx.min.js",
                    get(|| async {
                        let js = include_str!("../static/htmx.min.js");
                        (
                            StatusCode::OK,
                            [(header::CONTENT_TYPE, "application/javascript")],
                            js,
                        )
                    }),
                )
                .route("/", get(root))
                .route("/welcome", get(welcome))
                .route("/note/{page}", get(note_page))
                .route("/note", get(|| note_page(Path("1".to_string()))))
                .layer(middleware::from_fn(html_minifier)),
        )
        .layer(CompressionLayer::new());

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
