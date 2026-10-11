use crate::judge::{case, check_eq};
use crate::user::Post;

pub fn run() {
    case("sample-1", || {
        let mut post = Post::new();
        post.add_text("salad");
        check_eq(post.content(), "")?;
        post.request_review();
        check_eq(post.content(), "")?;
        post.approve();
        check_eq(post.content(), "")?;
        post.approve();
        check_eq(post.content(), "salad")
    });
    case("sample-2", || {
        let mut post = Post::new();
        post.add_text("I ate ");
        post.request_review();
        post.approve();
        post.reject();
        post.add_text("a salad");
        post.request_review();
        post.approve();
        check_eq(post.content(), "")?;
        post.approve();
        check_eq(post.content(), "I ate a salad")
    });
    case("hidden-1", || {
        // A new post is an empty draft, and text is appended only while it is a draft.
        let mut post = Post::new();
        check_eq(post.content(), "")?;
        post.add_text("caf\u{e9} ");
        post.add_text("\u{2615}");
        post.add_text("");
        post.request_review();
        post.add_text(" (pending)");
        post.approve();
        post.add_text(" (approved once)");
        post.approve();
        post.add_text(" (published)");
        check_eq(post.content(), "caf\u{e9} \u{2615}")?;
        check_eq(post.content(), "caf\u{e9} \u{2615}")
    });
    case("hidden-2", || {
        // Approving or rejecting a draft does nothing.
        let mut post = Post::new();
        post.add_text("draft");
        post.approve();
        post.approve();
        post.reject();
        post.approve();
        check_eq(post.content(), "")?;
        post.add_text("!");
        post.request_review();
        post.approve();
        check_eq(post.content(), "")?;
        post.approve();
        check_eq(post.content(), "draft!")
    });
    case("hidden-3", || {
        // reject discards the approval, so two new approvals are needed.
        let mut post = Post::new();
        post.add_text("v1");
        post.request_review();
        post.approve();
        post.reject();
        post.request_review();
        post.approve();
        check_eq(post.content(), "")?;
        post.reject();
        post.reject();
        post.add_text(" v2");
        post.request_review();
        post.approve();
        check_eq(post.content(), "")?;
        post.approve();
        check_eq(post.content(), "v1 v2")
    });
    case("hidden-4", || {
        // Requesting a review again while pending keeps the approval.
        let mut post = Post::new();
        post.add_text("keep");
        post.request_review();
        post.request_review();
        post.approve();
        post.request_review();
        check_eq(post.content(), "")?;
        post.approve();
        check_eq(post.content(), "keep")
    });
    case("hidden-5", || {
        // A published post stays published, and posts do not share state.
        let mut post = Post::new();
        let mut other = Post::new();
        post.add_text("final");
        other.add_text("other");
        post.request_review();
        other.request_review();
        post.approve();
        post.approve();
        other.approve();
        check_eq(post.content(), "final")?;
        check_eq(other.content(), "")?;
        post.reject();
        post.request_review();
        post.approve();
        post.add_text(" edit");
        check_eq(post.content(), "final")?;
        other.reject();
        other.add_text("!");
        other.request_review();
        other.approve();
        other.approve();
        check_eq(other.content(), "other!")?;
        let mut empty = Post::new();
        empty.request_review();
        empty.approve();
        empty.approve();
        empty.add_text("late");
        check_eq(empty.content(), "")
    });
}
