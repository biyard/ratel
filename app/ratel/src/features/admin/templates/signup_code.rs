// SES template: signup_code
// Sent by: EmailOperation::SignupSecurityCode
// Variables: display_name, code_1..code_6

#[allow(dead_code)]
pub const SUBJECT: &str = "Ratel Security Code";

#[allow(dead_code)]
pub const TEXT: &str =
    "Your security code is {{code_1}}{{code_2}}{{code_3}}{{code_4}}{{code_5}}{{code_6}}";

#[allow(dead_code)]
pub const HTML: &str = r#"<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <meta name="color-scheme" content="light only">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>Ratel Security Code</title>
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
                <h2 style="margin:18px 0 10px 0;font-weight:600;font-size:20px;line-height:20px;color:#171717;">
                  Ratel Security Code
                </h2>
                <div style="margin:0 0 20px 0;font-weight:400;font-size:13px;line-height:20px;color:#262626;">
                  Hi {{display_name}}<br>
                  Please verify your security code to activate your account.<br>
                  Your security code is
                </div>
                <div style="text-align:center;margin:0 0 20px 0;white-space:nowrap;font-size:0;">
                  <span style="display:inline-block;background:#F5F5F5;border-radius:8px;width:48px;height:48px;line-height:48px;text-align:center;font-weight:700;font-size:16px;color:#000;letter-spacing:0;margin-right:10px;">{{code_1}}</span><span style="display:inline-block;background:#F5F5F5;border-radius:8px;width:48px;height:48px;line-height:48px;text-align:center;font-weight:700;font-size:16px;color:#000;letter-spacing:0;margin-right:10px;">{{code_2}}</span><span style="display:inline-block;background:#F5F5F5;border-radius:8px;width:48px;height:48px;line-height:48px;text-align:center;font-weight:700;font-size:16px;color:#000;letter-spacing:0;margin-right:10px;">{{code_3}}</span><span style="display:inline-block;background:#F5F5F5;border-radius:8px;width:48px;height:48px;line-height:48px;text-align:center;font-weight:700;font-size:16px;color:#000;letter-spacing:0;margin-right:10px;">{{code_4}}</span><span style="display:inline-block;background:#F5F5F5;border-radius:8px;width:48px;height:48px;line-height:48px;text-align:center;font-weight:700;font-size:16px;color:#000;letter-spacing:0;margin-right:10px;">{{code_5}}</span><span style="display:inline-block;background:#F5F5F5;border-radius:8px;width:48px;height:48px;line-height:48px;text-align:center;font-weight:700;font-size:16px;color:#000;letter-spacing:0;">{{code_6}}</span>
                </div>
                <div style="margin:0;font-size:11px;line-height:16px;color:#262626;">
                  This code expires in 30 minutes. If you didn’t request this, you can safely ignore this email.
                </div>
              </td>
            </tr>
          </table>
        </td>
      </tr>
    </table>
  </body>
</html>
"#;
