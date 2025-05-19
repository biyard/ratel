import Foundation
import FirebaseCore
import Firebase
import FirebaseMessaging
import UserNotifications

@objc(AppDelegate)
class AppDelegate: NSObject, UIApplicationDelegate {
    func application(_ application: UIApplication,
                     didFinishLaunchingWithOptions launchOptions: [UIApplication.LaunchOptionsKey : Any]? = nil) -> Bool {
        print("AppDelegate launched")

        // initialize Firebase
        FirebaseApp.configure()

        // request alerm authentication
        UNUserNotificationCenter.current().delegate = self
        let authOptions: UNAuthorizationOptions = [.alert, .badge, .sound]
        UNUserNotificationCenter.current().requestAuthorization(
            options: authOptions,
            completionHandler: { granted, error in
                print("🔔 Notification permission granted: \(granted), error: \(String(describing: error))")
            }
        )

        application.registerForRemoteNotifications()
        Messaging.messaging().delegate = self

        return true
    }
}

extension AppDelegate: UNUserNotificationCenterDelegate {
    func application(_ application: UIApplication,
                     didRegisterForRemoteNotificationsWithDeviceToken deviceToken: Data) {
        print("background notification")
        Messaging.messaging().apnsToken = deviceToken
    }

    func userNotificationCenter(_ center: UNUserNotificationCenter,
                                willPresent notification: UNNotification,
                                withCompletionHandler completionHandler: @escaping (UNNotificationPresentationOptions) -> Void) {
        print("Foreground notification")
        
        if #available(iOS 14.0, *) {
                completionHandler([.list, .banner, .sound])
            } else {
                completionHandler([.sound])
            }
        
    }
}

extension AppDelegate: MessagingDelegate {
    func messaging(_ messaging: Messaging, didReceiveRegistrationToken fcmToken: String?) {
        print("Firebase registration token: \(String(describing: fcmToken))")
        // store token to db
    }
}
