/// Glyph Git Client
/// A high-density, keyboard-driven Git desktop client built on the Glyph GPU framework.
/// Four-zone layout: Nav Strip | Repo Sidebar | Diff Canvas | (Inspector, toggleable)
use core_glyph::{
    button, button_view, column, row, scroll, spacer, text, text_input,
    Color, FontFamily, FontWeight, Signal, Theme, View,
};
use platform_glyph::{App, WindowOpener, WindowCloser};
use ui_glyph::{
    gap, hgap,
    icon_checkmark_outline, icon_chevron_down_outline, icon_chevron_forward_outline,
    icon_close_outline, icon_code_slash_outline,
    icon_git_branch_outline, icon_git_commit_outline,
    icon_folder_outline, icon_document_outline,
    icon_refresh_outline, icon_settings_outline,
    RADIUS_MD, RADIUS_LG,
    SPACE_1, SPACE_2, SPACE_3, SPACE_4, SPACE_6,
    TEXT_XS, TEXT_SM, TEXT_BASE,
};
use std::{
    path::{Path, PathBuf},
    process::Command,
    thread,
};

// ── Color tokens (GitHub dark palette) ────────────────────────────────────────

const BG_BASE:     Color = Color::rgb(0.051, 0.067, 0.090);   // #0D1117
const BG_SURFACE:  Color = Color::rgb(0.086, 0.106, 0.133);   // #161B22
const BG_ELEVATED: Color = Color::rgb(0.129, 0.149, 0.176);   // #21262D
const BORDER:      Color = Color::rgb(0.188, 0.212, 0.239);   // #30363D
const FG:          Color = Color::rgb(0.902, 0.929, 0.961);   // #E6EDF3
const FG_MUTED:    Color = Color::rgb(0.490, 0.522, 0.565);   // #7D8590
const ACCENT:      Color = Color::rgb(0.345, 0.651, 1.000);   // #58A6FF
const GREEN:       Color = Color::rgb(0.247, 0.722, 0.314);   // #3FB950
const AMBER:       Color = Color::rgb(0.824, 0.600, 0.133);   // #D29922
const RED:         Color = Color::rgb(0.973, 0.318, 0.286);   // #F85149
#[allow(dead_code)]
const PURPLE:      Color = Color::rgb(0.737, 0.549, 1.000);   // #BC8CFF

const DIFF_ADD_BG:  Color = Color::rgb(0.051, 0.122, 0.071);  // #0D1F12
const DIFF_DEL_BG:  Color = Color::rgb(0.125, 0.055, 0.071);  // #200E12
const DIFF_HUNK_BG: Color = Color::rgb(0.110, 0.176, 0.243);  // #1C2D3E

// ── Git data types ─────────────────────────────────────────────────────────────

#[derive(Clone, PartialEq)]
enum FileStatus {
    Modified,
    Added,
    Deleted,
    Untracked,
    Renamed,
    Conflict,
}

