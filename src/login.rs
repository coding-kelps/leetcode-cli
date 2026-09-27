use std::{
    io,
    path::Path,
    process::Stdio,
    time::{
        Duration,
        Instant,
    },
};

use leetcoderustapi::UserApi;
use serde::Deserialize;
use serde_json::json;

use crate::config::RuntimeConfigSetup;

const LOGIN_URL: &str = "https://leetcode.com/accounts/login/";
const LOGIN_TIMEOUT: Duration = Duration::from_secs(300);
const POLL_INTERVAL: Duration = Duration::from_secs(2);
const BROWSER_STARTUP_TIMEOUT: Duration = Duration::from_secs(15);
const BROWSER_CANDIDATES: [&str; 6] = [
    "google-chrome-stable",
    "google-chrome",
    "chromium",
    "chromium-browser",
    "brave-browser",
    "microsoft-edge",
];

#[derive(Deserialize, Debug)]
struct Cookie {
    name:  String,
    value: String,
}

/// Finds a chromium based browser to run the login flow with.
fn find_browser() -> io::Result<String> {
    if let Ok(path) = std::env::var("LEETCODE_CLI_BROWSER") {
        if !path.trim().is_empty() {
            return Ok(path);
        }
    }
    for name in BROWSER_CANDIDATES {
        if command_exists(name)? {
            return Ok(name.to_string());
        }
    }
    Err(io::Error::other(
        "No chromium based browser found. Set LEETCODE_CLI_BROWSER to your \
         browser executable and try again.",
    ))
}

fn command_exists(cmd: &str) -> io::Result<bool> {
    Ok(std::process::Command::new("sh")
        .arg("-c")
        .arg(format!("command -v {cmd} >/dev/null 2>&1"))
        .status()?
        .success())
}

/// Spawns the browser with a dedicated debugging port and profile dir, then
/// waits for the devtools http endpoint to come up.
async fn launch_browser(
    browser: &str, user_data_dir: &Path,
) -> io::Result<(tokio::process::Child, u16)> {
    let listener = std::net::TcpListener::bind("127.0.0.1:0")?;
    let port = listener.local_addr()?.port();
    drop(listener);

    let child = tokio::process::Command::new(browser)
        .arg(format!("--remote-debugging-port={port}"))
        .arg(format!("--user-data-dir={}", user_data_dir.display()))
        .arg("--no-first-run")
        .arg("--no-default-browser-check")
        .arg(LOGIN_URL)
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| {
            io::Error::other(format!(
                "Could not start browser {browser}: {e} (set \
                 LEETCODE_CLI_BROWSER to override)"
            ))
        })?;

    let deadline = Instant::now() + BROWSER_STARTUP_TIMEOUT;
    loop {
        if Instant::now() > deadline {
            return Err(io::Error::other(format!(
                "Browser devtools endpoint did not come up on port {port}"
            )));
        }
        if let Ok(resp) =
            reqwest::get(format!("http://127.0.0.1:{port}/json/version")).await
        {
            if resp.status().is_success() {
                return Ok((child, port));
            }
        }
        tokio::time::sleep(Duration::from_millis(200)).await;
    }
}

/// Fetches leetcode cookies through the chrome devtools protocol.
async fn fetch_cookies(port: u16) -> io::Result<Vec<Cookie>> {
    use futures_util::{
        SinkExt,
        StreamExt,
    };

    let targets: serde_json::Value =
        reqwest::get(format!("http://127.0.0.1:{port}/json/list"))
            .await
            .map_err(io::Error::other)?
            .json()
            .await
            .map_err(io::Error::other)?;

    let page = targets
        .as_array()
        .and_then(|targets| {
            targets.iter().find(|target| {
                target["type"] == "page"
                    && target["url"]
                        .as_str()
                        .is_some_and(|url| url.contains("leetcode.com"))
            })
        })
        .ok_or_else(|| io::Error::other("No leetcode tab found in browser"))?;

    let ws_url = page["webSocketDebuggerUrl"]
        .as_str()
        .ok_or_else(|| io::Error::other("No devtools websocket url found"))?
        .to_string();

    let (ws, _) = tokio_tungstenite::connect_async(&ws_url)
        .await
        .map_err(io::Error::other)?;
    let (mut write, mut read) = ws.split();

    write
        .send(tokio_tungstenite::tungstenite::Message::text(
            json!({"id": 1, "method": "Network.enable"}).to_string(),
        ))
        .await
        .map_err(io::Error::other)?;
    write
        .send(tokio_tungstenite::tungstenite::Message::text(
            json!({
                "id": 2,
                "method": "Network.getCookies",
                "params": {"urls": ["https://leetcode.com"]}
            })
            .to_string(),
        ))
        .await
        .map_err(io::Error::other)?;

    while let Some(message) = read.next().await {
        let message = message.map_err(io::Error::other)?;
        if let tokio_tungstenite::tungstenite::Message::Text(text) = message {
            let payload: serde_json::Value =
                serde_json::from_str(&text).map_err(io::Error::other)?;
            if payload["id"] == 2 {
                let cookies: Vec<Cookie> = serde_json::from_value(
                    payload["result"]["cookies"].clone(),
                )
                .map_err(io::Error::other)?;
                return Ok(cookies);
            }
        }
    }

    Err(io::Error::other("Devtools websocket closed unexpectedly"))
}

