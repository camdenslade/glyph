use core_glyph::{
    button, button_view, column, image, row, scroll, spacer, text, text_input,
    Color, FontFamily, FontWeight, Signal, View,
};
use platform_glyph::{App, WindowCloser, WindowOpener};
use ui_glyph::{
    gap, hgap,
    icon_add_outline,
    icon_alert_circle_outline,
    icon_checkmark_outline,
    icon_chevron_down_outline, icon_chevron_forward_outline,
    icon_close_outline,
    icon_code_slash_outline,
    icon_document_outline,
    icon_extension_puzzle_outline,
    icon_flash_outline,
    icon_git_branch_outline,
    icon_git_commit_outline,
    icon_git_merge_outline,
    icon_git_pull_request_outline,
    icon_logo_github,
    icon_play_outline,
    icon_refresh_outline,
    icon_settings_outline,
    icon_time_outline,
    RADIUS_MD, RADIUS_LG,
    SPACE_1, SPACE_2, SPACE_3, SPACE_4, SPACE_6,
    TEXT_XS, TEXT_SM, TEXT_BASE,
};
use std::{
    path::{Path, PathBuf},
    process::Command,
    thread,
};

// ── GitHub Dark Default palette ────────────────────────────────────────────────

const CANVAS:      Color = Color::rgb(0.051, 0.067, 0.090);   // #0D1117
const SURFACE:     Color = Color::rgb(0.086, 0.106, 0.133);   // #161B22
const OVERLAY:     Color = Color::rgb(0.129, 0.149, 0.176);   // #21262D
const BORDER:      Color = Color::rgb(0.188, 0.212, 0.239);   // #30363D
const BORDER_MUTED:Color = Color::rgb(0.133, 0.153, 0.176);   // #21293B
const FG:          Color = Color::rgb(0.902, 0.929, 0.961);   // #E6EDF3
const FG_MUTED:    Color = Color::rgb(0.490, 0.541, 0.596);   // #7D8998
const FG_SUBTLE:   Color = Color::rgb(0.271, 0.302, 0.345);   // #454D58

// GitHub accent colors
const BLUE:        Color = Color::rgb(0.212, 0.506, 0.965);   // #3680F6  selection/links
const GREEN_DARK:  Color = Color::rgb(0.137, 0.525, 0.212);   // #238636  commit btn
const GREEN_HOVER: Color = Color::rgb(0.180, 0.627, 0.259);   // #2EA043
const GREEN_FG:    Color = Color::rgb(0.247, 0.722, 0.314);   // #3FB950  status added
const ORANGE:      Color = Color::rgb(0.969, 0.506, 0.400);   // #F78166  active tab underline
const AMBER:       Color = Color::rgb(0.824, 0.600, 0.133);   // #D29922  modified
const RED_FG:      Color = Color::rgb(0.973, 0.318, 0.286);   // #F85149  deleted/error

// Diff colors
const DIFF_ADD_BG:    Color = Color::rgb(0.051, 0.122, 0.071);   // #0D1F12
const DIFF_ADD_GUT:   Color = Color::rgb(0.059, 0.173, 0.094);   // #0F2C18
const DIFF_DEL_BG:    Color = Color::rgb(0.125, 0.055, 0.071);   // #200E12
const DIFF_DEL_GUT:   Color = Color::rgb(0.176, 0.063, 0.082);   // #2D1015
const DIFF_HUNK_BG:   Color = Color::rgb(0.075, 0.118, 0.200);   // #131E33
const DIFF_HUNK_TEXT: Color = Color::rgb(0.345, 0.506, 0.800);   // #5881CC

// ── Data types ─────────────────────────────────────────────────────────────────

#[derive(Clone, PartialEq)]
enum FileStatus { Modified, Added, Deleted, Untracked, Renamed, Conflict }

impl FileStatus {
    fn label(&self) -> &'static str {
        match self {
            Self::Modified  => "M",
            Self::Added     => "A",
            Self::Deleted   => "D",
            Self::Untracked => "U",
            Self::Renamed   => "R",
            Self::Conflict  => "!",
        }
    }
    fn color(&self) -> Color {
        match self {
            Self::Modified  => AMBER,
            Self::Added     => GREEN_FG,
            Self::Deleted   => RED_FG,
            Self::Untracked => FG_MUTED,
            Self::Renamed   => BLUE,
            Self::Conflict  => RED_FG,
        }
    }
}

#[derive(Clone)]
struct GitFile {
    path: String,
    status: FileStatus,
    #[allow(dead_code)]
    staged: bool,
}

#[derive(Clone)]
enum DiffLineKind { Added, Deleted, Context, HunkHeader }

#[derive(Clone)]
struct DiffLine {
    kind: DiffLineKind,
    old_num: Option<u32>,
    new_num: Option<u32>,
    content: String,
}

#[derive(Clone)]
struct GitCommit {
    sha: String,
    author: String,
    date_iso: String,
    message: String,
}

impl GitCommit {
    fn short_sha(&self) -> &str { &self.sha[..self.sha.len().min(7)] }
    fn message_first_line(&self) -> &str { self.message.lines().next().unwrap_or("") }
    fn relative_time(&self) -> String { relative_date(&self.date_iso) }
}

#[derive(Clone)]
struct GitBranch {
    name: String,
    is_current: bool,
    is_remote: bool,
    ahead: u32,
    behind: u32,
}

// ── App state ──────────────────────────────────────────────────────────────────

#[derive(Clone, PartialEq)]
enum SidebarTab { Changes, History, Branches }

// The "repo tab" in the top nav (Code / Issues / PRs / Actions / Projects)
#[derive(Clone, PartialEq)]
enum RepoTab { Code, Issues, PullRequests, Actions, Projects }

#[derive(Clone)]
struct AppState {
    repos:           Vec<PathBuf>,
    repo_path:       Option<PathBuf>,
    current_branch:  String,
    repo_tab:        RepoTab,

    sidebar_tab:     SidebarTab,
    staged:          Vec<GitFile>,
    unstaged:        Vec<GitFile>,
    commits:         Vec<GitCommit>,
    branches:        Vec<GitBranch>,

    diff_lines:      Vec<DiffLine>,
    active_file:     Option<String>,
    active_staged:   bool,
    selected_file_idx: Option<usize>,

    commit_summary:  String,
    commit_body:     String,
    commit_expanded: bool,

    // stable text-input signals
    summary_value:   Signal<String>,
    summary_focused: Signal<bool>,
    summary_cursor:  Signal<usize>,
    body_value:      Signal<String>,
    body_focused:    Signal<bool>,
    body_cursor:     Signal<usize>,

    error:   Option<String>,
    loading: bool,
}

