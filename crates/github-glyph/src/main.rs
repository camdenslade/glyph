use core_glyph::{
    button, column, image, rect, row, scroll, spacer, text, Color, FontWeight, Signal, Theme, View,
};
use platform_glyph::App;
use ui_glyph::{
    badge_colored, shadow_dark_lg, RADIUS_LG, RADIUS_MD, SPACE_1, SPACE_12, SPACE_16, SPACE_2,
    SPACE_3, SPACE_4, SPACE_6, SPACE_8, TEXT_BASE, TEXT_LG, TEXT_SM, TEXT_XL, TEXT_XS,
};
use serde::Deserialize;
use std::{fs, path::PathBuf, sync::Arc, thread};

const CLIENT_ID: &str = env!("GITHUB_CLIENT_ID");

// GitHub's exact design tokens
const BG: Color = Color::rgb(0.051, 0.067, 0.090); // #0d1117
const SURFACE: Color = Color::rgb(0.086, 0.106, 0.133); // #161b22
const SURFACE2: Color = Color::rgb(0.118, 0.122, 0.133); // #1e1f22  (neutral lighter panel bg)
const BORDER: Color = Color::rgb(0.188, 0.212, 0.243); // #30363d
const TEXT: Color = Color::rgb(0.902, 0.929, 0.961); // #e6edf3
const MUTED: Color = Color::rgb(0.486, 0.549, 0.624); // #7d8590
const SUBTLE: Color = Color::rgb(0.294, 0.333, 0.384); // #4b5562  (dimmer)
const BLUE: Color = Color::rgb(0.231, 0.557, 0.969); // #3b8eef  darker link blue
const GREEN: Color = Color::rgb(0.247, 0.722, 0.314); // #3fb950
const ORANGE: Color = Color::rgb(0.859, 0.427, 0.169); // #db6d28
const PURPLE: Color = Color::rgb(0.686, 0.431, 0.988); // #af6ef9  (merged PR)
const RED: Color = Color::rgb(0.953, 0.369, 0.369); // #f35e5e  (closed PR)
const TOPNAV: Color = Color::rgb(0.027, 0.027, 0.031); // #070709  near-black neutral

fn gh_theme() -> Theme {
    Theme {
        background: BG,
        surface: SURFACE,
        primary: BLUE,
        on_primary: TEXT,
        text: TEXT,
        text_muted: MUTED,
        border: BORDER,
        border_focused: BLUE,
        radius: 6.0,
        font_size: 14.0,
    }
}

fn asset(name: &str) -> String {
    format!("{}/assets/{}", env!("CARGO_MANIFEST_DIR"), name)
}

// --- Data types ---

#[derive(Deserialize, Clone)]
#[allow(dead_code)]
struct User {
    login: String,
    name: Option<String>,
    avatar_url: String,
    bio: Option<String>,
    public_repos: u32,
    followers: u32,
    following: u32,
    company: Option<String>,
    location: Option<String>,
    blog: Option<String>,
}

#[derive(Deserialize, Clone)]
struct Repo {
    #[allow(dead_code)]
    name: String,
    full_name: String,
    html_url: String,
    description: Option<String>,
    stargazers_count: u32,
    forks_count: u32,
    language: Option<String>,
    private: bool,
    fork: bool,
    updated_at: Option<String>,
}

#[derive(Deserialize, Clone)]
struct Notification {
    #[allow(dead_code)]
    id: String,
    reason: String,
    unread: bool,
    subject: NotificationSubject,
    repository: NotificationRepo,
    updated_at: Option<String>,
}

#[derive(Deserialize, Clone)]
struct NotificationSubject {
    title: String,
    #[serde(rename = "type")]
    kind: String,
}

#[derive(Deserialize, Clone)]
struct NotificationRepo {
    full_name: String,
}

#[derive(Deserialize, Clone)]
struct PullRequest {
    title: String,
    number: u32,
    #[allow(dead_code)]
    html_url: String,
    user: PrUser,
    repository_url: String,
    state: String,
    draft: Option<bool>,
    comments: Option<u32>,
}

#[derive(Deserialize, Clone)]
struct PrUser {
    login: String,
}

#[derive(Deserialize)]
struct DeviceCodeResponse {
    device_code: String,
    user_code: String,
    verification_uri: String,
    interval: u64,
}

#[derive(Deserialize)]
struct AccessTokenResponse {
    access_token: Option<String>,
    error: Option<String>,
}

// --- Auth ---

fn token_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("glyph-github")
        .join("token")
}
fn load_token() -> Option<String> {
    fs::read_to_string(token_path())
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}
fn save_token(t: &str) {
    let p = token_path();
    if let Some(d) = p.parent() {
        let _ = fs::create_dir_all(d);
    }
    let _ = fs::write(p, t);
}
#[allow(dead_code)]
fn clear_token() {
    let _ = fs::remove_file(token_path());
}

fn client() -> reqwest::blocking::Client {
    reqwest::blocking::Client::builder()
        .user_agent("glyph-github/0.1")
        .build()
        .unwrap()
}
fn api_get<T: for<'de> Deserialize<'de>>(token: &str, url: &str) -> Result<T, String> {
    let body = client()
        .get(url)
        .bearer_auth(token)
        .header("Accept", "application/vnd.github+json")
        .send()
        .map_err(|e| e.to_string())?
        .text()
        .map_err(|e| e.to_string())?;
    serde_json::from_str(&body).map_err(|e| format!("{e}: {}", &body[..body.len().min(200)]))
}

#[derive(Clone)]
struct DashboardData {
    user: User,
    repos: Vec<Repo>,
    notifications: Vec<Notification>,
    prs: Vec<PullRequest>,
    avatar_path: Option<String>,
}

fn fetch_avatar(url: &str, login: &str) -> Option<String> {
    // Always use .png path — image crate detects format by content, not extension.
    // Strip query params from URL before fetching so cache key is stable.
    let fetch_url = url.split('?').next().unwrap_or(url);
    let path = std::env::temp_dir().join(format!("glyph_avatar_{}.png", login));
    if path.exists() {
        return Some(path.to_string_lossy().into_owned());
    }
    let resp = client().get(fetch_url).send().ok()?;
    let bytes = resp.bytes().ok()?;
    if bytes.is_empty() {
        return None;
    }
    fs::write(&path, &bytes).ok()?;
    Some(path.to_string_lossy().into_owned())
}

