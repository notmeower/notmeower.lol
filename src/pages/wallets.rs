use actix_web::{get, HttpResponse, Responder};
use crate::pages::layout;
use crate::utils::constants;

#[get("/wallets")]
pub async fn wallets() -> impl Responder {
    let body = format!(r#"
    <style>
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
        font-family: sans-serif;
        z-index: 1000;
    }}
    .toast.show {{
        opacity: 1;
        transform: translateY(0);
    }}
    .tile:hover {{
        cursor: pointer;
        transform: scale(1.05);
        transition: transform 0.2s;
    }}
    </style>

    <div class="card">
      <h1><i class="fa-solid fa-wallet"></i> Crypto</h1>
      <p class="muted">Click a tile to copy it's address</p>
      <div class="grid">
        <div class="tile" onclick="copyAddress('{btc}')">
          <img class="tile-img" src="/static/images/bitcoin.png">
          <div>Bitcoin</div>
        </div>
        <div class="tile" onclick="copyAddress('{eth}')">
          <img class="tile-img" src="/static/images/ethereum.png">
          <div>Ethereum</div>
        </div>
        <div class="tile" onclick="copyAddress('{ltc}')">
          <img class="tile-img" src="/static/images/litecoin.png">
          <div>Litecoin</div>
        </div>
        <div class="tile" onclick="copyAddress('{xmr}')">
          <img class="tile-img" src="/static/images/monero.png">
          <div>Monero</div>
        </div>
      </div>
    </div>

    <div id="toast" class="toast">Address copied!</div>

    <script>
    function copyAddress(address) {{
        navigator.clipboard.writeText(address).then(() => {{
            const toast = document.getElementById('toast');
            toast.classList.add('show');
            setTimeout(() => {{
                toast.classList.remove('show');
            }}, 2000);
        }});
    }}
    </script>
    "#,
    btc = constants::BITCOIN_ADDRESS,
    eth = constants::ETHEREUM_ADDRESS,
    ltc = constants::LITECOIN_ADDRESS,
    xmr = constants::MONERO_ADDRESS);

    HttpResponse::Ok().content_type("text/html").body(layout("Crypto", &body))
}