impl AppState {
    fn new(repos: Vec<PathBuf>, repo_path: Option<PathBuf>) -> Self {
        Self {
            repos, repo_path,
            current_branch: String::new(),
            repo_tab: RepoTab::Code,
            sidebar_tab: SidebarTab::Changes,
            staged: vec![], unstaged: vec![],
            commits: vec![], branches: vec![],
            diff_lines: vec![], active_file: None, active_staged: false,
            selected_file_idx: None,
            commit_summary: String::new(),
            commit_body: String::new(),
            commit_expanded: false,
            summary_value:   Signal::new(String::new()),
            summary_focused: Signal::new(false),
            summary_cursor:  Signal::new(0usize),
            body_value:      Signal::new(String::new()),
            body_focused:    Signal::new(false),
            body_cursor:     Signal::new(0usize),
            error: None, loading: false,
        }
    }

    fn load_repo(&mut self, rp: &Path) {
        self.repo_path = Some(rp.to_path_buf());
        self.current_branch = git_current_branch(rp);
        let (staged, unstaged) = git_status(rp);
        self.staged = staged; self.unstaged = unstaged;
        self.commits = git_log(rp);
        self.branches = git_branches(rp);
        self.active_file = None; self.diff_lines = vec![];
        self.selected_file_idx = None;
        self.commit_summary = String::new(); self.commit_body = String::new();
        self.summary_value.set(String::new()); self.body_value.set(String::new());
        if !self.repos.contains(&rp.to_path_buf()) {
            self.repos.insert(0, rp.to_path_buf());
            self.repos.truncate(10);
        }
    }
}

// ── Git commands ───────────────────────────────────────────────────────────────

fn git(repo: &Path, args: &[&str]) -> Result<String, String> {
    let out = Command::new("git").current_dir(repo).args(args).output().map_err(|e| e.to_string())?;
    if out.status.success() { Ok(String::from_utf8_lossy(&out.stdout).into_owned()) }
    else { Err(String::from_utf8_lossy(&out.stderr).trim().to_string()) }
}

fn git_current_branch(repo: &Path) -> String {
    git(repo, &["rev-parse", "--abbrev-ref", "HEAD"]).unwrap_or_default().trim().to_string()
}

fn git_status(repo: &Path) -> (Vec<GitFile>, Vec<GitFile>) {
    let raw = match git(repo, &["status", "--porcelain", "-u"]) { Ok(s) => s, Err(_) => return (vec![], vec![]) };
    let mut staged = vec![]; let mut unstaged = vec![];
    for line in raw.lines() {
        if line.len() < 3 { continue; }
        let x = line.chars().next().unwrap_or(' ');
        let y = line.chars().nth(1).unwrap_or(' ');
        let path = line[3..].to_string();
        if x != ' ' && x != '?' { staged.push(GitFile { path: path.clone(), status: char_to_status(x), staged: true }); }
        if y != ' ' {
            let status = if y == '?' { FileStatus::Untracked } else { char_to_status(y) };
            unstaged.push(GitFile { path: path.clone(), status, staged: false });
        }
    }
    (staged, unstaged)
}

fn char_to_status(c: char) -> FileStatus {
    match c { 'M'=>FileStatus::Modified,'A'=>FileStatus::Added,'D'=>FileStatus::Deleted,'R'=>FileStatus::Renamed,_=>FileStatus::Conflict }
}

fn git_log(repo: &Path) -> Vec<GitCommit> {
    let raw = match git(repo, &["log", "--pretty=format:%H%x1f%an%x1f%ai%x1f%s", "-n", "200"]) { Ok(s)=>s, Err(_)=>return vec![] };
    raw.lines().filter_map(|line| {
        let p: Vec<&str> = line.splitn(4, '\x1f').collect();
        if p.len() < 4 { return None; }
        Some(GitCommit { sha: p[0].to_string(), author: p[1].to_string(), date_iso: p[2].to_string(), message: p[3].to_string() })
    }).collect()
}

fn git_diff(repo: &Path, path: &str, staged: bool) -> Vec<DiffLine> {
    let args = if staged { vec!["diff","--cached","--unified=5","--",path] } else { vec!["diff","--unified=5","--",path] };
    parse_diff(&git(repo, &args).unwrap_or_default())
}

fn parse_diff(raw: &str) -> Vec<DiffLine> {
    let mut lines = vec![]; let mut old_n = 0u32; let mut new_n = 0u32;
    for line in raw.lines() {
        if line.starts_with("@@") {
            if let Some(rest) = line.split_once(" -") {
                if let Some((op, rem)) = rest.1.split_once(' ') {
                    let os: u32 = op.split(',').next().and_then(|s| s.parse().ok()).unwrap_or(1);
                    if let Some(np) = rem.strip_prefix('+') {
                        let ns: u32 = np.split(',').next().and_then(|s| s.parse().ok()).unwrap_or(1);
                        old_n = os; new_n = ns;
                    }
                }
            }
            lines.push(DiffLine { kind: DiffLineKind::HunkHeader, old_num: None, new_num: None, content: line.to_string() });
        } else if line.starts_with('+') && !line.starts_with("+++") {
            lines.push(DiffLine { kind: DiffLineKind::Added, old_num: None, new_num: Some(new_n), content: line[1..].to_string() }); new_n += 1;
        } else if line.starts_with('-') && !line.starts_with("---") {
            lines.push(DiffLine { kind: DiffLineKind::Deleted, old_num: Some(old_n), new_num: None, content: line[1..].to_string() }); old_n += 1;
        } else if !line.starts_with("diff ") && !line.starts_with("index ") && !line.starts_with("---") && !line.starts_with("+++") {
            let content = if line.starts_with(' ') { line[1..].to_string() } else { line.to_string() };
            lines.push(DiffLine { kind: DiffLineKind::Context, old_num: Some(old_n), new_num: Some(new_n), content }); old_n += 1; new_n += 1;
        }
    }
    lines
}

fn git_branches(repo: &Path) -> Vec<GitBranch> {
    let raw = match git(repo, &["branch", "-vv", "--all"]) { Ok(s)=>s, Err(_)=>return vec![] };
    raw.lines().filter_map(|line| {
        let current = line.starts_with('*');
        let trimmed = line.trim_start_matches(['*', ' ']);
        let name = trimmed.split_whitespace().next()?.to_string();
        let is_remote = name.starts_with("remotes/");
        let clean_name = name.trim_start_matches("remotes/").to_string();
        Some(GitBranch { name: clean_name, is_current: current, is_remote, ahead: parse_num_after(line, "ahead "), behind: parse_num_after(line, "behind ") })
    }).collect()
}

