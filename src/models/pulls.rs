use super::*;
use crate::models::commits::CommentReactions;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, typed_builder::TypedBuilder)]
#[non_exhaustive]
pub struct PullRequest {
    pub url: String,
    pub id: PullRequestId,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub node_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub html_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub diff_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub patch_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub issue_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub commits_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub review_comments_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub review_comment_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub comments_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub statuses_url: Option<Url>,
    /// The pull request number.  Note that GitHub's REST API
    /// considers every pull-request an issue with the same number.
    pub number: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub state: Option<IssueState>,
    #[serde(default)]
    pub locked: bool,
    #[serde(default)]
    pub maintainer_can_modify: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub user: Option<Box<Author>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub body_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub body_html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub labels: Option<Vec<Label>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub milestone: Option<Box<Milestone>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub active_lock_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub closed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub mergeable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub mergeable_state: Option<MergeableState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub merged: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub merged_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub merged_by: Option<Box<Author>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub merge_commit_sha: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub assignee: Option<Box<Author>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub assignees: Option<Vec<Author>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub requested_reviewers: Option<Vec<Author>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub requested_teams: Option<Vec<teams::RequestedTeam>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub rebaseable: Option<bool>,
    pub head: Box<Head>,
    pub base: Box<Base>,
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub links: Option<Box<Links>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub author_association: Option<AuthorAssociation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub draft: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub repo: Option<Box<Repository>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub additions: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub deletions: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub changed_files: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub commits: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub review_comments: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub comments: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, typed_builder::TypedBuilder)]
#[non_exhaustive]
pub struct Head {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label: Option<String>,
    #[serde(rename = "ref")]
    pub ref_field: String,
    pub sha: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub user: Option<Author>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub repo: Option<Repository>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, typed_builder::TypedBuilder)]
#[non_exhaustive]
pub struct Base {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label: Option<String>,
    #[serde(rename = "ref")]
    pub ref_field: String,
    pub sha: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub user: Option<Author>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub repo: Option<Repository>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, typed_builder::TypedBuilder)]
#[non_exhaustive]
pub struct Links {
    #[serde(rename = "self")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub self_link: Option<SelfLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub html_link: Option<HtmlLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub issue_link: Option<IssueLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub comments_link: Option<CommentsLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub review_comments_link: Option<ReviewCommentsLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub review_comment_link: Option<ReviewCommentLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub commits_link: Option<CommitsLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub statuses_link: Option<StatusesLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    #[serde(rename = "pull_request")]
    #[builder(default)]
    pub pull_request_link: Option<PullRequestLink>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, typed_builder::TypedBuilder)]
#[non_exhaustive]
pub struct SelfLink {
    pub href: Url,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, typed_builder::TypedBuilder)]
#[non_exhaustive]
pub struct HtmlLink {
    pub href: Url,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, typed_builder::TypedBuilder)]
#[non_exhaustive]
pub struct IssueLink {
    pub href: Url,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, typed_builder::TypedBuilder)]
#[non_exhaustive]
pub struct CommentsLink {
    pub href: Url,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, typed_builder::TypedBuilder)]
#[non_exhaustive]
pub struct ReviewCommentsLink {
    pub href: Url,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, typed_builder::TypedBuilder)]
#[non_exhaustive]
pub struct ReviewCommentLink {
    pub href: Url,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, typed_builder::TypedBuilder)]
#[non_exhaustive]
pub struct CommitsLink {
    pub href: Url,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, typed_builder::TypedBuilder)]
#[non_exhaustive]
pub struct StatusesLink {
    pub href: Url,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, typed_builder::TypedBuilder)]
#[non_exhaustive]
pub struct PullRequestLink {
    pub href: Url,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, typed_builder::TypedBuilder)]
#[non_exhaustive]
pub struct Review {
    pub id: ReviewId,
    pub node_id: String,
    pub html_url: Url,
    #[builder(default)]
    pub user: Option<Author>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub body_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub body_html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub commit_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub state: Option<ReviewState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub pull_request_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub submitted_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub links: Option<Links>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub author_association: Option<AuthorAssociation>,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize)]
#[serde(rename_all(serialize = "SCREAMING_SNAKE_CASE"))]
#[non_exhaustive]
pub enum ReviewState {
    Open,
    Approved,
    Pending,
    ChangesRequested,
    Commented,
    Dismissed,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize)]
#[serde(rename_all(serialize = "SCREAMING_SNAKE_CASE"))]
#[non_exhaustive]
pub enum ReviewAction {
    Approve,
    RequestChanges,
    Comment,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, typed_builder::TypedBuilder)]
#[non_exhaustive]
pub struct Comment {
    pub url: Url,
    #[builder(default)]
    pub pull_request_review_id: Option<ReviewId>,
    pub id: CommentId,
    pub node_id: String,
    pub diff_hunk: String,
    pub path: String,
    pub position: Option<u64>,
    pub original_position: Option<u64>,
    pub commit_id: String,
    pub original_commit_id: String,
    #[serde(default)]
    #[builder(default)]
    pub in_reply_to_id: Option<CommentId>,
    pub user: Option<Author>,
    pub body: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub html_url: String,
    pub author_association: AuthorAssociation,
    #[serde(rename = "_links")]
    pub links: Links,
    #[builder(default)]
    pub start_line: Option<u64>,
    #[builder(default)]
    pub original_start_line: Option<u64>,
    #[builder(default)]
    pub start_side: Option<String>,
    #[builder(default)]
    pub line: Option<u64>,
    #[builder(default)]
    pub original_line: Option<u64>,
    #[builder(default)]
    pub side: Option<String>,
}

