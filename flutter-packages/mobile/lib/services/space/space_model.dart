import 'package:ratel/exports.dart';

class MySpaceModel {
  final List<SpaceSummary> spaces;
  final List<SpaceSummary> boostings;

  const MySpaceModel({required this.spaces, required this.boostings});
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
  final int likes;
  final int comments;
  final int shares;
  final int rewards;

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

  const SpaceModel({
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

  bool get isAdmin => permissions.isAdmin;

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
      likes: _asIntOrNull(j['likes']) ?? 0,
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

class ElearningModel {
  final int id;
  final List<FileModel> files;

  const ElearningModel({required this.id, required this.files});

  factory ElearningModel.fromJson(Json j) => ElearningModel(
    id: (j['id'] ?? 0) as int,
    files: (j['files'] as List? ?? const [])
        .whereType<Json>()
        .map(FileModel.fromJson)
        .toList(),
  );
}

class DiscussionModel {
  final int id;
  final int startedAt;
  final int endedAt;
  final String name;
  final String? record;

  const DiscussionModel({
    required this.id,
    required this.startedAt,
    required this.endedAt,
    required this.name,
    required this.record,
  });

  factory DiscussionModel.fromJson(Json j) => DiscussionModel(
    id: (j['id'] ?? 0) as int,
    startedAt: (j['started_at'] ?? j['startedAt'] ?? 0) as int,
    endedAt: (j['ended_at'] ?? j['endedAt'] ?? 0) as int,
    name: (j['name'] ?? '') as String,
    record: j['record'] as String?,
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
