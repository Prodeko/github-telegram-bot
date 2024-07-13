use super::types::{Commit, PushEvent};

fn ref_to_branch<'a>(ref_: &'a str) -> &'a str {
    return ref_.strip_prefix("refs/heads/").unwrap_or(ref_);
}

fn format_commit(commit: &Commit) -> String {
    let short_sha: String = commit.id.chars().take(7).collect();
    let url = &commit.url;
    let author = &commit.author.username;
    let message = &commit.message;
    return format!("[{short_sha}]({url}) ({author}): {message}");
}

pub fn format_push_event(event: &PushEvent) -> String {
    let commits = &event.commits;
    let number_of_commits = commits.len();
    let singular_form = number_of_commits == 1;

    let new_commits_possibly_pluralized = if singular_form {
        "uusi commit"
    } else {
        "uutta committia"
    };

    let branch = ref_to_branch(&event.r#ref);
    let full_name = &event.repository.full_name;

    let headline =
        format!("🔨 {number_of_commits} {new_commits_possibly_pluralized} - {full_name}:{branch}");

    let formatted_commits = event
        .commits
        .iter()
        .map(format_commit)
        .collect::<Vec<_>>()
        .join("\n\n");

    return format!("{headline}\n\n{formatted_commits}");
}