fn parse_num_after(s: &str, pat: &str) -> u32 {
    s.find(pat).map(|i| s[i+pat.len()..].split_whitespace().next().and_then(|n| n.trim_matches(|c:char| !c.is_numeric()).parse().ok()).unwrap_or(0)).unwrap_or(0)
}

fn git_stage(repo: &Path, path: &str) { let _ = git(repo, &["add", "--", path]); }
fn git_unstage(repo: &Path, path: &str) { let _ = git(repo, &["reset", "HEAD", "--", path]); }
fn git_commit(repo: &Path, summary: &str, body: &str) -> Result<(), String> {
    let msg = if body.is_empty() { summary.to_string() } else { format!("{}\n\n{}", summary, body) };
    git(repo, &["commit", "-m", &msg]).map(|_| ())
}

// ── Utilities ──────────────────────────────────────────────────────────────────

fn relative_date(iso: &str) -> String {
    let dp = iso.split(' ').next().unwrap_or(iso);
    let p: Vec<u32> = dp.split('-').filter_map(|s| s.parse().ok()).collect();
    if p.len() < 3 { return iso.to_string(); }
    format!("{} {}, {}", ["","Jan","Feb","Mar","Apr","May","Jun","Jul","Aug","Sep","Oct","Nov","Dec"].get(p[1] as usize).unwrap_or(&"?"), p[2], p[0])
}

fn trunc(s: &str, max: usize) -> String {
    if s.chars().count() <= max { s.to_string() }
    else { format!("{}…", &s[..s.char_indices().nth(max).map(|(i,_)|i).unwrap_or(s.len())]) }
}

fn pick_folder() -> Option<PathBuf> { rfd::FileDialog::new().pick_folder() }

fn load_recent_repos() -> Vec<PathBuf> {
    let mut repos = vec![];
    if let Some(home) = dirs::home_dir() {
        if let Ok(entries) = std::fs::read_dir(&home) {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.is_dir() && p.join(".git").exists() { repos.push(p); if repos.len() >= 8 { break; } }
            }
        }
    }
    repos
}

fn glyph_logo_path() -> String { format!("{}/assets/glyph_logo.png", env!("CARGO_MANIFEST_DIR")) }

// ── Main ───────────────────────────────────────────────────────────────────────

fn main() {
    let recent = load_recent_repos();
    let initial = std::env::current_dir().ok().filter(|p| p.join(".git").exists());
    let theme = ui_glyph::github_dark_theme();
    let state: Signal<AppState> = Signal::new(AppState::new(recent, initial.clone()));

    if let Some(ref rp) = initial {
        let rp = rp.clone(); let s = state.clone();
        thread::spawn(move || {
            let mut st = s.get(); st.loading = true; s.set(st.clone());
            st.load_repo(&rp); st.loading = false; s.set(st);
        });
    }

    let scroll_y   = Signal::new(0.0f32);
    let max_scroll = Signal::new((-1.0f32, -1.0f32));

    App::run(
        move |_: &WindowOpener, _: &WindowCloser| {
            let t = ui_glyph::github_dark_theme();
            let v = build_ui(state.clone(), scroll_y.clone(), max_scroll.clone());
            (t, v)
        },
        theme, "Glyph Git", 1340.0, 840.0,
    );
}

// ── Root ───────────────────────────────────────────────────────────────────────

fn build_ui(state: Signal<AppState>, scroll_y: Signal<f32>, max_scroll: Signal<(f32, f32)>) -> View {
    let st = state.get();

    let error_banner: View = if let Some(ref err) = st.error {
        let s2 = state.clone();
        row(vec![
            hgap(SPACE_3),
            icon_alert_circle_outline(RED_FG, 14.0),
            hgap(SPACE_2),
            text(trunc(err, 100), TEXT_SM).color(RED_FG).into(),
            spacer(),
            button_view(
                row(vec![icon_close_outline(FG_MUTED, 12.0)]).align_center().justify_center().into(),
                move || { let mut st = s2.get(); st.error = None; s2.set(st); },
            ).bg(Color::TRANSPARENT).hover_bg(OVERLAY).width(26.0).height(26.0).radius(RADIUS_MD).into(),
            hgap(SPACE_2),
        ])
        .gap(0.0).height(36.0).fill_width()
        .bg(Color::rgb(0.106, 0.047, 0.047)).align_center()
        .border(Color::rgb(0.361, 0.106, 0.106), 1.0).into()
    } else { column(vec![]).into() };

    // GitHub-style global header (dark bar at top)
    let global_header = render_global_header(&st, state.clone());

    // Repo tab bar (Code / Issues / Pull Requests / Actions / Projects)
    let repo_tabs = render_repo_tabs(&st, state.clone());

    // Main content: left panel + right canvas
    let content = row(vec![
        render_left_panel(&st, state.clone(), scroll_y.clone(), max_scroll.clone()),
        render_main_canvas(&st, state.clone(), scroll_y.clone(), max_scroll.clone()),
    ])
    .gap(0.0).fill_width().grow().into();

    column(vec![error_banner, global_header, repo_tabs, content])
        .gap(0.0).fill_width().fill_height().bg(CANVAS).into()
}

// ── Global Header ──────────────────────────────────────────────────────────────
// Dark bar: logo | repo name + branch | spacer | refresh | repo switcher dots

