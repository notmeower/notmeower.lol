use actix_web::{get, HttpResponse, Responder};
use crate::pages::layout;

#[get("/")]
pub async fn index() -> impl Responder {
    let body = r#"
    <style>
      .tile {
          text-decoration: none;
          display: flex;
          flex-direction: column;
          align-items: center;
          justify-content: center;
          padding: 1rem;
          border-radius: 14px;
          background: rgba(255,255,255,0.05);
          transition: transform 0.2s ease-in-out, box-shadow 0.2s ease-in-out;
          color: inherit;
      }
      .tile:hover {
          transform: scale(1.05);
          box-shadow: 0 6px 18px rgba(0,0,0,0.2);
          text-decoration: none;
      }
      a.tile:hover {
          text-decoration: none;
      }
      .muted {
          color: rgba(255,255,255,0.6);
          margin: 0.5rem 0;
      }
      .tile i {
          font-size: 2.5rem;
          margin-bottom: 0.5rem;
      }
    </style>

    <div class="card">
      <h1>My Digital Corner</h1>
      <p class="muted">
        Software developer | Crypto enthusiast | Privacy advocate
      </p>
      <p class="muted">
        Explore my wallets, PGP keys, communication platforms, and GitHub projects.
      </p>
      <div class="grid">
        <a class="tile" href="/wallets">
          <i class="fa-solid fa-wallet"></i>
          <div>Crypto Wallets</div>
        </a>
        <a class="tile" href="/pgp">
          <i class="fa-solid fa-key"></i>
          <div>PGP</div>
        </a>
        <a class="tile" href="/comm">
          <i class="fa-solid fa-comments"></i>
          <div>Contact Me</div>
        </a>
        <a class="tile" href="/github">
          <i class="fa-brands fa-github"></i>
          <div>GitHub</div>
        </a>
      </div>
    </div>
    "#;

    HttpResponse::Ok().content_type("text/html").body(layout("Home", body))
}
