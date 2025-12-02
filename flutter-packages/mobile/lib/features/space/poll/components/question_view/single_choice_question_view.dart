import 'package:ratel/exports.dart';
import 'package:ratel/features/space/poll/components/option_tile.dart';

class SingleChoiceQuestionView extends StatelessWidget {
  const SingleChoiceQuestionView({
    super.key,
    required this.question,
    required this.answer,
    required this.onChanged,
  });

  final ChoiceQuestionModel question;
  final SingleChoiceAnswer? answer;
  final ValueChanged<Answer> onChanged;

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
          if (question.options[i] != 'Others') ...[
            OptionTile(
              label: question.options[i],
              selected: selectedIndex == i,
              onTap: () => onChanged(SingleChoiceAnswer(i, null)),
            ),
            10.vgap,
          ],
        ],
        if (_allowOther) ...[
          Row(
            children: [
              GestureDetector(
                behavior: HitTestBehavior.opaque,
                onTap: () {
                  if (isOtherSelected) {
                    onChanged(const SingleChoiceAnswer(null, null));
                  } else {
                    onChanged(
                      SingleChoiceAnswer(othersIndex!, otherText ?? ''),
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
                    initialValue: otherText ?? '',
                    onTap: () {
                      if (!isOtherSelected && othersIndex != null) {
                        onChanged(
                          SingleChoiceAnswer(othersIndex, otherText ?? ''),
                        );
                      }
                    },
                    onChanged: (value) {
                      if (othersIndex != null) {
                        onChanged(SingleChoiceAnswer(othersIndex, value));
                      }
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
