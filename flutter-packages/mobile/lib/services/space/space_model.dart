typedef Json = Map<String, dynamic>;

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
  });
}

class SpaceModel {
  final int id;
  final int feedId;
  final String title;
  final String htmlContents;
  final List<FileModel> files;
  final List<DiscussionModel> discussions;
  final List<ElearningModel> elearnings;
  final List<SurveyModel> surveys;
  final List<CommentModel> comments;
  final List<SurveyResponse> userResponses;

  const SpaceModel({
    required this.id,
    required this.feedId,
    required this.title,
    required this.htmlContents,
    required this.files,
    required this.discussions,
    required this.elearnings,
    required this.surveys,
    required this.comments,
    required this.userResponses,
  });

  factory SpaceModel.fromJson(Json j) {
    List<T> _list<T>(String k, T Function(Json) f) =>
        (j[k] as List? ?? const []).whereType<Json>().map(f).toList();

    return SpaceModel(
      id: (j['id'] ?? 0) as int,
      feedId: (j['feed_id'] ?? 0) as int,
      title: (j['title'] ?? '') as String,
      htmlContents: (j['html_contents'] ?? j['htmlContents'] ?? '') as String,
      files: _list('files', FileModel.fromJson),
      discussions: _list('discussions', DiscussionModel.fromJson),
      elearnings: _list('elearnings', ElearningModel.fromJson),
      surveys: _list('surveys', SurveyModel.fromJson),
      comments: _list('feed_comments', CommentModel.fromJson),
      userResponses: _list('user_responses', SurveyResponse.fromJson),
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
    profileUrl: j['profile_url'] ?? "",
    nickname: j['nickname'] ?? "",
    comment: j["html_contents"] ?? "",
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

String answerTypeToString(AnswerType t) => switch (t) {
  AnswerType.singleChoice => 'single_choice',
  AnswerType.multipleChoice => 'multiple_choice',
  AnswerType.shortAnswer => 'short_answer',
  AnswerType.subjective => 'subjective',
  AnswerType.checkbox => 'checkbox',
  AnswerType.dropdown => 'dropdown',
  AnswerType.linearScale => 'linear_scale',
};

AnswerType answerTypeFromString(String s) => switch (s) {
  'single_choice' => AnswerType.singleChoice,
  'multiple_choice' => AnswerType.multipleChoice,
  'short_answer' => AnswerType.shortAnswer,
  'subjective' => AnswerType.subjective,
  'checkbox' => AnswerType.checkbox,
  'dropdown' => AnswerType.dropdown,
  'linear_scale' => AnswerType.linearScale,
  _ => throw ArgumentError('Unknown answer_type: $s'),
};

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