impl FileStatus {
    fn label(&self) -> &'static str {
        match self {
            Self::Modified  => "M",
            Self::Added     => "A",
            Self::Deleted   => "D",
            Self::Untracked => "?",
            Self::Renamed   => "R",
            Self::Conflict  => "C",
        }
    }
    fn color(&self) -> Color {
        match self {
            Self::Modified  => AMBER,
            Self::Added     => GREEN,
            Self::Deleted   => RED,
            Self::Untracked => FG_MUTED,
            Self::Renamed   => ACCENT,
            Self::Conflict  => RED,
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
enum DiffLineKind {
    Added,
    Deleted,
    Context,
    HunkHeader,
}

#[derive(Clone)]
struct DiffLine {
    kind: DiffLineKind,
    old_num: Option<u32>,
    new_num: Option<u32>,
    content: String,
}

#[derive(Clone)]
struct GitCommit {
    sha: String,        // full
    author: String,
    date_iso: String,
    message: String,
}

impl GitCommit {
    fn short_sha(&self) -> &str {
        &self.sha[..self.sha.len().min(7)]
    }
    fn message_first_line(&self) -> &str {
        self.message.lines().next().unwrap_or("")
    }
    fn relative_time(&self) -> String {
        relative_date(&self.date_iso)
    }
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

#[derive(Clone, PartialEq)]
enum FocusRegion {
    Sidebar,
    DiffCanvas,
    #[allow(dead_code)]
    CommitSummary,
    #[allow(dead_code)]
    CommitBody,
}

#[derive(Clone)]
struct AppState {
    repo_path:       Option<PathBuf>,
    current_branch:  String,

    // sidebar
    tab:             SidebarTab,
    staged:          Vec<GitFile>,
    unstaged:        Vec<GitFile>,
    commits:         Vec<GitCommit>,
    branches:        Vec<GitBranch>,

    // diff
    diff_lines:      Vec<DiffLine>,
    active_file:     Option<String>,
    selected_file_idx: Option<usize>, // index into (unstaged ++ staged)

    // commit panel
    commit_summary:  String,
    commit_body:     String,
    commit_expanded: bool,

    // focus
    focus:           FocusRegion,

    // error banner
    error:           Option<String>,

    // loading
    loading:         bool,
}

impl AppState {
    fn new(repo_path: Option<PathBuf>) -> Self {
        Self {
            repo_path,
            current_branch: String::new(),
            tab: SidebarTab::Changes,
            staged: vec![],
            unstaged: vec![],
            commits: vec![],
            branches: vec![],
            diff_lines: vec![],
            active_file: None,
            selected_file_idx: None,
            commit_summary: String::new(),
            commit_body: String::new(),
            commit_expanded: false,
            focus: FocusRegion::Sidebar,
            error: None,
            loading: false,
        }
    }

    #[allow(dead_code)]
    fn all_files(&self) -> Vec<&GitFile> {
        // conflicts first, then unstaged, then staged
        let mut v: Vec<&GitFile> = self.unstaged.iter()
            .filter(|f| f.status == FileStatus::Conflict)
            .collect();
        v.extend(self.unstaged.iter().filter(|f| f.status != FileStatus::Conflict));
        v.extend(self.staged.iter());
        v
    }

    #[allow(dead_code)]
    fn selected_file(&self) -> Option<&GitFile> {
        self.selected_file_idx.and_then(|i| self.all_files().into_iter().nth(i))
    }
}

// ── Git shell commands ─────────────────────────────────────────────────────────

fn git(repo: &Path, args: &[&str]) -> Result<String, String> {
    let out = Command::new("git")
        .current_dir(repo)
        .args(args)
        .output()
        .map_err(|e| e.to_string())?;
    if out.status.success() {
        Ok(String::from_utf8_lossy(&out.stdout).into_owned())
    } else {
        Err(String::from_utf8_lossy(&out.stderr).trim().to_string())
    }
}

fn git_current_branch(repo: &Path) -> String {
    git(repo, &["rev-parse", "--abbrev-ref", "HEAD"])
        .unwrap_or_default().trim().to_string()
}

fn git_status(repo: &Path) -> (Vec<GitFile>, Vec<GitFile>) {
    let raw = match git(repo, &["status", "--porcelain", "-u"]) {
        Ok(s) => s,
        Err(_) => return (vec![], vec![]),
    };
    let mut staged = vec![];
    let mut unstaged = vec![];
    for line in raw.lines() {
        if line.len() < 3 { continue; }
        let xy = &line[..2];
        let path = line[3..].to_string();
        let x = xy.chars().next().unwrap_or(' ');
        let y = xy.chars().nth(1).unwrap_or(' ');

        // Staged column (X)
        if x != ' ' && x != '?' {
            let status = char_to_status(x);
            staged.push(GitFile { path: path.clone(), status, staged: true });
        }
        // Unstaged / untracked column (Y)
        if y != ' ' {
            let status = if y == '?' { FileStatus::Untracked } else { char_to_status(y) };
            unstaged.push(GitFile { path: path.clone(), status, staged: false });
        }
    }
    (staged, unstaged)
}

fn char_to_status(c: char) -> FileStatus {
    match c {
        'M' => FileStatus::Modified,
        'A' => FileStatus::Added,
        'D' => FileStatus::Deleted,
        'R' => FileStatus::Renamed,
        'U' | 'C' => FileStatus::Conflict,
        _ => FileStatus::Untracked,
    }
}

fn git_log(repo: &Path) -> Vec<GitCommit> {
    let raw = match git(repo, &[
        "log", "--pretty=format:%H%x1f%an%x1f%ai%x1f%s", "-n", "200",
    ]) {
        Ok(s) => s,
        Err(_) => return vec![],
    };
    raw.lines().filter_map(|line| {
        let parts: Vec<&str> = line.splitn(4, '\x1f').collect();
        if parts.len() < 4 { return None; }
        Some(GitCommit {
            sha: parts[0].to_string(),
            author: parts[1].to_string(),
            date_iso: parts[2].to_string(),
            message: parts[3].to_string(),
        })
    }).collect()
}

fn git_diff(repo: &Path, path: &str, staged: bool) -> Vec<DiffLine> {
    let args = if staged {
        vec!["diff", "--cached", "--unified=5", "--", path]
    } else {
        vec!["diff", "--unified=5", "--", path]
    };
    let raw = match git(repo, &args) {
        Ok(s) => s,
        Err(_) => return vec![],
    };
    parse_diff(&raw)
}

fn parse_diff(raw: &str) -> Vec<DiffLine> {
    let mut lines = vec![];
    let mut old_n = 0u32;
    let mut new_n = 0u32;
    for line in raw.lines() {
        if line.starts_with("@@") {
            // parse @@ -a,b +c,d @@
            if let Some(rest) = line.split_once(" -") {
                if let Some((old_part, rem)) = rest.1.split_once(' ') {
                    let old_start: u32 = old_part.split(',').next()
                        .and_then(|s| s.parse().ok()).unwrap_or(1);
                    if let Some(new_part) = rem.strip_prefix('+') {
                        let new_start: u32 = new_part.split(',').next()
                            .and_then(|s| s.parse().ok()).unwrap_or(1);
                        old_n = old_start;
                        new_n = new_start;
                    }
                }
            }
            lines.push(DiffLine {
                kind: DiffLineKind::HunkHeader,
                old_num: None, new_num: None,
                content: line.to_string(),
            });
        } else if line.starts_with('+') && !line.starts_with("+++") {
            lines.push(DiffLine {
                kind: DiffLineKind::Added,
                old_num: None, new_num: Some(new_n),
                content: line[1..].to_string(),
            });
            new_n += 1;
        } else if line.starts_with('-') && !line.starts_with("---") {
            lines.push(DiffLine {
                kind: DiffLineKind::Deleted,
                old_num: Some(old_n), new_num: None,
                content: line[1..].to_string(),
            });
            old_n += 1;
        } else if !line.starts_with("diff ") && !line.starts_with("index ")
               && !line.starts_with("---") && !line.starts_with("+++") {
            let content = if line.starts_with(' ') { line[1..].to_string() } else { line.to_string() };
            lines.push(DiffLine {
                kind: DiffLineKind::Context,
                old_num: Some(old_n), new_num: Some(new_n),
                content,
            });
            old_n += 1;
            new_n += 1;
        }
    }
    lines
}

fn git_branches(repo: &Path) -> Vec<GitBranch> {
    let raw = match git(repo, &["branch", "-vv", "--all"]) {
        Ok(s) => s,
        Err(_) => return vec![],
    };
    raw.lines().filter_map(|line| {
        let current = line.starts_with('*');
        let trimmed = line.trim_start_matches(['*', ' ']);
        let name = trimmed.split_whitespace().next()?.to_string();
        let is_remote = name.starts_with("remotes/");
        let clean_name = name.trim_start_matches("remotes/").to_string();
        // parse ahead/behind from [origin/main: ahead 2, behind 1]
        let ahead = parse_num_after(line, "ahead ");
        let behind = parse_num_after(line, "behind ");
        Some(GitBranch { name: clean_name, is_current: current, is_remote, ahead, behind })
    }).collect()
}

fn parse_num_after(s: &str, pat: &str) -> u32 {
    if let Some(idx) = s.find(pat) {
        s[idx + pat.len()..].split_whitespace().next()
            .and_then(|n| n.trim_matches(|c: char| !c.is_numeric()).parse().ok())
            .unwrap_or(0)
    } else { 0 }
}

fn git_stage(repo: &Path, path: &str) {
    let _ = git(repo, &["add", "--", path]);
}

fn git_unstage(repo: &Path, path: &str) {
    let _ = git(repo, &["reset", "HEAD", "--", path]);
}

fn git_commit(repo: &Path, summary: &str, body: &str) -> Result<(), String> {
    let msg = if body.is_empty() {
        summary.to_string()
    } else {
        format!("{}\n\n{}", summary, body)
    };
    git(repo, &["commit", "-m", &msg]).map(|_| ())
}

// ── Utilities ──────────────────────────────────────────────────────────────────

fn relative_date(iso: &str) -> String {
    // iso: "2024-05-18 12:34:56 +0000" — parse date part only
    let date_part = iso.split(' ').next().unwrap_or(iso);
    let parts: Vec<u32> = date_part.split('-').filter_map(|s| s.parse().ok()).collect();
    if parts.len() < 3 { return iso.to_string(); }
    // Rough approximation using wall-clock date comparison is hard without chrono.
    // Return the date string formatted nicely.
    format!("{} {}, {}", month_name(parts[1]), parts[2], parts[0])
}

fn month_name(m: u32) -> &'static str {
    match m {
        1=>"Jan",2=>"Feb",3=>"Mar",4=>"Apr",5=>"May",6=>"Jun",
        7=>"Jul",8=>"Aug",9=>"Sep",10=>"Oct",11=>"Nov",12=>"Dec",_=>"?"
    }
}

fn trunc(s: &str, max: usize) -> String {
    if s.chars().count() <= max { s.to_string() }
    else { format!("{}…", &s[..s.char_indices().nth(max).map(|(i,_)| i).unwrap_or(s.len())]) }
}

fn open_repo_picker() -> Option<PathBuf> {
    // Use `open` CLI or system dialog — fallback: env current dir
    let cwd = std::env::current_dir().ok()?;
    if cwd.join(".git").exists() { Some(cwd) } else { None }
}

// ── Main ───────────────────────────────────────────────────────────────────────

fn main() {
    let repo_path = open_repo_picker();
    let theme = github_dark();

    let state: Signal<AppState> = Signal::new(AppState::new(repo_path.clone()));

    // Initial load
    if let Some(ref rp) = repo_path {
        let rp = rp.clone();
        let s = state.clone();
        thread::spawn(move || {
            let mut st = s.get();
            st.loading = true;
            s.set(st.clone());
            st.current_branch = git_current_branch(&rp);
            let (staged, unstaged) = git_status(&rp);
            st.staged = staged;
            st.unstaged = unstaged;
            st.commits = git_log(&rp);
            st.branches = git_branches(&rp);
            st.loading = false;
            s.set(st);
        });
    }

    let scroll_y = Signal::new(0.0f32);
    let max_scroll = Signal::new((-1.0f32, -1.0f32));

    App::run(
        move |_opener: &WindowOpener, _closer: &WindowCloser| {
            let t = github_dark();
            let view = build_ui(state.clone(), scroll_y.clone(), max_scroll.clone(), &t);
            (t, view)
        },
        theme,
        "Glyph Git",
        1280.0,
        800.0,
    );
}

fn github_dark() -> Theme {
    ui_glyph::github_dark_theme()
}

// ── UI Build ───────────────────────────────────────────────────────────────────

fn build_ui(
    state: Signal<AppState>,
    scroll_y: Signal<f32>,
    max_scroll: Signal<(f32, f32)>,
    theme: &Theme,
) -> View {
    let st = state.get();

    // ── Error banner ───────────────────────────────────────────────────────────
    let error_banner: View = if let Some(ref err) = st.error {
        let s2 = state.clone();
        row(vec![
            column(vec![]).width(3.0).bg(RED).into(), // 3px left accent strip
            hgap(SPACE_3),
            text(trunc(err, 80), TEXT_SM).color(RED).into(),
            spacer(),
            button_view(
                row(vec![icon_close_outline(FG_MUTED, 14.0)]).align_center().justify_center().into(),
                move || { let mut st = s2.get(); st.error = None; s2.set(st); },
            ).bg(Color::TRANSPARENT).hover_bg(Color::rgba(1.,1.,1.,0.06))
             .width(28.0).height(28.0).radius(RADIUS_MD).into(),
            hgap(SPACE_2),
        ])
        .gap(0.0)
        .height(32.0)
        .fill_width()
        .bg(Color::rgb(0.176, 0.082, 0.082)) // #2D1515
        .align_center()
        .into()
    } else {
        column(vec![]).into()
    };

    // ── Topbar (40px) ──────────────────────────────────────────────────────────
    let topbar = render_topbar(&st, theme, state.clone());

    // ── Main body: nav + sidebar + canvas ─────────────────────────────────────
    let nav_strip  = render_nav(&st, theme, state.clone());
    let sidebar    = render_sidebar(&st, theme, state.clone(), scroll_y.clone(), max_scroll.clone());
    let diff_canvas = render_diff_canvas(&st, theme, state.clone(), scroll_y.clone(), max_scroll.clone());

    let body = row(vec![nav_strip, sidebar, diff_canvas])
        .gap(0.0)
        .fill_width()
        .grow()
        .into();

    column(vec![error_banner, topbar, body])
        .gap(0.0)
        .fill_width()
        .fill_height()
        .bg(BG_BASE)
        .into()
}

// ── Topbar ─────────────────────────────────────────────────────────────────────

fn render_topbar(st: &AppState, _theme: &Theme, state: Signal<AppState>) -> View {
    let branch = if st.current_branch.is_empty() {
        "—".to_string()
    } else {
        st.current_branch.clone()
    };

    let repo_name = st.repo_path.as_ref()
        .and_then(|p| p.file_name())
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| "No repository".to_string());

    // Refresh button
    let s2 = state.clone();
    let repo = st.repo_path.clone();
    let refresh_btn = button_view(
        row(vec![icon_refresh_outline(FG_MUTED, 14.0)]).align_center().justify_center().into(),
        move || {
            if let Some(ref rp) = repo {
                let rp = rp.clone();
                let s = s2.clone();
                thread::spawn(move || {
                    let mut st = s.get();
                    st.loading = true;
                    s.set(st.clone());
                    st.current_branch = git_current_branch(&rp);
                    let (staged, unstaged) = git_status(&rp);
                    st.staged = staged;
                    st.unstaged = unstaged;
                    st.commits = git_log(&rp);
                    st.branches = git_branches(&rp);
                    st.loading = false;
                    s.set(st);
                });
            }
        },
    )
    .bg(Color::TRANSPARENT).hover_bg(BG_ELEVATED)
    .width(32.0).height(32.0).radius(RADIUS_MD)
    .into();

    row(vec![
        hgap(SPACE_4),
        icon_git_branch_outline(FG_MUTED, 14.0),
        hgap(SPACE_2),
        text(&repo_name, TEXT_SM).color(FG).weight(FontWeight::Bold).into(),
        hgap(SPACE_3),
        column(vec![]).width(1.0).height(20.0).bg(BORDER).into(), // separator
        hgap(SPACE_3),
        icon_git_branch_outline(FG_MUTED, 12.0),
        hgap(SPACE_1),
        text(&branch, TEXT_SM).color(FG_MUTED).family(FontFamily::Monospace).into(),
        spacer(),
        if st.loading {
            text("Fetching…", TEXT_XS).color(FG_MUTED).into()
        } else {
            column(vec![]).into()
        },
        hgap(SPACE_2),
        refresh_btn,
        hgap(SPACE_3),
    ])
    .gap(0.0)
    .height(40.0)
    .fill_width()
    .bg(BG_SURFACE)
    .align_center()
    .border(BORDER, 1.0) // bottom border via border
    .into()
}

// ── Nav strip (48px wide) ──────────────────────────────────────────────────────

fn render_nav(st: &AppState, _theme: &Theme, state: Signal<AppState>) -> View {
    let tab = st.tab.clone();

    let mk_nav_btn = |icon: View, this_tab: SidebarTab, s: Signal<AppState>| -> View {
        let active = tab == this_tab;
        let bg = if active { BG_ELEVATED } else { Color::TRANSPARENT };
        let accent_bar: View = if active {
            column(vec![]).width(2.0).height(28.0).bg(ACCENT).into()
        } else {
            column(vec![]).width(2.0).into()
        };
        row(vec![
            accent_bar,
            button_view(
                column(vec![icon]).width(32.0).height(32.0).align_center().justify_center().into(),
                move || { let mut st = s.get(); st.tab = this_tab.clone(); s.set(st); },
            ).bg(bg).hover_bg(BG_ELEVATED).width(40.0).height(40.0).radius(RADIUS_MD).into(),
            hgap(SPACE_1),
        ])
        .gap(0.0)
        .align_center()
        .into()
    };

    let s1 = state.clone();
    let s2 = state.clone();
    let s3 = state.clone();

    column(vec![
        gap(SPACE_4),
        mk_nav_btn(icon_code_slash_outline(if tab == SidebarTab::Changes { FG } else { FG_MUTED }, 18.0), SidebarTab::Changes, s1),
        gap(SPACE_1),
        mk_nav_btn(icon_git_commit_outline(if tab == SidebarTab::History { FG } else { FG_MUTED }, 18.0), SidebarTab::History, s2),
        gap(SPACE_1),
        mk_nav_btn(icon_git_branch_outline(if tab == SidebarTab::Branches { FG } else { FG_MUTED }, 18.0), SidebarTab::Branches, s3),
        spacer(),
        column(vec![
            icon_settings_outline(FG_MUTED, 18.0),
        ]).width(40.0).height(40.0).align_center().justify_center().into(),
        gap(SPACE_2),
    ])
    .gap(0.0)
    .width(48.0)
    .fill_height()
    .bg(BG_SURFACE)
    .border(BORDER, 1.0)
    .into()
}

// ── Sidebar (240px) ────────────────────────────────────────────────────────────

fn render_sidebar(
    st: &AppState,
    theme: &Theme,
    state: Signal<AppState>,
    scroll_y: Signal<f32>,
    max_scroll: Signal<(f32, f32)>,
) -> View {
    // Tab bar
    let tab_bar = render_tab_bar(st, state.clone());

    // Active panel
    let panel: View = match st.tab {
        SidebarTab::Changes  => render_changes_panel(st, theme, state.clone()),
        SidebarTab::History  => render_history_panel(st, theme, state.clone(), scroll_y.clone(), max_scroll.clone()),
        SidebarTab::Branches => render_branches_panel(st, theme, state.clone()),
    };

    // Commit panel pinned at bottom (only for Changes tab)
    let commit_panel: View = if st.tab == SidebarTab::Changes {
        render_commit_panel(st, state.clone())
    } else {
        column(vec![]).into()
    };

    column(vec![tab_bar, panel, commit_panel])
        .gap(0.0)
        .width(240.0)
        .fill_height()
        .bg(BG_SURFACE)
        .border(BORDER, 1.0)
        .into()
}

fn render_tab_bar(st: &AppState, state: Signal<AppState>) -> View {
    let tab = st.tab.clone();

    let mk_tab = |label: &str, this_tab: SidebarTab, s: Signal<AppState>| -> View {
        let active = tab == this_tab;
        let fg = if active { FG } else { FG_MUTED };
        let underline: View = if active {
            column(vec![]).height(2.0).fill_width().bg(ACCENT).into()
        } else {
            column(vec![]).height(2.0).into()
        };
        button_view(
            column(vec![
                text(label, TEXT_XS).color(fg).weight(FontWeight::Bold).into(),
                underline,
            ])
            .gap(0.0)
            .width(80.0)
            .height(32.0)
            .align_center()
            .justify_center()
            .into(),
            move || { let mut st = s.get(); st.tab = this_tab.clone(); s.set(st); },
        )
        .bg(Color::TRANSPARENT)
        .hover_bg(BG_ELEVATED)
        .width(80.0).height(32.0)
        .into()
    };

    let s1 = state.clone();
    let s2 = state.clone();
    let s3 = state.clone();

    row(vec![
        mk_tab("CHANGES",  SidebarTab::Changes,  s1),
        mk_tab("HISTORY",  SidebarTab::History,  s2),
        mk_tab("BRANCHES", SidebarTab::Branches, s3),
    ])
    .gap(0.0)
    .height(32.0)
    .fill_width()
    .bg(BG_SURFACE)
    .border(BORDER, 1.0)
    .into()
}

// ── Changes panel ──────────────────────────────────────────────────────────────

fn render_changes_panel(st: &AppState, _theme: &Theme, state: Signal<AppState>) -> View {
    if st.repo_path.is_none() {
        return column(vec![
            gap(SPACE_6),
            text("No repository open", TEXT_SM).color(FG_MUTED).into(),
        ])
        .align_center()
        .fill_width()
        .grow()
        .into();
    }

    let has_unstaged = !st.unstaged.is_empty();
    let has_staged   = !st.staged.is_empty();

    if !has_unstaged && !has_staged {
        return column(vec![
            gap(SPACE_6),
            icon_checkmark_outline(GREEN, 24.0),
            gap(SPACE_2),
            text("Nothing to commit", TEXT_SM).color(FG_MUTED).into(),
            text("Working tree clean", TEXT_XS).color(FG_MUTED).into(),
        ])
        .align_center()
        .fill_width()
        .grow()
        .into();
    }

    let mut rows: Vec<View> = vec![];

    // Unstaged section
    if has_unstaged {
        rows.push(section_header(&format!("UNSTAGED  ({})", st.unstaged.len())));
        for (i, f) in st.unstaged.iter().enumerate() {
            let file = f.clone();
            let file_path_cb = f.path.clone();
            let s = state.clone();
            let repo = st.repo_path.clone();
            let selected = st.selected_file_idx == Some(i);

            rows.push(file_row(file, selected, false, move || {
                // Stage on checkbox click
                if let Some(ref rp) = repo {
                    git_stage(rp, &file_path_cb);
                }
                let mut st = s.get();
                let (staged, unstaged) = git_status(st.repo_path.as_ref().unwrap());
                st.staged = staged; st.unstaged = unstaged;
                st.max_scroll_reset();
                s.set(st);
            }, {
                let s2 = state.clone();
                let idx = i;
                let repo2 = st.repo_path.clone();
                let fp = f.path.clone();
                let is_staged = false;
                move || {
                    let mut st = s2.get();
                    st.selected_file_idx = Some(idx);
                    st.active_file = Some(fp.clone());
                    if let Some(ref rp) = repo2 {
                        st.diff_lines = git_diff(rp, &fp, is_staged);
                    }
                    st.focus = FocusRegion::DiffCanvas;
                    s2.set(st);
                }
            }));
        }
    }

    // Staged section
    if has_staged {
        rows.push(section_header(&format!("STAGED  ({})", st.staged.len())));
        let offset = st.unstaged.len();
        for (i, f) in st.staged.iter().enumerate() {
            let file = f.clone();
            let file_path_cb = f.path.clone();
            let s = state.clone();
            let repo = st.repo_path.clone();
            let selected = st.selected_file_idx == Some(offset + i);

            rows.push(file_row(file, selected, true, move || {
                // Unstage on checkbox click
                if let Some(ref rp) = repo {
                    git_unstage(rp, &file_path_cb);
                }
                let mut st = s.get();
                let (staged, unstaged) = git_status(st.repo_path.as_ref().unwrap());
                st.staged = staged; st.unstaged = unstaged;
                st.max_scroll_reset();
                s.set(st);
            }, {
                let s2 = state.clone();
                let idx = offset + i;
                let repo2 = st.repo_path.clone();
                let fp = f.path.clone();
                let is_staged = true;
                move || {
                    let mut st = s2.get();
                    st.selected_file_idx = Some(idx);
                    st.active_file = Some(fp.clone());
                    if let Some(ref rp) = repo2 {
                        st.diff_lines = git_diff(rp, &fp, is_staged);
                    }
                    st.focus = FocusRegion::DiffCanvas;
                    s2.set(st);
                }
            }));
        }
    }

    column(rows)
        .gap(0.0)
        .fill_width()
        .grow()
        .clip()
        .into()
}

fn section_header(label: &str) -> View {
    row(vec![
        hgap(SPACE_3),
        text(label, TEXT_XS).color(FG_MUTED).weight(FontWeight::Bold).into(),
        spacer(),
    ])
    .gap(0.0)
    .height(24.0)
    .fill_width()
    .bg(BG_SURFACE)
    .align_center()
    .border(BORDER, 1.0)
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
    let filename = Path::new(&file.path)
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| file.path.clone());
    let filename = trunc(&filename, 24);

    let bg = if selected { DIFF_HUNK_BG } else { Color::TRANSPARENT };
    let left_bar: View = if selected {
        column(vec![]).width(2.0).fill_height().bg(ACCENT).into()
    } else {
        column(vec![]).width(2.0).into()
    };

    // Checkbox — filled square if staged, outline if not
    let checkbox: View = button_view(
        column(vec![
            if is_staged {
                icon_checkmark_outline(Color::rgb(1., 1., 1.), 10.0)
            } else {
                column(vec![]).into()
            },
        ])
        .width(14.0).height(14.0)
        .align_center().justify_center()
        .bg(if is_staged { ACCENT } else { Color::TRANSPARENT })
        .border(if is_staged { ACCENT } else { BORDER }, 1.0)
        .radius(2.0)
        .into(),
        on_checkbox,
    )
    .bg(Color::TRANSPARENT).hover_bg(Color::TRANSPARENT)
    .width(20.0).height(22.0)
    .into();

    let row_content = row(vec![
        left_bar,
        checkbox,
        // filename — grows
        button(trunc(&filename, 22), on_select)
            .bg(Color::TRANSPARENT).hover_bg(Color::TRANSPARENT)
            .text_color(FG).font_size(TEXT_XS)
            .padding(0.0)
            .into(),
        spacer(),
        // status chip
        text(status_label, TEXT_XS).color(status_color).weight(FontWeight::Bold).into(),
        hgap(SPACE_2),
    ])
    .gap(0.0)
    .height(22.0)
    .fill_width()
    .bg(bg)
    .align_center()
    .into();

    column(vec![
        row_content,
        column(vec![]).height(1.0).fill_width().bg(BORDER).into(),
    ])
    .gap(0.0)
    .into()
}

