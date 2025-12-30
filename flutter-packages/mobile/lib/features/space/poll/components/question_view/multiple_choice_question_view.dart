import 'package:ratel/exports.dart';
import 'package:ratel/features/space/poll/components/question_view/choice_box.dart';

class MultipleChoiceQuestionView extends StatelessWidget {
  const MultipleChoiceQuestionView({
    super.key,
    required this.question,
    required this.answer,
    required this.onChanged,
    required this.readOnly,
  });

  final ChoiceQuestionModel question;
  final MultipleChoiceAnswer? answer;
  final ValueChanged<Answer> onChanged;
  final bool readOnly;

  bool get _allowOther => question.allowOther == true;

  @override
  Widget build(BuildContext context) {
    final selected = (answer?.answer ?? const <int>[]).toSet();
    final otherText = answer?.other;
    final othersIndex = _allowOther ? question.options.length - 1 : null;
    final isOtherSelected =
        _allowOther && othersIndex != null && selected.contains(othersIndex);

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        for (int i = 0; i < question.options.length; i++) ...[
          if (!_allowOther || i != othersIndex) ...[
            ChoiceBox(
              enabled: !readOnly,
              selected: selected.contains(i),
              onTap: readOnly
                  ? null
                  : () {
                      final set = selected.toSet();
                      if (set.contains(i)) {
                        set.remove(i);
                      } else {
                        set.add(i);
                      }
                      onChanged(
                        MultipleChoiceAnswer(set.toList()..sort(), otherText),
                      );
                    },
              child: Text(
                question.options[i],
                style: const TextStyle(
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
                    final set = selected.toSet();
                    if (isOtherSelected) {
                      set.remove(othersIndex);
                      onChanged(
                        MultipleChoiceAnswer(set.toList()..sort(), null),
                      );
                    } else {
                      set.add(othersIndex);
                      onChanged(
                        MultipleChoiceAnswer(
                          set.toList()..sort(),
                          otherText ?? '',
                        ),
                      );
                    }
                  },
            child: TextFormField(
              key: ValueKey('${question.title}_multiple_other'),
              initialValue: otherText ?? '',
              enabled: !readOnly,
              readOnly: readOnly,
              onTap: () {
                if (readOnly) return;
                if (!isOtherSelected) {
                  final set = selected.toSet();
                  set.add(othersIndex);
                  onChanged(
                    MultipleChoiceAnswer(set.toList()..sort(), otherText ?? ''),
                  );
                }
              },
              onChanged: (value) {
                if (readOnly) return;
                final set = selected.toSet();
                if (!set.contains(othersIndex)) {
                  set.add(othersIndex);
                }
                onChanged(MultipleChoiceAnswer(set.toList()..sort(), value));
              },
              style: const TextStyle(
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
