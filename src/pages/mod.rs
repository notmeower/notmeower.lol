pub mod index;
pub mod wallets;
pub mod pgp;
pub mod comm;
pub mod github;

pub const STYLE: &str = r#"
:root {
  --bg-dark: #0d1117;
  --card-bg: rgba(255,255,255,0.06);
  --accent: #8b5cf6;
  --text-light: #cdd9e5;
  --text-muted: #8b949e;
  --tile-hover: rgba(139,92,246,0.15);
}

* { box-sizing:border-box; margin:0; padding:0; }

body {
  font-family:'Inter',sans-serif;
  background:var(--bg-dark);
  color:var(--text-light);
  min-height:100vh;
  display:flex;
  flex-direction:column;
  align-items:center;
}

header {
  display:flex;
  justify-content:space-between;
  align-items:center;
  width:100%;
  max-width:1100px;
  padding:0.75rem 1.5rem;
  margin-top:1rem;
}

.logo {
  display:flex;
  align-items:center;
  gap:0.5rem;
  font-weight:600;
  font-size:1rem;
}

.logo img { width:36px; height:36px; border-radius:10px; }

nav a {
  margin-left:0.8rem;
  text-decoration:none;
  font-weight:500;
  color:var(--text-light);
  font-size:0.9rem;
  padding:0.35rem 0.7rem;
  border-radius:12px;
  transition:all 0.2s ease;
}

nav a:hover {
  background:var(--tile-hover);
  color:var(--accent);
  transform:scale(1.05);
}

.container {
  width:100%;
  max-width:900px;
  margin:2rem auto;
  display:flex;
  flex-direction:column;
  align-items:center;
}

.card {
  background:var(--card-bg);
  padding:2rem;
  border-radius:18px;
  box-shadow:0 12px 28px rgba(0,0,0,0.5);
  margin-bottom:2rem;
  animation:fadeIn 0.6s ease forwards;
  width:100%;
}

@keyframes fadeIn { from { opacity:0; transform:translateY(12px);} to {opacity:1; transform:translateY(0);} }

.grid {
  display:grid;
  grid-template-columns:repeat(auto-fit,minmax(180px,1fr));
  gap:1.5rem;
  margin-top:1.5rem;
}

.tile {
  background:rgba(255,255,255,0.05);
  padding:1.2rem;
  border-radius:14px;
  text-align:center;
  display:flex;
  flex-direction:column;
  align-items:center;
  gap:0.5rem;
  transition:all 0.25s ease;
}

.tile:hover {
  background:var(--tile-hover);
  transform:translateY(-4px) scale(1.04);
}

.tile i {
  font-size:1.6rem;
  color:var(--accent);
  transition:transform 0.25s ease;
}

.tile:hover i { transform:scale(1.2) rotate(6deg); }

h1 {
  font-size:1.6rem;
  margin-bottom:0.75rem;
  text-align:center;
}

p.muted {
  color:var(--text-muted);
  font-size:0.9rem;
  text-align:center;
}

.footer {
  text-align:center;
  padding:1rem;
  color:var(--text-muted);
  font-size:0.85rem;
  margin-bottom:1rem;
}

img.tile-img {
  width:42px;
  height:42px;
  object-fit:cover;
  border-radius:8px;
  margin-bottom:0.3rem;
}
"#;

pub const SCRIPT: &str = r#"
document.addEventListener('DOMContentLoaded',()=>{
  console.log('Dashboard loaded');
});
"#;

pub fn layout(title: &str, body: &str) -> String {
    format!(r#"
<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<title>{}</title>
<meta name="viewport" content="width=device-width, initial-scale=1">
<link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.4.0/css/all.min.css">
<style>{}</style>
</head>
<body>
  <header>
    <div class="logo"><img src="/static/images/notmeower.png"><span>notmeower</span></div>
    <nav>
      <a href="/">Home</a>
      <a href="/wallets">Wallets</a>
      <a href="/pgp">PGP</a>
      <a href="/comm">Contact Me</a>
      <a href="/github">GitHub</a>
    </nav>
  </header>
  <main class="container">{}</main>
  <footer class="footer">Copyright 2025 Meower Development • https://github.com/notmeower/notmeower.lol</footer>
  <script>{}</script>
</body>
</html>
"#, title, STYLE, body, SCRIPT)
}
