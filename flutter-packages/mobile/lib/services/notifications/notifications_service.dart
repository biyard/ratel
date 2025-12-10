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
    const darwinInit = DarwinInitializationSettings();
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
    final route = data['route'] as String?;
    if (route != null && route.isNotEmpty) {
      Get.rootDelegate.toNamed(route);
      return;
    }

    final type = data['type'] as String?;
    if (type == 'space_post') {
      final spacePk = data['space_pk'] as String?;
      final postPk = data['post_pk'] as String?;
      if (spacePk != null && postPk != null) {
        Get.rootDelegate.toNamed(AppRoutes.spacePostWithPk(spacePk, postPk));
      }
    }
  }
}
