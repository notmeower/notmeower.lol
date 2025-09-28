use actix_web::{get, HttpResponse, Responder};
use crate::pages::layout;

#[get("/pgp")]
pub async fn pgp() -> impl Responder {
    let body = r#"
    <style>
    .card {
        max-width: 800px;
        margin: 2rem auto;
        padding: 2rem;
        border-radius: 16px;
        background: rgba(255, 255, 255, 0.05);
        box-shadow: 0 4px 20px rgba(0,0,0,0.3);
    }
    h1 {
        margin-bottom: 1rem;
    }
    .pgp-container {
        position: relative;
        background: rgba(0,0,0,0.7);
        border-radius: 12px;
        padding: 1rem;
    }
    pre {
        font-family: monospace;
        font-size: 0.85rem;
        line-height: 1.4;
        margin: 0;
        overflow-x: auto;
        white-space: pre-wrap;
        word-break: break-word;
    }
    .copy-btn {
        position: absolute;
        top: 10px;
        right: 10px;
        background: #1E90FF;
        color: #fff;
        border: none;
        border-radius: 6px;
        padding: 0.3rem 0.6rem;
        cursor: pointer;
        font-size: 0.85rem;
        transition: background 0.2s;
    }
    .copy-btn:hover {
        background: #1C86EE;
    }
    .toast {
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
    }
    .toast.show {
        opacity: 1;
        transform: translateY(0);
    }
    </style>

    <div class="card">
      <h1><i class="fa-solid fa-key"></i> My PGP Public Key</h1>
      <div class="pgp-container">
        <button class="copy-btn" onclick="copyPGP()">Copy</button>
        <pre id="pgp-block">
-----BEGIN PGP PUBLIC KEY BLOCK-----

mDMEaGL4IhYJKwYBBAHaRw8BAQdAO6YApTfcIvMTPIi6gZejw1dFh1qOxc+in7KA
9OKu1rq0IW5vdG1lb3dlciA8bmlla290QHByb3Rvbm1haWwuY29tPoiWBBMWCgA+
FiEEGJJ0uvkhJc4DnwZ3upj9O/KFnkoFAmhi+CICGwMFCZeapI4FCwkIBwIGFQoJ
CAsCBBYCAwECHgECF4AACgkQupj9O/KFnkoKcgEA+/RoWG7MB5pqTwd7+fsaUF8r
ng6kOGSQDIIx1goaKYIBAOsAOLAEX6XCyKuiO2+leTTGAkAkfv9syz7C5Lha8YQG
uDgEaGL4IhIKKwYBBAGXVQEFAQEHQB5ZEJFC5Wff53eUR3ThU/QAUCLYe9yaZDig
CPBXd2IlAwEIB4h+BBgWCgAmFiEEGJJ0uvkhJc4DnwZ3upj9O/KFnkoFAmhi+CIC
GwwFCZeapI4ACgkQupj9O/KFnkoImwEA0DYXHcObsPbOwRkSYf0qhvmZGz8QHrjN
hBQp6zRJJFIA/jvSB2qbiyv7xaJuClfsjx8NN4Sx6rhTpTh6HTnr8uoL
=Xrj1
-----END PGP PUBLIC KEY BLOCK-----
        </pre>
      </div>
    </div>

    <div id="toast" class="toast">PGP key copied!</div>

    <script>
    function copyPGP() {
        const text = document.getElementById('pgp-block').innerText;
        navigator.clipboard.writeText(text).then(() => {
            const toast = document.getElementById('toast');
            toast.classList.add('show');
            setTimeout(() => {
                toast.classList.remove('show');
            }, 2000);
        });
    }
    </script>
    "#;

    HttpResponse::Ok().content_type("text/html").body(layout("PGP", body))
}
