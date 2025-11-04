pub fn create_space_post_html(
    title: String,
    desc: String,
    author_profile: String,
    author_display_name: String,
    author_username: String,
    connect_link: String,
) -> String {
    let html = format!(
        r#"<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>Create Space Contents</title>
    <meta name="color-scheme" content="light dark">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <style>
      .btn {{ display:inline-block;padding:12px 12px;border-radius:10px;background:#F7B300;color:#000;font-weight:700;text-decoration:none }}
      .card {{ border:1px solid #ddd;border-radius:10px;padding:20px }}
    </style>
  </head>
  <body style="margin:0;padding:24px;font-family:Arial,Helvetica,sans-serif;background:#f7f7f7;">
    <table role="presentation" width="100%" cellspacing="0" cellpadding="0">
      <tr><td align="center">
        <table role="presentation" width="560" cellspacing="0" cellpadding="0" style="background:#fff;border-radius:14px;padding:24px">
          <tr><td>

            <table role="presentation" width="100%" cellspacing="0" cellpadding="0" style="margin:0 0 16px;">
              <tr>
                <td style="padding:0 0 12px;">
                  <img src="https://metadata.ratel.foundation/ratel-logo.png" alt="Ratel" height="28" style="display:block;">
                </td>
              </tr>
              <tr>
                <td style="height:1px;background:#E5E5E5;line-height:1px;"></td>
              </tr>
            </table>
            <h2 style="margin:18px 0 10px; color:#171717; font-size:20px; font-weight:600">Space members are posting new space contents.</h2>

            <table role="presentation" cellspacing="0" cellpadding="0" border="0" style="margin:8px 0 16px; width:100%;">
              <tr>
                <td width="48" valign="top">
                  <img src="{profile}" alt="{display_name}" width="48" height="48"
                       style="display:block;border-radius:100%;object-fit:cover;">
                </td>
                <td width="8"></td>
                <td valign="middle">
                  <div style="font-weight:700;font-size:16px;color:#171717;margin-bottom:2px;">{display_name}</div>
                  <div style="font-weight:600;font-size:12px;color:#8C8C8C;">@{username}</div>
                </td>
              </tr>
            </table>

            <div class="card">
              <div style="font-weight:700;margin-bottom:10px;color:#171717;">{post_title}</div>
              <p style="
                margin:0 0 10px;
                color:#8c8c8c;
                display:-webkit-box;
                -webkit-line-clamp:2;
                -webkit-box-orient:vertical;
                overflow:hidden;
                text-overflow:ellipsis;">
                {post_desc}
              </p>
            </div>

            <p style="color:#262626;font-size:11px">
              If the button doesn’t work, use this link: <a href="{connect_link}">{connect_link}</a>
            </p>

            <p style="margin-top:24px; text-align:center;">
              <a href="{connect_link}"
                 style="display:inline-block;padding:12px 12px;border-radius:10px;
                        background:#F7B300;color:#000 !important;text-decoration:none !important;
                        font-weight:700;">
                <span style="color:#000 !important;">Go to Space</span>
              </a>
            </p>
          </td></tr>
        </table>
      </td></tr>
    </table>
  </body>
</html>"#,
        post_title = title.clone(),
        post_desc = desc.clone(),
        connect_link = connect_link,
        profile = author_profile,
        display_name = author_display_name,
        username = author_username,
    );

    return html;
}
