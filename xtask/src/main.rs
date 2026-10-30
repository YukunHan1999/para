use std::env;
use std::fs;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::process::Command;

/// (slug, title, description, category)
const BOOKS: &[(&str, &str, &str, &str)] = &[
    (
        "Projects",
        "项目",
        "有明确目标和期限的事情，例如“完成个人网站”“准备面试”。项目结束后通常归档。",
        "project",
    ),
    (
        "Areas",
        "领域",
        "需要长期维护的责任范围，没有固定终点，例如健康、工作、财务、家庭。",
        "area",
    ),
    (
        "Resources",
        "资源",
        "以后可能用得上的主题资料，例如 Rust 笔记、网页设计灵感、旅行信息。",
        "resource",
    ),
    (
        "Archives",
        "归档",
        "已经完成、暂停或暂时不再活跃的项目，以及不再维护的领域和资源。",
        "archive",
    ),
];

fn project_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask must live in a workspace subdirectory")
        .to_path_buf()
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    match args.first().map(|s| s.as_str()) {
        Some("build") => cmd_build(),
        Some("serve") => {
            cmd_build();
            cmd_serve();
        }
        Some("deploy") => cmd_deploy(),
        Some("clean") => cmd_clean(),
        Some("--help" | "-h" | "help") | None => print_usage(0),
        Some(other) => {
            eprintln!("Unknown command: {other}\n");
            print_usage(1);
        }
    }
}

fn print_usage(code: i32) {
    let stream: &mut dyn Write = if code == 0 {
        &mut std::io::stdout()
    } else {
        &mut std::io::stderr()
    };
    let _ = writeln!(
        stream,
        "\
Usage: cargo xtask <COMMAND>

Commands:
  build    Build all books into site/ (for local preview)
  serve    Build and serve at http://localhost:3000
  deploy   Build all books into docs/ (for GitHub Pages)
  clean    Remove site/ and docs/ directories"
    );
    std::process::exit(code);
}

// ── build ────────────────────────────────────────────────────────────

fn cmd_build() {
    if !check_mdbook() {
        eprintln!("Error: 'mdbook' not found in PATH. Please install it: https://rust-lang.github.io/mdbook/guide/installation.html");
        std::process::exit(1);
    }
    build_to("site");
}

fn cmd_deploy() {
    if !check_mdbook() {
        eprintln!("Error: 'mdbook' not found in PATH.");
        std::process::exit(1);
    }
    build_to("docs");
    println!("\nTo publish, commit docs/ and enable GitHub Pages → \"Deploy from a branch\" → /docs.");
}

