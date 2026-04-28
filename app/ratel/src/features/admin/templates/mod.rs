// Source-of-truth mirrors for the transactional email templates registered in AWS SES.
// These files are not used at runtime — the runtime path is
// `EmailTemplate::send_email` → `SesClient::send_bulk_with_template(name, ...)`,
// which renders the template stored in AWS SES.
//
// Keep each constant in sync with the corresponding SES template. To sync:
//   aws ses update-template --template file://<exported>.json
// or to create a new one:
//   aws ses create-template --template file://<exported>.json

pub mod email_verification;
pub mod mention_notification;
pub mod reply_on_comment;
pub mod signup_code;
pub mod space_action_ongoing_notification;
pub mod space_status_notification;