fn fetch_dashboard(token: &str) -> Result<DashboardData, String> {
    let user: User = api_get(token, "https://api.github.com/user")?;
    let repos: Vec<Repo> = api_get(
        token,
        "https://api.github.com/user/repos?sort=pushed&per_page=30",
    )?;
    let notifications: Vec<Notification> =
        api_get(token, "https://api.github.com/notifications?per_page=50")?;
    #[derive(Deserialize)]
    struct SR {
        items: Vec<PullRequest>,
    }
    let prs = api_get::<SR>(
        token,
        &format!(
            "https://api.github.com/search/issues?q=is:pr+is:open+assignee:{}&per_page=20",
            user.login
        ),
    )
    .map(|r| r.items)
    .unwrap_or_default();
    let avatar_path = fetch_avatar(&user.avatar_url, &user.login);
    Ok(DashboardData {
        user,
        repos,
        notifications,
        prs,
        avatar_path,
    })
}

fn start_device_flow() -> Result<DeviceCodeResponse, String> {
    let body = client()
        .post("https://github.com/login/device/code")
        .header("Accept", "application/json")
        .form(&[("client_id", CLIENT_ID), ("scope", "repo notifications")])
        .send()
        .map_err(|e| e.to_string())?
        .text()
        .map_err(|e| e.to_string())?;
    serde_json::from_str(&body).map_err(|e| format!("{e}: {body}"))
}
fn poll_for_token(code: &str, interval: u64) -> Result<String, String> {
    loop {
        thread::sleep(std::time::Duration::from_secs(interval));
        let body = client()
            .post("https://github.com/login/oauth/access_token")
            .header("Accept", "application/json")
            .form(&[
                ("client_id", CLIENT_ID),
                ("device_code", code),
                ("grant_type", "urn:ietf:params:oauth:grant-type:device_code"),
            ])
            .send()
            .map_err(|e| e.to_string())?
            .text()
            .map_err(|e| e.to_string())?;
        let r: AccessTokenResponse =
            serde_json::from_str(&body).map_err(|e| format!("{e}: {body}"))?;
        match (r.access_token, r.error.as_deref()) {
            (Some(t), _) => return Ok(t),
            (_, Some("authorization_pending")) => continue,
            (_, Some("slow_down")) => {
                thread::sleep(std::time::Duration::from_secs(5));
                continue;
            }
            (_, Some(e)) => return Err(e.to_string()),
            _ => continue,
        }
    }
}

// --- Tab enum ---

#[derive(Clone, PartialEq)]
enum Tab {
    Overview,
    Repos,
    Notifications,
    PullRequests,
}

// --- Login screen ---

struct LoginScreen {
    user_code: Signal<Option<String>>,
    error: Signal<Option<String>>,
    token_out: Signal<Option<String>>,
}

impl LoginScreen {
    fn new() -> Self {
        let s = Self {
            user_code: Signal::new(None),
            error: Signal::new(None),
            token_out: Signal::new(None),
        };
        let (uc, err, tok) = (s.user_code.clone(), s.error.clone(), s.token_out.clone());
        thread::spawn(move || match start_device_flow() {
            Err(e) => err.set(Some(e)),
            Ok(resp) => {
                uc.set(Some(resp.user_code.clone()));
                let _ = open::that(&resp.verification_uri);
                match poll_for_token(&resp.device_code, resp.interval) {
                    Ok(t) => {
                        save_token(&t);
                        tok.set(Some(t));
                    }
                    Err(e) => err.set(Some(e)),
                }
            }
        });
        s
    }

    fn render(&self) -> View {
        let body: Vec<View> = if let Some(err) = self.error.get() {
            vec![row(vec![
                rect(RED).width(3.0).height(40.0).into(),
                column(vec![
                    text("Authentication failed", TEXT_SM)
                        .weight(FontWeight::Bold)
                        .color(RED)
                        .into(),
                    text(err, TEXT_XS).color(MUTED).wrap().into(),
                ])
                .gap(SPACE_1)
                .padding_x(SPACE_3)
                .into(),
            ])
            .bg(Color::rgba(0.953, 0.369, 0.369, 0.08))
            .border(Color::rgba(0.953, 0.369, 0.369, 0.25), 1.0)
            .radius(RADIUS_MD)
            .into()]
        } else if let Some(code) = self.user_code.get() {
            vec![
                text("Open in your browser:", TEXT_SM).color(MUTED).into(),
                text("github.com/login/device", TEXT_SM)
                    .weight(FontWeight::Bold)
                    .color(BLUE)
                    .into(),
                sp(SPACE_2),
                text("Enter this code:", TEXT_SM).color(MUTED).into(),
                column(vec![text(code, 32.0)
                    .weight(FontWeight::Bold)
                    .color(TEXT)
                    .into()])
                .padding(SPACE_4)
                .bg(SURFACE2)
                .border(BORDER, 1.0)
                .radius(RADIUS_LG)
                .into(),
                sp(SPACE_2),
                row(vec![
                    rect(GREEN).width(6.0).height(6.0).radius(3.0).into(),
                    text("Waiting for authorization…", TEXT_XS)
                        .color(MUTED)
                        .into(),
                ])
                .gap(SPACE_2)
                .into(),
            ]
        } else {
            vec![row(vec![
                rect(BLUE).width(6.0).height(6.0).radius(3.0).into(),
                text("Opening browser…", TEXT_SM).color(MUTED).into(),
            ])
            .gap(SPACE_2)
            .into()]
        };

        // Full-screen centered card
        column(vec![column(vec![
            // Header
            row(vec![
                image(asset("github_mark.png")).size(28.0, 28.0).into(),
                text("GitHub", TEXT_LG)
                    .weight(FontWeight::Bold)
                    .color(TEXT)
                    .into(),
            ])
            .gap(SPACE_3)
            .into(),
            sp(SPACE_2),
            text("Sign in to your account", TEXT_XL)
                .weight(FontWeight::Bold)
                .color(TEXT)
                .into(),
            text(
                "Glyph needs read access to your repos and notifications.",
                TEXT_SM,
            )
            .color(MUTED)
            .wrap()
            .into(),
            sp(SPACE_4),
            rect(BORDER).height(1.0).fill_width().into(),
            sp(SPACE_4),
            column(body).gap(SPACE_3).into(),
        ])
        .gap(0.0)
        .padding(SPACE_8)
        .width(420.0)
        .bg(SURFACE)
        .border(BORDER, 1.0)
        .radius(RADIUS_LG)
        .shadow(shadow_dark_lg())
        .into()])
        .padding(80.0)
        .align_center()
        .fill_width()
        .into()
    }
}

// --- Dashboard ---

struct DashboardScreen {
    token: String,
    tab: Signal<Tab>,
    data: Signal<Option<DashboardData>>,
    error: Signal<Option<String>>,
    scroll_y: Signal<f32>,
    scroll_x: Signal<f32>,
    max_scroll: Signal<(f32, f32)>,
}

