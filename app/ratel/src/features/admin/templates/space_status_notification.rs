// SES template: space_status_notification
// Sent by: EmailOperation::SpaceStatusNotification
// Variables: headline, body, space_title, cta_url

#[allow(dead_code)]
pub const SUBJECT: &str = "{{headline}} — {{space_title}}";

#[allow(dead_code)]
pub const TEXT: &str = "{{headline}}\n\n{{body}}\n\nSpace: {{space_title}}\nView: {{cta_url}}\n\n— Ratel";

#[allow(dead_code)]
pub const HTML: &str = r#"<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <meta name="color-scheme" content="light only">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>{{headline}}</title>
  </head>
  <body style="margin:0;padding:24px;background:#f7f7f7;font-family:Arial,Helvetica,sans-serif;">
    <table role="presentation" width="100%" cellspacing="0" cellpadding="0" border="0">
      <tr>
        <td align="center">
          <table role="presentation" width="560" cellspacing="0" cellpadding="0" border="0" style="background:#FFFFFF;border-radius:14px;padding:24px;">
            <tr>
              <td>
                <table role="presentation" width="100%" cellspacing="0" cellpadding="0" border="0" style="margin:0 0 16px 0;">
                  <tr>
                    <td style="padding:0 0 12px 0;">
                      <img src="https://metadata.ratel.foundation/ratel-logo.png"
                           alt="Ratel"
                           height="28"
                           style="display:block;">
                    </td>
                  </tr>
                  <tr>
                    <td style="height:1px;background:#E5E5E5;line-height:1px;font-size:0;">&nbsp;</td>
                  </tr>
                </table>

                <h2 style="margin:18px 0 10px 0;font-weight:600;font-size:20px;line-height:26px;color:#171717;">
                  {{headline}}
                </h2>

                <div style="margin:0 0 16px 0;font-weight:400;font-size:13px;line-height:20px;color:#262626;">
                  {{body}}
                </div>

                <div style="border:1px solid #E5E5E5;border-radius:10px;padding:16px;margin:0 0 20px 0;">
                  <div style="font-weight:700;font-size:14px;color:#171717;">
                    {{space_title}}
                  </div>
                </div>

                <p style="margin:24px 0 0 0;text-align:center;">
                  <a href="{{cta_url}}"
                     style="display:inline-block;padding:12px 20px;border-radius:10px;
                            background:#F7B300;color:#000 !important;text-decoration:none !important;
                            font-weight:700;font-size:14px;">
                    <span style="color:#000 !important;">Open Space</span>
                  </a>
                </p>

                <p style="margin:16px 0 0 0;color:#8C8C8C;font-size:11px;text-align:center;">
                  If the button doesn’t work, use this link:
                  <a href="{{cta_url}}" style="color:#8C8C8C;">{{cta_url}}</a>
                </p>
              </td>
            </tr>
          </table>
        </td>
      </tr>
    </table>
  </body>
</html>
"#;
