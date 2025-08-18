typedef Json = Map<String, dynamic>;

class SpaceModel {
  final int id;
  final String title;
  final String htmlContents;
  final List<FileModel> files;
  final List<DiscussionModel> discussions;
  final List<ElearningModel> elearnings;
  final List<SurveyModel> surveys;

  const SpaceModel({
    required this.id,
    required this.title,
    required this.htmlContents,
    required this.files,
    required this.discussions,
    required this.elearnings,
    required this.surveys,
  });

  factory SpaceModel.fromJson(Json j) {
    List<T> _list<T>(String k, T Function(Json) f) =>
        (j[k] as List? ?? const []).whereType<Json>().map(f).toList();

    return SpaceModel(
      id: (j['id'] ?? 0) as int,
      title: (j['title'] ?? '') as String,
      htmlContents: (j['html_contents'] ?? j['htmlContents'] ?? '') as String,
      files: _list('files', FileModel.fromJson),
      discussions: _list('discussions', DiscussionModel.fromJson),
      elearnings: _list('elearnings', ElearningModel.fromJson),
      surveys: _list('surveys', SurveyModel.fromJson),
    );
  }
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
