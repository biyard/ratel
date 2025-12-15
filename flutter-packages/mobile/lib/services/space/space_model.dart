import 'package:ratel/exports.dart';

class MySpaces {
  final List<MySpaceItem> items;
  final String? bookmark;

  const MySpaces({required this.items, this.bookmark});

  factory MySpaces.fromJson(Json j) {
    final list = (j['items'] as List? ?? const [])
        .whereType<Json>()
        .map(MySpaceItem.fromJson)
        .toList();

    final bm = j['bookmark']?.toString();
    return MySpaces(
      items: list,
      bookmark: (bm == null || bm.isEmpty) ? null : bm,
    );
  }
}

enum MySpaceInvitationStatus { pending, participating }

MySpaceInvitationStatus invitationStatusFromJson(dynamic v) {
  final s = (v ?? '').toString().toLowerCase();
  switch (s) {
    case 'pending':
      return MySpaceInvitationStatus.pending;
    case 'participating':
      return MySpaceInvitationStatus.participating;
    default:
      return MySpaceInvitationStatus.participating;
  }
}

class MySpaceItem {
  final MySpaceInvitationStatus invitationStatus;

  final String pk;
  final String sk;
  final int createdAt;
  final int updatedAt;

  final SpaceStatus? status;
  final SpaceVisibility visibility;
  final SpacePublishState publishState;
  final SpaceType spaceType;
  final BoosterType booster;

  final String postPk;
  final String content;

  final String userPk;
  final String authorDisplayName;
  final String authorProfileUrl;
  final String authorUsername;

  final int? startedAt;
  final int? endedAt;

  final bool anonymousParticipation;
  final bool changeVisibility;
  final int participants;
  final bool blockParticipate;
  final int quota;
  final int remains;

  final String title;
  final List<FileModel> files;

  const MySpaceItem({
    required this.invitationStatus,
    required this.pk,
    required this.sk,
    required this.createdAt,
    required this.updatedAt,
    required this.status,
    required this.visibility,
    required this.publishState,
    required this.spaceType,
    required this.booster,
    required this.postPk,
    required this.content,
    required this.userPk,
    required this.authorDisplayName,
    required this.authorProfileUrl,
    required this.authorUsername,
    required this.startedAt,
    required this.endedAt,
    required this.anonymousParticipation,
    required this.changeVisibility,
    required this.participants,
    required this.blockParticipate,
    required this.quota,
    required this.remains,
    required this.title,
    required this.files,
  });

  factory MySpaceItem.fromJson(Json j) {
    List<FileModel> fileList() {
      final raw = j['files'];
      if (raw is List) {
        return raw.whereType<Json>().map(FileModel.fromJson).toList();
      }
      return const [];
    }

    return MySpaceItem(
      invitationStatus: invitationStatusFromJson(j['invitation_status']),
      pk: j['pk']?.toString() ?? '',
      sk: j['sk']?.toString() ?? '',
      createdAt: _asIntOrNull(j['created_at']) ?? 0,
      updatedAt: _asIntOrNull(j['updated_at']) ?? 0,
      status: spaceStatusFromJson(j['status']),
      visibility: spaceVisibilityFromJson(j['visibility']),
      publishState: spacePublishStateFromJson(j['publish_state']),
      spaceType: spaceTypeFromJson(j['space_type']),
      booster: boosterTypeFromJson(j['booster']),
      postPk: (j['post_pk'] ?? '') as String,
      content: (j['content'] ?? '') as String,
      userPk: (j['user_pk'] ?? '') as String,
      authorDisplayName: (j['author_display_name'] ?? '') as String,
      authorProfileUrl: (j['author_profile_url'] ?? '') as String,
      authorUsername: (j['author_username'] ?? '') as String,
      startedAt: _asIntOrNull(j['started_at']),
      endedAt: _asIntOrNull(j['ended_at']),
      anonymousParticipation: (j['anonymous_participation'] as bool?) ?? false,
      changeVisibility: (j['change_visibility'] as bool?) ?? false,
      participants: _asIntOrNull(j['participants']) ?? 0,
      blockParticipate: (j['block_participate'] as bool?) ?? false,
      quota: _asIntOrNull(j['quota']) ?? 0,
      remains: _asIntOrNull(j['remains']) ?? 0,
      title: (j['title'] ?? '') as String,
      files: fileList(),
    );
  }