impl DashboardScreen {
    fn new(token: String) -> Self {
        let s = Self {
            token: token.clone(),
            tab: Signal::new(Tab::Overview),
            data: Signal::new(None),
            error: Signal::new(None),
            scroll_y: Signal::new(0.0),
            scroll_x: Signal::new(0.0),
            max_scroll: Signal::new((-1.0, -1.0)),
        };
        s.refresh();
        s
    }

    fn refresh(&self) {
        let (token, data, error) = (self.token.clone(), self.data.clone(), self.error.clone());
        self.data.set(None);
        self.error.set(None);
        thread::spawn(move || match fetch_dashboard(&token) {
            Ok(d) => data.set(Some(d)),
            Err(e) => error.set(Some(e)),
        });
    }

    fn render(&self) -> View {
        let tab = self.tab.get();

        let avatar_path = self.data.get().as_ref().and_then(|d| d.avatar_path.clone());
        let topnav = self.render_topnav(&tab, avatar_path.as_deref());

        let content = match &tab {
            Tab::Overview => self.render_overview(),
            Tab::Repos => self.render_repos(),
            Tab::Notifications => self.render_notifications(),
            Tab::PullRequests => self.render_prs(),
        };

        let body = scroll(
            content,
            self.scroll_x.clone(),
            self.scroll_y.clone(),
            self.max_scroll.clone(),
        )
        .fill_width()
        .grow()
        .into();

        column(vec![topnav, body])
            .gap(0.0)
            .fill_width()
            .fill_height()
            .into()
    }

    fn render_topnav(&self, tab: &Tab, avatar_path: Option<&str>) -> View {
        let login = self
            .data
            .get()
            .as_ref()
            .map(|d| d.user.login.clone())
            .unwrap_or_default();

        let unread = self
            .data
            .get()
            .as_ref()
            .map(|d| d.notifications.iter().filter(|n| n.unread).count())
            .unwrap_or(0);

        let tabs: &[(&str, Tab)] = &[
            ("Overview", Tab::Overview),
            ("Repositories", Tab::Repos),
            ("Notifications", Tab::Notifications),
            ("Pull Requests", Tab::PullRequests),
        ];

        let tab_items: Vec<View> = tabs
            .iter()
            .map(|(label, t)| {
                let is_active = tab == t;
                let tab_sig = self.tab.clone();
                let t = t.clone();
                let fg = if is_active { TEXT } else { MUTED };
                let underline = if is_active {
                    ORANGE
                } else {
                    Color::TRANSPARENT
                };
                let hover_bg = if is_active {
                    Color::TRANSPARENT
                } else {
                    Color::rgba(1.0, 1.0, 1.0, 0.06)
                };

                // Build label with optional inline badge
                let has_badge = matches!(t, Tab::Notifications) && unread > 0;
                let badge_str = if has_badge {
                    format!("  {}", unread)
                } else {
                    String::new()
                };
                let btn_label = format!("{}{}", label, badge_str);

                let btn = button(btn_label, move || {
                    if !is_active {
                        tab_sig.set(t.clone());
                    }
                })
                .bg(Color::TRANSPARENT)
                .hover_bg(hover_bg)
                .text_color(fg)
                .font_size(13.0)
                .padding(SPACE_3)
                .height(36.0)
                .into();

                column(vec![btn, rect(underline).height(2.0).fill_width().into()])
                    .gap(0.0)
                    .auto_size()
                    .into()
            })
            .collect();

        // Top black bar
        let black_bar = row(vec![
            // Left: logos
            row(vec![
                image(asset("glyph_logo.png"))
                    .size(20.0, 20.0)
                    .radius(4.0)
                    .into(),
                image(asset("github_mark.png")).size(20.0, 20.0).into(),
            ])
            .gap(SPACE_3)
            .into(),
            spacer(),
            // Right: username
            if !login.is_empty() {
                let avatar_view: View = if let Some(path) = avatar_path {
                    image(path).size(24.0, 24.0).radius(12.0).into()
                } else {
                    column(vec![text(
                        login
                            .chars()
                            .next()
                            .map(|c| c.to_uppercase().to_string())
                            .unwrap_or_default(),
                        11.0,
                    )
                    .weight(FontWeight::Bold)
                    .color(TEXT)
                    .into()])
                    .width(24.0)
                    .height(24.0)
                    .bg(Color::rgb(0.18, 0.18, 0.20))
                    .border(BORDER, 1.0)
                    .radius(12.0)
                    .align_center()
                    .justify_center()
                    .into()
                };
                row(vec![
                    avatar_view,
                    text(&login, TEXT_SM)
                        .weight(FontWeight::Bold)
                        .color(TEXT)
                        .into(),
                ])
                .gap(SPACE_2)
                .into()
            } else {
                spacer()
            },
        ])
        .gap(SPACE_4)
        .padding_x(SPACE_6)
        .padding_y(SPACE_3)
        .fill_width()
        .bg(TOPNAV)
        .into();

        // Tab bar below — left-aligned
        let tab_bar = column(vec![
            row(tab_items).gap(0.0).justify_start().into(),
            rect(BORDER).height(1.0).fill_width().into(),
        ])
        .gap(0.0)
        .padding_x(SPACE_6)
        .fill_width()
        .bg(SURFACE)
        .into();

        column(vec![black_bar, tab_bar]).gap(0.0).into()
    }

    fn render_overview(&self) -> View {
        let Some(data) = self.data.get() else {
            return self.loading_or_error();
        };
        let u = &data.user;

        let pinned: Vec<&Repo> = data
            .repos
            .iter()
            .filter(|r| !r.fork && r.full_name.starts_with(&format!("{}/", u.login)))
            .take(6)
            .collect();

        let sidebar = self.render_profile_sidebar(u, data.avatar_path.as_deref());
        let main = self.render_overview_main(u, &pinned, &data.repos);

        row(vec![sidebar, main])
            .gap(0.0)
            .fill_width()
            .align_start()
            .into()
    }

