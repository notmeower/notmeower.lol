use actix_web::{get, HttpResponse, Responder};
use crate::pages::layout;
use crate::utils::constants;

#[get("/comm")]
pub async fn comm() -> impl Responder {
    let body = format!(r#"
    <style>
    .tile {{
        text-align: center;
        padding: 1rem;
        border-radius: 10px;
        background: rgba(255, 255, 255, 0.05);
        transition: transform 0.2s;
        cursor: pointer;
    }}
    .tile:hover {{
        transform: scale(1.05);
    }}
    .tile-img {{
        width: 42px;
        height: 42px;
        margin-bottom: 0.5rem;
    }}
    .platform-name {{
        font-weight: bold;
    }}
    .toast {{
        position: fixed;
        bottom: 20px;
        right: 20px;
        background: rgba(0,0,0,0.9);
        color: #fff;
        padding: 1rem 1.5rem;
        border-radius: 10px;
        opacity: 0;
        transform: translateY(50px);
        transition: opacity 0.4s, transform 0.4s;
        z-index: 1000;
        font-family: sans-serif;
    }}
    .toast.show {{
        opacity: 1;
        transform: translateY(0);
    }}
    </style>

    <div class="card">
      <h1>Contact Me</h1>
      <div class="grid">
        <div class="tile" onclick="copyUsername('{tg}')">
          <img class="tile-img" src="/static/images/telegram.png">
          <div class="platform-name">Telegram</div>
          <div>{tg}</div>
        </div>
        <div class="tile" onclick="copyUsername('{dc}')">
          <img class="tile-img" src="/static/images/discord.png">
          <div class="platform-name">Discord</div>
          <div>{dc}</div>
          <div>ID: {dc_id}</div>
        </div>
        <div class="tile" onclick="copyUsername('{sig}')">
          <img class="tile-img" src="/static/images/signal.png">
          <div class="platform-name">Signal</div>
          <div>{sig}</div>
        </div>
        <div class="tile" onclick="copyUsername('{mx}')">
          <img class="tile-img" src="/static/images/matrix.png">
          <div class="platform-name">Matrix</div>
          <div>{mx}</div>
        </div>
      </div>
    </div>

    <div id="toast" class="toast">Username copied!</div>

    <script>
    function copyUsername(username) {{
        navigator.clipboard.writeText(username).then(() => {{
            const toast = document.getElementById('toast');
            toast.classList.add('show');
            setTimeout(() => {{
                toast.classList.remove('show');
            }}, 2000);
        }});
    }}
    </script>
    "#,
    tg = constants::TELEGRAM,
    dc = constants::DISCORD_USER,
    dc_id = constants::DISCORD_ID,
    sig = constants::SIGNAL,
    mx = constants::MATRIX);

    HttpResponse::Ok().content_type("text/html").body(layout("Contact Me", &body))
}