// ── History panel ──────────────────────────────────────────────────────────────

fn render_history_panel(
    st: &AppState,
    _theme: &Theme,
    _state: Signal<AppState>,
    scroll_y: Signal<f32>,
    max_scroll: Signal<(f32, f32)>,
) -> View {
    if st.commits.is_empty() {
        return column(vec![
            gap(SPACE_6),
            text("No commits yet", TEXT_SM).color(FG_MUTED).into(),
        ])
        .align_center()
        .fill_width()
        .grow()
        .into();
    }

    let rows: Vec<View> = st.commits.iter().map(|c| commit_row(c)).collect();

    scroll(
        column(rows).gap(0.0).fill_width().into(),
        Signal::new(0.0),
        scroll_y,
        max_scroll,
    )
    .fill_width()
    .grow()
    .into()
}

fn commit_row(c: &GitCommit) -> View {
    let sha   = c.short_sha().to_string();
    let msg   = trunc(c.message_first_line(), 26);
    let author = trunc(&c.author, 18);
    let date  = c.relative_time();

    column(vec![
        row(vec![
            hgap(SPACE_3),
            icon_git_commit_outline(FG_MUTED, 12.0),
            hgap(SPACE_2),
            column(vec![
                text(&msg, TEXT_XS).color(FG).into(),
                row(vec![
                    text(&author, TEXT_XS).color(FG_MUTED).into(),
                    hgap(SPACE_2),
                    text(&date, TEXT_XS).color(FG_MUTED).into(),
                    spacer(),
                    text(&sha, TEXT_XS).color(FG_MUTED).family(FontFamily::Monospace).into(),
                    hgap(SPACE_2),
                ])
                .gap(0.0)
                .fill_width()
                .into(),
            ])
            .gap(SPACE_1)
            .grow()
            .into(),
        ])
        .gap(0.0)
        .fill_width()
        .height(40.0)
        .align_center()
        .padding_y(SPACE_2)
        .into(),
        column(vec![]).height(1.0).fill_width().bg(BORDER).into(),
    ])
    .gap(0.0)
    .into()
}