    fn render_profile_sidebar(&self, u: &User, avatar_path: Option<&str>) -> View {
        let initial = u
            .login
            .chars()
            .next()
            .map(|c| c.to_uppercase().to_string())
            .unwrap_or_default();

        let avatar_view: View = if let Some(path) = avatar_path {
            image(path).size(88.0, 88.0).radius(44.0).into()
        } else {
            column(vec![text(initial, 36.0)
                .weight(FontWeight::Bold)
                .color(TEXT)
                .into()])
            .width(88.0)
            .height(88.0)
            .bg(Color::rgb(0.15, 0.15, 0.17))
            .border(BORDER, 2.0)
            .radius(44.0)
            .align_center()
            .justify_center()
            .into()
        };

        let mut items: Vec<View> = vec![
            avatar_view,
            sp(SPACE_3),
            // Name + login
            text(u.name.as_deref().unwrap_or(&u.login), 22.0)
                .weight(FontWeight::Bold)
                .color(TEXT)
                .into(),
            text(format!("@{}", u.login), TEXT_BASE).color(MUTED).into(),
            sp(SPACE_3),
        ];

        // Bio
        if let Some(bio) = &u.bio {
            if !bio.is_empty() {
                items.push(text(bio, TEXT_SM).color(TEXT).wrap().into());
                items.push(sp(SPACE_3));
            }
        }

        // Follow button
        items.push(
            button("Follow", || {})
                .bg(SURFACE2)
                .hover_bg(Color::rgb(0.20, 0.21, 0.23))
                .text_color(TEXT)
                .border_not_supported_use_container_border()
                .font_size(TEXT_SM)
                .height(30.0)
                .padding(SPACE_4)
                .radius(RADIUS_MD)
                .into(),
        );

        items.push(sp(SPACE_4));
        items.push(rect(BORDER).height(1.0).fill_width().into());
        items.push(sp(SPACE_4));

        // Followers / following
        items.push(
            row(vec![
                text(format_count(u.followers), TEXT_SM)
                    .weight(FontWeight::Bold)
                    .color(TEXT)
                    .into(),
                text(" followers", TEXT_SM).color(MUTED).into(),
                text("  ·  ", TEXT_SM).color(SUBTLE).into(),
                text(format_count(u.following), TEXT_SM)
                    .weight(FontWeight::Bold)
                    .color(TEXT)
                    .into(),
                text(" following", TEXT_SM).color(MUTED).into(),
            ])
            .gap(0.0)
            .into(),
        );

        // Company / location / blog
        if let Some(company) = &u.company {
            if !company.is_empty() {
                items.push(sp(SPACE_2));
                items.push(
                    row(vec![
                        text("@", TEXT_SM).color(SUBTLE).into(),
                        text(company.trim_start_matches('@'), TEXT_SM)
                            .color(MUTED)
                            .into(),
                    ])
                    .gap(0.0)
                    .into(),
                );
            }
        }
        if let Some(location) = &u.location {
            if !location.is_empty() {
                items.push(sp(SPACE_2));
                items.push(text(location, TEXT_SM).color(MUTED).into());
            }
        }
        if let Some(blog) = &u.blog {
            if !blog.is_empty() {
                items.push(sp(SPACE_2));
                items.push(text(blog, TEXT_SM).color(BLUE).into());
            }
        }

        column(items).gap(0.0).padding(SPACE_6).width(280.0).into()
    }

    fn render_overview_main(&self, _u: &User, pinned: &[&Repo], all_repos: &[Repo]) -> View {
        // Pinned repos grid — 2 columns
        let pinned_rows: Vec<View> = pinned
            .chunks(2)
            .map(|chunk| {
                if chunk.len() == 2 {
                    row(vec![
                        column(vec![repo_card(chunk[0])]).grow().into(),
                        column(vec![repo_card(chunk[1])]).grow().into(),
                    ])
                    .gap(SPACE_3)
                    .fill_width()
                    .align_start()
                    .into()
                } else {
                    row(vec![
                        column(vec![repo_card(chunk[0])]).grow().into(),
                        column(vec![]).grow().into(),
                    ])
                    .gap(SPACE_3)
                    .fill_width()
                    .align_start()
                    .into()
                }
            })
            .collect();

        // Recent activity section — last 5 pushed repos
        let recent: Vec<&Repo> = all_repos.iter().take(5).collect();

        column(vec![
            // Pinned
            if !pinned.is_empty() {
                column(vec![
                    row(vec![
                        text("Pinned", TEXT_SM)
                            .weight(FontWeight::Bold)
                            .color(TEXT)
                            .into(),
                        spacer(),
                        text("Customize your pins", TEXT_XS).color(BLUE).into(),
                    ])
                    .fill_width()
                    .into(),
                    sp(SPACE_3),
                    column(pinned_rows).gap(SPACE_3).into(),
                ])
                .gap(0.0)
                .into()
            } else {
                column(vec![]).into()
            },
            sp(SPACE_6),
            // Recent activity
            column(vec![
                text("Recent activity", TEXT_SM)
                    .weight(FontWeight::Bold)
                    .color(TEXT)
                    .into(),
                sp(SPACE_3),
                column(recent.iter().map(|r| activity_row(r)).collect())
                    .gap(0.0)
                    .bg(SURFACE)
                    .border(BORDER, 1.0)
                    .radius(RADIUS_LG)
                    .clip()
                    .into(),
            ])
            .gap(0.0)
            .into(),
        ])
        .gap(0.0)
        .padding(SPACE_6)
        .fill_width()
        .into()
    }

    fn render_repos(&self) -> View {
        let Some(data) = self.data.get() else {
            return self.loading_or_error();
        };

        let total = data.repos.len();
        let own: Vec<&Repo> = data.repos.iter().filter(|r| !r.fork).collect();
        let forked: Vec<&Repo> = data.repos.iter().filter(|r| r.fork).collect();

        column(vec![
            // Header row
            row(vec![
                column(vec![row(vec![
                    text("Repositories", TEXT_LG)
                        .weight(FontWeight::Bold)
                        .color(TEXT)
                        .into(),
                    badge_colored(total.to_string(), Color::rgba(1.0, 1.0, 1.0, 0.08), MUTED),
                ])
                .gap(SPACE_2)
                .into()])
                .into(),
                spacer(),
                button("New", || {})
                    .bg(GREEN)
                    .hover_bg(Color::rgb(0.18, 0.60, 0.25))
                    .text_color(Color::rgb(0.012, 0.027, 0.020))
                    .font_size(TEXT_SM)
                    .height(30.0)
                    .padding(SPACE_3)
                    .radius(RADIUS_MD)
                    .into(),
            ])
            .fill_width()
            .into(),
            sp(SPACE_4),
            // Repo list — owned first
            if !own.is_empty() {
                column(vec![
                    text("Owned", TEXT_XS)
                        .weight(FontWeight::Bold)
                        .color(SUBTLE)
                        .into(),
                    sp(SPACE_2),
                    column(own.iter().map(|r| repo_list_row(r)).collect())
                        .gap(0.0)
                        .bg(SURFACE)
                        .border(BORDER, 1.0)
                        .radius(RADIUS_LG)
                        .clip()
                        .into(),
                ])
                .gap(0.0)
                .into()
            } else {
                column(vec![]).into()
            },
            sp(SPACE_6),
            // Forked
            if !forked.is_empty() {
                column(vec![
                    text("Forked", TEXT_XS)
                        .weight(FontWeight::Bold)
                        .color(SUBTLE)
                        .into(),
                    sp(SPACE_2),
                    column(forked.iter().map(|r| repo_list_row(r)).collect())
                        .gap(0.0)
                        .bg(SURFACE)
                        .border(BORDER, 1.0)
                        .radius(RADIUS_LG)
                        .clip()
                        .into(),
                ])
                .gap(0.0)
                .into()
            } else {
                column(vec![]).into()
            },
        ])
        .gap(0.0)
        .padding(SPACE_6)
        .fill_width()
        .into()
    }

