#import <Foundation/Foundation.h>

/// iOS Firebase Cloud Messaging bridge for the Tauri WebView app.
///
/// Tauri owns the UIApplication / AppDelegate (created inside Rust's
/// `ffi::start_app()`), so there is no Swift AppDelegate to add Firebase hooks
/// to. Instead `main.mm` observes `UIApplicationDidFinishLaunchingNotification`
/// and calls `+start` once the app instance exists. Firebase's default
/// AppDelegate-proxy swizzling then forwards the APNs device token to FCM
/// automatically — we only configure Firebase, ask for notification
/// permission, and inject the resulting FCM token + tap deep links into the
/// WKWebView (mirroring Android's MainActivity).
@interface RatelPush : NSObject
+ (void)start;
@end