// ── Branches panel ─────────────────────────────────────────────────────────────

fn render_branches_panel(st: &AppState, _theme: &Theme, _state: Signal<AppState>) -> View {
    if st.branches.is_empty() {
        return column(vec![
            gap(SPACE_6),
            text("No branches", TEXT_SM).color(FG_MUTED).into(),
        ])
        .align_center()
        .fill_width()
        .grow()
        .into();
    }

    let local: Vec<&GitBranch> = st.branches.iter().filter(|b| !b.is_remote).collect();
    let remote: Vec<&GitBranch> = st.branches.iter().filter(|b| b.is_remote).collect();

    let mut rows: Vec<View> = vec![];

    if !local.is_empty() {
        rows.push(section_header(&format!("LOCAL  ({})", local.len())));
        for b in &local {
            rows.push(branch_row(b));
        }
    }
    if !remote.is_empty() {
        rows.push(section_header(&format!("REMOTE  ({})", remote.len())));
        for b in &remote {
            rows.push(branch_row(b));
        }
    }

    column(rows)
        .gap(0.0)
        .fill_width()
        .grow()
        .clip()
        .into()
}

fn branch_row(b: &GitBranch) -> View {
    let name = trunc(&b.name, 26);
    let fg = if b.is_current { FG } else { FG_MUTED };
    let icon: View = if b.is_current {
        icon_chevron_forward_outline(GREEN, 12.0)
    } else {
        icon_git_branch_outline(FG_MUTED, 12.0)
    };

    let tracking: View = if b.ahead > 0 || b.behind > 0 {
        row(vec![
            if b.ahead > 0 {
                text(format!("↑{}", b.ahead), TEXT_XS).color(ACCENT).into()
            } else { column(vec![]).into() },
            if b.behind > 0 {
                text(format!("↓{}", b.behind), TEXT_XS).color(AMBER).into()
            } else { column(vec![]).into() },
        ])
        .gap(SPACE_1)
        .into()
    } else {
        column(vec![]).into()
    };

    column(vec![
        row(vec![
            hgap(SPACE_3),
            icon,
            hgap(SPACE_2),
            text(&name, TEXT_XS).color(fg).family(FontFamily::Monospace).into(),
            spacer(),
            tracking,
            hgap(SPACE_2),
        ])
        .gap(0.0)
        .height(22.0)
        .fill_width()
        .align_center()
        .into(),
        column(vec![]).height(1.0).fill_width().bg(BORDER).into(),
    ])
    .gap(0.0)
    .into()
}