    fn render_notifications(&self) -> View {
        let Some(data) = self.data.get() else {
            return self.loading_or_error();
        };

        if data.notifications.is_empty() {
            return column(vec![
                sp(SPACE_16),
                column(vec![
                    text("All caught up!", TEXT_LG)
                        .weight(FontWeight::Bold)
                        .color(TEXT)
                        .into(),
                    text("Nothing new in your inbox.", TEXT_SM)
                        .color(MUTED)
                        .into(),
                ])
                .gap(SPACE_2)
                .align_center()
                .into(),
            ])
            .padding(SPACE_6)
            .align_center()
            .fill_width()
            .into();
        }

        let unread: Vec<&Notification> = data.notifications.iter().filter(|n| n.unread).collect();
        let read: Vec<&Notification> = data.notifications.iter().filter(|n| !n.unread).collect();

        column(vec![
            row(vec![
                text("Inbox", TEXT_LG)
                    .weight(FontWeight::Bold)
                    .color(TEXT)
                    .into(),
                if !unread.is_empty() {
                    badge_colored(
                        unread.len().to_string(),
                        Color::rgba(0.345, 0.651, 1.0, 0.18),
                        BLUE,
                    )
                } else {
                    column(vec![]).into()
                },
                spacer(),
                button("Mark all read", || {})
                    .bg(Color::TRANSPARENT)
                    .hover_bg(Color::rgba(1.0, 1.0, 1.0, 0.06))
                    .text_color(MUTED)
                    .font_size(TEXT_XS)
                    .height(28.0)
                    .padding(SPACE_2)
                    .radius(RADIUS_MD)
                    .into(),
            ])
            .gap(SPACE_2)
            .fill_width()
            .into(),
            sp(SPACE_4),
            if !unread.is_empty() {
                column(vec![
                    text("Unread", TEXT_XS)
                        .weight(FontWeight::Bold)
                        .color(SUBTLE)
                        .into(),
                    sp(SPACE_2),
                    column(unread.iter().map(|n| notif_row(n)).collect())
                        .gap(0.0)
                        .bg(SURFACE)
                        .border(BORDER, 1.0)
                        .radius(RADIUS_LG)
                        .clip()
                        .into(),
                ])
                .gap(0.0)
                .into()
            } else {
                column(vec![]).into()
            },
            sp(SPACE_6),
            if !read.is_empty() {
                column(vec![
                    text("Read", TEXT_XS)
                        .weight(FontWeight::Bold)
                        .color(SUBTLE)
                        .into(),
                    sp(SPACE_2),
                    column(read.iter().map(|n| notif_row(n)).collect())
                        .gap(0.0)
                        .bg(SURFACE)
                        .border(BORDER, 1.0)
                        .radius(RADIUS_LG)
                        .clip()
                        .into(),
                ])
                .gap(0.0)
                .into()
            } else {
                column(vec![]).into()
            },
        ])
        .gap(0.0)
        .padding(SPACE_6)
        .fill_width()
        .into()
    }

    fn render_prs(&self) -> View {
        let Some(data) = self.data.get() else {
            return self.loading_or_error();
        };

        if data.prs.is_empty() {
            return column(vec![
                sp(SPACE_16),
                column(vec![
                    text("No open pull requests", TEXT_LG)
                        .weight(FontWeight::Bold)
                        .color(TEXT)
                        .into(),
                    text("No open PRs assigned to you right now.", TEXT_SM)
                        .color(MUTED)
                        .into(),
                ])
                .gap(SPACE_2)
                .align_center()
                .into(),
            ])
            .padding(SPACE_6)
            .align_center()
            .fill_width()
            .into();
        }

        let open: Vec<&PullRequest> = data.prs.iter().filter(|p| p.state == "open").collect();
        let closed: Vec<&PullRequest> = data.prs.iter().filter(|p| p.state != "open").collect();

        column(vec![
            row(vec![
                text("Pull Requests", TEXT_LG)
                    .weight(FontWeight::Bold)
                    .color(TEXT)
                    .into(),
                badge_colored(
                    open.len().to_string(),
                    Color::rgba(0.247, 0.722, 0.314, 0.18),
                    GREEN,
                ),
            ])
            .gap(SPACE_2)
            .into(),
            sp(SPACE_4),
            if !open.is_empty() {
                column(vec![
                    text("Open", TEXT_XS)
                        .weight(FontWeight::Bold)
                        .color(SUBTLE)
                        .into(),
                    sp(SPACE_2),
                    column(open.iter().map(|p| pr_row(p)).collect())
                        .gap(0.0)
                        .bg(SURFACE)
                        .border(BORDER, 1.0)
                        .radius(RADIUS_LG)
                        .clip()
                        .into(),
                ])
                .gap(0.0)
                .into()
            } else {
                column(vec![]).into()
            },
            sp(SPACE_6),
            if !closed.is_empty() {
                column(vec![
                    text("Closed / Merged", TEXT_XS)
                        .weight(FontWeight::Bold)
                        .color(SUBTLE)
                        .into(),
                    sp(SPACE_2),
                    column(closed.iter().map(|p| pr_row(p)).collect())
                        .gap(0.0)
                        .bg(SURFACE)
                        .border(BORDER, 1.0)
                        .radius(RADIUS_LG)
                        .clip()
                        .into(),
                ])
                .gap(0.0)
                .into()
            } else {
                column(vec![]).into()
            },
        ])
        .gap(0.0)
        .padding(SPACE_6)
        .fill_width()
        .into()
    }