/// Builds the cookie header string expected by the api, csrftoken first.
fn build_cookie_string(csrf_token: &str, leetcode_session: &str) -> String {
    format!("csrftoken={csrf_token}; LEETCODE_SESSION={leetcode_session}")
}

/// Pulls the two cookies from the browser and validates them against leetcode.
async fn try_get_valid_cookie(port: u16) -> io::Result<String> {
    let cookies = fetch_cookies(port).await?;

    let csrf_token = cookies
        .iter()
        .find(|cookie| cookie.name == "csrftoken")
        .map(|cookie| cookie.value.clone())
        .ok_or_else(|| io::Error::other("csrftoken cookie not set yet"))?;
    let leetcode_session = cookies
        .iter()
        .find(|cookie| cookie.name == "LEETCODE_SESSION")
        .map(|cookie| cookie.value.clone())
        .ok_or_else(|| {
            io::Error::other("LEETCODE_SESSION cookie not set yet")
        })?;

    let cookie = build_cookie_string(&csrf_token, &leetcode_session);

    UserApi::new(&cookie).await.map_err(|e| {
        io::Error::other(format!("Leetcode rejected the cookie: {e}"))
    })?;

    Ok(cookie)
}

/// Polls the browser until the user logged in and the cookie validates.
async fn poll_until_signed_in(port: u16) -> io::Result<String> {
    let start = Instant::now();
    let mut last_error = String::from("no attempt made yet");

    loop {
        if start.elapsed() > LOGIN_TIMEOUT {
            return Err(io::Error::other(format!(
                "Timed out waiting for leetcode login (last error: \
                 {last_error})"
            )));
        }
        match try_get_valid_cookie(port).await {
            Ok(cookie) => return Ok(cookie),
            Err(e) => {
                last_error = e.to_string();
                tokio::time::sleep(POLL_INTERVAL).await;
            },
        }
    }
}

/// One time login flow:
/// spawns a browser on a dedicated profile, waits for the user to log into
/// leetcode, grabs the session cookies through the devtools protocol,
/// validates them and saves them into the leetcode_token config entry.
pub async fn run_login(rcs: &RuntimeConfigSetup) -> io::Result<String> {
    let browser = find_browser()?;
    let profile_dir = rcs.config_dir.join("browser-profile");
    std::fs::create_dir_all(&profile_dir)?;

    let (mut child, port) = launch_browser(&browser, &profile_dir).await?;
    let outcome = poll_until_signed_in(port).await;
    let _ = child.kill().await;

    let cookie = outcome?;
    RuntimeConfigSetup::write_token_to_file(&rcs.config_file, &cookie)?;

    Ok(format!(
        "Login successful, leetcode_token saved in {}",
        rcs.config_file.display()
    ))
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_build_cookie_string() {
        assert_eq!(
            build_cookie_string("abc", "session"),
            "csrftoken=abc; LEETCODE_SESSION=session"
        );
    }

    #[test]
    fn test_write_token_to_file_creates_and_updates() {
        let dir = tempfile::TempDir::new().unwrap();
        let config_file: PathBuf = dir.path().join("config.toml");

        RuntimeConfigSetup::write_token_to_file(&config_file, "tok").unwrap();
        let content = std::fs::read_to_string(&config_file).unwrap();
        assert!(content.contains("leetcode_token = 'tok'"));

        RuntimeConfigSetup::write_token_to_file(&config_file, "tok2").unwrap();
        let content = std::fs::read_to_string(&config_file).unwrap();
        assert!(content.contains("leetcode_token = 'tok2'"));
        assert!(!content.contains("'tok'"));
    }

    #[test]
    fn test_write_token_to_file_preserves_other_fields() {
        let dir = tempfile::TempDir::new().unwrap();
        let config_file = dir.path().join("config.toml");
        std::fs::write(
            &config_file,
            "leetcode_token = 'old'\ndefault_language = 'Rust'\n",
        )
        .unwrap();

        RuntimeConfigSetup::write_token_to_file(&config_file, "new").unwrap();
        let content = std::fs::read_to_string(&config_file).unwrap();
        assert!(content.contains("default_language = 'Rust'"));
        assert!(content.contains("leetcode_token = 'new'"));
    }
}