// ── Commit panel (pinned bottom, 80px / 140px expanded) ───────────────────────

fn render_commit_panel(st: &AppState, state: Signal<AppState>) -> View {
    let summary_sig: Signal<String> = {
        let s = Signal::new(st.commit_summary.clone());
        s
    };
    let body_sig: Signal<String> = Signal::new(st.commit_body.clone());
    let expanded = st.commit_expanded;
    let char_count = st.commit_summary.len();
    let over_limit = char_count >= 72;
    let counter_color = if over_limit { RED } else { FG_MUTED };
    let has_staged = !st.staged.is_empty();

    let s_sum = state.clone();
    let s_exp = state.clone();
    let s_commit = state.clone();
    let s_push = state.clone();

    // Summary input
    let summary_input: View = {
        let cur_val = st.commit_summary.clone();
        let s = s_sum.clone();
        row(vec![
            text_input(summary_sig.clone(), Signal::new(true), Signal::new(0))
                .placeholder("Summary (required)")
                .font_size(TEXT_SM)
                .bg(BG_BASE)
                .text_color(FG)
                .border_color(BORDER)
                .on_change(move |v| {
                    let mut st = s.get(); st.commit_summary = v; s.set(st);
                })
                .fill_width()
                .into(),
            text(format!("{}/72", char_count), TEXT_XS).color(counter_color).into(),
            hgap(SPACE_2),
        ])
        .gap(0.0)
        .height(40.0)
        .fill_width()
        .bg(BG_BASE)
        .align_center()
        .border(BORDER, 1.0)
        .into()
    };

    // Body textarea (only when expanded)
    let body_input: View = if expanded {
        let s = state.clone();
        text_input(body_sig.clone(), Signal::new(true), Signal::new(0))
            .placeholder("Extended description…")
            .font_size(TEXT_SM)
            .bg(BG_BASE)
            .text_color(FG)
            .border_color(BORDER)
            .on_change(move |v| {
                let mut st = s.get(); st.commit_body = v; s.set(st);
            })
            .fill_width()
            .into()
    } else {
        column(vec![]).into()
    };

    // Expand toggle
    let expand_btn: View = {
        let s = s_exp.clone();
        button_view(
            row(vec![
                if expanded {
                    icon_chevron_down_outline(FG_MUTED, 12.0)
                } else {
                    icon_chevron_forward_outline(FG_MUTED, 12.0)
                },
            ]).align_center().justify_center().into(),
            move || { let mut st = s.get(); st.commit_expanded = !st.commit_expanded; s.set(st); },
        )
        .bg(Color::TRANSPARENT).hover_bg(BG_ELEVATED)
        .width(24.0).height(24.0).radius(RADIUS_MD)
        .into()
    };

    // Commit button
    let commit_bg  = if has_staged { Color::rgb(0.137, 0.525, 0.212) } else { BG_ELEVATED }; // #238636
    let commit_fg  = if has_staged { FG } else { FG_MUTED };
    let summary_c  = st.commit_summary.clone();
    let body_c     = st.commit_body.clone();
    let repo_c     = st.repo_path.clone();
    let commit_btn: View = button("Commit", move || {
        if !has_staged || summary_c.is_empty() { return; }
        if let Some(ref rp) = repo_c {
            let _ = git_commit(rp, &summary_c, &body_c);
            let mut st = s_commit.get();
            let (staged, unstaged) = git_status(rp);
            st.staged = staged; st.unstaged = unstaged;
            st.commits = git_log(rp);
            st.commit_summary = String::new();
            st.commit_body = String::new();
            st.commit_expanded = false;
            st.max_scroll_reset();
            s_commit.set(st);
        }
    })
    .bg(commit_bg).hover_bg(if has_staged { Color::rgb(0.180, 0.627, 0.259) } else { BG_ELEVATED })
    .text_color(commit_fg)
    .font_size(TEXT_XS).padding(SPACE_3).radius(RADIUS_MD)
    .grow()
    .into();

    let action_row = row(vec![
        hgap(SPACE_2),
        commit_btn,
        hgap(SPACE_2),
        expand_btn,
        hgap(SPACE_1),
    ])
    .gap(0.0)
    .height(40.0)
    .fill_width()
    .bg(BG_SURFACE)
    .align_center()
    .border(BORDER, 1.0)
    .into();

    column(vec![
        column(vec![]).height(1.0).fill_width().bg(BORDER).into(),
        body_input,
        summary_input,
        action_row,
    ])
    .gap(0.0)
    .fill_width()
    .into()
}

