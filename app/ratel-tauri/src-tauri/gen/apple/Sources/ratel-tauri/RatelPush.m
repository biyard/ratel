#import "RatelPush.h"

#import <UIKit/UIKit.h>
#import <WebKit/WebKit.h>
#import <UserNotifications/UserNotifications.h>

@import FirebaseCore;
@import FirebaseMessaging;

// Same staggered delays Android uses: `start` runs at launch, BEFORE the SPA
// (WASM) has booted and attached its `ratel-fcm-ready` / `ratel-deeplink`
// listeners, so a single inject would land on a window the SPA then replaces.
// Re-injecting on a few delays guarantees at least one lands after the web
// listeners are up. Re-firing is idempotent (token upsert; same-route nav).
static NSArray<NSNumber *> *RatelInjectDelays(void) {
  return @[ @0.8, @2.0, @4.0, @7.0 ];
}

@interface RatelPush () <UNUserNotificationCenterDelegate, FIRMessagingDelegate>
@property(nonatomic, copy) NSString *fcmToken;
@property(nonatomic, copy) NSString *pendingURL;
@end

@implementation RatelPush

+ (instancetype)shared {
  static RatelPush *shared = nil;
  static dispatch_once_t once;
  dispatch_once(&once, ^{
    shared = [RatelPush new];
  });
  return shared;
}

+ (void)start {
  [[self shared] start];
}

- (void)start {
  // `GoogleService-Info.plist` is bundled as an app resource, so the no-arg
  // configure finds it. Safe to call after the app delegate is installed —
  // Firebase's proxy swizzles it here so the later APNs token registration is
  // forwarded to FCM without us implementing the AppDelegate callback.
  [FIRApp configure];
  [FIRMessaging messaging].delegate = self;

  UNUserNotificationCenter *center = [UNUserNotificationCenter currentNotificationCenter];
  center.delegate = self;
  UNAuthorizationOptions opts =
      UNAuthorizationOptionAlert | UNAuthorizationOptionSound | UNAuthorizationOptionBadge;
  [center requestAuthorizationWithOptions:opts
                        completionHandler:^(BOOL granted, NSError *_Nullable error) {
                          if (error) {
                            NSLog(@"[RatelPush] auth error: %@", error);
                          }
                          // Register regardless of `granted`: even silently we
                          // want an APNs token so FCM can mint its token; the
                          // user can enable alerts later in Settings.
                          dispatch_async(dispatch_get_main_queue(), ^{
                            [[UIApplication sharedApplication] registerForRemoteNotifications];
                          });
                        }];
}

#pragma mark - FIRMessagingDelegate

- (void)messaging:(FIRMessaging *)messaging
    didReceiveRegistrationToken:(NSString *)fcmToken {
  // Fires with the initial token and on every refresh.
  if (fcmToken.length == 0) {
    return;
  }
  self.fcmToken = fcmToken;
  [self scheduleInjects];
}

#pragma mark - UNUserNotificationCenterDelegate

// Foreground arrival: iOS suppresses the banner by default, so opt in (Android
// renders these in RatelMessagingService.onMessageReceived for parity).
- (void)userNotificationCenter:(UNUserNotificationCenter *)center
       willPresentNotification:(UNNotification *)notification
         withCompletionHandler:
             (void (^)(UNNotificationPresentationOptions options))completionHandler {
  completionHandler(UNNotificationPresentationOptionBanner | UNNotificationPresentationOptionSound);
}

// Tap → deep link. FCM merges the message `data` keys into the notification
// userInfo, so `data.url` arrives as userInfo["url"] (same value Android reads
// from the launch intent extra).
- (void)userNotificationCenter:(UNUserNotificationCenter *)center
    didReceiveNotificationResponse:(UNNotificationResponse *)response
             withCompletionHandler:(void (^)(void))completionHandler {
  NSString *url = response.notification.request.content.userInfo[@"url"];
  if ([url isKindOfClass:[NSString class]] && url.length > 0) {
    self.pendingURL = url;
    [self injectDeepLink:url];
    // Cold start from a tap: the WebView/SPA may not exist yet, so also retry.
    [self scheduleInjects];
  }
  completionHandler();
}

#pragma mark - WebView injection

- (WKWebView *)findWebView:(UIView *)view {
  if ([view isKindOfClass:[WKWebView class]]) {
    return (WKWebView *)view;
  }
  for (UIView *sub in view.subviews) {
    WKWebView *found = [self findWebView:sub];
    if (found) {
      return found;
    }
  }
  return nil;
}

- (WKWebView *)webView {
  UIWindow *keyWindow = nil;
  for (UIScene *scene in UIApplication.sharedApplication.connectedScenes) {
    if (![scene isKindOfClass:[UIWindowScene class]]) {
      continue;
    }
    for (UIWindow *window in ((UIWindowScene *)scene).windows) {
      if (window.isKeyWindow) {
        keyWindow = window;
        break;
      }
    }
    if (keyWindow) {
      break;
    }
  }
  if (!keyWindow) {
    // No key window yet (early launch): fall back to the first window of any
    // window scene. Avoids the deprecated UIApplication.windows.
    for (UIScene *scene in UIApplication.sharedApplication.connectedScenes) {
      if ([scene isKindOfClass:[UIWindowScene class]]) {
        keyWindow = ((UIWindowScene *)scene).windows.firstObject;
        if (keyWindow) {
          break;
        }
      }
    }
  }
  UIView *root = keyWindow.rootViewController.view ?: keyWindow;
  return [self findWebView:root];
}

- (void)scheduleInjects {
  for (NSNumber *delay in RatelInjectDelays()) {
    dispatch_after(
        dispatch_time(DISPATCH_TIME_NOW, (int64_t)(delay.doubleValue * NSEC_PER_SEC)),
        dispatch_get_main_queue(), ^{
          [self injectNow];
        });
  }
}

- (void)injectNow {
  WKWebView *webView = [self webView];
  if (!webView) {
    return;
  }
  if (self.fcmToken.length > 0) {
    NSString *deviceId = UIDevice.currentDevice.identifierForVendor.UUIDString ?: @"ios";
    NSString *js = [NSString
        stringWithFormat:@"window.__RATEL_FCM__={token:'%@',deviceId:'%@',platform:'ios'};"
                         @"window.dispatchEvent(new Event('ratel-fcm-ready'));",
                         self.fcmToken, deviceId];
    [webView evaluateJavaScript:js completionHandler:nil];
  }
  if (self.pendingURL.length > 0) {
    [self injectDeepLink:self.pendingURL];
  }
}

- (void)injectDeepLink:(NSString *)url {
  WKWebView *webView = [self webView];
  if (!webView) {
    return;
  }
  NSString *js = [NSString
      stringWithFormat:@"window.__RATEL_PENDING_URL__='%@';"
                       @"window.dispatchEvent(new CustomEvent('ratel-deeplink',{detail:'%@'}));",
                       url, url];
  [webView evaluateJavaScript:js completionHandler:nil];
}

@end