    fn loading_or_error(&self) -> View {
        if let Some(err) = self.error.get() {
            column(vec![
                sp(SPACE_8),
                row(vec![
                    rect(RED).width(3.0).height(48.0).into(),
                    column(vec![
                        text("Failed to load", TEXT_SM)
                            .weight(FontWeight::Bold)
                            .color(RED)
                            .into(),
                        text(err, TEXT_XS).color(MUTED).wrap().into(),
                    ])
                    .gap(SPACE_1)
                    .padding_x(SPACE_3)
                    .into(),
                ])
                .bg(Color::rgba(0.953, 0.369, 0.369, 0.08))
                .border(Color::rgba(0.953, 0.369, 0.369, 0.25), 1.0)
                .radius(RADIUS_MD)
                .fill_width()
                .into(),
            ])
            .padding(SPACE_6)
            .into()
        } else {
            column(vec![
                sp(SPACE_12),
                row(vec![
                    rect(BLUE).width(6.0).height(6.0).radius(3.0).into(),
                    text("Loading…", TEXT_SM).color(MUTED).into(),
                ])
                .gap(SPACE_2)
                .into(),
            ])
            .gap(0.0)
            .padding(SPACE_6)
            .align_center()
            .fill_width()
            .into()
        }
    }
}

// --- Row / card components ---

// Compact card used in the 2-col pinned grid
fn repo_card(repo: &Repo) -> View {
    let parts: Vec<&str> = repo.full_name.splitn(2, '/').collect();
    let repo_name = if parts.len() == 2 {
        parts[1]
    } else {
        &repo.full_name
    };
    let url = repo.html_url.clone();

    let mut footer: Vec<View> = vec![];
    if let Some(lang) = &repo.language {
        footer.push(lang_dot(lang));
        footer.push(text(lang, TEXT_XS).color(MUTED).into());
    }
    if repo.stargazers_count > 0 {
        if !footer.is_empty() {
            footer.push(hsp(SPACE_3));
        }
        footer.push(
            row(vec![
                image(asset("star.png")).size(12.0, 12.0).into(),
                text(format_count(repo.stargazers_count), TEXT_XS)
                    .color(MUTED)
                    .into(),
            ])
            .gap(SPACE_1)
            .into(),
        );
    }

    column(vec![
        // Header: name + visibility badge
        row(vec![
            column(vec![row(vec![
                if repo.private {
                    image(asset("lock.png")).size(12.0, 12.0).into()
                } else {
                    rect(Color::TRANSPARENT).width(1.0).height(1.0).into()
                },
                button(repo_name, move || {
                    let _ = open::that(&url);
                })
                .bg(Color::TRANSPARENT)
                .hover_bg(Color::TRANSPARENT)
                .text_color(BLUE)
                .font_size(TEXT_SM)
                .padding(0.0)
                .into(),
            ])
            .gap(SPACE_2)
            .into()])
            .grow()
            .into(),
            if repo.private {
                badge_colored("Private", Color::rgba(1.0, 1.0, 1.0, 0.06), MUTED)
            } else {
                badge_colored("Public", Color::rgba(1.0, 1.0, 1.0, 0.04), SUBTLE)
            },
        ])
        .fill_width()
        .into(),
        // Description (or empty filler to push footer down)
        if let Some(desc) = &repo.description {
            if !desc.is_empty() {
                text(desc, TEXT_XS).color(MUTED).wrap().into()
            } else {
                spacer()
            }
        } else {
            spacer()
        },
        // Footer: language dot + stars
        if !footer.is_empty() {
            row(footer).gap(SPACE_1).into()
        } else {
            rect(Color::TRANSPARENT).height(14.0).into()
        },
    ])
    .gap(SPACE_2)
    .padding(SPACE_4)
    .fill_width()
    .min_size(0.0, 90.0)
    .bg(SURFACE)
    .border(BORDER, 1.0)
    .radius(RADIUS_LG)
    .into()
}

// Full-width row used in activity feed
fn activity_row(repo: &Repo) -> View {
    let url = repo.html_url.clone();
    let parts: Vec<&str> = repo.full_name.splitn(2, '/').collect();
    let (owner, name) = if parts.len() == 2 {
        (parts[0], parts[1])
    } else {
        ("", repo.full_name.as_str())
    };

    let mut meta: Vec<View> = vec![];
    if let Some(lang) = &repo.language {
        meta.push(lang_dot(lang));
        meta.push(text(lang, TEXT_XS).color(MUTED).into());
    }
    if repo.stargazers_count > 0 {
        if !meta.is_empty() {
            meta.push(hsp(SPACE_4));
        }
        meta.push(
            row(vec![
                image(asset("star.png")).size(11.0, 11.0).into(),
                text(format_count(repo.stargazers_count), TEXT_XS)
                    .color(MUTED)
                    .into(),
            ])
            .gap(SPACE_1)
            .into(),
        );
    }
    if repo.forks_count > 0 {
        if !meta.is_empty() {
            meta.push(hsp(SPACE_4));
        }
        meta.push(
            row(vec![
                text("⑂", TEXT_XS).color(MUTED).into(),
                text(format_count(repo.forks_count), TEXT_XS)
                    .color(MUTED)
                    .into(),
            ])
            .gap(SPACE_1)
            .into(),
        );
    }

    column(vec![
        row(vec![column(vec![
            row(vec![
                text(owner, TEXT_SM).color(MUTED).into(),
                text("/", TEXT_SM).color(SUBTLE).into(),
                button(name, move || {
                    let _ = open::that(&url);
                })
                .bg(Color::TRANSPARENT)
                .hover_bg(Color::TRANSPARENT)
                .text_color(BLUE)
                .font_size(TEXT_SM)
                .padding(0.0)
                .into(),
                if repo.private {
                    row(vec![
                        hsp(SPACE_2),
                        badge_colored("Private", Color::rgba(1.0, 1.0, 1.0, 0.06), MUTED),
                    ])
                    .gap(0.0)
                    .into()
                } else {
                    column(vec![]).into()
                },
            ])
            .gap(SPACE_2)
            .align_center()
            .no_wrap()
            .into(),
            if let Some(desc) = &repo.description {
                if !desc.is_empty() {
                    text(desc, TEXT_XS).color(MUTED).wrap().into()
                } else {
                    column(vec![]).into()
                }
            } else {
                column(vec![]).into()
            },
            if !meta.is_empty() {
                row(meta).gap(SPACE_1).into()
            } else {
                column(vec![]).into()
            },
        ])
        .gap(SPACE_1)
        .grow()
        .into()])
        .fill_width()
        .padding_x(SPACE_4)
        .padding_y(SPACE_3)
        .into(),
        rect(BORDER).height(1.0).fill_width().into(),
    ])
    .gap(0.0)
    .into()
}