// ── Diff canvas ─────────────────────────────────────────────────────────────────

fn render_diff_canvas(
    st: &AppState,
    _theme: &Theme,
    state: Signal<AppState>,
    scroll_y: Signal<f32>,
    max_scroll: Signal<(f32, f32)>,
) -> View {
    if st.repo_path.is_none() {
        return render_no_repo(state);
    }

    // Context topbar
    let file_label = st.active_file.as_deref().unwrap_or("");
    let diff_topbar = row(vec![
        hgap(SPACE_4),
        icon_document_outline(FG_MUTED, 12.0),
        hgap(SPACE_2),
        text(trunc(file_label, 60), TEXT_XS).color(FG_MUTED).family(FontFamily::Monospace).into(),
        spacer(),
    ])
    .gap(0.0)
    .height(32.0)
    .fill_width()
    .bg(BG_SURFACE)
    .align_center()
    .border(BORDER, 1.0)
    .into();

    // Diff content
    let diff_content: View = if st.diff_lines.is_empty() && st.active_file.is_none() {
        render_clean_state(st)
    } else if st.diff_lines.is_empty() && st.active_file.is_some() {
        column(vec![
            gap(SPACE_6),
            text("No changes", TEXT_SM).color(FG_MUTED).into(),
        ])
        .align_center()
        .fill_width()
        .grow()
        .into()
    } else {
        render_diff_lines(st, scroll_y, max_scroll)
    };

    column(vec![diff_topbar, diff_content])
        .gap(0.0)
        .grow()
        .fill_height()
        .bg(BG_BASE)
        .into()
}

