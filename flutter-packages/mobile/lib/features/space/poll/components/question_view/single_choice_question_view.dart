import 'package:ratel/exports.dart';
import 'package:ratel/features/space/poll/components/question_view/choice_box.dart';

class SingleChoiceQuestionView extends StatelessWidget {
  const SingleChoiceQuestionView({
    super.key,
    required this.question,
    required this.answer,
    required this.onChanged,
    required this.readOnly,
  });

  final ChoiceQuestionModel question;
  final SingleChoiceAnswer? answer;
  final ValueChanged<Answer> onChanged;
  final bool readOnly;

  bool get _allowOther => question.allowOther == true;

  @override
  Widget build(BuildContext context) {
    final selectedIndex = answer?.answer;
    final otherText = answer?.other;
    final othersIndex = _allowOther ? question.options.length - 1 : null;
    final isOtherSelected = _allowOther && selectedIndex == othersIndex;

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        for (int i = 0; i < question.options.length; i++) ...[
          if (!_allowOther || i != othersIndex) ...[
            ChoiceBox(
              enabled: !readOnly,
              selected: selectedIndex == i,
              onTap: readOnly
                  ? null
                  : () => onChanged(SingleChoiceAnswer(i, null)),
              child: Text(
                question.options[i],
                style: const TextStyle(
                  fontFamily: 'Raleway',
                  fontWeight: FontWeight.w400,
                  fontSize: 16,
                  height: 24 / 16,
                  letterSpacing: 0.5,
                  color: Colors.white,
                ),
                softWrap: true,
              ),
            ),
            10.vgap,
          ],
        ],
        if (_allowOther && othersIndex != null) ...[
          ChoiceBox(
            enabled: !readOnly,
            selected: isOtherSelected,
            onTap: readOnly
                ? null
                : () {
                    if (isOtherSelected) {
                      onChanged(const SingleChoiceAnswer(null, null));
                    } else {
                      onChanged(
                        SingleChoiceAnswer(othersIndex, otherText ?? ''),
                      );
                    }
                  },
            child: TextFormField(
              key: ValueKey('${question.title}_single_other'),
              initialValue: otherText ?? '',
              enabled: !readOnly,
              readOnly: readOnly,
              onTap: () {
                if (readOnly) return;
                if (!isOtherSelected) {
                  onChanged(SingleChoiceAnswer(othersIndex, otherText ?? ''));
                }
              },
              onChanged: (value) {
                if (readOnly) return;
                onChanged(SingleChoiceAnswer(othersIndex, value));
              },
              style: const TextStyle(
                fontFamily: 'Raleway',
                fontWeight: FontWeight.w400,
                fontSize: 16,
                height: 24 / 16,
                letterSpacing: 0.5,
                color: Colors.white,
              ),
              decoration: const InputDecoration(
                isDense: true,
                border: InputBorder.none,
                hintText: 'Input the option.',
                hintStyle: TextStyle(
                  fontFamily: 'Raleway',
                  fontWeight: FontWeight.w400,
                  fontSize: 16,
                  height: 24 / 16,
                  letterSpacing: 0.5,
                  color: Color(0xFF737373),
                ),
              ),
            ),
          ),
        ],
      ],
    );
  }
}
