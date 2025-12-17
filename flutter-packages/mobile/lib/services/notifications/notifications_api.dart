import 'package:ratel/exports.dart';

class NotificationApi extends GetConnect {
  final apiEndpoint = Config.apiEndpoint;

  NotificationApi() {
    httpClient.addRequestModifier<void>((req) async {
      final authApi = Get.find<AuthApi>();
      final cookie = await authApi.cookieHeaderAsync();
      logger.d('${req.method} ${req.url} | NotificationApi Cookie: $cookie');
      if (cookie?.isNotEmpty == true) req.headers['Cookie'] = cookie!;
      return req;
    });
  }

  Uri _baseUri([String path = '']) {
    final base = Uri.parse(apiEndpoint);
    const basePath = '/v3/notifications';
    if (path.isEmpty) {
      return base.resolve(basePath);
    }
    return base.resolve('$basePath$path');
  }

  Uri _meUri([String path = '']) {
    final base = Uri.parse(apiEndpoint);
    const basePath = '/v3/me';
    if (path.isEmpty) {
      return base.resolve(basePath);
    }
    return base.resolve('$basePath$path');
  }

  Future<NotificationsPage> getNotifications({String? bookmark}) async {
    final base = _baseUri();
    final uri = (bookmark == null || bookmark.isEmpty)
        ? base
        : base.replace(queryParameters: {'bookmark': bookmark});

    final headers = <String, String>{'Content-Type': 'application/json'};

    final res = await get(uri.toString(), headers: headers);

    logger.d('GET notifications: status=${res.statusCode}, body=${res.body}');

    if (!res.isOk || res.body == null) {
      return const NotificationsPage(items: [], bookmark: null);
    }

    final data = res.body as Map<String, dynamic>;

    final itemsJson = (data['items'] as List? ?? []);
    final items = itemsJson
        .map((e) => AppNotification.fromJson(e as Map<String, dynamic>))
        .toList();

    final bookmarkRes = data['bookmark']?.toString();

    return NotificationsPage(
      items: items,
      bookmark: bookmarkRes?.isEmpty == true ? null : bookmarkRes,
    );
  }

  Future<MarkAsReadResult> markAsRead(List<String> notificationIds) async {
    if (notificationIds.isEmpty) {
      return const MarkAsReadResult(success: true, updatedCount: 0);
    }

    final uri = _baseUri('/mark-as-read');
    final headers = <String, String>{'Content-Type': 'application/json'};

    final payload = {'notification_ids': notificationIds};

    final res = await post(uri.toString(), payload, headers: headers);

    logger.d('POST mark-as-read: status=${res.statusCode}, body=${res.body}');

    if (!res.isOk || res.body == null) {
      return const MarkAsReadResult(success: false, updatedCount: 0);
    }

    final data = res.body as Map<String, dynamic>;
    return MarkAsReadResult.fromJson(data);
  }

  Future<MarkAsReadResult> markAllAsRead() async {
    final uri = _baseUri('/mark-all-as-read');
    final headers = <String, String>{'Content-Type': 'application/json'};

    final res = await post(uri.toString(), {}, headers: headers);

    logger.d(
      'POST mark-all-as-read: status=${res.statusCode}, body=${res.body}',
    );

    if (!res.isOk || res.body == null) {
      return const MarkAsReadResult(success: false, updatedCount: 0);
    }

    final data = res.body as Map<String, dynamic>;
    return MarkAsReadResult.fromJson(data);
  }

  Future<DeleteNotificationResult> deleteNotification(
    String notificationId,
  ) async {
    final uri = _baseUri('/$notificationId');
    final headers = <String, String>{'Content-Type': 'application/json'};

    final res = await delete(uri.toString(), headers: headers);

    logger.d(
      'DELETE notification: id=$notificationId, status=${res.statusCode}, body=${res.body}',
    );

    if (!res.isOk || res.body == null) {
      return const DeleteNotificationResult(success: false);
    }

    final data = res.body as Map<String, dynamic>;
    return DeleteNotificationResult.fromJson(data);
  }

  Future<bool> registerNotificationDevice({
    required String deviceToken,
    required String platform, // "android" | "ios" | "web"
    String? deviceId,
  }) async {
    final uri = _meUri('/notification-devices');
    final headers = <String, String>{'Content-Type': 'application/json'};

    final payload = <String, dynamic>{
      'device_token': deviceToken,
      'platform': platform,
      if (deviceId != null && deviceId.isNotEmpty) 'device_id': deviceId,
    };

    final res = await post(uri.toString(), payload, headers: headers);

    logger.d(
      'POST register-notification-device: status=${res.statusCode}, body=${res.body}',
    );

    if (!res.isOk) {
      return false;
    }

    return true;
  }
}