// Repo row in the repositories tab
fn repo_list_row(repo: &Repo) -> View {
    let url = repo.html_url.clone();
    let parts: Vec<&str> = repo.full_name.splitn(2, '/').collect();
    let (owner, name) = if parts.len() == 2 {
        (parts[0], parts[1])
    } else {
        ("", repo.full_name.as_str())
    };

    let mut meta: Vec<View> = vec![];
    if let Some(lang) = &repo.language {
        meta.push(lang_dot(lang));
        meta.push(text(lang, TEXT_XS).color(MUTED).into());
    }
    if repo.stargazers_count > 0 {
        if !meta.is_empty() {
            meta.push(hsp(SPACE_6));
        }
        meta.push(
            row(vec![
                image(asset("star.png")).size(12.0, 12.0).into(),
                text(format_count(repo.stargazers_count), TEXT_XS)
                    .color(MUTED)
                    .into(),
            ])
            .gap(SPACE_1)
            .into(),
        );
    }
    if repo.forks_count > 0 {
        if !meta.is_empty() {
            meta.push(hsp(SPACE_6));
        }
        meta.push(
            row(vec![
                text("⑂", TEXT_XS).color(MUTED).into(),
                text(format_count(repo.forks_count), TEXT_XS)
                    .color(MUTED)
                    .into(),
            ])
            .gap(SPACE_1)
            .into(),
        );
    }
    if let Some(updated) = &repo.updated_at {
        if !meta.is_empty() {
            meta.push(hsp(SPACE_6));
        }
        meta.push(
            text(format!("Updated {}", short_date(updated)), TEXT_XS)
                .color(MUTED)
                .into(),
        );
    }

    column(vec![
        row(vec![column(vec![
            row(vec![
                text(owner, TEXT_SM).color(MUTED).into(),
                text("/", TEXT_SM).color(SUBTLE).into(),
                button(name, move || {
                    let _ = open::that(&url);
                })
                .bg(Color::TRANSPARENT)
                .hover_bg(Color::TRANSPARENT)
                .text_color(BLUE)
                .font_size(TEXT_SM)
                .padding(0.0)
                .into(),
                if repo.fork {
                    row(vec![
                        hsp(SPACE_2),
                        badge_colored("Fork", Color::rgba(1.0, 1.0, 1.0, 0.05), SUBTLE),
                    ])
                    .gap(0.0)
                    .into()
                } else {
                    column(vec![]).into()
                },
                if repo.private {
                    row(vec![
                        hsp(SPACE_2),
                        badge_colored("Private", Color::rgba(1.0, 1.0, 1.0, 0.06), MUTED),
                    ])
                    .gap(0.0)
                    .into()
                } else {
                    column(vec![]).into()
                },
            ])
            .gap(SPACE_2)
            .align_center()
            .no_wrap()
            .into(),
            if let Some(desc) = &repo.description {
                if !desc.is_empty() {
                    text(desc, TEXT_XS).color(MUTED).wrap().into()
                } else {
                    column(vec![]).into()
                }
            } else {
                column(vec![]).into()
            },
            if !meta.is_empty() {
                row(meta).gap(SPACE_1).into()
            } else {
                column(vec![]).into()
            },
        ])
        .gap(SPACE_1)
        .grow()
        .into()])
        .padding_x(SPACE_4)
        .padding_y(SPACE_3)
        .fill_width()
        .into(),
        rect(BORDER).height(1.0).fill_width().into(),
    ])
    .gap(0.0)
    .into()
}

fn notif_row(n: &Notification) -> View {
    let (kind_label, kind_color, kind_bg) = match n.subject.kind.as_str() {
        "PullRequest" => ("PR", GREEN, Color::rgba(0.247, 0.722, 0.314, 0.12)),
        "Issue" => ("Issue", ORANGE, Color::rgba(0.859, 0.427, 0.169, 0.12)),
        "Release" => ("Release", BLUE, Color::rgba(0.345, 0.651, 1.000, 0.12)),
        "Discussion" => ("Discussion", PURPLE, Color::rgba(0.686, 0.431, 0.988, 0.12)),
        other => (other, MUTED, Color::rgba(1.0, 1.0, 1.0, 0.05)),
    };

    let dot_color = if n.unread { BLUE } else { Color::TRANSPARENT };
    let row_bg = if n.unread {
        Color::rgba(0.345, 0.651, 1.0, 0.04)
    } else {
        Color::TRANSPARENT
    };

    column(vec![
        row(vec![
            // Unread dot
            column(vec![rect(dot_color)
                .width(8.0)
                .height(8.0)
                .radius(4.0)
                .into()])
            .width(20.0)
            .align_center()
            .justify_center()
            .into(),
            // Type badge
            badge_colored(kind_label, kind_bg, kind_color),
            // Content
            column(vec![
                row(vec![
                    text(&n.repository.full_name, TEXT_XS).color(MUTED).into(),
                    spacer(),
                    if let Some(ts) = &n.updated_at {
                        text(short_date(ts), TEXT_XS).color(SUBTLE).into()
                    } else {
                        column(vec![]).into()
                    },
                ])
                .fill_width()
                .into(),
                text(&n.subject.title, TEXT_SM).color(TEXT).wrap().into(),
                text(reason_label(&n.reason), TEXT_XS).color(SUBTLE).into(),
            ])
            .gap(SPACE_1)
            .grow()
            .into(),
        ])
        .gap(SPACE_3)
        .padding_x(SPACE_4)
        .padding_y(SPACE_3)
        .fill_width()
        .bg(row_bg)
        .into(),
        rect(BORDER).height(1.0).fill_width().into(),
    ])
    .gap(0.0)
    .into()
}

