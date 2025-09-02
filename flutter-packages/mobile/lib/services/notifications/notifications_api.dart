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

  Future<NotificationsModel> getNotifications() async {
    final uri = Uri.parse(apiEndpoint).resolve('/v2/notifications');
    final headers = <String, String>{'Content-Type': 'application/json'};

    final res = await get(uri.toString(), headers: headers);

    if (!res.isOk) {
      return NotificationsModel(networks: []);
    }

    logger.d("notifications: ${res.body}");

    final List<NotificationFollower> networks = [];

    final networkItems = res.body["networks"];

    for (var i = 0; i < networkItems.length; i++) {
      final item = networkItems[i];
      final follower = item["follower"];

      networks.add(
        NotificationFollower(
          createdAt: int.parse(item["created_at"].toString()),
          follower: NetworkModel(
            id: int.parse(follower["id"].toString()),
            profileUrl: follower["profile_url"] ?? "",
            nickname: follower["nickname"] ?? "",
            username: follower["username"] ?? "",
            description: follower["html_contents"] ?? "",
          ),
          isFollowing: bool.parse(item["is_following"].toString()),
          isRejecting: bool.parse(item["is_rejecting"].toString()),
        ),
      );
    }

    return NotificationsModel(networks: networks);
  }
}