fn render_global_header(st: &AppState, state: Signal<AppState>) -> View {
    let repo_name = st.repo_path.as_ref()
        .and_then(|p| p.file_name())
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| "No repository".to_string());
    let branch = if st.current_branch.is_empty() { "main".to_string() } else { st.current_branch.clone() };

    // Refresh
    let s_ref = state.clone();
    let repo_ref = st.repo_path.clone();
    let refresh = button_view(
        row(vec![icon_refresh_outline(FG_MUTED, 13.0)]).align_center().justify_center().into(),
        move || {
            if let Some(ref rp) = repo_ref {
                let rp = rp.clone(); let s = s_ref.clone();
                thread::spawn(move || {
                    let mut st = s.get(); st.loading = true; s.set(st.clone());
                    st.load_repo(&rp); st.loading = false; s.set(st);
                });
            }
        },
    ).bg(Color::TRANSPARENT).hover_bg(OVERLAY).width(28.0).height(28.0).radius(RADIUS_MD).into();

    // Add repo
    let s_add = state.clone();
    let add_btn = button_view(
        row(vec![icon_add_outline(FG_MUTED, 13.0)]).align_center().justify_center().into(),
        move || {
            if let Some(path) = pick_folder() {
                if path.join(".git").exists() {
                    let s = s_add.clone(); let p = path.clone();
                    thread::spawn(move || {
                        let mut st = s.get(); st.loading = true; s.set(st.clone());
                        st.load_repo(&p); st.loading = false; s.set(st);
                    });
                }
            }
        },
    ).bg(Color::TRANSPARENT).hover_bg(OVERLAY).width(28.0).height(28.0).radius(RADIUS_MD).into();

    // Repo switcher dots
    let dots: Vec<View> = st.repos.iter().map(|repo| {
        let is_active = st.repo_path.as_deref() == Some(repo.as_path());
        let letter = repo.file_name()
            .and_then(|n| n.to_string_lossy().chars().next())
            .map(|c| c.to_uppercase().next().unwrap_or('?'))
            .unwrap_or('?');
        let s = state.clone(); let rc = repo.clone();
        button_view(
            text(letter.to_string(), TEXT_XS).color(if is_active { CANVAS } else { FG_MUTED }).weight(FontWeight::Bold).into(),
            move || {
                let p = rc.clone(); let s2 = s.clone();
                thread::spawn(move || {
                    let mut st = s2.get(); st.loading = true; s2.set(st.clone());
                    st.load_repo(&p); st.loading = false; s2.set(st);
                });
            },
        )
        .bg(if is_active { BLUE } else { OVERLAY })
        .hover_bg(if is_active { BLUE } else { Color::rgb(0.18, 0.20, 0.24) })
        .width(22.0).height(22.0).radius(11.0).into()
    }).collect();

    row(vec![
        hgap(12.0),
        // Logo
        image(glyph_logo_path()).size(22.0, 22.0).radius(4.0).into(),
        hgap(SPACE_3),
        column(vec![]).width(1.0).height(16.0).bg(FG_SUBTLE).into(),
        hgap(SPACE_3),
        // Repo path: owner/repo
        icon_logo_github(FG_MUTED, 14.0),
        hgap(SPACE_2),
        text(&repo_name, TEXT_SM).color(FG).weight(FontWeight::Bold).into(),
        hgap(SPACE_2),
        column(vec![]).width(1.0).height(16.0).bg(FG_SUBTLE).into(),
        hgap(SPACE_2),
        icon_git_branch_outline(FG_MUTED, 12.0),
        hgap(SPACE_1),
        text(&branch, TEXT_SM).color(FG_MUTED).family(FontFamily::Monospace).into(),
        spacer(),
        if st.loading {
            row(vec![icon_refresh_outline(FG_MUTED, 11.0), hgap(SPACE_1), text("Loading…", TEXT_XS).color(FG_MUTED).into()])
                .gap(0.0).align_center().into()
        } else { column(vec![]).into() },
        hgap(SPACE_3),
        // Repo dots
        row(dots).gap(SPACE_1).align_center().into(),
        hgap(SPACE_2),
        add_btn,
        refresh,
        hgap(SPACE_2),
    ])
    .gap(0.0).height(44.0).fill_width()
    .bg(Color::rgb(0.047, 0.055, 0.067)) // #0C0E11 — GitHub's near-black header
    .align_center()
    .border(BORDER, 1.0)
    .into()
}

// ── Repo Tab Bar ───────────────────────────────────────────────────────────────
// GitHub-style horizontal tabs: Code | Issues | Pull Requests | Actions | Projects

fn render_repo_tabs(st: &AppState, state: Signal<AppState>) -> View {
    let active = st.repo_tab.clone();

    let mk_tab = |icon: View, label: &str, this_tab: RepoTab, s: Signal<AppState>| -> View {
        let is_active = active == this_tab;
        let fg = if is_active { FG } else { FG_MUTED };
        // Underline bar: orange (#f78166) on active, transparent otherwise
        let underline: View = if is_active {
            column(vec![]).height(2.0).fill_width().bg(ORANGE).into()
        } else {
            column(vec![]).height(2.0).into()
        };
        let label_str = label.to_string();
        button_view(
            column(vec![
                row(vec![icon, hgap(SPACE_2), text(label_str, TEXT_SM).color(fg).into()])
                    .gap(0.0).align_center().into(),
                gap(SPACE_1),
                underline,
            ])
            .gap(0.0).padding_x(SPACE_3).padding_y(SPACE_2)
            .align_center().into(),
            move || { let mut st = s.get(); st.repo_tab = this_tab.clone(); s.set(st); },
        )
        .bg(Color::TRANSPARENT)
        .hover_bg(Color::TRANSPARENT)
        .into()
    };

    let s1 = state.clone(); let s2 = state.clone(); let s3 = state.clone();
    let s4 = state.clone(); let s5 = state.clone();

    row(vec![
        hgap(SPACE_4),
        mk_tab(icon_code_slash_outline(if active==RepoTab::Code{FG}else{FG_MUTED}, 14.0), "Code", RepoTab::Code, s1),
        mk_tab(icon_alert_circle_outline(if active==RepoTab::Issues{FG}else{FG_MUTED}, 14.0), "Issues", RepoTab::Issues, s2),
        mk_tab(icon_git_pull_request_outline(if active==RepoTab::PullRequests{FG}else{FG_MUTED}, 14.0), "Pull requests", RepoTab::PullRequests, s3),
        mk_tab(icon_play_outline(if active==RepoTab::Actions{FG}else{FG_MUTED}, 14.0), "Actions", RepoTab::Actions, s4),
        mk_tab(icon_flash_outline(if active==RepoTab::Projects{FG}else{FG_MUTED}, 14.0), "Projects", RepoTab::Projects, s5),
    ])
    .gap(0.0).fill_width()
    .bg(SURFACE)
    .border(BORDER, 1.0)
    .align_center()
    .into()
}

// ── Left Panel (270px) ─────────────────────────────────────────────────────────
// Contains: sidebar tabs + file list OR commit log OR branches + commit panel

fn render_left_panel(
    st: &AppState,
    state: Signal<AppState>,
    scroll_y: Signal<f32>,
    max_scroll: Signal<(f32, f32)>,
) -> View {
    if st.repo_path.is_none() {
        return column(vec![]).width(270.0).fill_height().bg(SURFACE).border(BORDER, 1.0).into();
    }

    let sidebar_tabs = render_sidebar_tabs(st, state.clone());
    let panel: View = match st.sidebar_tab {
        SidebarTab::Changes  => render_changes_panel(st, state.clone()),
        SidebarTab::History  => render_history_panel(st, state.clone(), scroll_y.clone(), max_scroll.clone()),
        SidebarTab::Branches => render_branches_panel(st, state.clone()),
    };
    let commit_footer: View = if st.sidebar_tab == SidebarTab::Changes {
        render_commit_panel(st, state.clone())
    } else { column(vec![]).into() };

    column(vec![sidebar_tabs, panel, commit_footer])
        .gap(0.0).width(270.0).fill_height()
        .bg(SURFACE).border(BORDER, 1.0).into()
}