fn render_no_repo(state: Signal<AppState>) -> View {
    let s = state.clone();
    column(vec![
        gap(SPACE_6),
        icon_folder_outline(FG_MUTED, 40.0),
        gap(SPACE_4),
        text("No repository open", TEXT_BASE).color(FG).weight(FontWeight::Bold).into(),
        text("Open a local repository to get started", TEXT_SM).color(FG_MUTED).into(),
        gap(SPACE_4),
        button("Open Repository", move || {
            // Try current directory
            if let Some(p) = open_repo_picker() {
                let mut st = s.get();
                st.repo_path = Some(p.clone());
                st.current_branch = git_current_branch(&p);
                let (staged, unstaged) = git_status(&p);
                st.staged = staged; st.unstaged = unstaged;
                st.commits = git_log(&p);
                st.branches = git_branches(&p);
                s.set(st);
            }
        })
        .bg(Color::rgb(0.137, 0.525, 0.212))
        .hover_bg(Color::rgb(0.180, 0.627, 0.259))
        .text_color(FG)
        .font_size(TEXT_SM).padding(SPACE_4).radius(RADIUS_MD)
        .into(),
    ])
    .gap(0.0)
    .align_center()
    .fill_width()
    .fill_height()
    .justify_center()
    .grow()
    .bg(BG_BASE)
    .into()
}

