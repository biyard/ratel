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

class SurveyModel {
  final int id;
  final ProjectStatus status;
  final int startedAt;
  final int endedAt;
  final List<QuestionModel> questions;
  final int responseCount;

  const SurveyModel({
    required this.id,
    required this.status,
    required this.startedAt,
    required this.endedAt,
    required this.questions,
    required this.responseCount,
  });

  factory SurveyModel.fromJson(Json j) => SurveyModel(
    id: (j['id'] ?? 0) as int,
    status: projectStatusFrom(j['status']),
    startedAt: (j['started_at'] ?? 0) as int,
    endedAt: (j['ended_at'] ?? 0) as int,
    questions: (j['questions'] as List? ?? const [])
        .whereType<Json>()
        .map(QuestionModel.fromJson)
        .toList(),
    responseCount: (j['response_count'] ?? 0) as int,
  );
}

enum AnswerType {
  singleChoice,
  multipleChoice,
  shortAnswer,
  subjective,
  checkbox,
  dropdown,
  linearScale,
}

AnswerType answerTypeFrom(String s) {
  switch (s) {
    case 'single_choice':
      return AnswerType.singleChoice;
    case 'multiple_choice':
      return AnswerType.multipleChoice;
    case 'short_answer':
      return AnswerType.shortAnswer;
    case 'subjective':
      return AnswerType.subjective;
    case 'checkbox':
      return AnswerType.checkbox;
    case 'dropdown':
      return AnswerType.dropdown;
    case 'linear_scale':
      return AnswerType.linearScale;
    default:
      return AnswerType.shortAnswer;
  }
}

abstract class QuestionModel {
  final AnswerType type;
  final String title;

  const QuestionModel({required this.type, required this.title});

  factory QuestionModel.fromJson(Json j) {
    final t = answerTypeFrom(j['answer_type'] as String);
    switch (t) {
      case AnswerType.singleChoice:
      case AnswerType.multipleChoice:
        return ChoiceQuestionModel.fromJson(j, t);
      case AnswerType.shortAnswer:
      case AnswerType.subjective:
        return SubjectiveQuestionModel.fromJson(j, t);
      case AnswerType.checkbox:
        return CheckboxQuestionModel.fromJson(j);
      case AnswerType.dropdown:
        return DropdownQuestionModel.fromJson(j);
      case AnswerType.linearScale:
        return LinearScaleQuestionModel.fromJson(j);
    }
  }
}

class ChoiceQuestionModel extends QuestionModel {
  final String? description;
  final String? imageUrl;
  final List<String> options;
  final bool isRequired;

  ChoiceQuestionModel({
    required super.type,
    required super.title,
    this.description,
    this.imageUrl,
    required this.options,
    required this.isRequired,
  });

  factory ChoiceQuestionModel.fromJson(Json j, AnswerType t) =>
      ChoiceQuestionModel(
        type: t,
        title: j['title'] as String,
        description: j['description'] as String?,
        imageUrl: j['image_url'] as String?,
        options: (j['options'] as List? ?? const []).cast<String>(),
        isRequired: (j['is_required'] as bool?) ?? false,
      );
}

class SubjectiveQuestionModel extends QuestionModel {
  final String description;
  final bool isRequired;

  SubjectiveQuestionModel({
    required super.type,
    required super.title,
    required this.description,
    required this.isRequired,
  });

  factory SubjectiveQuestionModel.fromJson(Json j, AnswerType t) =>
      SubjectiveQuestionModel(
        type: t,
        title: j['title'] as String,
        description: (j['description'] as String?) ?? '',
        isRequired: (j['is_required'] as bool?) ?? false,
      );
}

class CheckboxQuestionModel extends QuestionModel {
  final String? description;
  final String? imageUrl;
  final List<String> options;
  final bool isMulti;
  final bool isRequired;

  CheckboxQuestionModel({
    required String title,
    this.description,
    this.imageUrl,
    required this.options,
    required this.isMulti,
    required this.isRequired,
  }) : super(type: AnswerType.checkbox, title: title);

  factory CheckboxQuestionModel.fromJson(Json j) => CheckboxQuestionModel(
    title: j['title'] as String,
    description: j['description'] as String?,
    imageUrl: j['image_url'] as String?,
    options: (j['options'] as List? ?? const []).cast<String>(),
    isMulti: (j['is_multi'] as bool?) ?? false,
    isRequired: (j['is_required'] as bool?) ?? false,
  );
}

class DropdownQuestionModel extends QuestionModel {
  final String? description;
  final String? imageUrl;
  final List<String> options;
  final bool isRequired;

  DropdownQuestionModel({
    required String title,
    this.description,
    this.imageUrl,
    required this.options,
    required this.isRequired,
  }) : super(type: AnswerType.dropdown, title: title);

  factory DropdownQuestionModel.fromJson(Json j) => DropdownQuestionModel(
    title: j['title'] as String,
    description: j['description'] as String?,
    imageUrl: j['image_url'] as String?,
    options: (j['options'] as List? ?? const []).cast<String>(),
    isRequired: (j['is_required'] as bool?) ?? false,
  );
}

