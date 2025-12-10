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

class RespondPollResult {
  final String pollSpacePk;

  const RespondPollResult({required this.pollSpacePk});

  factory RespondPollResult.fromJson(Map<String, dynamic> j) =>
      RespondPollResult(pollSpacePk: j['poll_space_pk']?.toString() ?? '');

  Map<String, dynamic> toJson() => {'poll_space_pk': pollSpacePk};
}

class PollResult {
  final int createdAt;
  final List<PollSummary> summaries;
  final Map<String, List<PollSummary>> summariesByGender;
  final Map<String, List<PollSummary>> summariesByAge;
  final Map<String, List<PollSummary>> summariesBySchool;
  final List<PollUserAnswer> sampleAnswers;
  final List<PollUserAnswer> finalAnswers;

  const PollResult({
    required this.createdAt,
    required this.summaries,
    required this.summariesByGender,
    required this.summariesByAge,
    required this.summariesBySchool,
    required this.sampleAnswers,
    required this.finalAnswers,
  });

  factory PollResult.fromJson(Json j) {
    return PollResult(
      createdAt: (j['created_at'] as num?)?.toInt() ?? 0,
      summaries: (j['summaries'] as List? ?? const [])
          .whereType<Map>()
          .map((e) => PollSummary.fromJson(e.cast<String, dynamic>()))
          .toList(),
      summariesByGender: _parseGroupedSummaries(j['summaries_by_gender']),
      summariesByAge: _parseGroupedSummaries(j['summaries_by_age']),
      summariesBySchool: _parseGroupedSummaries(j['summaries_by_school']),
      sampleAnswers: (j['sample_answers'] as List? ?? const [])
          .whereType<Map>()
          .map((e) => PollUserAnswer.fromJson(e.cast<String, dynamic>()))
          .toList(),
      finalAnswers: (j['final_answers'] as List? ?? const [])
          .whereType<Map>()
          .map((e) => PollUserAnswer.fromJson(e.cast<String, dynamic>()))
          .toList(),
    );
  }

  Json toJson() => {
    'created_at': createdAt,
    'summaries': summaries.map((s) => s.toJson()).toList(),
    'summaries_by_gender': summariesByGender.map(
      (k, v) => MapEntry(k, v.map((e) => e.toJson()).toList()),
    ),
    'summaries_by_age': summariesByAge.map(
      (k, v) => MapEntry(k, v.map((e) => e.toJson()).toList()),
    ),
    'summaries_by_school': summariesBySchool.map(
      (k, v) => MapEntry(k, v.map((e) => e.toJson()).toList()),
    ),
    'sample_answers': sampleAnswers.map((a) => a.toJson()).toList(),
    'final_answers': finalAnswers.map((a) => a.toJson()).toList(),
  };
}

Map<String, List<PollSummary>> _parseGroupedSummaries(dynamic raw) {
  final result = <String, List<PollSummary>>{};
  if (raw is Map) {
    raw.forEach((key, value) {
      final list = (value as List? ?? const [])
          .whereType<Map>()
          .map((e) => PollSummary.fromJson(e.cast<String, dynamic>()))
          .toList();
      result[key.toString()] = list;
    });
  }
  return result;
}

abstract class PollSummary {
  final AnswerType type;
  final int totalCount;

  const PollSummary(this.type, this.totalCount);

  factory PollSummary.fromJson(Json j) {
    final t = answerTypeFromString(j['answer_type'] as String);
    final total = (j['total_count'] as num?)?.toInt() ?? 0;
    switch (t) {
      case AnswerType.singleChoice:
        return SingleChoiceSummary(
          total,
          _parseIntCountMap(j['answers']),
          _parseStringCountMap(j['other_answers']),
        );
      case AnswerType.multipleChoice:
        return MultipleChoiceSummary(
          total,
          _parseIntCountMap(j['answers']),
          _parseStringCountMap(j['other_answers']),
        );
      case AnswerType.shortAnswer:
        return ShortAnswerSummary(total, _parseStringCountMap(j['answers']));
      case AnswerType.subjective:
        return SubjectiveSummary(total, _parseStringCountMap(j['answers']));
      case AnswerType.checkbox:
        return CheckboxSummary(total, _parseIntCountMap(j['answers']));
      case AnswerType.dropdown:
        return DropdownSummary(total, _parseIntCountMap(j['answers']));
      case AnswerType.linearScale:
        return LinearScaleSummary(total, _parseIntCountMap(j['answers']));
    }
  }

