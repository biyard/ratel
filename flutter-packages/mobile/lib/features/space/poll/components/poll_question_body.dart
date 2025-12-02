import 'package:ratel/exports.dart';
import 'package:ratel/features/space/poll/components/question_view/checkbox_question_view.dart';
import 'package:ratel/features/space/poll/components/question_view/dropdown_question_view.dart';
import 'package:ratel/features/space/poll/components/question_view/linear_scale_question_view.dart';
import 'package:ratel/features/space/poll/components/question_view/multiple_choice_question_view.dart';
import 'package:ratel/features/space/poll/components/question_view/short_answer_question_view.dart';
import 'package:ratel/features/space/poll/components/question_view/single_choice_question_view.dart';

class PollQuestionBody extends StatelessWidget {
  const PollQuestionBody({
    super.key,
    required this.question,
    required this.answer,
    required this.onChanged,
  });

  final QuestionModel question;
  final Answer? answer;
  final ValueChanged<Answer> onChanged;

  @override
  Widget build(BuildContext context) {
    switch (question.type) {
      case AnswerType.singleChoice:
        return SingleChoiceQuestionView(
          question: question as ChoiceQuestionModel,
          answer: answer as SingleChoiceAnswer?,
          onChanged: onChanged,
        );
      case AnswerType.multipleChoice:
        return MultipleChoiceQuestionView(
          question: question as ChoiceQuestionModel,
          answer: answer as MultipleChoiceAnswer?,
          onChanged: onChanged,
        );
      case AnswerType.shortAnswer:
        return ShortAnswerQuestionView(
          question: question as SubjectiveQuestionModel,
          answer: answer as ShortAnswer?,
          onChanged: onChanged,
          multiline: false,
        );
      case AnswerType.subjective:
        return ShortAnswerQuestionView(
          question: question as SubjectiveQuestionModel,
          answer: answer as SubjectiveAnswer?,
          onChanged: onChanged,
          multiline: true,
        );
      case AnswerType.checkbox:
        return CheckboxQuestionView(
          question: question as CheckboxQuestionModel,
          answer: answer as CheckboxAnswer?,
          onChanged: onChanged,
        );
      case AnswerType.dropdown:
        return DropdownQuestionView(
          question: question as DropdownQuestionModel,
          answer: answer as DropdownAnswer?,
          onChanged: onChanged,
        );
      case AnswerType.linearScale:
        return LinearScaleQuestionView(
          question: question as LinearScaleQuestionModel,
          answer: answer as LinearScaleAnswer?,
          onChanged: onChanged,
        );
    }
  }
}
