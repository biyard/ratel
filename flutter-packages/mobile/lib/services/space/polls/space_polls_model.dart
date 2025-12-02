import 'package:ratel/exports.dart';

enum AnswerType {
  singleChoice,
  multipleChoice,
  shortAnswer,
  subjective,
  checkbox,
  dropdown,
  linearScale,
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

enum PollStatus { notStarted, inProgress, finish, unknown }

PollStatus pollStatusFromString(String s) {
  final v = s.toLowerCase();
  if (v == 'not_started' || v == 'notstarted' || v == '1') {
    return PollStatus.notStarted;
  }
  if (v == 'in_progress' || v == 'inprogress' || v == '2') {
    return PollStatus.inProgress;
  }
  if (v == 'finish' || v == 'finished' || v == '3') {
    return PollStatus.finish;
  }
  return PollStatus.unknown;
}

String pollStatusToString(PollStatus status) {
  switch (status) {
    case PollStatus.notStarted:
      return 'not_started';
    case PollStatus.inProgress:
      return 'in_progress';
    case PollStatus.finish:
      return 'finish';
    case PollStatus.unknown:
      return 'unknown';
  }
}

class PollListResult {
  final List<PollModel> polls;
  final String? bookmark;

  const PollListResult({required this.polls, required this.bookmark});

  factory PollListResult.fromJson(Json j) {
    final pollsJson = (j['polls'] as List? ?? const []);
    final polls = pollsJson
        .whereType<Map>()
        .map((e) => PollModel.fromJson(e.cast<String, dynamic>()))
        .toList();
    return PollListResult(polls: polls, bookmark: j['bookmark'] as String?);
  }

  Json toJson() => {
    'polls': polls.map((p) => p.toJson()).toList(),
    'bookmark': bookmark,
  };
}

class PollModel {
  final String sk;
  final int createdAt;
  final int updatedAt;
  final int startedAt;
  final int endedAt;
  final bool responseEditable;
  final int userResponseCount;
  final List<QuestionModel> questions;
  final List<Answer>? myResponse;
  final PollStatus status;
  final bool isDefault;

  const PollModel({
    required this.sk,
    required this.createdAt,
    required this.updatedAt,
    required this.startedAt,
    required this.endedAt,
    required this.responseEditable,
    required this.userResponseCount,
    required this.questions,
    required this.myResponse,
    required this.status,
    required this.isDefault,
  });

  factory PollModel.fromJson(Json j) {
    final questionsJson = (j['questions'] as List? ?? const []);
    final myRespJson = j['my_response'] as List?;

    return PollModel(
      sk: j['sk']?.toString() ?? '',
      createdAt: (j['created_at'] as num?)?.toInt() ?? 0,
      updatedAt: (j['updated_at'] as num?)?.toInt() ?? 0,
      startedAt: (j['started_at'] as num?)?.toInt() ?? 0,
      endedAt: (j['ended_at'] as num?)?.toInt() ?? 0,
      responseEditable: j['response_editable'] as bool? ?? false,
      userResponseCount: (j['user_response_count'] as num?)?.toInt() ?? 0,
      questions: questionsJson
          .whereType<Map>()
          .map((e) => QuestionModel.fromJson(e.cast<String, dynamic>()))
          .toList(),
      myResponse: myRespJson == null
          ? null
          : myRespJson
                .whereType<Map>()
                .map((e) => Answer.fromJson(e.cast<String, dynamic>()))
                .toList(),
      status: pollStatusFromString(j['status']?.toString() ?? ''),
      isDefault: j['default'] as bool? ?? false,
    );
  }

  Json toJson() => {
    'sk': sk,
    'created_at': createdAt,
    'updated_at': updatedAt,
    'started_at': startedAt,
    'ended_at': endedAt,
    'response_editable': responseEditable,
    'user_response_count': userResponseCount,
    'questions': questions.map((q) => q.toJson()).toList(),
    'my_response': myResponse?.map((a) => a.toJson()).toList(),
    'status': pollStatusToString(status),
    'default': isDefault,
  };
}

abstract class QuestionModel {
  final AnswerType type;
  final String title;

  const QuestionModel({required this.type, required this.title});

  factory QuestionModel.fromJson(Json j) {
    final t = answerTypeFromString(j['answer_type'] as String);
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

  Json toJson();
}

class ChoiceQuestionModel extends QuestionModel {
  final String? description;
  final String? imageUrl;
  final List<String> options;
  final bool isRequired;
  final bool allowOther;

  ChoiceQuestionModel({
    required super.type,
    required super.title,
    this.description,
    this.imageUrl,
    required this.options,
    required this.isRequired,
    required this.allowOther,
  });

  factory ChoiceQuestionModel.fromJson(Json j, AnswerType t) =>
      ChoiceQuestionModel(
        type: t,
        title: j['title'] as String,
        description: j['description'] as String?,
        imageUrl: j['image_url'] as String?,
        options: (j['options'] as List? ?? const []).cast<String>(),
        isRequired: (j['is_required'] as bool?) ?? false,
        allowOther: (j['allow_other'] as bool?) ?? false,
      );

  @override
  Json toJson() => {
    'answer_type': answerTypeToString(type),
    'title': title,
    'description': description,
    'image_url': imageUrl,
    'options': options,
    'is_required': isRequired,
    'allow_other': allowOther,
  };
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

  @override
  Json toJson() => {
    'answer_type': answerTypeToString(type),
    'title': title,
    'description': description,
    'is_required': isRequired,
  };
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

  @override
  Json toJson() => {
    'answer_type': answerTypeToString(type),
    'title': title,
    'description': description,
    'image_url': imageUrl,
    'options': options,
    'is_multi': isMulti,
    'is_required': isRequired,
  };
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

  @override
  Json toJson() => {
    'answer_type': answerTypeToString(type),
    'title': title,
    'description': description,
    'image_url': imageUrl,
    'options': options,
    'is_required': isRequired,
  };
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

  @override
  Json toJson() => {
    'answer_type': answerTypeToString(type),
    'title': title,
    'description': description,
    'image_url': imageUrl,
    'min_value': minValue,
    'max_value': maxValue,
    'min_label': minLabel,
    'max_label': maxLabel,
    'is_required': isRequired,
  };
}

abstract class Answer {
  final AnswerType type;
  const Answer(this.type);

  Json toJson();

  factory Answer.fromJson(Json j) {
    final t = answerTypeFromString(j['answer_type'] as String);
    final a = j['answer'];
    final other = j['other'];
    switch (t) {
      case AnswerType.singleChoice:
        return SingleChoiceAnswer(_asIntOrNull(a), other as String?);
      case AnswerType.multipleChoice:
        return MultipleChoiceAnswer(_asIntListOrNull(a), other as String?);
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
  final String? other;
  const SingleChoiceAnswer(this.answer, this.other)
    : super(AnswerType.singleChoice);

  @override
  Json toJson() => {
    'answer_type': answerTypeToString(type),
    'answer': answer,
    'other': other,
  };
}

class MultipleChoiceAnswer extends Answer {
  final List<int>? answer;
  final String? other;
  const MultipleChoiceAnswer(this.answer, this.other)
    : super(AnswerType.multipleChoice);

  @override
  Json toJson() => {
    'answer_type': answerTypeToString(type),
    'answer': answer,
    'other': other,
  };
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
