pub fn signup_html(name: &str, code: &str) -> String {
    let display_name = if name.trim().is_empty() { "" } else { name };

    let mut chars: Vec<char> = code.chars().collect();
    chars.truncate(6);
    while chars.len() < 6 {
        chars.push('•');
    }

    let box_td = |d: char| {
        format!(
            r#"<td align="center" style="background:#F5F5F5;border-radius:8px;width:48px;height:48px;">
                  <div style="font-weight:700;font-size:16px;line-height:24px;color:#000;">
                    {d}
                  </div>
               </td>"#
        )
    };

    let boxes_html = chars
        .into_iter()
        .map(box_td)
        .collect::<Vec<_>>()
        .join(r#"<td style="width:10px">&nbsp;</td>"#);

    let html = format!(
        r#"<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <meta name="color-scheme" content="light only">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>Ratel Security Code</title>
  </head>
  <body style="margin:0;padding:24px;background:#f7f7f7;">
    <table role="presentation" width="100%" cellspacing="0" cellpadding="0" border="0">
      <tr>
        <td align="center">
          <table role="presentation" width="560" cellspacing="0" cellpadding="0" border="0" style="background:#FFFFFF;border-radius:14px;padding:24px;">
            <tr>
              <td>
                <table role="presentation" width="100%" cellspacing="0" cellpadding="0" border="0" style="margin:0 0 16px 0;">
                  <tr>
                    <td style="padding:0 0 12px 0;">
                      <img src="https://metadata.ratel.foundation/ratel-logo.png" alt="Ratel" height="28" style="display:block;">
                    </td>
                  </tr>
                  <tr><td style="height:1px;background:#E5E5E5;line-height:1px;font-size:0;">&nbsp;</td></tr>
                </table>

                <h2 style="margin:18px 0 10px 0;font-weight:600;font-size:20px;line-height:20px;color:#171717;">
                  Ratel Security Code
                </h2>

                <div style="margin:0 0 20px 0;font-weight:400;font-size:13px;line-height:20px;color:#262626;">
                  Hi {display_name}<br>
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
                      {boxes_html}
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
</html>"#
    );

    html
}