  bool get isClosed {
    return invitationStatus == MySpaceInvitationStatus.pending &&
        blockParticipate;
  }
}

class SpaceSummary {
  final int id;
  final int createdAt;
  final int updatedAt;

  final int feedId;

  final String title;
  final String htmlContents;
  final String imageUrl;

  final String authorUrl;
  final String authorName;

  final int likes;
  final int rewards;
  final int comments;

  final bool isBookmarked;

  const SpaceSummary({
    required this.id,
    required this.createdAt,
    required this.updatedAt,
    required this.feedId,
    required this.title,
    required this.htmlContents,
    required this.imageUrl,
    required this.authorUrl,
    required this.authorName,
    required this.likes,
    required this.rewards,
    required this.comments,
    required this.isBookmarked,
  });
}

class SpaceModel {
  final String pk;
  final String sk;
  final String title;
  final String content;
  final int createdAt;
  final int updatedAt;
  final List<String> urls;

  final SpaceType spaceType;
  final List<String> features;
  final SpaceStatus? status;
  final TeamGroupPermissions permissions;

  final UserType authorType;
  final String authorDisplayName;
  final String authorUsername;
  final String authorProfileUrl;

  final bool certified;
  int reports;
  final int likes;
  final int comments;
  final int shares;
  final int rewards;

  bool isReport;

  final SpaceVisibility visibility;
  final SpacePublishState publishState;
  final BoosterType booster;

  final List<FileModel> files;

  final bool anonymousParticipation;
  final bool canParticipate;
  final bool changeVisibility;
  final bool participated;
  final String? participantDisplayName;
  final String? participantProfileUrl;
  final String? participantUsername;

  final bool blockParticipate;

  final List<SpaceRequirementModel> requirements;
  final int remains;
  final int quota;

  SpaceModel({
    required this.pk,
    required this.sk,
    required this.title,
    required this.content,
    required this.createdAt,
    required this.updatedAt,
    required this.urls,
    required this.spaceType,
    required this.features,
    required this.status,
    required this.permissions,
    required this.authorType,
    required this.authorDisplayName,
    required this.authorUsername,
    required this.authorProfileUrl,
    required this.certified,
    required this.likes,
    required this.reports,
    required this.isReport,
    required this.comments,
    required this.shares,
    required this.rewards,
    required this.visibility,
    required this.publishState,
    required this.booster,
    required this.files,
    required this.anonymousParticipation,
    required this.canParticipate,
    required this.changeVisibility,
    required this.participated,
    required this.participantDisplayName,
    required this.participantProfileUrl,
    required this.participantUsername,
    required this.blockParticipate,
    required this.requirements,
    required this.remains,
    required this.quota,
  });

  bool get isFinished => status == SpaceStatus.finished;
  bool get isAdmin => permissions.isAdmin;
  bool get havePreTasks => requirements.any((e) => !e.responded);

  factory SpaceModel.fromJson(Json j) {
    List<String> stringList(String key) {
      final raw = j[key];
      if (raw is List) {
        return raw.map((e) => e.toString()).toList();
      }
      return const [];
    }

    List<FileModel> fileList() {
      final raw = j['files'];
      if (raw is List) {
        return raw.whereType<Json>().map(FileModel.fromJson).toList();
      }
      return const [];
    }

    List<SpaceRequirementModel> requirementList() {
      final raw = j['requirements'];
      if (raw is List) {
        return raw
            .whereType<Json>()
            .map(SpaceRequirementModel.fromJson)
            .toList();
      }
      return const [];
    }

    final permRaw = _asIntOrNull(j['permissions']) ?? 0;

    return SpaceModel(
      pk: j['pk']?.toString() ?? '',
      sk: j['sk']?.toString() ?? '',
      title: (j['title'] ?? '') as String,
      content: (j['content'] ?? '') as String,
      createdAt: _asIntOrNull(j['created_at']) ?? 0,
      updatedAt: _asIntOrNull(j['updated_at']) ?? 0,
      urls: stringList('urls'),
      spaceType: spaceTypeFromJson(j['space_type']),
      features: stringList('features'),
      status: spaceStatusFromJson(j['status']),
      permissions: TeamGroupPermissions.fromInt(permRaw),
      authorType: userTypeFromJson(j['author_type']),
      authorDisplayName: (j['author_display_name'] ?? '') as String,
      authorUsername: (j['author_username'] ?? '') as String,
      authorProfileUrl: (j['author_profile_url'] ?? '') as String,
      certified: (j['certified'] as bool?) ?? false,
      isReport: (j['is_report'] as bool?) ?? false,
      likes: _asIntOrNull(j['likes']) ?? 0,
      reports: _asIntOrNull(j['reports']) ?? 0,
      comments: _asIntOrNull(j['comments']) ?? 0,
      shares: _asIntOrNull(j['shares']) ?? 0,
      rewards: _asIntOrNull(j['rewards']) ?? 0,
      visibility: spaceVisibilityFromJson(j['visibility']),
      publishState: spacePublishStateFromJson(j['publish_state']),
      booster: boosterTypeFromJson(j['booster']),
      files: fileList(),
      anonymousParticipation: (j['anonymous_participation'] as bool?) ?? false,
      canParticipate: (j['can_participate'] as bool?) ?? false,
      changeVisibility: (j['change_visibility'] as bool?) ?? false,
      participated: (j['participated'] as bool?) ?? false,
      participantDisplayName: j['participant_display_name'] as String?,
      participantProfileUrl: j['participant_profile_url'] as String?,
      participantUsername: j['participant_username'] as String?,
      blockParticipate: (j["block_participate"] as bool?) ?? false,
      requirements: requirementList(),
      remains: _asIntOrNull(j['remains']) ?? 0,
      quota: _asIntOrNull(j['quota']) ?? 0,
    );
  }
}