  Json toJson();
}

class SingleChoiceSummary extends PollSummary {
  final Map<int, int> answers;
  final Map<String, int> otherAnswers;

  const SingleChoiceSummary(int totalCount, this.answers, this.otherAnswers)
    : super(AnswerType.singleChoice, totalCount);

  @override
  Json toJson() => {
    'answer_type': answerTypeToString(type),
    'total_count': totalCount,
    'answers': answers.map((k, v) => MapEntry(k.toString(), v)),
    'other_answers': otherAnswers,
  };
}

class MultipleChoiceSummary extends PollSummary {
  final Map<int, int> answers;
  final Map<String, int> otherAnswers;

  const MultipleChoiceSummary(int totalCount, this.answers, this.otherAnswers)
    : super(AnswerType.multipleChoice, totalCount);

  @override
  Json toJson() => {
    'answer_type': answerTypeToString(type),
    'total_count': totalCount,
    'answers': answers.map((k, v) => MapEntry(k.toString(), v)),
    'other_answers': otherAnswers,
  };
}

class ShortAnswerSummary extends PollSummary {
  final Map<String, int> answers;

  const ShortAnswerSummary(int totalCount, this.answers)
    : super(AnswerType.shortAnswer, totalCount);

  @override
  Json toJson() => {
    'answer_type': answerTypeToString(type),
    'total_count': totalCount,
    'answers': answers,
  };
}

class SubjectiveSummary extends PollSummary {
  final Map<String, int> answers;

  const SubjectiveSummary(int totalCount, this.answers)
    : super(AnswerType.subjective, totalCount);

  @override
  Json toJson() => {
    'answer_type': answerTypeToString(type),
    'total_count': totalCount,
    'answers': answers,
  };
}

class CheckboxSummary extends PollSummary {
  final Map<int, int> answers;

  const CheckboxSummary(int totalCount, this.answers)
    : super(AnswerType.checkbox, totalCount);

  @override
  Json toJson() => {
    'answer_type': answerTypeToString(type),
    'total_count': totalCount,
    'answers': answers.map((k, v) => MapEntry(k.toString(), v)),
  };
}

class DropdownSummary extends PollSummary {
  final Map<int, int> answers;

  const DropdownSummary(int totalCount, this.answers)
    : super(AnswerType.dropdown, totalCount);

  @override
  Json toJson() => {
    'answer_type': answerTypeToString(type),
    'total_count': totalCount,
    'answers': answers.map((k, v) => MapEntry(k.toString(), v)),
  };
}

class LinearScaleSummary extends PollSummary {
  final Map<int, int> answers;

  const LinearScaleSummary(int totalCount, this.answers)
    : super(AnswerType.linearScale, totalCount);

  @override
  Json toJson() => {
    'answer_type': answerTypeToString(type),
    'total_count': totalCount,
    'answers': answers.map((k, v) => MapEntry(k.toString(), v)),
  };
}

Map<int, int> _parseIntCountMap(dynamic raw) {
  final result = <int, int>{};
  if (raw is Map) {
    raw.forEach((k, v) {
      final key = int.tryParse(k.toString());
      final val = (v as num?)?.toInt();
      if (key != null && val != null) {
        result[key] = val;
      }
    });
  }
  return result;
}

Map<String, int> _parseStringCountMap(dynamic raw) {
  final result = <String, int>{};
  if (raw is Map) {
    raw.forEach((k, v) {
      final key = k.toString();
      final val = (v as num?)?.toInt();
      if (val != null) {
        result[key] = val;
      }
    });
  }
  return result;
}

class PollUserAnswer {
  final int createdAt;
  final List<Answer> answers;
  final RespondentAttr? respondent;
  final String? userPk;
  final String? displayName;
  final String? profileUrl;
  final String? username;

