// SES template: mention_notification
// Sent by: EmailOperation::MentionNotification
// Variables: mentioned_by_name, comment_preview, cta_url

#[allow(dead_code)]
pub const SUBJECT: &str = "{{mentioned_by_name}} mentioned you on Ratel";

#[allow(dead_code)]
pub const TEXT: &str = "{{mentioned_by_name}} mentioned you in a comment:\n\n\"{{comment_preview}}\"\n\nView: {{cta_url}}\n\n— Ratel";

#[allow(dead_code)]
pub const HTML: &str = r#"<!DOCTYPE html><html><body style="font-family:Arial,sans-serif;padding:24px;background:#f7f7f7"><div style="max-width:560px;margin:0 auto;background:#fff;border-radius:8px;padding:32px"><h2 style="margin:0 0 16px;color:#12121a">You were mentioned</h2><p style="color:#333"><strong>{{mentioned_by_name}}</strong> mentioned you in a comment:</p><blockquote style="border-left:4px solid #fcb300;padding:12px 16px;margin:16px 0;background:#fafafa;color:#444">{{comment_preview}}</blockquote><p><a href="{{cta_url}}" style="display:inline-block;background:#fcb300;color:#12121a;padding:12px 24px;border-radius:6px;text-decoration:none;font-weight:600">View Comment</a></p><p style="color:#888;font-size:12px;margin-top:32px">— Ratel</p></div></body></html>"#;
