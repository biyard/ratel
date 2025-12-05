class NotificationsPage {
  final List<AppNotification> items;
  final String? bookmark;

  const NotificationsPage({required this.items, required this.bookmark});

  factory NotificationsPage.fromJson(Map<String, dynamic> json) {
    final itemsJson = (json['items'] as List? ?? []);
    final items = itemsJson
        .map((e) => AppNotification.fromJson(e as Map<String, dynamic>))
        .toList();

    final bookmarkRes = json['bookmark']?.toString();
    return NotificationsPage(
      items: items,
      bookmark: bookmarkRes?.isEmpty == true ? null : bookmarkRes,
    );
  }
}

class AppNotification {
  final String pk;
  final String sk;
  final int createdAt;
  final int? readedAt;
  final NotificationStatus status;
  final EmailOperation operation;

  AppNotification({
    required this.pk,
    required this.sk,
    required this.createdAt,
    required this.readedAt,
    required this.status,
    required this.operation,
  });

  factory AppNotification.fromJson(Map<String, dynamic> json) {
    final statusRaw = json['status']?.toString() ?? 'Unread';
    final opJson = (json['operation'] as Map?)?.cast<String, dynamic>() ?? {};

    return AppNotification(
      pk: json['pk']?.toString() ?? '',
      sk: json['sk']?.toString() ?? '',
      createdAt: int.tryParse(json['created_at'].toString()) ?? 0,
      readedAt: json['readed_at'] == null
          ? null
          : int.tryParse(json['readed_at'].toString()),
      status: NotificationStatusX.fromString(statusRaw),
      operation: EmailOperation.fromJson(opJson),
    );
  }

  bool get isRead => status == NotificationStatus.read;
  bool get isUnread => status == NotificationStatus.unread;

  String get notificationId {
    final parts = sk.split('#');
    if (parts.isEmpty) return sk;
    return parts.last;
  }
}

enum NotificationStatus { unread, read }

extension NotificationStatusX on NotificationStatus {
  static NotificationStatus fromString(String v) {
    switch (v.toLowerCase()) {
      case 'read':
        return NotificationStatus.read;
      case 'unread':
      default:
        return NotificationStatus.unread;
    }
  }

  String get asString {
    switch (this) {
      case NotificationStatus.read:
        return 'Read';
      case NotificationStatus.unread:
        return 'Unread';
    }
  }
}

abstract class EmailOperation {
  const EmailOperation();

  factory EmailOperation.fromJson(Map<String, dynamic> json) {
    if (json.containsKey('post_title')) {
      return SpacePostNotificationOperation.fromJson(json);
    }
    if (json.containsKey('team_name')) {
      return TeamInviteOperation.fromJson(json);
    }
    if (json.containsKey('space_desc') && json.containsKey('cta_url')) {
      return SpaceInviteVerificationOperation.fromJson(json);
    }
    if (json.containsKey('code_1') &&
        json.containsKey('code_2') &&
        json.containsKey('code_3') &&
        json.containsKey('code_4') &&
        json.containsKey('code_5') &&
        json.containsKey('code_6')) {
      return SignupSecurityCodeOperation.fromJson(json);
    }
    if (json.containsKey('survey_title')) {
      return StartSurveyOperation.fromJson(json);
    }
    return UnknownEmailOperation(json: json);
  }
}

class SpacePostNotificationOperation extends EmailOperation {
  final String authorProfile;
  final String authorDisplayName;
  final String authorUsername;
  final String postTitle;
  final String postDesc;
  final String connectLink;

  const SpacePostNotificationOperation({
    required this.authorProfile,
    required this.authorDisplayName,
    required this.authorUsername,
    required this.postTitle,
    required this.postDesc,
    required this.connectLink,
  });

  factory SpacePostNotificationOperation.fromJson(Map<String, dynamic> json) {
    return SpacePostNotificationOperation(
      authorProfile: json['author_profile']?.toString() ?? '',
      authorDisplayName: json['author_display_name']?.toString() ?? '',
      authorUsername: json['author_username']?.toString() ?? '',
      postTitle: json['post_title']?.toString() ?? '',
      postDesc: json['post_desc']?.toString() ?? '',
      connectLink: json['connect_link']?.toString() ?? '',
    );
  }
}

class TeamInviteOperation extends EmailOperation {
  final String teamName;
  final String teamProfile;
  final String teamDisplayName;
  final String url;

