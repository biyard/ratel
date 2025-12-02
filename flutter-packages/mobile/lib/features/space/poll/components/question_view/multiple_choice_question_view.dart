import 'package:ratel/exports.dart';
import 'package:ratel/features/space/poll/components/option_tile.dart';

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
            OptionTile(
              enabled: !readOnly,
              label: question.options[i],
              selected: selected.contains(i),
              onTap: readOnly
                  ? () {}
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
            ),
            10.vgap,
          ],
        ],
        if (_allowOther && othersIndex != null) ...[
          Row(
            children: [
              GestureDetector(
                behavior: HitTestBehavior.opaque,
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
                child: Container(
                  width: 20,
                  height: 20,
                  decoration: BoxDecoration(
                    borderRadius: BorderRadius.circular(4),
                    border: Border.all(
                      color: isOtherSelected
                          ? AppColors.primary
                          : AppColors.neutral80,
                      width: 1.5,
                    ),
                    color: isOtherSelected
                        ? AppColors.primary
                        : Colors.transparent,
                  ),
                  alignment: Alignment.center,
                  child: isOtherSelected
                      ? const Icon(
                          Icons.check,
                          size: 16,
                          color: Color(0xFF1D1D1D),
                        )
                      : null,
                ),
              ),
              10.gap,
              Expanded(
                child: Container(
                  height: 40,
                  decoration: BoxDecoration(
                    borderRadius: BorderRadius.circular(8),
                    border: Border.all(color: const Color(0xFF404040)),
                  ),
                  padding: const EdgeInsets.symmetric(horizontal: 12),
                  alignment: Alignment.centerLeft,
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
                          MultipleChoiceAnswer(
                            set.toList()..sort(),
                            otherText ?? '',
                          ),
                        );
                      }
                    },
                    onChanged: (value) {
                      if (readOnly) return;
                      final set = selected.toSet();
                      if (!set.contains(othersIndex)) {
                        set.add(othersIndex);
                      }
                      onChanged(
                        MultipleChoiceAnswer(set.toList()..sort(), value),
                      );
                    },
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
                  ),
                ),
              ),
            ],
          ),
        ],
      ],
    );
  }
}