fn render_sidebar_tabs(st: &AppState, state: Signal<AppState>) -> View {
    let tab = st.sidebar_tab.clone();

    let mk = |icon: View, label: &str, this: SidebarTab, s: Signal<AppState>| -> View {
        let active = tab == this;
        let fg = if active { FG } else { FG_MUTED };
        let underline: View = if active {
            column(vec![]).height(2.0).fill_width().bg(ORANGE).into()
        } else {
            column(vec![]).height(2.0).into()
        };
        let label_s = label.to_string();
        button_view(
            column(vec![
                row(vec![icon, hgap(SPACE_1), text(label_s, TEXT_XS).color(fg).weight(FontWeight::Bold).into()])
                    .gap(0.0).align_center().into(),
                gap(2.0),
                underline,
            ])
            .gap(0.0).width(90.0).height(36.0).align_center().justify_center().into(),
            move || { let mut st = s.get(); st.sidebar_tab = this.clone(); s.set(st); },
        )
        .bg(Color::TRANSPARENT).hover_bg(Color::TRANSPARENT)
        .width(90.0).height(36.0).into()
    };

    let s1 = state.clone(); let s2 = state.clone(); let s3 = state.clone();
    row(vec![
        mk(icon_code_slash_outline(if tab==SidebarTab::Changes{FG}else{FG_MUTED}, 11.0), "Changes", SidebarTab::Changes, s1),
        mk(icon_time_outline(if tab==SidebarTab::History{FG}else{FG_MUTED}, 11.0), "History", SidebarTab::History, s2),
        mk(icon_git_branch_outline(if tab==SidebarTab::Branches{FG}else{FG_MUTED}, 11.0), "Branches", SidebarTab::Branches, s3),
    ])
    .gap(0.0).height(36.0).fill_width()
    .bg(SURFACE).border(BORDER, 1.0).into()
}

// ── Changes panel ──────────────────────────────────────────────────────────────

fn render_changes_panel(st: &AppState, state: Signal<AppState>) -> View {
    let has_unstaged = !st.unstaged.is_empty();
    let has_staged   = !st.staged.is_empty();

    if !has_unstaged && !has_staged {
        return column(vec![
            gap(SPACE_6),
            icon_checkmark_outline(GREEN_FG, 18.0),
            gap(SPACE_2),
            text("Nothing to commit", TEXT_SM).color(FG_MUTED).into(),
            text("Working tree clean", TEXT_XS).color(FG_MUTED).into(),
        ]).align_center().fill_width().grow().into();
    }

    let mut rows: Vec<View> = vec![];

    if has_unstaged {
        rows.push(changes_section_header(&format!("Unstaged  ({})", st.unstaged.len())));
        for (i, f) in st.unstaged.iter().enumerate() {
            let file = f.clone();
            let fp_cb = f.path.clone();
            let s = state.clone();
            let repo = st.repo_path.clone();
            let selected = st.selected_file_idx == Some(i);
            rows.push(file_row(file, selected, false,
                move || {
                    if let Some(ref rp) = repo { git_stage(rp, &fp_cb); }
                    let mut st = s.get();
                    let (sg, un) = git_status(st.repo_path.as_ref().unwrap());
                    st.staged = sg; st.unstaged = un; s.set(st);
                }, {
                    let s2 = state.clone(); let fp = f.path.clone(); let repo2 = st.repo_path.clone();
                    move || {
                        let mut st = s2.get(); st.selected_file_idx = Some(i);
                        st.active_file = Some(fp.clone()); st.active_staged = false;
                        if let Some(ref rp) = repo2 { st.diff_lines = git_diff(rp, &fp, false); }
                        s2.set(st);
                    }
                },
            ));
        }
    }

    if has_staged {
        rows.push(changes_section_header(&format!("Staged  ({})", st.staged.len())));
        let offset = st.unstaged.len();
        for (i, f) in st.staged.iter().enumerate() {
            let file = f.clone();
            let fp_cb = f.path.clone();
            let s = state.clone();
            let repo = st.repo_path.clone();
            let selected = st.selected_file_idx == Some(offset + i);
            rows.push(file_row(file, selected, true,
                move || {
                    if let Some(ref rp) = repo { git_unstage(rp, &fp_cb); }
                    let mut st = s.get();
                    let (sg, un) = git_status(st.repo_path.as_ref().unwrap());
                    st.staged = sg; st.unstaged = un; s.set(st);
                }, {
                    let s2 = state.clone(); let fp = f.path.clone(); let repo2 = st.repo_path.clone();
                    move || {
                        let mut st = s2.get(); st.selected_file_idx = Some(offset + i);
                        st.active_file = Some(fp.clone()); st.active_staged = true;
                        if let Some(ref rp) = repo2 { st.diff_lines = git_diff(rp, &fp, true); }
                        s2.set(st);
                    }
                },
            ));
        }
    }

    column(rows).gap(0.0).fill_width().grow().clip().into()
}

fn changes_section_header(label: &str) -> View {
    row(vec![
        hgap(SPACE_3),
        text(label, TEXT_XS).color(FG_MUTED).weight(FontWeight::Bold).into(),
        spacer(),
    ])
    .gap(0.0).height(22.0).fill_width()
    .bg(CANVAS).align_center()
    .border(BORDER_MUTED, 1.0)
    .into()
}