  const TeamInviteOperation({
    required this.teamName,
    required this.teamProfile,
    required this.teamDisplayName,
    required this.url,
  });

  factory TeamInviteOperation.fromJson(Map<String, dynamic> json) {
    return TeamInviteOperation(
      teamName: json['team_name']?.toString() ?? '',
      teamProfile: json['team_profile']?.toString() ?? '',
      teamDisplayName: json['team_display_name']?.toString() ?? '',
      url: json['url']?.toString() ?? '',
    );
  }
}

class SpaceInviteVerificationOperation extends EmailOperation {
  final String spaceTitle;
  final String spaceDesc;
  final String authorProfile;
  final String authorDisplayName;
  final String authorUsername;
  final String ctaUrl;

  const SpaceInviteVerificationOperation({
    required this.spaceTitle,
    required this.spaceDesc,
    required this.authorProfile,
    required this.authorDisplayName,
    required this.authorUsername,
    required this.ctaUrl,
  });

  factory SpaceInviteVerificationOperation.fromJson(Map<String, dynamic> json) {
    return SpaceInviteVerificationOperation(
      spaceTitle: json['space_title']?.toString() ?? '',
      spaceDesc: json['space_desc']?.toString() ?? '',
      authorProfile: json['author_profile']?.toString() ?? '',
      authorDisplayName: json['author_display_name']?.toString() ?? '',
      authorUsername: json['author_username']?.toString() ?? '',
      ctaUrl: json['cta_url']?.toString() ?? '',
    );
  }
}

class SignupSecurityCodeOperation extends EmailOperation {
  final String displayName;
  final String code1;
  final String code2;
  final String code3;
  final String code4;
  final String code5;
  final String code6;

  const SignupSecurityCodeOperation({
    required this.displayName,
    required this.code1,
    required this.code2,
    required this.code3,
    required this.code4,
    required this.code5,
    required this.code6,
  });

  factory SignupSecurityCodeOperation.fromJson(Map<String, dynamic> json) {
    return SignupSecurityCodeOperation(
      displayName: json['display_name']?.toString() ?? '',
      code1: json['code_1']?.toString() ?? '',
      code2: json['code_2']?.toString() ?? '',
      code3: json['code_3']?.toString() ?? '',
      code4: json['code_4']?.toString() ?? '',
      code5: json['code_5']?.toString() ?? '',
      code6: json['code_6']?.toString() ?? '',
    );
  }

  List<String> get codes => [code1, code2, code3, code4, code5, code6];
}

class StartSurveyOperation extends EmailOperation {
  final String spaceTitle;
  final String surveyTitle;
  final String authorProfile;
  final String authorDisplayName;
  final String authorUsername;
  final String connectLink;

  const StartSurveyOperation({
    required this.spaceTitle,
    required this.surveyTitle,
    required this.authorProfile,
    required this.authorDisplayName,
    required this.authorUsername,
    required this.connectLink,
  });

  factory StartSurveyOperation.fromJson(Map<String, dynamic> json) {
    return StartSurveyOperation(
      spaceTitle: json['space_title']?.toString() ?? '',
      surveyTitle: json['survey_title']?.toString() ?? '',
      authorProfile: json['author_profile']?.toString() ?? '',
      authorDisplayName: json['author_display_name']?.toString() ?? '',
      authorUsername: json['author_username']?.toString() ?? '',
      connectLink: json['connect_link']?.toString() ?? '',
    );
  }
}

class UnknownEmailOperation extends EmailOperation {
  final Map<String, dynamic> json;

  const UnknownEmailOperation({required this.json});
}

class MarkAsReadResult {
  final bool success;
  final int updatedCount;

  const MarkAsReadResult({required this.success, required this.updatedCount});

  factory MarkAsReadResult.fromJson(Map<String, dynamic> json) {
    return MarkAsReadResult(
      success: json['success'] == true,
      updatedCount: json['updated_count'] is int
          ? json['updated_count'] as int
          : int.tryParse(json['updated_count']?.toString() ?? '0') ?? 0,
    );
  }
}

class DeleteNotificationResult {
  final bool success;

  const DeleteNotificationResult({required this.success});

  factory DeleteNotificationResult.fromJson(Map<String, dynamic> json) {
    return DeleteNotificationResult(success: json['success'] == true);
  }
}
