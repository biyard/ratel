import 'package:ratel/exports.dart';

class PollQuestionHeader extends StatelessWidget {
  const PollQuestionHeader({super.key, required this.question});

  final QuestionModel question;

  bool get _required {
    if (question is ChoiceQuestionModel) {
      return (question as ChoiceQuestionModel).isRequired;
    }
    if (question is SubjectiveQuestionModel) {
      return (question as SubjectiveQuestionModel).isRequired;
    }
    if (question is CheckboxQuestionModel) {
      return (question as CheckboxQuestionModel).isRequired;
    }
    if (question is DropdownQuestionModel) {
      return (question as DropdownQuestionModel).isRequired;
    }
    if (question is LinearScaleQuestionModel) {
      return (question as LinearScaleQuestionModel).isRequired;
    }
    return false;
  }

  @override
  Widget build(BuildContext context) {
    final spans = <TextSpan>[];

    if (_required) {
      spans.add(
        const TextSpan(
          text: '[Required] ',
          style: TextStyle(
            fontWeight: FontWeight.w600,
            fontSize: 14,
            color: Color(0xFFFF6467),
          ),
        ),
      );
    }

    if (question.type == AnswerType.singleChoice) {
      spans.add(
        const TextSpan(
          text: '[Single Choice] ',
          style: TextStyle(
            fontWeight: FontWeight.w600,
            fontSize: 14,
            color: Color(0xFF60A5FA),
          ),
        ),
      );
    } else if (question.type == AnswerType.multipleChoice) {
      spans.add(
        const TextSpan(
          text: '[Multiple Choice] ',
          style: TextStyle(
            fontWeight: FontWeight.w600,
            fontSize: 14,
            color: Color(0xFF60A5FA),
          ),
        ),
      );
    } else if (question is CheckboxQuestionModel) {
      final q = question as CheckboxQuestionModel;
      spans.add(
        TextSpan(
          text: q.isMulti ? '[Multiple Choice] ' : '[Single Choice] ',
          style: TextStyle(
            fontWeight: FontWeight.w600,
            fontSize: 14,
            color: q.isMulti
                ? const Color(0xFF60A5FA)
                : const Color(0xFFFF6467),
          ),
        ),
      );
    }

    spans.add(const TextSpan(text: ' ', style: TextStyle(fontSize: 14)));

    spans.add(
      TextSpan(
        text: question.title,
        style: const TextStyle(
          fontWeight: FontWeight.w600,
          fontSize: 16,
          color: Colors.white,
        ),
      ),
    );

    return Text.rich(TextSpan(children: spans), softWrap: true);
  }
}