fn pr_row(pr: &PullRequest) -> View {
    let url = pr.html_url.clone();
    let repo = pr
        .repository_url
        .strip_prefix("https://api.github.com/repos/")
        .unwrap_or(&pr.repository_url);

    let is_draft = pr.draft.unwrap_or(false);
    let (status_color, status_bg, status_label) = if is_draft {
        (MUTED, Color::rgba(1.0, 1.0, 1.0, 0.06), "Draft")
    } else if pr.state == "open" {
        (GREEN, Color::rgba(0.247, 0.722, 0.314, 0.12), "Open")
    } else if pr.state == "merged" {
        (PURPLE, Color::rgba(0.686, 0.431, 0.988, 0.12), "Merged")
    } else {
        (RED, Color::rgba(0.953, 0.369, 0.369, 0.12), "Closed")
    };

    column(vec![
        row(vec![
            // PR icon circle
            column(vec![text("⤴", 13.0).color(status_color).into()])
                .width(28.0)
                .height(28.0)
                .bg(status_bg)
                .border(
                    Color::rgba(status_color.r, status_color.g, status_color.b, 0.3),
                    1.0,
                )
                .radius(14.0)
                .align_center()
                .justify_center()
                .into(),
            column(vec![
                row(vec![
                    text(repo, TEXT_XS).color(MUTED).into(),
                    text(format!("  #{}", pr.number), TEXT_XS)
                        .color(SUBTLE)
                        .into(),
                    spacer(),
                    badge_colored(status_label, status_bg, status_color),
                ])
                .fill_width()
                .into(),
                button(&pr.title, move || {
                    let _ = open::that(&url);
                })
                .bg(Color::TRANSPARENT)
                .hover_bg(Color::TRANSPARENT)
                .text_color(TEXT)
                .font_size(TEXT_SM)
                .padding(0.0)
                .into(),
                row(vec![
                    text(format!("by @{}", pr.user.login), TEXT_XS)
                        .color(MUTED)
                        .into(),
                    if let Some(c) = pr.comments {
                        if c > 0 {
                            row(vec![
                                hsp(SPACE_4),
                                text(
                                    format!("{} comment{}", c, if c == 1 { "" } else { "s" }),
                                    TEXT_XS,
                                )
                                .color(SUBTLE)
                                .into(),
                            ])
                            .gap(0.0)
                            .into()
                        } else {
                            column(vec![]).into()
                        }
                    } else {
                        column(vec![]).into()
                    },
                ])
                .into(),
            ])
            .gap(SPACE_1)
            .grow()
            .into(),
        ])
        .gap(SPACE_3)
        .padding_x(SPACE_4)
        .padding_y(SPACE_3)
        .fill_width()
        .into(),
        rect(BORDER).height(1.0).fill_width().into(),
    ])
    .gap(0.0)
    .into()
}

// --- Helpers ---

fn sp(h: f32) -> View {
    rect(Color::TRANSPARENT).width(1.0).height(h).into()
}

fn hsp(w: f32) -> View {
    rect(Color::TRANSPARENT).width(w).height(1.0).into()
}

fn format_count(n: u32) -> String {
    if n >= 1_000 {
        format!("{:.1}k", n as f32 / 1000.0)
    } else {
        n.to_string()
    }
}

fn short_date(iso: &str) -> String {
    // "2024-11-03T14:22:00Z" → "Nov 3"
    let months = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    let parts: Vec<&str> = iso.split('T').next().unwrap_or("").split('-').collect();
    if parts.len() == 3 {
        let month = parts[1].parse::<usize>().unwrap_or(1).saturating_sub(1);
        let day = parts[2].trim_start_matches('0');
        format!("{} {}", months.get(month).unwrap_or(&"?"), day)
    } else {
        iso[..iso.len().min(10)].to_string()
    }
}

fn reason_label(r: &str) -> &str {
    match r {
        "assign" => "Assigned to you",
        "author" => "You authored this",
        "comment" => "You commented",
        "mention" => "You were mentioned",
        "review_requested" => "Review requested",
        "subscribed" => "Subscribed",
        "team_mention" => "Your team was mentioned",
        _ => r,
    }
}

// Small colored dot indicating a programming language
fn lang_dot(lang: &str) -> View {
    let color = lang_color(lang);
    rect(color).width(10.0).height(10.0).radius(5.0).into()
}

fn lang_color(lang: &str) -> Color {
    match lang {
        "Rust" => Color::rgb(0.871, 0.298, 0.188),
        "TypeScript" => Color::rgb(0.173, 0.561, 0.776),
        "JavaScript" => Color::rgb(0.945, 0.820, 0.157),
        "Python" => Color::rgb(0.220, 0.451, 0.659),
        "Go" => Color::rgb(0.004, 0.643, 0.788),
        "Swift" => Color::rgb(0.980, 0.361, 0.243),
        "Kotlin" => Color::rgb(0.490, 0.349, 0.761),
        "C++" => Color::rgb(0.361, 0.506, 0.722),
        "C" => Color::rgb(0.341, 0.463, 0.631),
        "Java" => Color::rgb(0.702, 0.408, 0.188),
        "Ruby" => Color::rgb(0.702, 0.086, 0.086),
        "Dart" => Color::rgb(0.208, 0.592, 0.827),
        "C#" => Color::rgb(0.373, 0.153, 0.647),
        "HTML" => Color::rgb(0.890, 0.361, 0.161),
        "CSS" => Color::rgb(0.361, 0.408, 0.780),
        "Shell" => Color::rgb(0.557, 0.686, 0.212),
        "Nix" => Color::rgb(0.302, 0.498, 0.761),
        _ => MUTED,
    }
}

// Hack: ButtonView has no border method — we use a workaround
trait ButtonBorderHack {
    fn border_not_supported_use_container_border(self) -> Self;
}
impl ButtonBorderHack for core_glyph::ButtonView {
    fn border_not_supported_use_container_border(self) -> Self {
        self
    }
}

// --- App shell ---

struct GlyphGitHub {
    login: Signal<Option<Arc<LoginScreen>>>,
    dashboard: Signal<Option<Arc<DashboardScreen>>>,
}

impl GlyphGitHub {
    fn new() -> Self {
        let login: Signal<Option<Arc<LoginScreen>>> = Signal::new(None);
        let dashboard: Signal<Option<Arc<DashboardScreen>>> = Signal::new(None);
        let (lt, dt) = (login.clone(), dashboard.clone());
        thread::spawn(move || {
            if let Some(token) = load_token() {
                dt.set(Some(Arc::new(DashboardScreen::new(token))));
            } else {
                let screen = Arc::new(LoginScreen::new());
                let tok = screen.token_out.clone();
                lt.set(Some(screen));
                thread::spawn(move || loop {
                    thread::sleep(std::time::Duration::from_millis(500));
                    if let Some(token) = tok.get() {
                        lt.set(None);
                        dt.set(Some(Arc::new(DashboardScreen::new(token))));
                        break;
                    }
                });
            }
        });
        Self { login, dashboard }
    }

    fn render(&self) -> View {
        if let Some(dash) = self.dashboard.get() {
            return dash.render();
        }
        if let Some(login) = self.login.get() {
            return login.render();
        }
        column(vec![
            sp(SPACE_12),
            row(vec![
                rect(BLUE).width(6.0).height(6.0).radius(3.0).into(),
                text("Starting…", TEXT_SM).color(MUTED).into(),
            ])
            .gap(SPACE_2)
            .into(),
        ])
        .padding(SPACE_6)
        .align_center()
        .fill_width()
        .into()
    }
}

fn main() {
    let app = GlyphGitHub::new();
    App::run(
        move |_opener, _closer| {
            let theme = gh_theme();
            let view = app.render();
            (theme, view)
        },
        gh_theme(),
        "Glyph for GitHub",
        1280.0,
        800.0,
    );
}
