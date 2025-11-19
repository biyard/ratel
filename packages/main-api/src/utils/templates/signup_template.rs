pub const SIGNUP_SECURITY_CODE_TEMPLATE_SUBJECT: &str = "Ratel Security Code";

pub const SIGNUP_SECURITY_CODE_TEMPLATE_HTML: &str = r#"<!doctype html>
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
                <div style="text-align:center;margin:0 0 20px 0;">
                  <table role="presentation"
                         cellspacing="0"
                         cellpadding="0"
                         border="0"
                         style="margin:0 auto;">
                    <tr>
                      <td align="center" style="background:#F5F5F5;border-radius:8px;width:48px;height:48px;">
                        <div style="font-weight:700;font-size:16px;line-height:24px;color:#000;">
                          {{code_1}}
                        </div>
                      </td>
                      <td style="width:10px">&nbsp;</td>
                      <td align="center" style="background:#F5F5F5;border-radius:8px;width:48px;height:48px;">
                        <div style="font-weight:700;font-size:16px;line-height:24px;color:#000;">
                          {{code_2}}
                        </div>
                      </td>
                      <td style="width:10px">&nbsp;</td>
                      <td align="center" style="background:#F5F5F5;border-radius:8px;width:48px;height:48px;">
                        <div style="font-weight:700;font-size:16px;line-height:24px;color:#000;">
                          {{code_3}}
                        </div>
                      </td>
                      <td style="width:10px">&nbsp;</td>
                      <td align="center" style="background:#F5F5F5;border-radius:8px;width:48px;height:48px;">
                        <div style="font-weight:700;font-size:16px;line-height:24px;color:#000;">
                          {{code_4}}
                        </div>
                      </td>
                      <td style="width:10px">&nbsp;</td>
                      <td align="center" style="background:#F5F5F5;border-radius:8px;width:48px;height:48px;">
                        <div style="font-weight:700;font-size:16px;line-height:24px;color:#000;">
                          {{code_5}}
                        </div>
                      </td>
                      <td style="width:10px">&nbsp;</td>
                      <td align="center" style="background:#F5F5F5;border-radius:8px;width:48px;height:48px;">
                        <div style="font-weight:700;font-size:16px;line-height:24px;color:#000;">
                          {{code_6}}
                        </div>
                      </td>
                    </tr>
                  </table>
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
