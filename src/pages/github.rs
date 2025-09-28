use actix_web::{get, HttpResponse, Responder};
use crate::pages::layout;

#[get("/github")]
pub async fn github() -> impl Responder {
    let github_username = "notmeower"; 
    let github_url = format!("https://github.com/{}", github_username);

    let body = format!(r#"
    <style>
    .tile {{
        text-align: center;
        padding: 1rem;
        border-radius: 12px;
        background: rgba(255,255,255,0.05);
        transition: transform 0.2s, box-shadow 0.2s;
        display: inline-block;
        min-width: 200px;
        cursor: pointer;
    }}
    .tile:hover {{
        transform: scale(1.05);
        box-shadow: 0 6px 20px rgba(0,0,0,0.3);
    }}
    .tile img {{
        width: 64px;
        height: 64px;
        margin-bottom: 0.5rem;
        border-radius: 50%;
    }}
    .platform-name {{
        font-weight: bold;
        margin-bottom: 0.3rem;
    }}
    </style>

    <div class="card">
      <h1><i class="fa-brands fa-github"></i> GitHub</h1>
      <p class="muted">Explore my projects and repositories</p>
      <div class="grid">
        <a href="{url}" target="_blank" class="tile" style="text-decoration:none; color: white;">
          <img src="https://github.com/{username}.png" alt="GitHub Avatar">
          <div class="platform-name">{username}</div>
          <div>View my repositories</div>
        </a>
      </div>
    </div>
    "#,
    url = github_url,
    username = github_username);

    HttpResponse::Ok().content_type("text/html").body(layout("GitHub", &body))
}
