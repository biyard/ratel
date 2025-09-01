import 'package:ratel/exports.dart';

class NotificationsModel {
  final List<NotificationFollower> networks;

  const NotificationsModel({required this.networks});
}

class NotificationFollower {
  final int createdAt;
  final NetworkModel follower;
  final bool isFollowing;
  final bool isRejecting;

  const NotificationFollower({
    required this.createdAt,
    required this.follower,
    required this.isFollowing,
    required this.isRejecting,
  });
}
