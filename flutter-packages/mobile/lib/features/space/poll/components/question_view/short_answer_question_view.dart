import 'package:ratel/exports.dart';

class ShortAnswerQuestionView extends StatelessWidget {
  const ShortAnswerQuestionView({
    super.key,
    required this.question,
    required this.answer,
    required this.onChanged,
    required this.multiline,
    required this.readOnly,
  });

  final SubjectiveQuestionModel question;
  final Answer? answer;
  final ValueChanged<Answer> onChanged;
  final bool multiline;
  final bool readOnly;

  @override
  Widget build(BuildContext context) {
    final initial = (answer is ShortAnswer)
        ? (answer as ShortAnswer).answer
        : (answer is SubjectiveAnswer)
        ? (answer as SubjectiveAnswer).answer
        : null;

    final bool isMultiline = multiline;

    return Container(
      height: isMultiline ? null : 40,
      decoration: BoxDecoration(
        borderRadius: BorderRadius.circular(8),
        border: Border.all(color: const Color(0xFF404040)),
      ),
      padding: const EdgeInsets.symmetric(horizontal: 12),
      alignment: isMultiline ? Alignment.topLeft : Alignment.centerLeft,
      child: TextFormField(
        key: ValueKey(question.title),
        initialValue: initial ?? '',
        enabled: !readOnly,
        readOnly: readOnly,
        maxLines: isMultiline ? null : 1,
        minLines: isMultiline ? 4 : 1,
        style: const TextStyle(
          fontFamily: 'Inter',
          fontSize: 14,
          color: Colors.white,
        ),
        decoration: const InputDecoration(
          isDense: true,
          border: InputBorder.none,
          hintText: 'Input the option.',
          hintStyle: TextStyle(
            fontFamily: 'Inter',
            fontSize: 14,
            color: Color(0xFF6B6B6B),
          ),
        ),
        onChanged: readOnly
            ? null
            : (v) {
                if (question.type == AnswerType.subjective) {
                  onChanged(SubjectiveAnswer(v));
                } else {
                  onChanged(ShortAnswer(v));
                }
              },
      ),
    );
  }
}
