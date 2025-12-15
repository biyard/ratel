import 'dart:convert';

import 'package:firebase_core/firebase_core.dart';
import 'package:firebase_messaging/firebase_messaging.dart';
import 'package:flutter_local_notifications/flutter_local_notifications.dart';
import 'package:ratel/exports.dart';

import 'package:shared_preferences/shared_preferences.dart';
import 'package:uuid/uuid.dart';

class NotificationsService extends GetxService {
  static NotificationsService get to => Get.find<NotificationsService>();

  static Future<void> init() async {
    if (!Get.isRegistered<NotificationsService>()) {
      Get.put<NotificationsService>(NotificationsService());
    }
    if (!Get.isRegistered<NotificationApi>()) {
      Get.put<NotificationApi>(NotificationApi());
    }
    await NotificationsService.to._initFcm();
  }

  final FlutterLocalNotificationsPlugin _flutterLocal =
      FlutterLocalNotificationsPlugin();

  String? _currentToken;

  Future<void> debugLocalNotification() async {
    logger.d('debugLocalNotification called >>>');

    const androidDetails = AndroidNotificationDetails(
      'default_channel',
      'Default',
      importance: Importance.defaultImportance,
      priority: Priority.defaultPriority,
    );
    const darwinDetails = DarwinNotificationDetails(
      presentAlert: true,
      presentBadge: true,
      presentSound: true,
    );
    const details = NotificationDetails(
      android: androidDetails,
      iOS: darwinDetails,
    );

    await _flutterLocal.show(
      0,
      'iOS Test Notification',
      'This is a local test notification.',
      details,
      payload: jsonEncode(<String, dynamic>{
        'type': 'space_post',
        'space_pk': 'SPACE#debug',
        'post_pk': 'POST#debug',
      }),
    );

    logger.d('debugLocalNotification show() completed');
  }

  Future<String> getOrCreateDeviceId() async {
    const deviceIdKey = 'ratel_device_id';
    final prefs = await SharedPreferences.getInstance();
    final existing = prefs.getString(deviceIdKey);
    if (existing != null && existing.isNotEmpty) {
      return existing;
    }

    final newId = const Uuid().v4();
    await prefs.setString(deviceIdKey, newId);
    return newId;
  }

  Future<void> _initFcm() async {
    const androidInit = AndroidInitializationSettings('@mipmap/ic_launcher');
    const darwinInit = DarwinInitializationSettings(
      requestAlertPermission: true,
      requestBadgePermission: true,
      requestSoundPermission: true,
    );

    const initSettings = InitializationSettings(
      android: androidInit,
      iOS: darwinInit,
    );

    await _flutterLocal.initialize(
      initSettings,
      onDidReceiveNotificationResponse: (resp) {
        final payload = resp.payload;
        if (payload != null && payload.isNotEmpty) {
          _handleClick(jsonDecode(payload) as Map<String, dynamic>);
        }
      },
    );

    final messaging = FirebaseMessaging.instance;

    final settings = await messaging.requestPermission(
      alert: true,
      badge: true,
      sound: true,
    );
    logger.d(
      'NotificationsService: permission=${settings.authorizationStatus}',
    );

    if (GetPlatform.isIOS) {
      final apnsToken = await messaging.getAPNSToken();
      logger.d('NotificationsService: APNS token=$apnsToken');
    }

    String? token;
    try {
      token = await messaging.getToken();
      logger.d('NotificationsService: initial FCM token=$token');
    } on FirebaseException catch (e) {
      if (e.code == 'apns-token-not-set') {
        logger.w(
          'NotificationsService: APNS token not set yet.'
          'NotificationsService: APNS token not set yet.',
        );
      } else {
        rethrow;
      }
    }

    if (token != null && token.isNotEmpty) {
      await _registerToken(token);
    }

    FirebaseMessaging.instance.onTokenRefresh.listen((token) async {
      logger.d('NotificationsService: onTokenRefresh token=$token');
      await _registerToken(token);
    });

    FirebaseMessaging.onMessage.listen((RemoteMessage message) {
      _showForegroundNotification(message);
    });

    FirebaseMessaging.onMessageOpenedApp.listen((RemoteMessage message) {
      _handleClick(message.data);
    });
  }