///Legacy Review Comment
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, typed_builder::TypedBuilder)]
#[non_exhaustive]
pub struct ReviewComment {
    pub url: Url,
    pub pull_request_review_id: Option<ReviewId>,
    pub id: CommentId,
    pub node_id: String,
    pub diff_hunk: String,
    pub path: String,
    pub position: Option<u64>,
    pub original_position: Option<u64>,
    pub commit_id: String,
    pub original_commit_id: String,
    #[serde(default)]
    #[builder(default)]
    pub in_reply_to_id: Option<CommentId>,
    pub user: Option<Author>,
    pub body: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub html_url: String,
    pub pull_request_url: String,
    pub author_association: AuthorAssociation,
    #[serde(rename = "_links")]
    pub links: Links,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub body_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub body_html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub reactions: Option<CommentReactions>,
    #[builder(default)]
    pub side: Option<Side>,
    #[builder(default)]
    pub start_side: Option<Side>,
    #[builder(default)]
    pub line: Option<u64>,
    #[builder(default)]
    pub original_line: Option<u64>,
    #[builder(default)]
    pub start_line: Option<u64>,
    #[builder(default)]
    pub original_start_line: Option<u64>,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize)]
#[serde(rename_all(serialize = "SCREAMING_SNAKE_CASE"))]
#[non_exhaustive]
pub enum Side {
    Left,
    Right,
}

/// A Thread in a pull request review
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, typed_builder::TypedBuilder)]
#[non_exhaustive]
pub struct Thread {
    pub comments: Vec<Comment>,
    pub node_id: String,
}

// This is rather annoying, but Github uses both SCREAMING_SNAKE_CASE and snake_case
// for the review state, it's uppercase when coming from an API request, but
// lowercase when coming from a webhook payload, so we need to deserialize both,
// but still use uppercase for serialization
impl<'de> Deserialize<'de> for ReviewState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;

        impl serde::de::Visitor<'_> for Visitor {
            type Value = ReviewState;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(match value {
                    "OPEN" | "open" => ReviewState::Open,
                    "APPROVED" | "approved" => ReviewState::Approved,
                    "PENDING" | "pending" => ReviewState::Pending,
                    "CHANGES_REQUESTED" | "changes_requested" => ReviewState::ChangesRequested,
                    "COMMENTED" | "commented" => ReviewState::Commented,
                    "DISMISSED" | "dismissed" => ReviewState::Dismissed,
                    unknown => return Err(E::custom(format!("unknown variant `{unknown}`, expected one of `open`, `approved`, `pending`, `changes_requested`, `commented`, `dismissed`"))),
                })
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}

//same, see above
impl<'de> Deserialize<'de> for Side {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;

        impl serde::de::Visitor<'_> for Visitor {
            type Value = Side;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(match value.to_uppercase().as_str() {
                    "LEFT" => Side::Left,
                    "RIGHT" => Side::Right,
                    unknown => {
                        return Err(E::custom(format!(
                            "unknown variant `{unknown}`, expected one of `left`, `right`"
                        )))
                    }
                })
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum PullRequestReviewAction {
    Submitted,
    Edited,
    Dismissed,
}

/// The complete list of actions that can trigger the sending of a
/// `pull_request` webhook
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum PullRequestAction {
    Opened,
    Edited,
    Closed,
    Assigned,
    Unassigned,
    ReviewRequested,
    ReviewRequestRemoved,
    ReadyForReview,
    Labeled,
    Unlabeled,
    Synchronize,
    Locked,
    Unlocked,
    Reopened,
}

#[derive(Debug, Clone, Serialize, Deserialize, typed_builder::TypedBuilder)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct Merge {
    #[builder(default)]
    pub sha: Option<String>,
    #[builder(default)]
    pub message: Option<String>,
    pub merged: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum MergeableState {
    /// The head ref is out of date.
    Behind,
    /// The merge is blocked, eg. the base branch is protected by a required
    /// status check that is pending
    Blocked,
    /// Mergeable and passing commit status.
    Clean,
    /// The merge commit cannot be cleanly created.
    Dirty,
    /// The merge is blocked due to the pull request being a draft.
    Draft,
    /// Mergeable with passing commit status and pre-receive hooks.
    HasHooks,
    /// The state cannot currently be determined.
    Unknown,
    /// Mergeable with non-passing commit status.
    Unstable,
}

#[deprecated(note = "use repos::DiffEntry instead")]
pub type FileDiff = repos::DiffEntry;

#[deprecated(note = "use repos::DiffEntryStatus instead")]
pub type FileDiffStatus = repos::DiffEntryStatus;

#[cfg(test)]
mod test {
    #[test]
    fn deserializes_review_state() {
        use super::ReviewState;

        let states: Vec<ReviewState> = serde_json::from_str(
            r#"["APPROVED","pending","CHANGES_REQUESTED","commented", "dismissed"]"#,
        )
        .unwrap();

        assert_eq!(
            states,
            &[
                ReviewState::Approved,
                ReviewState::Pending,
                ReviewState::ChangesRequested,
                ReviewState::Commented,
                ReviewState::Dismissed,
            ]
        );
    }
}