class LinearScaleQuestionModel extends QuestionModel {
  final String? description;
  final String? imageUrl;
  final int minValue;
  final int maxValue;
  final String minLabel;
  final String maxLabel;
  final bool isRequired;

  LinearScaleQuestionModel({
    required String title,
    this.description,
    this.imageUrl,
    required this.minValue,
    required this.maxValue,
    required this.minLabel,
    required this.maxLabel,
    required this.isRequired,
  }) : super(type: AnswerType.linearScale, title: title);

  factory LinearScaleQuestionModel.fromJson(Json j) => LinearScaleQuestionModel(
    title: j['title'] as String,
    description: j['description'] as String?,
    imageUrl: j['image_url'] as String?,
    minValue: (j['min_value'] ?? 1) as int,
    maxValue: (j['max_value'] ?? 5) as int,
    minLabel: (j['min_label'] ?? '') as String,
    maxLabel: (j['max_label'] ?? '') as String,
    isRequired: (j['is_required'] as bool?) ?? false,
  );
}

String answerTypeToString(AnswerType t) {
  switch (t) {
    case AnswerType.singleChoice:
      return 'single_choice';
    case AnswerType.multipleChoice:
      return 'multiple_choice';
    case AnswerType.shortAnswer:
      return 'short_answer';
    case AnswerType.subjective:
      return 'subjective';
    case AnswerType.checkbox:
      return 'checkbox';
    case AnswerType.dropdown:
      return 'dropdown';
    case AnswerType.linearScale:
      return 'linear_scale';
  }
}

AnswerType answerTypeFromString(String s) {
  switch (s) {
    case 'single_choice':
      return AnswerType.singleChoice;
    case 'multiple_choice':
      return AnswerType.multipleChoice;
    case 'short_answer':
      return AnswerType.shortAnswer;
    case 'subjective':
      return AnswerType.subjective;
    case 'checkbox':
      return AnswerType.checkbox;
    case 'dropdown':
      return AnswerType.dropdown;
    case 'linear_scale':
      return AnswerType.linearScale;
    default:
      throw ArgumentError('Unknown answer_type: $s');
  }
}

abstract class Answer {
  final AnswerType type;
  const Answer(this.type);

  Json toJson();

  factory Answer.fromJson(Json j) {
    final t = answerTypeFromString(j['answer_type'] as String);
    final a = j['answer'];
    switch (t) {
      case AnswerType.singleChoice:
        return SingleChoiceAnswer(_asIntOrNull(a));
      case AnswerType.multipleChoice:
        return MultipleChoiceAnswer(_asIntListOrNull(a));
      case AnswerType.shortAnswer:
        return ShortAnswer(a as String?);
      case AnswerType.subjective:
        return SubjectiveAnswer(a as String?);
      case AnswerType.checkbox:
        return CheckboxAnswer(_asIntListOrNull(a));
      case AnswerType.dropdown:
        return DropdownAnswer(_asIntOrNull(a));
      case AnswerType.linearScale:
        return LinearScaleAnswer(_asIntOrNull(a));
    }
  }
}

class SingleChoiceAnswer extends Answer {
  final int? answer;
  const SingleChoiceAnswer(this.answer) : super(AnswerType.singleChoice);

  @override
  Json toJson() => {'answer_type': answerTypeToString(type), 'answer': answer};
}

class MultipleChoiceAnswer extends Answer {
  final List<int>? answer;
  const MultipleChoiceAnswer(this.answer) : super(AnswerType.multipleChoice);

  @override
  Json toJson() => {'answer_type': answerTypeToString(type), 'answer': answer};
}

class ShortAnswer extends Answer {
  final String? answer;
  const ShortAnswer(this.answer) : super(AnswerType.shortAnswer);

  @override
  Json toJson() => {'answer_type': answerTypeToString(type), 'answer': answer};
}

class SubjectiveAnswer extends Answer {
  final String? answer;
  const SubjectiveAnswer(this.answer) : super(AnswerType.subjective);

  @override
  Json toJson() => {'answer_type': answerTypeToString(type), 'answer': answer};
}

class CheckboxAnswer extends Answer {
  final List<int>? answer;
  const CheckboxAnswer(this.answer) : super(AnswerType.checkbox);

  @override
  Json toJson() => {'answer_type': answerTypeToString(type), 'answer': answer};
}

class DropdownAnswer extends Answer {
  final int? answer;
  const DropdownAnswer(this.answer) : super(AnswerType.dropdown);

  @override
  Json toJson() => {'answer_type': answerTypeToString(type), 'answer': answer};
}

class LinearScaleAnswer extends Answer {
  final int? answer;
  const LinearScaleAnswer(this.answer) : super(AnswerType.linearScale);

  @override
  Json toJson() => {'answer_type': answerTypeToString(type), 'answer': answer};
}

int? _asIntOrNull(dynamic v) {
  if (v == null) return null;
  if (v is int) return v;
  if (v is num) return v.toInt();
  return int.tryParse(v.toString());
}

List<int>? _asIntListOrNull(dynamic v) {
  if (v == null) return null;
  if (v is List) {
    return v
        .map((e) => _asIntOrNull(e))
        .where((e) => e != null)
        .cast<int>()
        .toList();
  }
  return null;
}
