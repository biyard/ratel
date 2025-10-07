export function isWebView(): boolean {
  const ua = navigator.userAgent || '';

  const isIOSWebView =
    /(iPhone|iPod|iPad).*AppleWebKit/.test(ua) && !/Safari/.test(ua);

  const isAndroidWebView =
    /Android/.test(ua) && (!/Chrome/.test(ua) || /wv/.test(ua));

  return isIOSWebView || isAndroidWebView;
}
