use glyph_core::{
    Color, FontWeight, Signal, Theme, View,
    button, column, row, scroll, spacer, text, rect,
};
use glyph_platform::{App, WindowOpener};
use serde::Deserialize;
use std::{fs, path::PathBuf, sync::Arc, thread};

const CLIENT_ID: &str = env!("GITHUB_CLIENT_ID");

#[derive(Deserialize, Clone)]
struct User {
    login: String,
    name: Option<String>,
    #[allow(dead_code)]
    avatar_url: String,
    public_repos: u32,
    followers: u32,
    following: u32,
}

#[derive(Deserialize, Clone)]
struct Repo {
    full_name: String,
    description: Option<String>,
    stargazers_count: u32,
    language: Option<String>,
    private: bool,
}

#[derive(Deserialize, Clone)]
struct Notification {
    #[allow(dead_code)]
    id: String,
    reason: String,
    subject: NotificationSubject,
    repository: NotificationRepo,
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

fn save_token(token: &str) {
    let path = token_path();
    if let Some(dir) = path.parent() {
        let _ = fs::create_dir_all(dir);
    }
    let _ = fs::write(path, token);
}

fn clear_token() {
    let _ = fs::remove_file(token_path());
}

fn client() -> reqwest::blocking::Client {
    reqwest::blocking::Client::builder()
        .user_agent("glyph-github/0.1")
        .build()
        .expect("http client")
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
    serde_json::from_str(&body)
        .map_err(|e| format!("{e}: {}", &body[..body.len().min(300)]))
}

fn fetch_dashboard(token: &str) -> Result<DashboardData, String> {
    let user: User = api_get(token, "https://api.github.com/user")?;
    let repos: Vec<Repo> =
        api_get(token, "https://api.github.com/user/repos?sort=pushed&per_page=30")?;
    let notifications: Vec<Notification> =
        api_get(token, "https://api.github.com/notifications?per_page=30")?;

    let login = user.login.clone();
    let pr_url = format!(
        "https://api.github.com/search/issues?q=is:pr+is:open+assignee:{}&per_page=20",
        login
    );
    #[derive(Deserialize)]
    struct SearchResult {
        items: Vec<PullRequest>,
    }
    let prs = api_get::<SearchResult>(token, &pr_url)
        .map(|r| r.items)
        .unwrap_or_default();

    Ok(DashboardData { user, repos, notifications, prs })
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

fn poll_for_token(device_code: &str, interval: u64) -> Result<String, String> {
    loop {
        thread::sleep(std::time::Duration::from_secs(interval));
        let body = client()
            .post("https://github.com/login/oauth/access_token")
            .header("Accept", "application/json")
            .form(&[
                ("client_id", CLIENT_ID),
                ("device_code", device_code),
                ("grant_type", "urn:ietf:params:oauth:grant-type:device_code"),
            ])
            .send()
            .map_err(|e| e.to_string())?
            .text()
            .map_err(|e| e.to_string())?;
        let resp: AccessTokenResponse =
            serde_json::from_str(&body).map_err(|e| format!("{e}: {body}"))?;
        match (resp.access_token, resp.error.as_deref()) {
            (Some(token), _) => return Ok(token),
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

#[derive(Clone)]
struct DashboardData {
    user: User,
    repos: Vec<Repo>,
    notifications: Vec<Notification>,
    prs: Vec<PullRequest>,
}

#[derive(Clone, PartialEq)]
enum Tab {
    Overview,
    Repos,
    Notifications,
    PullRequests,
}

struct LoginScreen {
    user_code:  Signal<Option<String>>,
    verify_url: Signal<Option<String>>,
    error:      Signal<Option<String>>,
    token_out:  Signal<Option<String>>,
}

impl LoginScreen {
    fn new() -> Self {
        let screen = Self {
            user_code:  Signal::new(None),
            verify_url: Signal::new(None),
            error:      Signal::new(None),
            token_out:  Signal::new(None),
        };
        screen.start_flow();
        screen
    }

    fn start_flow(&self) {
        let user_code  = self.user_code.clone();
        let verify_url = self.verify_url.clone();
        let error      = self.error.clone();
        let token_out  = self.token_out.clone();

        thread::spawn(move || match start_device_flow() {
            Err(e) => error.set(Some(e)),
            Ok(resp) => {
                user_code.set(Some(resp.user_code.clone()));
                verify_url.set(Some(resp.verification_uri.clone()));
                let _ = open::that(&resp.verification_uri);
                match poll_for_token(&resp.device_code, resp.interval) {
                    Ok(token) => {
                        save_token(&token);
                        token_out.set(Some(token));
                    }
                    Err(e) => error.set(Some(e)),
                }
            }
        });
    }

    fn render_view(&self, theme: &Theme) -> View {
        let gh_blue = Color::rgb(0.24, 0.47, 0.91);

        let content: Vec<View> = if let Some(err) = self.error.get() {
            vec![
                text("Authentication failed", 18.0)
                    .weight(FontWeight::Bold)
                    .color(Color::rgb(0.9, 0.3, 0.3))
                    .into(),
                text(err, 14.0).color(theme.text_muted).wrap().into(),
            ]
        } else if let Some(code) = self.user_code.get() {
            vec![
                text("Enter this code at", 14.0).color(theme.text_muted).into(),
                text("github.com/login/device", 14.0)
                    .weight(FontWeight::Bold)
                    .color(gh_blue)
                    .into(),
                rect(theme.border).fill_width().height(1.0).into(),
                text(code, 42.0)
                    .weight(FontWeight::Bold)
                    .color(theme.text)
                    .into(),
                text("Browser opened automatically. Waiting for authorization...", 13.0)
                    .color(theme.text_muted)
                    .wrap()
                    .into(),
            ]
        } else {
            vec![
                text("Connecting to GitHub...", 14.0).color(theme.text_muted).into(),
            ]
        };

        // Centered card
        let card = column(content)
            .gap(16.0)
            .padding(40.0)
            .bg(theme.surface)
            .radius(12.0)
            .width(420.0)
            .into();

        let header = column(vec![
            text("Glyph for GitHub", 22.0)
                .weight(FontWeight::Bold)
                .color(theme.text)
                .into(),
            text("Sign in to continue", 14.0).color(theme.text_muted).into(),
        ])
        .gap(6.0)
        .into();

        column(vec![header, card])
            .gap(24.0)
            .padding(64.0)
            .fill_width()
            .into()
    }
}

struct DashboardScreen {
    token:      String,
    tab:        Signal<Tab>,
    data:       Signal<Option<DashboardData>>,
    error:      Signal<Option<String>>,
    scroll_y:   Signal<f32>,
    scroll_x:   Signal<f32>,
    max_scroll: Signal<(f32, f32)>,
}

impl DashboardScreen {
    fn new(token: String) -> Self {
        let data: Signal<Option<DashboardData>> = Signal::new(None);
        let error: Signal<Option<String>> = Signal::new(None);
        let s = Self {
            token: token.clone(),
            tab: Signal::new(Tab::Overview),
            data,
            error,
            scroll_y:   Signal::new(0.0),
            scroll_x:   Signal::new(0.0),
            max_scroll: Signal::new((0.0, 0.0)),
        };
        s.refresh();
        s
    }

    fn refresh(&self) {
        let token = self.token.clone();
        let data  = self.data.clone();
        let error = self.error.clone();
        self.data.set(None);
        self.error.set(None);
        thread::spawn(move || match fetch_dashboard(&token) {
            Ok(d)  => data.set(Some(d)),
            Err(e) => error.set(Some(e)),
        });
    }

    fn render_view(&self, theme: &Theme) -> View {
        let tab = self.tab.get();
        let sidebar = self.render_sidebar(theme, &tab);
        let content = match &tab {
            Tab::Overview      => self.render_overview(theme),
            Tab::Repos         => self.render_repos(theme),
            Tab::Notifications => self.render_notifications(theme),
            Tab::PullRequests  => self.render_prs(theme),
        };
        let content_scroll = scroll(
            content,
            self.scroll_x.clone(),
            self.scroll_y.clone(),
            self.max_scroll.clone(),
        )
        .fill_width()
        .into();

        row(vec![sidebar, content_scroll])
            .fill_width()
            .into()
    }

    fn render_sidebar(&self, theme: &Theme, active: &Tab) -> View {
        let nav: &[(&str, Tab)] = &[
            ("Overview",       Tab::Overview),
            ("Repositories",   Tab::Repos),
            ("Notifications",  Tab::Notifications),
            ("Pull Requests",  Tab::PullRequests),
        ];

        let mut items: Vec<View> = vec![
            // App name
            text("GitHub", 15.0)
                .weight(FontWeight::Bold)
                .color(theme.text)
                .into(),
            spacer_h(8.0),
        ];

        for (label, t) in nav {
            let is_active = active == t;
            let tab_sig = self.tab.clone();
            let t = t.clone();
            let nav_item = if is_active {
                button(*label, move || {})
                    .bg(Color::rgba(0.35, 0.55, 1.0, 0.15))
                    .text_color(Color::rgb(0.45, 0.65, 1.0))
                    .radius(6.0)
                    .font_size(14.0)
                    .width(176.0)
                    .into()
            } else {
                let tc = theme.text_muted;
                button(*label, move || { tab_sig.set(t.clone()); })
                    .bg(Color::TRANSPARENT)
                    .text_color(tc)
                    .radius(6.0)
                    .font_size(14.0)
                    .hover_bg(Color::rgba(1.0, 1.0, 1.0, 0.05))
                    .width(176.0)
                    .into()
            };
            items.push(nav_item);
        }

        items.push(spacer());

        items.push(
            button("Sign out", move || {
                clear_token();
                std::process::exit(0);
            })
            .bg(Color::TRANSPARENT)
            .text_color(theme.text_muted)
            .hover_bg(Color::rgba(1.0, 1.0, 1.0, 0.05))
            .radius(6.0)
            .font_size(13.0)
            .width(176.0)
            .into(),
        );

        column(items)
            .gap(2.0)
            .padding(16.0)
            .width(208.0)
            .bg(Color::rgb(0.12, 0.12, 0.14))
            .into()
    }

    fn render_overview(&self, theme: &Theme) -> View {
        let Some(data) = self.data.get() else {
            return self.loading_or_error(theme);
        };
        let u = &data.user;
        let notif_count = data.notifications.len();
        let pr_count = data.prs.len();

        column(vec![
            text(u.name.as_deref().unwrap_or(&u.login), 26.0)
                .weight(FontWeight::Bold)
                .color(theme.text)
                .into(),
            text(format!("@{}", u.login), 14.0).color(theme.text_muted).into(),
            spacer_h(8.0),
            row(vec![
                stat_card("Repos",         u.public_repos,       theme),
                stat_card("Followers",     u.followers,          theme),
                stat_card("Following",     u.following,          theme),
                stat_card("Notifications", notif_count as u32,   theme),
                stat_card("Open PRs",      pr_count as u32,      theme),
            ])
            .gap(10.0)
            .into(),
        ])
        .gap(6.0)
        .padding(32.0)
        .fill_width()
        .into()
    }

    fn render_repos(&self, theme: &Theme) -> View {
        let Some(data) = self.data.get() else {
            return self.loading_or_error(theme);
        };
        let mut rows: Vec<View> = vec![
            text("Repositories", 20.0)
                .weight(FontWeight::Bold)
                .color(theme.text)
                .into(),
        ];
        for repo in &data.repos {
            rows.push(repo_card(repo, theme));
        }
        column(rows).gap(8.0).padding(32.0).fill_width().into()
    }

    fn render_notifications(&self, theme: &Theme) -> View {
        let Some(data) = self.data.get() else {
            return self.loading_or_error(theme);
        };
        let mut rows: Vec<View> = vec![
            text("Notifications", 20.0)
                .weight(FontWeight::Bold)
                .color(theme.text)
                .into(),
        ];
        if data.notifications.is_empty() {
            rows.push(
                text("All caught up.", 14.0)
                    .color(theme.text_muted)
                    .into(),
            );
        }
        for n in &data.notifications {
            rows.push(notification_card(n, theme));
        }
        column(rows).gap(8.0).padding(32.0).fill_width().into()
    }

    fn render_prs(&self, theme: &Theme) -> View {
        let Some(data) = self.data.get() else {
            return self.loading_or_error(theme);
        };
        let mut rows: Vec<View> = vec![
            text("Pull Requests", 20.0)
                .weight(FontWeight::Bold)
                .color(theme.text)
                .into(),
        ];
        if data.prs.is_empty() {
            rows.push(
                text("No open PRs assigned to you.", 14.0)
                    .color(theme.text_muted)
                    .into(),
            );
        }
        for pr in &data.prs {
            rows.push(pr_card(pr, theme));
        }
        column(rows).gap(8.0).padding(32.0).fill_width().into()
    }

    fn loading_or_error(&self, theme: &Theme) -> View {
        if let Some(err) = self.error.get() {
            column(vec![
                text("Failed to load data", 16.0)
                    .weight(FontWeight::Bold)
                    .color(Color::rgb(0.9, 0.3, 0.3))
                    .into(),
                text(err, 13.0).color(theme.text_muted).wrap().into(),
            ])
            .gap(8.0)
            .padding(32.0)
            .into()
        } else {
            column(vec![
                text("Loading...", 15.0).color(theme.text_muted).into(),
            ])
            .padding(32.0)
            .into()
        }
    }
}

fn spacer_h(h: f32) -> View {
    rect(Color::TRANSPARENT).width(1.0).height(h).into()
}

fn stat_card(label: &str, value: u32, theme: &Theme) -> View {
    column(vec![
        text(value.to_string(), 26.0)
            .weight(FontWeight::Bold)
            .color(theme.text)
            .into(),
        text(label, 12.0).color(theme.text_muted).into(),
    ])
    .gap(2.0)
    .padding(16.0)
    .bg(theme.surface)
    .radius(8.0)
    .into()
}

fn repo_card(repo: &Repo, theme: &Theme) -> View {
    let mut children = vec![
        row(vec![
            text(&repo.full_name, 14.0)
                .weight(FontWeight::Bold)
                .color(theme.text)
                .into(),
            spacer(),
            if repo.private {
                pill("private", Color::rgba(1.0, 1.0, 1.0, 0.08), theme.text_muted, theme)
            } else {
                text(format!("★ {}", repo.stargazers_count), 12.0)
                    .color(theme.text_muted)
                    .into()
            },
        ])
        .fill_width()
        .into(),
    ];
    if let Some(desc) = &repo.description {
        if !desc.is_empty() {
            children.push(
                text(desc, 13.0).color(theme.text_muted).wrap().into(),
            );
        }
    }
    if let Some(lang) = &repo.language {
        children.push(text(lang, 12.0).color(theme.text_muted).into());
    }
    column(children)
        .gap(6.0)
        .padding(16.0)
        .fill_width()
        .bg(theme.surface)
        .radius(8.0)
        .into()
}

fn notification_card(n: &Notification, theme: &Theme) -> View {
    let kind_color = match n.subject.kind.as_str() {
        "PullRequest" => Color::rgb(0.35, 0.75, 0.45),
        "Issue"       => Color::rgb(0.9,  0.45, 0.2),
        _             => theme.primary,
    };
    column(vec![
        row(vec![
            text(&n.subject.kind, 11.0).color(kind_color).into(),
            text("  ·  ", 11.0).color(theme.text_muted).into(),
            text(&n.repository.full_name, 11.0).color(theme.text_muted).into(),
        ])
        .into(),
        text(&n.subject.title, 14.0).color(theme.text).wrap().into(),
        text(&n.reason, 11.0).color(theme.text_muted).into(),
    ])
    .gap(4.0)
    .padding(14.0)
    .fill_width()
    .bg(theme.surface)
    .radius(8.0)
    .into()
}

fn pr_card(pr: &PullRequest, theme: &Theme) -> View {
    let repo = pr
        .repository_url
        .strip_prefix("https://api.github.com/repos/")
        .unwrap_or(&pr.repository_url);
    column(vec![
        row(vec![
            text(format!("#{}", pr.number), 12.0).color(theme.primary).into(),
            text(format!("  {repo}"), 12.0).color(theme.text_muted).into(),
        ])
        .into(),
        text(&pr.title, 14.0).color(theme.text).wrap().into(),
        text(format!("by @{}", pr.user.login), 12.0)
            .color(theme.text_muted)
            .into(),
    ])
    .gap(4.0)
    .padding(14.0)
    .fill_width()
    .bg(theme.surface)
    .radius(8.0)
    .into()
}

fn pill(label: &str, bg: Color, fg: Color, _theme: &Theme) -> View {
    column(vec![
        text(label, 11.0).color(fg).into(),
    ])
    .padding(6.0)
    .bg(bg)
    .radius(4.0)
    .into()
}

struct GlyphGitHub {
    login:     Signal<Option<Arc<LoginScreen>>>,
    dashboard: Signal<Option<Arc<DashboardScreen>>>,
}

impl GlyphGitHub {
    fn new() -> Self {
        let login: Signal<Option<Arc<LoginScreen>>> = Signal::new(None);
        let dashboard: Signal<Option<Arc<DashboardScreen>>> = Signal::new(None);

        let login_tx     = login.clone();
        let dashboard_tx = dashboard.clone();

        thread::spawn(move || {
            if let Some(token) = load_token() {
                dashboard_tx.set(Some(Arc::new(DashboardScreen::new(token))));
            } else {
                let screen    = Arc::new(LoginScreen::new());
                let token_sig = screen.token_out.clone();
                login_tx.set(Some(screen));

                thread::spawn(move || loop {
                    thread::sleep(std::time::Duration::from_millis(500));
                    if let Some(token) = token_sig.get() {
                        login_tx.set(None);
                        dashboard_tx.set(Some(Arc::new(DashboardScreen::new(token))));
                        break;
                    }
                });
            }
        });

        Self { login, dashboard }
    }

    fn render_view(&self, theme: &Theme, _opener: &WindowOpener) -> View {
        if let Some(dash) = self.dashboard.get() {
            return dash.render_view(theme);
        }
        if let Some(login) = self.login.get() {
            return login.render_view(theme);
        }
        column(vec![
            text("Starting...", 15.0).color(theme.text_muted).into(),
        ])
        .padding(64.0)
        .into()
    }
}

fn main() {
    let app = GlyphGitHub::new();
    App::run(
        move |opener| {
            let theme = Theme::dark();
            let view  = app.render_view(&theme, opener);
            (theme, view)
        },
        Theme::dark(),
        "Glyph for GitHub",
        1100.0,
        720.0,
    );
}