fn file_row(
    file: GitFile,
    selected: bool,
    is_staged: bool,
    on_checkbox: impl Fn() + 'static,
    on_select: impl Fn() + 'static,
) -> View {
    let status_color = file.status.color();
    let status_label = file.status.label();
    // Show directory prefix dimmed + filename
    let path = file.path.clone();
    let filename = Path::new(&path).file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_else(|| path.clone());
    let dir_prefix = {
        let p = Path::new(&path);
        p.parent().and_then(|d| if d == Path::new("") { None } else { Some(format!("{}/", d.display())) }).unwrap_or_default()
    };

    let bg = if selected { Color::rgb(0.157, 0.204, 0.282) } else { Color::TRANSPARENT };
    let hover_bg = if selected { bg } else { Color::rgb(0.086, 0.106, 0.133) };
    let left_bar: View = if selected {
        column(vec![]).width(2.0).fill_height().bg(BLUE).into()
    } else {
        column(vec![]).width(2.0).into()
    };

    // Checkbox: filled = staged, empty = unstaged
    let checkbox: View = button_view(
        column(vec![
            if is_staged { icon_checkmark_outline(FG, 8.0) } else { column(vec![]).into() }
        ])
        .width(12.0).height(12.0).align_center().justify_center()
        .bg(if is_staged { BLUE } else { Color::TRANSPARENT })
        .border(if is_staged { BLUE } else { FG_SUBTLE }, 1.0)
        .radius(2.0).into(),
        on_checkbox,
    ).bg(Color::TRANSPARENT).hover_bg(Color::TRANSPARENT).width(18.0).height(20.0).into();

    // File type icon based on extension
    let ext = Path::new(&path).extension().and_then(|e| e.to_str()).unwrap_or("");
    let file_icon = file_type_icon(ext, FG_MUTED);

    let name_row = row(vec![
        if dir_prefix.is_empty() { column(vec![]).into() }
        else { text(trunc(&dir_prefix, 14), TEXT_XS).color(FG_SUBTLE).family(FontFamily::Monospace).into() },
        text(trunc(&filename, 18), TEXT_XS).color(FG).family(FontFamily::Monospace).into(),
    ]).gap(0.0).align_center().into();

    // Wrap the whole row in a button for selection
    let row_inner = row(vec![
        left_bar,
        hgap(4.0),
        checkbox,
        hgap(4.0),
        file_icon,
        hgap(4.0),
        name_row,
        spacer(),
        // Status badge — small colored letter in a subtle pill
        row(vec![
            text(status_label, TEXT_XS).color(status_color).weight(FontWeight::Bold).into(),
        ])
        .gap(0.0).padding_x(4.0).padding_y(1.0)
        .bg(Color::rgba(status_color.r, status_color.g, status_color.b, 0.15))
        .radius(3.0).align_center().into(),
        hgap(SPACE_2),
    ])
    .gap(0.0).height(20.0).fill_width().bg(bg).align_center().into();

    button_view(row_inner, on_select)
        .bg(Color::TRANSPARENT).hover_bg(hover_bg)
        .fill_width().into()
}

fn file_type_icon(ext: &str, color: Color) -> View {
    match ext {
        "rs"   => icon_code_slash_outline(color, 10.0),
        "toml" | "json" | "yaml" | "yml" => icon_settings_outline(color, 10.0),
        "md" | "txt"  => icon_document_outline(color, 10.0),
        "lock" => icon_extension_puzzle_outline(color, 10.0),
        _      => icon_document_outline(color, 10.0),
    }
}

// ── History panel ──────────────────────────────────────────────────────────────

fn render_history_panel(
    st: &AppState,
    _state: Signal<AppState>,
    scroll_y: Signal<f32>,
    max_scroll: Signal<(f32, f32)>,
) -> View {
    if st.commits.is_empty() {
        return column(vec![
            gap(SPACE_6),
            icon_git_commit_outline(FG_MUTED, 20.0),
            gap(SPACE_2),
            text("No commits yet", TEXT_SM).color(FG_MUTED).into(),
        ]).align_center().fill_width().grow().into();
    }
    let rows: Vec<View> = st.commits.iter().map(commit_row).collect();
    scroll(column(rows).gap(0.0).fill_width().into(), Signal::new(0.0), scroll_y, max_scroll)
        .fill_width().grow().into()
}

fn commit_row(c: &GitCommit) -> View {
    let sha    = c.short_sha().to_string();
    let msg    = trunc(c.message_first_line(), 30);
    let author = trunc(&c.author, 20);
    let date   = c.relative_time();

    column(vec![
        row(vec![
            hgap(SPACE_3),
            icon_git_commit_outline(FG_MUTED, 11.0),
            hgap(SPACE_2),
            column(vec![
                text(&msg, TEXT_XS).color(FG).into(),
                row(vec![
                    text(&author, TEXT_XS).color(FG_MUTED).into(),
                    spacer(),
                    text(&date, TEXT_XS).color(FG_MUTED).into(),
                    hgap(SPACE_2),
                    text(&sha, TEXT_XS).color(FG_SUBTLE).family(FontFamily::Monospace).into(),
                ]).gap(0.0).fill_width().into(),
            ]).gap(2.0).grow().into(),
            hgap(SPACE_2),
        ])
        .gap(0.0).fill_width().height(38.0).align_center().into(),
        column(vec![]).height(1.0).fill_width().bg(BORDER_MUTED).into(),
    ]).gap(0.0).into()
}

// ── Branches panel ─────────────────────────────────────────────────────────────

fn render_branches_panel(st: &AppState, state: Signal<AppState>) -> View {
    if st.branches.is_empty() {
        return column(vec![
            gap(SPACE_6),
            icon_git_branch_outline(FG_MUTED, 20.0),
            gap(SPACE_2),
            text("No branches", TEXT_SM).color(FG_MUTED).into(),
        ]).align_center().fill_width().grow().into();
    }
    let local:  Vec<&GitBranch> = st.branches.iter().filter(|b| !b.is_remote).collect();
    let remote: Vec<&GitBranch> = st.branches.iter().filter(|b| b.is_remote).collect();
    let mut rows: Vec<View> = vec![];
    if !local.is_empty() {
        rows.push(changes_section_header(&format!("Local  ({})", local.len())));
        for b in &local { rows.push(branch_row(b, &st.repo_path, state.clone())); }
    }
    if !remote.is_empty() {
        rows.push(changes_section_header(&format!("Remote  ({})", remote.len())));
        for b in &remote { rows.push(branch_row(b, &st.repo_path, state.clone())); }
    }
    column(rows).gap(0.0).fill_width().grow().clip().into()
}

fn branch_row(b: &GitBranch, repo_path: &Option<PathBuf>, state: Signal<AppState>) -> View {
    let name = trunc(&b.name, 26);
    let fg = if b.is_current { FG } else { FG_MUTED };
    let icon: View = if b.is_current { icon_chevron_forward_outline(GREEN_FG, 10.0) }
                     else if b.is_remote { icon_git_merge_outline(FG_MUTED, 10.0) }
                     else { icon_git_branch_outline(FG_MUTED, 10.0) };
    let tracking: View = if b.ahead > 0 || b.behind > 0 {
        row(vec![
            if b.ahead  > 0 { text(format!("↑{}", b.ahead),  TEXT_XS).color(BLUE).into()  } else { column(vec![]).into() },
            if b.behind > 0 { text(format!("↓{}", b.behind), TEXT_XS).color(AMBER).into() } else { column(vec![]).into() },
        ]).gap(SPACE_1).into()
    } else { column(vec![]).into() };

    let can_checkout = !b.is_current && !b.is_remote;
    let bn = b.name.clone();
    let repo = repo_path.clone();
    let name_view: View = if can_checkout {
        let s = state.clone();
        button(trunc(&name, 24), move || {
            if let Some(ref rp) = repo {
                let _ = git(rp, &["checkout", &bn]);
                let mut st = s.get();
                st.current_branch = git_current_branch(rp);
                st.branches = git_branches(rp);
                s.set(st);
            }
        }).bg(Color::TRANSPARENT).hover_bg(Color::TRANSPARENT)
          .text_color(fg).font_size(TEXT_XS).padding(0.0).into()
    } else {
        text(&name, TEXT_XS).color(fg).family(FontFamily::Monospace).into()
    };

    column(vec![
        row(vec![hgap(SPACE_3), icon, hgap(SPACE_2), name_view, spacer(), tracking, hgap(SPACE_2)])
            .gap(0.0).height(20.0).fill_width().align_center().into(),
        column(vec![]).height(1.0).fill_width().bg(BORDER_MUTED).into(),
    ]).gap(0.0).into()
}