class SurveyResponse {
  final int id;
  final int createdAt;
  final int surveyId;

  const SurveyResponse({
    required this.id,
    required this.createdAt,
    required this.surveyId,
  });

  factory SurveyResponse.fromJson(Json j) => SurveyResponse(
    id: (j['id'] ?? 0) as int,
    createdAt: (j['created_at'] ?? 0) as int,
    surveyId: (j['survey_id'] ?? 0) as int,
  );
}

class CommentModel {
  final int id;
  final int createdAt;
  final String profileUrl;
  final String nickname;
  final String comment;

  const CommentModel({
    required this.id,
    required this.createdAt,
    required this.profileUrl,
    required this.nickname,
    required this.comment,
  });

  factory CommentModel.fromJson(Json j) => CommentModel(
    id: (j['id'] ?? 0) as int,
    createdAt: (j['created_at'] ?? 0) as int,
    profileUrl: j['profile_url'] ?? '',
    nickname: j['nickname'] ?? '',
    comment: j['html_contents'] ?? '',
  );
}

class FileModel {
  final String name;
  final String size;
  final String ext;
  final String url;

  const FileModel({
    required this.name,
    required this.size,
    required this.ext,
    required this.url,
  });

  factory FileModel.fromJson(Json j) => FileModel(
    name: (j['name'] ?? '') as String,
    size: (j['size'] ?? '') as String,
    ext: (j['ext'] ?? '') as String,
    url: (j['url'] ?? '') as String,
  );

  Json toJson() => {'name': name, 'size': size, 'ext': ext, 'url': url};
}

enum ProjectStatus { ready, inProgress, finish }

ProjectStatus projectStatusFrom(dynamic v) {
  if (v is int) {
    switch (v) {
      case 1:
        return ProjectStatus.ready;
      case 2:
        return ProjectStatus.inProgress;
      case 3:
        return ProjectStatus.finish;
    }
  }
  if (v is String) {
    switch (v) {
      case 'ready':
        return ProjectStatus.ready;
      case 'in_progress':
        return ProjectStatus.inProgress;
      case 'finish':
        return ProjectStatus.finish;
    }
  }
  return ProjectStatus.ready;
}

int? _asIntOrNull(dynamic v) {
  if (v == null) return null;
  if (v is int) return v;
  if (v is num) return v.toInt();
  return int.tryParse(v.toString());
}

class ParticipateSpaceResponse {
  final String username;
  final String displayName;
  final String profileUrl;

  const ParticipateSpaceResponse({
    required this.username,
    required this.displayName,
    required this.profileUrl,
  });

  factory ParticipateSpaceResponse.fromJson(Map<String, dynamic> json) {
    return ParticipateSpaceResponse(
      username: json['username'] as String? ?? '',
      displayName: json['display_name'] as String? ?? '',
      profileUrl: json['profile_url'] as String? ?? '',
    );
  }
}
