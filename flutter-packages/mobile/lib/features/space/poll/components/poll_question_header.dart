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

  bool get _isMultiSelect {
    if (question.type == AnswerType.multipleChoice) return true;
    if (question is CheckboxQuestionModel) {
      return (question as CheckboxQuestionModel).isMulti;
    }
    return false;
  }

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text.rich(
          TextSpan(
            children: [
              if (_required)
                const TextSpan(
                  text: '*',
                  style: TextStyle(
                    fontFamily: 'Raleway',
                    fontWeight: FontWeight.w700,
                    fontSize: 18,
                    height: 24 / 18,
                    color: Color(0xFFEF4444),
                  ),
                ),
              TextSpan(
                text: question.title,
                style: const TextStyle(
                  fontFamily: 'Raleway',
                  fontWeight: FontWeight.w700,
                  fontSize: 18,
                  height: 24 / 18,
                  color: Colors.white,
                ),
              ),
            ],
          ),
          softWrap: true,
        ),
        if (_isMultiSelect)
          const Text(
            'Select all that apply',
            style: TextStyle(
              fontFamily: 'Raleway',
              fontWeight: FontWeight.w600,
              fontSize: 13,
              height: 20 / 13,
              color: Color(0xFF3B82F6),
            ),
            softWrap: true,
          ),
      ],
    );
  }
}
