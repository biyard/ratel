# THIS FILE IS AUTO-GENERATED. DO NOT MODIFY!!

# Copyright 2020-2023 Tauri Programme within The Commons Conservancy
# SPDX-License-Identifier: Apache-2.0
# SPDX-License-Identifier: MIT

-keep class com.ratel.ratelMobile.* {
  native <methods>;
}

-keep class com.ratel.ratelMobile.WryActivity {
  public <init>(...);

  void setWebView(com.ratel.ratelMobile.RustWebView);
  java.lang.Class getAppClass(...);
  java.lang.String getVersion();
}

-keep class com.ratel.ratelMobile.Ipc {
  public <init>(...);

  @android.webkit.JavascriptInterface public <methods>;
}

-keep class com.ratel.ratelMobile.RustWebView {
  public <init>(...);

  void loadUrlMainThread(...);
  void loadHTMLMainThread(...);
  void evalScript(...);
}

-keep class com.ratel.ratelMobile.RustWebChromeClient,com.ratel.ratelMobile.RustWebViewClient {
  public <init>(...);
}