fn render_clean_state(st: &AppState) -> View {
    let last = st.commits.first();
    let commit_card: View = if let Some(c) = last {
        column(vec![
            row(vec![
                icon_git_commit_outline(FG_MUTED, 14.0),
                hgap(SPACE_2),
                text(c.short_sha(), TEXT_XS).color(FG_MUTED).family(FontFamily::Monospace).into(),
                spacer(),
                text(c.relative_time(), TEXT_XS).color(FG_MUTED).into(),
            ])
            .gap(0.0)
            .align_center()
            .into(),
            text(trunc(c.message_first_line(), 48), TEXT_SM).color(FG).into(),
            text(&c.author, TEXT_XS).color(FG_MUTED).into(),
        ])
        .gap(SPACE_2)
        .padding(SPACE_4)
        .bg(BG_ELEVATED)
        .border(BORDER, 1.0)
        .radius(RADIUS_LG)
        .into()
    } else {
        column(vec![]).into()
    };

    column(vec![
        gap(SPACE_6),
        icon_checkmark_outline(GREEN, 32.0),
        gap(SPACE_3),
        text("Nothing to commit", TEXT_BASE).color(FG_MUTED).into(),
        text("Working tree clean", TEXT_XS).color(FG_MUTED).into(),
        gap(SPACE_4),
        commit_card,
    ])
    .gap(0.0)
    .align_center()
    .fill_width()
    .grow()
    .justify_center()
    .into()
}

fn render_diff_lines(
    st: &AppState,
    scroll_y: Signal<f32>,
    max_scroll: Signal<(f32, f32)>,
) -> View {
    let rows: Vec<View> = st.diff_lines.iter().map(|line| diff_line_row(line)).collect();

    scroll(
        column(rows).gap(0.0).fill_width().into(),
        Signal::new(0.0),
        scroll_y,
        max_scroll,
    )
    .fill_width()
    .grow()
    .into()
}

fn diff_line_row(line: &DiffLine) -> View {
    let (bg, gutter_bg, text_color, sign) = match line.kind {
        DiffLineKind::Added     => (DIFF_ADD_BG, Color::rgb(0.102, 0.251, 0.129), GREEN, "+"),
        DiffLineKind::Deleted   => (DIFF_DEL_BG, Color::rgb(0.239, 0.082, 0.094), RED,   "-"),
        DiffLineKind::Context   => (BG_BASE,     BG_BASE,                          FG,    " "),
        DiffLineKind::HunkHeader=> (DIFF_HUNK_BG, DIFF_HUNK_BG,                  FG_MUTED," "),
    };

    let is_hunk = matches!(line.kind, DiffLineKind::HunkHeader);
    let h = if is_hunk { 20.0 } else { 24.0 };

    // Gutter: old / new line numbers + sign
    let old_str = line.old_num.map(|n| n.to_string()).unwrap_or_default();
    let new_str = line.new_num.map(|n| n.to_string()).unwrap_or_default();

    let gutter = row(vec![
        hgap(SPACE_2),
        text(old_str, TEXT_XS).color(FG_MUTED).family(FontFamily::Monospace).into(),
        hgap(SPACE_1),
        text(new_str, TEXT_XS).color(FG_MUTED).family(FontFamily::Monospace).into(),
        hgap(SPACE_1),
        text(sign, TEXT_XS).color(text_color).family(FontFamily::Monospace).into(),
        hgap(SPACE_1),
    ])
    .gap(0.0)
    .width(52.0)
    .height(h)
    .bg(gutter_bg)
    .align_center()
    .into();

    let code_text = if is_hunk {
        text(trunc(&line.content, 80), TEXT_XS).color(text_color).family(FontFamily::Monospace).into()
    } else {
        text(line.content.clone(), TEXT_XS).color(text_color).family(FontFamily::Monospace).into()
    };

    row(vec![
        gutter,
        column(vec![]).width(1.0).height(h).bg(BORDER).into(),
        hgap(SPACE_2),
        code_text,
    ])
    .gap(0.0)
    .height(h)
    .fill_width()
    .bg(bg)
    .align_center()
    .into()
}

// ── AppState helper for max_scroll reset ───────────────────────────────────────

impl AppState {
    fn max_scroll_reset(&mut self) {
        // Signals are managed externally; this is a no-op marker
        // Actual reset happens via signal.set((-1.0,-1.0)) in closures
    }
}
