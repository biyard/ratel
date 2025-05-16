# THIS FILE IS AUTO-GENERATED. DO NOT MODIFY!!

# Copyright 2020-2023 Tauri Programme within The Commons Conservancy
# SPDX-License-Identifier: Apache-2.0
# SPDX-License-Identifier: MIT

-keep class foundation.ratel.app.* {
  native <methods>;
}

-keep class foundation.ratel.app.WryActivity {
  public <init>(...);

  void setWebView(foundation.ratel.app.RustWebView);
  java.lang.Class getAppClass(...);
  java.lang.String getVersion();
}

-keep class foundation.ratel.app.Ipc {
  public <init>(...);

  @android.webkit.JavascriptInterface public <methods>;
}

-keep class foundation.ratel.app.RustWebView {
  public <init>(...);

  void loadUrlMainThread(...);
  void loadHTMLMainThread(...);
  void evalScript(...);
}

-keep class foundation.ratel.app.RustWebChromeClient,foundation.ratel.app.RustWebViewClient {
  public <init>(...);
}