  const PollUserAnswer({
    required this.createdAt,
    required this.answers,
    required this.respondent,
    required this.userPk,
    required this.displayName,
    required this.profileUrl,
    required this.username,
  });

  factory PollUserAnswer.fromJson(Json j) {
    final answersJson = (j['answers'] as List? ?? const []);
    return PollUserAnswer(
      createdAt: (j['created_at'] as num?)?.toInt() ?? 0,
      answers: answersJson
          .whereType<Map>()
          .map((e) => Answer.fromJson(e.cast<String, dynamic>()))
          .toList(),
      respondent: j['respondent'] is Map
          ? RespondentAttr.fromJson(
              (j['respondent'] as Map).cast<String, dynamic>(),
            )
          : null,
      userPk: j['user_pk']?.toString(),
      displayName: j['display_name']?.toString(),
      profileUrl: j['profile_url']?.toString(),
      username: j['username']?.toString(),
    );
  }

  Json toJson() => {
    'created_at': createdAt,
    'answers': answers.map((a) => a.toJson()).toList(),
    'respondent': respondent?.toJson(),
    'user_pk': userPk,
    'display_name': displayName,
    'profile_url': profileUrl,
    'username': username,
  };
}

enum GenderType { male, female, unknown }

GenderType genderTypeFromString(String? s) {
  switch (s) {
    case 'male':
      return GenderType.male;
    case 'female':
      return GenderType.female;
    default:
      return GenderType.unknown;
  }
}

String genderTypeToString(GenderType g) {
  switch (g) {
    case GenderType.male:
      return 'male';
    case GenderType.female:
      return 'female';
    case GenderType.unknown:
      return 'unknown';
  }
}

enum AgeKind { specific, range }

class AgeModel {
  final AgeKind kind;
  final int? value;
  final int? inclusiveMin;
  final int? inclusiveMax;

  const AgeModel._({
    required this.kind,
    this.value,
    this.inclusiveMin,
    this.inclusiveMax,
  });

  factory AgeModel.specific(int? value) =>
      AgeModel._(kind: AgeKind.specific, value: value);

  factory AgeModel.range({int? inclusiveMin, int? inclusiveMax}) => AgeModel._(
    kind: AgeKind.range,
    inclusiveMin: inclusiveMin,
    inclusiveMax: inclusiveMax,
  );

  factory AgeModel.fromJson(Json j) {
    final t = j['age_type']?.toString();
    if (t == 'specific') {
      final v = j['value'];
      return AgeModel.specific(v is num ? v.toInt() : _asIntOrNull(v));
    } else if (t == 'range') {
      final v = j['value'];
      if (v is Map) {
        final min = v['inclusive_min'];
        final max = v['inclusive_max'];
        return AgeModel.range(
          inclusiveMin: min is num ? min.toInt() : _asIntOrNull(min),
          inclusiveMax: max is num ? max.toInt() : _asIntOrNull(max),
        );
      }
      return AgeModel.range();
    }
    return AgeModel.specific(null);
  }

  Json toJson() {
    switch (kind) {
      case AgeKind.specific:
        return {'age_type': 'specific', 'value': value};
      case AgeKind.range:
        return {
          'age_type': 'range',
          'value': {
            'inclusive_min': inclusiveMin,
            'inclusive_max': inclusiveMax,
          },
        };
    }
  }
}

class RespondentAttr {
  final GenderType? gender;
  final AgeModel? age;
  final String? school;

  const RespondentAttr({
    required this.gender,
    required this.age,
    required this.school,
  });

  factory RespondentAttr.fromJson(Json j) {
    return RespondentAttr(
      gender: j['gender'] == null
          ? null
          : genderTypeFromString(j['gender']?.toString()),
      age: j['age'] is Map
          ? AgeModel.fromJson((j['age'] as Map).cast<String, dynamic>())
          : null,
      school: j['school']?.toString(),
    );
  }

  Json toJson() => {
    'gender': gender == null ? null : genderTypeToString(gender!),
    'age': age?.toJson(),
    'school': school,
  };
}
