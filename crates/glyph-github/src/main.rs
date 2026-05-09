use glyph_core::{
    Color, Component, FontWeight, Signal, Theme, View,
    column, row, text,
};
use glyph_platform::App;
use serde::Deserialize;
use std::thread;

#[derive(Deserialize, Clone)]
struct RepoInfo {
    full_name: String,
    description: Option<String>,
    stargazers_count: u32,
    forks_count: u32,
    open_issues_count: u32,
    language: Option<String>,
}

#[derive(Deserialize, Clone)]
struct Commit {
    commit: CommitDetail,
}

#[derive(Deserialize, Clone)]
struct CommitDetail {
    message: String,
    author: CommitAuthor,
}

#[derive(Deserialize, Clone)]
struct CommitAuthor {
    name: String,
}

#[derive(Clone)]
struct DashboardData {
    repo: RepoInfo,
    commits: Vec<Commit>,
}

fn fetch(token: &str, repo: &str) -> Result<DashboardData, String> {
    let client = reqwest::blocking::Client::builder()
        .user_agent("glyph-github/0.1")
        .build()
        .map_err(|e| e.to_string())?;

    let repo_url = format!("https://api.github.com/repos/{}", repo);
    let commits_url = format!("https://api.github.com/repos/{}/commits?per_page=5", repo);

    let repo_body = client
        .get(&repo_url)
        .bearer_auth(token)
        .send()
        .map_err(|e| e.to_string())?
        .text()
        .map_err(|e| e.to_string())?;

    let repo_info: RepoInfo = serde_json::from_str(&repo_body)
        .map_err(|e| format!("{}: {}", e, &repo_body[..repo_body.len().min(200)]))?;

    let commits_body = client
        .get(&commits_url)
        .bearer_auth(token)
        .send()
        .map_err(|e| e.to_string())?
        .text()
        .map_err(|e| e.to_string())?;

    let commits: Vec<Commit> = serde_json::from_str(&commits_body)
        .map_err(|e| format!("{}: {}", e, &commits_body[..commits_body.len().min(200)]))?;

    Ok(DashboardData { repo: repo_info, commits })
}

struct Dashboard {
    data: Signal<Option<DashboardData>>,
    error: Signal<Option<String>>,
}

impl Dashboard {
    fn new(token: String, repo: String) -> Self {
        let data: Signal<Option<DashboardData>> = Signal::new(None);
        let error: Signal<Option<String>> = Signal::new(None);

        let data_tx = data.clone();
        let error_tx = error.clone();
        thread::spawn(move || match fetch(&token, &repo) {
            Ok(d) => data_tx.set(Some(d)),
            Err(e) => error_tx.set(Some(e)),
        });

        Self { data, error }
    }
}

impl Component for Dashboard {
    fn render(&self, theme: &Theme) -> View {
        if let Some(err) = self.error.get() {
            return column(vec![
                text("Failed to load", 18.0).color(Color::rgb(0.8, 0.1, 0.1)).into(),
                text(err, 14.0).color(theme.text_muted).into(),
            ]).into();
        }

        let Some(data) = self.data.get() else {
            return column(vec![
                text("Loading...", 20.0).color(theme.text_muted).into(),
            ]).into();
        };

        let repo = &data.repo;

        let mut children: Vec<View> = vec![
            text(&repo.full_name, 28.0).weight(FontWeight::Bold).color(theme.text).into(),
        ];

        if let Some(desc) = &repo.description {
            children.push(text(desc, 16.0).color(theme.text_muted).wrap().into());
        }

        children.push(
            row(vec![
                stat("Stars", repo.stargazers_count, theme),
                stat("Forks", repo.forks_count, theme),
                stat("Issues", repo.open_issues_count, theme),
            ]).gap(32.0).into()
        );

        if let Some(lang) = &repo.language {
            children.push(text(format!("Language: {}", lang), 14.0).color(theme.text_muted).into());
        }

        children.push(text("Recent Commits", 18.0).weight(FontWeight::Bold).color(theme.text).into());

        for commit in &data.commits {
            let msg = commit.commit.message.lines().next().unwrap_or("").to_string();
            let author = &commit.commit.author.name;
            children.push(
                column(vec![
                    text(msg, 14.0).color(theme.text).into(),
                    text(format!("by {}", author), 12.0).color(theme.text_muted).into(),
                ]).gap(4.0).into()
            );
        }

        column(children).gap(12.0).padding(24.0).into()
    }
}

fn stat(label: &str, value: u32, theme: &Theme) -> View {
    column(vec![
        text(value.to_string(), 24.0).weight(FontWeight::Bold).color(theme.primary).into(),
        text(label, 13.0).color(theme.text_muted).into(),
    ]).gap(4.0).into()
}

fn main() {
    let token = std::env::var("GITHUB_TOKEN").unwrap_or_default();
    let repo = std::env::var("GITHUB_REPO").unwrap_or_else(|_| "anthropics/anthropic-sdk-python".to_string());

    let title = format!("glyph — {}", repo);
    let dashboard = Dashboard::new(token, repo);
    let default_theme = glyph_platform::Theme::light();
    App::run(move |_opener| { let t = default_theme.clone(); let v = dashboard.render(&t); (t, v) }, glyph_platform::Theme::light(), &title, 900.0, 700.0);
}