fn check_mdbook() -> bool {
    Command::new("mdbook")
        .arg("--version")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn build_to(dir_name: &str) {
    let root = project_root();
    let out = root.join(dir_name);

    if out.exists() {
        fs::remove_dir_all(&out).expect("failed to clean output dir");
    }
    fs::create_dir_all(&out).expect("failed to create output dir");

    println!("Building unified site into {dir_name}/\n");

    let mut ok = 0u32;
    for &(slug, _, _, _) in BOOKS {
        let book_dir = root.join(slug);
        if !book_dir.is_dir() {
            eprintln!("  ✗ {slug}/ not found, skipping");
            continue;
        }
        let dest = out.join(slug);
        let status = Command::new("mdbook")
            .args(["build", "--dest-dir"])
            .arg(&dest)
            .current_dir(&book_dir)
            .status()
            .expect("failed to run mdbook — is it installed?");

        if status.success() {
            println!("  ✓ {slug}");
            ok += 1;
        } else {
            eprintln!("  ✗ {slug} FAILED");
        }
    }
    println!("\n  {ok}/{} books built", BOOKS.len());

    write_landing_page(&out);

    // Prevent GitHub Pages from processing the output with Jekyll
    fs::write(out.join(".nojekyll"), "").expect("failed to create .nojekyll");
    println!("\nDone! Output in {dir_name}/");
}

fn category_label(cat: &str) -> &str {
    match cat {
        "project" => "Project",
        "area" => "Area",
        "resource" => "Resource",
        "archive" => "Archive",
        _ => cat,
    }
}

fn write_landing_page(site: &Path) {
    let cards: String = BOOKS
        .iter()
        .map(|&(slug, title, desc, cat)| {
            let label = category_label(cat);
            format!(
                r#"    <a class="card cat-{cat}" href="{slug}/">
      <h2>{title} <span class="label">{label}</span></h2>
      <p>{desc}</p>
    </a>"#
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let html = format!(
        r##"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>PARA</title>
  <style>
    :root {{
      --bg: #1a1a2e;
      --card-bg: #16213e;
      --accent: #e94560;
      --text: #eee;
      --muted: #a8a8b3;
      --clr-project: #E98A3A;
      --clr-resource: #4F9D69;
      --clr-area: #4A83C4;
      --clr-archive: #8A8F98;
    }}
    * {{ margin: 0; padding: 0; box-sizing: border-box; }}
    body {{
      font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, sans-serif;
      background: var(--bg);
      color: var(--text);
      min-height: 100vh;
      display: flex;
      flex-direction: column;
      align-items: center;
      padding: 3rem 1rem;
    }}
    h1 {{ font-size: 2.5rem; margin-bottom: 0.5rem; }}
    h1 span {{ color: var(--accent); }}
    .subtitle {{ color: var(--muted); font-size: 1.1rem; margin-bottom: 1.2rem; }}

    /* Legend */
    .legend {{
      display: flex; flex-wrap: wrap; gap: 0.6rem 1.4rem;
      justify-content: center; margin-bottom: 2.2rem;
      font-size: 0.8rem; color: var(--muted);
    }}
    .legend-item {{ display: flex; align-items: center; gap: 0.35rem; }}
    .legend-dot {{
      width: 10px; height: 10px; border-radius: 50%; flex-shrink: 0;
    }}

    /* Grid & Cards */
    .grid {{
      display: grid;
      grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
      gap: 1.5rem;
      max-width: 1000px;
      width: 100%;
    }}
    .card {{
      background: var(--card-bg);
      border-radius: 12px;
      padding: 1.5rem 1.5rem 1.5rem 1.25rem;
      text-decoration: none;
      color: var(--text);
      transition: transform 0.15s, box-shadow 0.15s;
      border: 1px solid rgba(255,255,255,0.05);
      border-left: 4px solid var(--stripe);
    }}
    .card:hover {{
      transform: translateY(-4px);
      box-shadow: 0 8px 25px color-mix(in srgb, var(--stripe) 30%, transparent);
      border-color: rgba(255,255,255,0.08);
      border-left-color: var(--stripe);
    }}
    .card h2 {{ font-size: 1.2rem; margin-bottom: 0.5rem; display: flex; align-items: center; gap: 0.6rem; flex-wrap: wrap; }}
    .card p  {{ color: var(--muted); font-size: 0.9rem; line-height: 1.4; }}

    /* Category colours */
    .cat-project     {{ --stripe: var(--clr-project); }}
    .cat-area  {{ --stripe: var(--clr-area); }}
    .cat-resource   {{ --stripe: var(--clr-resource); }}
    .cat-archive     {{ --stripe: var(--clr-archive); }}

    /* Label pill */
    .label {{
      font-size: 0.55rem; font-weight: 700; letter-spacing: 0.08em;
      text-transform: uppercase; padding: 0.15em 0.55em;
      border-radius: 4px; white-space: nowrap; flex-shrink: 0;
      color: var(--bg); background: var(--stripe);
    }}

    footer {{ margin-top: 3rem; color: var(--muted); font-size: 0.85rem; }}
  </style>
</head>
<body>
  <h1>🦀 <span>PARA</span> 个人信息管理</h1>

  <div class="legend">
    <span class="legend-item"><span class="legend-dot" style="background:var(--clr-project)"></span> Project</span>
    <span class="legend-item"><span class="legend-dot" style="background:var(--clr-area)"></span> Area</span>
    <span class="legend-item"><span class="legend-dot" style="background:var(--clr-resource)"></span> Resource</span>
    <span class="legend-item"><span class="legend-dot" style="background:var(--clr-archive)"></span> Archive</span>
  </div>

  <div class="grid">
{cards}
  </div>
  <footer>Built with <a href="https://rust-lang.github.io/mdBook/" style="color:var(--accent)">mdBook</a></footer>
</body>
</html>
"##
    );

    let path = site.join("index.html");
    fs::write(&path, html).expect("failed to write index.html");
    println!("  ✓ index.html");
}

enum ResolveResult {
    File(PathBuf),
    Redirect(String),
    NotFound,
}

/// Resolve `request_target` (HTTP request path, e.g. `/foo/bar?x=1`) to a file under `site_canon`.
/// Returns `ResolveResult::File` for success, `Redirect` if a trailing slash is needed for a directory,
/// or `NotFound` for traversal attempts or missing files.
///
/// NOTE: This function preserves and hardens the multi-layer security from PR#18:
/// 1. Percent-decoding via `percent_decode_path`.
/// 2. Null byte rejection.
/// 3. Traversal blocking (`..`).
/// 4. Symlink escape prevention via canonicalization and prefix checking.
fn resolve_site_file(site_canon: &Path, request_target: &str) -> ResolveResult {
    let path_only = match request_target
        .split('?')
        .next()
        .and_then(|s| s.split('#').next())
    {
        Some(p) => p,
        None => return ResolveResult::NotFound,
    };

    // [Security] Handle percent-encoding and reject null bytes (from PR#18)
    let decoded = percent_decode_path(path_only);
    if decoded.as_bytes().contains(&0) {
        return ResolveResult::NotFound;
    }

    let rel = decoded.trim_start_matches('/');
    let mut file_path = site_canon.to_path_buf();
    if !rel.is_empty() {
        for seg in rel.split('/').filter(|s| !s.is_empty()) {
            // [Security] Block directory traversal (from PR#18)
            if seg == ".." {
                return ResolveResult::NotFound;
            }
            file_path.push(seg);
        }
    }

    if file_path.is_dir() {
        // If it refers to a directory but lacks a trailing slash, redirect so relative links work.
        if !request_target.ends_with('/') && !request_target.is_empty() {
            return ResolveResult::Redirect(format!("{path_only}/"));
        }
        file_path.push("index.html");
    }

    // [Security] Canonicalize and verify we're still within site_canon (from PR#18)
    let real = match fs::canonicalize(&file_path) {
        Ok(r) => r,
        Err(_) => return ResolveResult::NotFound,
    };

    if !real.starts_with(site_canon) || !real.is_file() {
        return ResolveResult::NotFound;
    }

    ResolveResult::File(real)
}

fn hex_val(c: u8) -> Option<u8> {
    match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'a'..=b'f' => Some(c - b'a' + 10),
        b'A'..=b'F' => Some(c - b'A' + 10),
        _ => None,
    }
}

fn percent_decode_path(input: &str) -> String {
    let mut decoded = Vec::with_capacity(input.len());
    let b = input.as_bytes();
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'%' && i + 2 < b.len() {
            if let (Some(hi), Some(lo)) = (hex_val(b[i + 1]), hex_val(b[i + 2])) {
                decoded.push(hi << 4 | lo);
                i += 3;
                continue;
            }
        }
        decoded.push(b[i]);
        i += 1;
    }
    String::from_utf8_lossy(&decoded).into_owned()
}

// ── serve ────────────────────────────────────────────────────────────

fn cmd_serve() {
    let site = project_root().join("site");
    let site_canon = fs::canonicalize(&site).expect(
        "site/ not found — run `cargo xtask build` first (e.g. `cargo xtask serve` runs build automatically)",
    );
    let addr = "0.0.0.0:3000";
    let listener = TcpListener::bind(addr).expect("failed to bind port 3000");

    // Handle Ctrl+C gracefully so cargo doesn't report an error
    ctrlc_exit();

    println!("\nServing at http://localhost:3000  (Ctrl+C to stop)");

    for stream in listener.incoming() {
        let Ok(mut stream) = stream else { continue };
        let mut buf = [0u8; 4096];
        let n = stream.read(&mut buf).unwrap_or(0);
        let request = String::from_utf8_lossy(&buf[..n]);

        let path = request
            .lines()
            .next()
            .and_then(|line| line.split_whitespace().nth(1))
            .unwrap_or("/");

        match resolve_site_file(&site_canon, path) {
            ResolveResult::File(file_path) => {
                let body = fs::read(&file_path).unwrap_or_default();
                let mime = guess_mime(&file_path);
                let header = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: {mime}\r\nContent-Length: {}\r\n\r\n",
                    body.len()
                );
                let _ = stream.write_all(header.as_bytes());
                let _ = stream.write_all(&body);
            }
            ResolveResult::Redirect(new_path) => {
                let header = format!(
                    "HTTP/1.1 301 Moved Permanently\r\nLocation: {new_path}\r\nContent-Length: 0\r\n\r\n"
                );
                let _ = stream.write_all(header.as_bytes());
            }
            ResolveResult::NotFound => {
                let body = b"404 Not Found";
                let header = format!(
                    "HTTP/1.1 404 Not Found\r\nContent-Length: {}\r\n\r\n",
                    body.len()
                );
                let _ = stream.write_all(header.as_bytes());
                let _ = stream.write_all(body);
            }
        }
    }
}

/// Install a Ctrl+C handler that exits cleanly (code 0) instead of
/// letting the OS terminate with STATUS_CONTROL_C_EXIT.
fn ctrlc_exit() {
    ctrlc::set_handler(move || {
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");
}

fn guess_mime(path: &Path) -> &'static str {
    match path.extension().and_then(|e| e.to_str()) {
        Some("html") => "text/html; charset=utf-8",
        Some("css") => "text/css",
        Some("js") => "application/javascript",
        Some("svg") => "image/svg+xml",
        Some("png") => "image/png",
        Some("jpg" | "jpeg") => "image/jpeg",
        Some("woff2") => "font/woff2",
        Some("woff") => "font/woff",
        Some("json") => "application/json",
        _ => "application/octet-stream",
    }
}

// ── clean ────────────────────────────────────────────────────────────

fn cmd_clean() {
    let root = project_root();
    for dir_name in ["site", "docs"] {
        let dir = root.join(dir_name);
        if dir.exists() {
            fs::remove_dir_all(&dir).expect("failed to remove dir");
            println!("Removed {dir_name}/");
        }
    }
}