// ── Commit panel (bottom of left panel) ────────────────────────────────────────
// Mimics GitHub's "Commit changes" box

fn render_commit_panel(st: &AppState, state: Signal<AppState>) -> View {
    let expanded   = st.commit_expanded;
    let char_count = st.commit_summary.len();
    let over_limit = char_count >= 72;
    let counter_color = if over_limit { RED_FG } else { FG_MUTED };
    let has_staged = !st.staged.is_empty();
    let can_commit = has_staged && !st.commit_summary.is_empty();

    let s_exp    = state.clone();
    let s_commit = state.clone();

    // Summary input
    let summary_input: View = {
        let s = state.clone();
        column(vec![
            // Label
            row(vec![
                text("Commit summary", TEXT_XS).color(FG_MUTED).into(),
                spacer(),
                text(format!("{}/72", char_count), TEXT_XS).color(counter_color).into(),
            ]).gap(0.0).fill_width().align_center().into(),
            gap(4.0),
            text_input(st.summary_value.clone(), st.summary_focused.clone(), st.summary_cursor.clone())
                .placeholder("Summary (required)")
                .font_size(TEXT_SM).bg(CANVAS).text_color(FG).border_color(BORDER)
                .on_change(move |v| { let mut st = s.get(); st.commit_summary = v; s.set(st); })
                .fill_width().into(),
        ])
        .gap(0.0).fill_width().padding_x(SPACE_3).padding_y(SPACE_2).into()
    };

    // Body (expanded)
    let body_input: View = if expanded {
        let s = state.clone();
        column(vec![
            text("Extended description", TEXT_XS).color(FG_MUTED).into(),
            gap(4.0),
            text_input(st.body_value.clone(), st.body_focused.clone(), st.body_cursor.clone())
                .placeholder("Add an optional extended description…")
                .font_size(TEXT_SM).bg(CANVAS).text_color(FG).border_color(BORDER)
                .on_change(move |v| { let mut st = s.get(); st.commit_body = v; s.set(st); })
                .fill_width().into(),
        ])
        .gap(0.0).fill_width().padding_x(SPACE_3).padding_y(SPACE_1).into()
    } else { column(vec![]).into() };

    // Branch info line
    let branch_label = if st.current_branch.is_empty() { "main".to_string() } else { st.current_branch.clone() };
    let branch_info = row(vec![
        hgap(SPACE_3),
        icon_git_branch_outline(FG_MUTED, 11.0),
        hgap(SPACE_1),
        text(format!("Commit to {}", trunc(&branch_label, 22)), TEXT_XS).color(FG_MUTED).into(),
        spacer(),
    ]).gap(0.0).fill_width().height(22.0).align_center().into();

    // Expand toggle
    let expand_btn: View = button_view(
        row(vec![
            if expanded { icon_chevron_down_outline(FG_MUTED, 11.0) }
            else        { icon_chevron_forward_outline(FG_MUTED, 11.0) },
            hgap(SPACE_1),
            text(if expanded { "Hide description" } else { "Add description" }, TEXT_XS).color(FG_MUTED).into(),
        ]).gap(0.0).align_center().into(),
        move || { let mut st = s_exp.get(); st.commit_expanded = !st.commit_expanded; s_exp.set(st); },
    ).bg(Color::TRANSPARENT).hover_bg(OVERLAY).into();

    // Commit button
    let sum_sig  = st.summary_value.clone();
    let body_sig = st.body_value.clone();
    let repo_c   = st.repo_path.clone();
    let btn_bg   = if can_commit { GREEN_DARK } else { OVERLAY };
    let btn_fg   = if can_commit { FG } else { FG_MUTED };
    let branch_for_btn = branch_label.clone();
    let commit_btn: View = button(
        format!("Commit to {}", trunc(&branch_for_btn, 16)),
        move || {
            let summary = sum_sig.get();
            let body    = body_sig.get();
            if summary.is_empty() { return; }
            if let Some(ref rp) = repo_c {
                match git_commit(rp, &summary, &body) {
                    Ok(()) => {
                        let mut st = s_commit.get();
                        let (sg, un) = git_status(rp);
                        st.staged = sg; st.unstaged = un;
                        st.commits = git_log(rp);
                        st.commit_summary = String::new(); st.commit_body = String::new();
                        st.commit_expanded = false;
                        st.summary_value.set(String::new()); st.body_value.set(String::new());
                        s_commit.set(st);
                    }
                    Err(e) => {
                        let mut st = s_commit.get(); st.error = Some(e); s_commit.set(st);
                    }
                }
            }
        },
    )
    .bg(btn_bg).hover_bg(if can_commit { GREEN_HOVER } else { OVERLAY })
    .text_color(btn_fg).font_size(TEXT_SM).padding(SPACE_3).radius(RADIUS_MD)
    .fill_width().into();

    column(vec![
        column(vec![]).height(1.0).fill_width().bg(BORDER).into(),
        summary_input,
        body_input,
        row(vec![hgap(SPACE_3), expand_btn, spacer()]).gap(0.0).fill_width().height(28.0).align_center().into(),
        gap(SPACE_2),
        branch_info,
        row(vec![hgap(SPACE_3), commit_btn, hgap(SPACE_3)]).gap(0.0).fill_width().into(),
        gap(SPACE_3),
    ]).gap(0.0).fill_width().bg(SURFACE).into()
}

// ── Main canvas (right pane) ───────────────────────────────────────────────────