  Future<void> _registerToken(String token) async {
    _currentToken = token;
    logger.d('NotificationsService: got FCM token=$token');

    try {
      final platform = GetPlatform.isIOS
          ? 'ios'
          : GetPlatform.isAndroid
          ? 'android'
          : 'web';

      final deviceId = await getOrCreateDeviceId();

      final api = Get.find<NotificationApi>();
      final ok = await api.registerNotificationDevice(
        deviceToken: token,
        platform: platform,
        deviceId: deviceId,
      );

      logger.d(
        'NotificationsService: registerNotificationDevice result=$ok, '
        'token=$token, platform=$platform, deviceId=$deviceId',
      );
    } catch (e) {
      logger.e('NotificationsService: Failed to register fcm token: $e');
    }
  }

  Future<void> registerForCurrentUserIfPossible() async {
    if (_currentToken == null || _currentToken!.isEmpty) {
      logger.d('NotificationsService: no cached FCM token, skip register');
      return;
    }

    logger.d(
      'NotificationsService: registerForCurrentUserIfPossible with token=$_currentToken',
    );
    await _registerToken(_currentToken!);
  }

  Future<void> _showForegroundNotification(RemoteMessage message) async {
    final notification = message.notification;
    final androidDetails = AndroidNotificationDetails(
      'default_channel',
      'Default',
      importance: Importance.defaultImportance,
      priority: Priority.defaultPriority,
    );
    final details = NotificationDetails(android: androidDetails);

    await _flutterLocal.show(
      notification.hashCode,
      notification?.title ?? 'Ratel',
      notification?.body ?? '',
      details,
      payload: jsonEncode(message.data),
    );
  }

  void _handleClick(Map<String, dynamic> data) {
    logger.d('Notification clicked: $data');

    WidgetsBinding.instance.addPostFrameCallback((_) {
      _navigateFromNotification(data);
    });
  }

  void _navigateFromNotification(Map<String, dynamic> data) {
    final deeplink = data['deeplink'] as String?;
    if (deeplink != null && deeplink.isNotEmpty) {
      logger.d('Notification deeplink: $deeplink');

      Uri uri;
      try {
        uri = Uri.parse(deeplink);
      } catch (e) {
        logger.e('Invalid deeplink: $deeplink, error=$e');
        uri = Uri();
      }

      logger.d(
        'uri.scheme=${uri.scheme}, host=${uri.host}, pathSegments=${uri.pathSegments}',
      );

      if (uri.scheme == 'ratelapp') {
        if (uri.host == 'space') {
          final segments = uri.pathSegments;
          logger.d('space deeplink segments: $segments');

          if (segments.isNotEmpty) {
            final encodedPk = segments[0];
            logger.d('encodedPk from deeplink: $encodedPk');

            Get.rootDelegate.toNamed(spaceWithPk(encodedPk));
            return;
          }
        }
      }

      if (uri.scheme.isEmpty) {
        logger.d('Navigate using raw deeplink as route: $deeplink');
        Get.rootDelegate.toNamed(deeplink);
        return;
      }
    }

    final route = data['route'] as String?;
    if (route != null && route.isNotEmpty) {
      logger.d('Navigate using data.route: $route');
      Get.rootDelegate.toNamed(route);
      return;
    }

    final type = data['type'] as String?;
    if (type == 'space_post') {
      final spacePk = data['space_pk'] as String?;
      final postPk = data['post_pk'] as String?;
      if (spacePk != null && postPk != null) {
        logger.d(
          'Navigate using type=space_post: space=$spacePk, post=$postPk',
        );
        Get.rootDelegate.toNamed(AppRoutes.spacePostWithPk(spacePk, postPk));
      }
    }
  }
}
