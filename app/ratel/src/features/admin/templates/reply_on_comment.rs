// SES template: reply_on_comment_notification
// Sent by: EmailOperation::ReplyOnCommentNotification
// Variables: replier_name, comment_preview, reply_preview, cta_url

#[allow(dead_code)]
pub const SUBJECT: &str = "{{replier_name}} replied to a comment on Ratel";

#[allow(dead_code)]
pub const TEXT: &str = "{{replier_name}} replied to a comment thread you're part of.\n\nOriginal comment:\n\"{{comment_preview}}\"\n\nReply:\n\"{{reply_preview}}\"\n\nView: {{cta_url}}\n\n— Ratel";

#[allow(dead_code)]
pub const HTML: &str = r#"<!DOCTYPE html><html><body style="font-family:Arial,sans-serif;padding:24px;background:#f7f7f7"><div style="max-width:560px;margin:0 auto;background:#fff;border-radius:8px;padding:32px"><h2 style="margin:0 0 16px;color:#12121a">New reply on your comment thread</h2><p style="color:#333"><strong>{{replier_name}}</strong> replied to a comment thread you're part of.</p><p style="color:#888;font-size:12px;margin:24px 0 4px">Original comment</p><blockquote style="border-left:4px solid #d0d0d8;padding:12px 16px;margin:0 0 16px;background:#fafafa;color:#555">{{comment_preview}}</blockquote><p style="color:#888;font-size:12px;margin:16px 0 4px">Reply</p><blockquote style="border-left:4px solid #fcb300;padding:12px 16px;margin:0 0 24px;background:#fafafa;color:#444">{{reply_preview}}</blockquote><p><a href="{{cta_url}}" style="display:inline-block;background:#fcb300;color:#12121a;padding:12px 24px;border-radius:6px;text-decoration:none;font-weight:600">View Reply</a></p><p style="color:#888;font-size:12px;margin-top:32px">— Ratel</p></div></body></html>"#;