fn render_main_canvas(
    st: &AppState,
    state: Signal<AppState>,
    scroll_y: Signal<f32>,
    max_scroll: Signal<(f32, f32)>,
) -> View {
    if st.repo_path.is_none() {
        return render_no_repo(state);
    }

    // File path breadcrumb bar
    let file_label = st.active_file.as_deref().unwrap_or("");
    let staged_badge: View = if !file_label.is_empty() {
        let badge_text = if st.active_staged { "staged" } else { "unstaged" };
        let badge_color = if st.active_staged { BLUE } else { AMBER };
        row(vec![
            text(badge_text, TEXT_XS).color(badge_color).into(),
        ])
        .gap(0.0).padding_x(6.0).padding_y(2.0)
        .bg(Color::rgba(badge_color.r, badge_color.g, badge_color.b, 0.15))
        .border(Color::rgba(badge_color.r, badge_color.g, badge_color.b, 0.3), 1.0)
        .radius(10.0).align_center().into()
    } else { column(vec![]).into() };

    let diff_topbar = row(vec![
        hgap(SPACE_4),
        icon_document_outline(FG_MUTED, 12.0),
        hgap(SPACE_2),
        text(trunc(file_label, 70), TEXT_XS).color(FG_MUTED).family(FontFamily::Monospace).into(),
        hgap(SPACE_2),
        staged_badge,
        spacer(),
    ])
    .gap(0.0).height(34.0).fill_width()
    .bg(SURFACE).align_center().border(BORDER, 1.0).into();

    let content: View = if file_label.is_empty() {
        render_clean_state(st)
    } else if st.diff_lines.is_empty() {
        column(vec![
            gap(SPACE_6),
            text("No diff to display", TEXT_SM).color(FG_MUTED).into(),
        ]).align_center().fill_width().grow().into()
    } else {
        render_diff_lines(st, scroll_y, max_scroll)
    };

    column(vec![diff_topbar, content])
        .gap(0.0).grow().fill_height().bg(CANVAS).into()
}

fn render_no_repo(state: Signal<AppState>) -> View {
    let s = state.clone();
    column(vec![
        image(glyph_logo_path()).size(52.0, 52.0).radius(10.0).into(),
        gap(SPACE_3),
        text("Glyph Git", TEXT_BASE).color(FG).weight(FontWeight::Bold).into(),
        text("A GitHub-style Git client", TEXT_SM).color(FG_MUTED).into(),
        gap(SPACE_4),
        button("Open Repository…", move || {
            if let Some(path) = pick_folder() {
                if path.join(".git").exists() {
                    let s2 = s.clone(); let p = path.clone();
                    thread::spawn(move || {
                        let mut st = s2.get(); st.loading = true; s2.set(st.clone());
                        st.load_repo(&p); st.loading = false; s2.set(st);
                    });
                }
            }
        })
        .bg(GREEN_DARK).hover_bg(GREEN_HOVER)
        .text_color(FG).font_size(TEXT_SM).padding(SPACE_4).radius(RADIUS_MD).into(),
    ])
    .gap(0.0).align_center().fill_width().fill_height().justify_center().grow().bg(CANVAS).into()
}

fn render_clean_state(st: &AppState) -> View {
    let last = st.commits.first();
    let commit_card: View = if let Some(c) = last {
        column(vec![
            row(vec![
                icon_git_commit_outline(FG_MUTED, 13.0), hgap(SPACE_2),
                text(c.short_sha(), TEXT_XS).color(FG_MUTED).family(FontFamily::Monospace).into(),
                spacer(),
                text(c.relative_time(), TEXT_XS).color(FG_MUTED).into(),
            ]).gap(0.0).fill_width().align_center().into(),
            text(trunc(c.message_first_line(), 60), TEXT_SM).color(FG).into(),
            text(&c.author, TEXT_XS).color(FG_MUTED).into(),
        ])
        .gap(SPACE_2).padding(SPACE_4)
        .bg(OVERLAY).border(BORDER, 1.0).radius(RADIUS_LG).into()
    } else { column(vec![]).into() };

    column(vec![
        gap(SPACE_6),
        icon_checkmark_outline(GREEN_FG, 28.0),
        gap(SPACE_3),
        text("Working tree clean", TEXT_BASE).color(FG).weight(FontWeight::Bold).into(),
        text("No uncommitted changes", TEXT_SM).color(FG_MUTED).into(),
        gap(SPACE_4),
        commit_card,
    ])
    .gap(0.0).align_center().fill_width().grow().justify_center().padding_x(SPACE_6).into()
}

fn render_diff_lines(st: &AppState, scroll_y: Signal<f32>, max_scroll: Signal<(f32, f32)>) -> View {
    let rows: Vec<View> = st.diff_lines.iter().map(diff_line_row).collect();
    scroll(column(rows).gap(0.0).fill_width().into(), Signal::new(0.0), scroll_y, max_scroll)
        .fill_width().grow().into()
}

fn diff_line_row(line: &DiffLine) -> View {
    let (bg, gutter_bg, text_color, sign) = match line.kind {
        DiffLineKind::Added     => (DIFF_ADD_BG, DIFF_ADD_GUT, GREEN_FG, "+"),
        DiffLineKind::Deleted   => (DIFF_DEL_BG, DIFF_DEL_GUT, RED_FG,  "-"),
        DiffLineKind::Context   => (CANVAS, CANVAS, FG_MUTED, " "),
        DiffLineKind::HunkHeader=> (DIFF_HUNK_BG, DIFF_HUNK_BG, DIFF_HUNK_TEXT, " "),
    };
    let is_hunk = matches!(line.kind, DiffLineKind::HunkHeader);
    let h = if is_hunk { 22.0 } else { 20.0 };

    let old_str = line.old_num.map(|n| n.to_string()).unwrap_or_default();
    let new_str = line.new_num.map(|n| n.to_string()).unwrap_or_default();

    let gutter = row(vec![
        hgap(SPACE_2),
        text(old_str, TEXT_XS).color(FG_SUBTLE).family(FontFamily::Monospace).into(),
        hgap(4.0),
        text(new_str, TEXT_XS).color(FG_SUBTLE).family(FontFamily::Monospace).into(),
        hgap(4.0),
        text(sign, TEXT_XS).color(text_color).family(FontFamily::Monospace).weight(FontWeight::Bold).into(),
        hgap(SPACE_1),
    ])
    .gap(0.0).width(62.0).height(h).bg(gutter_bg).align_center().into();

    let code = if is_hunk {
        text(trunc(&line.content, 100), TEXT_XS).color(text_color).family(FontFamily::Monospace).into()
    } else {
        text(line.content.clone(), TEXT_XS).color(text_color).family(FontFamily::Monospace).into()
    };

    row(vec![
        gutter,
        column(vec![]).width(1.0).height(h).bg(BORDER_MUTED).into(),
        hgap(SPACE_2),
        code,
    ])
    .gap(0.0).height(h).fill_width().bg(bg).align_center().into()
}
