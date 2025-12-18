import 'package:ratel/exports.dart';

class CheckboxQuestionView extends StatelessWidget {
  const CheckboxQuestionView({
    super.key,
    required this.question,
    required this.answer,
    required this.onChanged,
    required this.readOnly,
  });

  final CheckboxQuestionModel question;
  final CheckboxAnswer? answer;
  final ValueChanged<Answer> onChanged;
  final bool readOnly;

  void _toggle(Set<int> selected, int i) {
    final set = selected.toSet();

    if (question.isMulti) {
      if (set.contains(i)) {
        set.remove(i);
      } else {
        set.add(i);
      }
    } else {
      if (set.contains(i)) {
        set.clear();
      } else {
        set
          ..clear()
          ..add(i);
      }
    }

    onChanged(CheckboxAnswer(set.toList()..sort()));
  }

  @override
  Widget build(BuildContext context) {
    final selected = (answer?.answer ?? const <int>[]).toSet();

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        for (int i = 0; i < question.options.length; i++) ...[
          GestureDetector(
            behavior: HitTestBehavior.opaque,
            onTap: readOnly ? null : () => _toggle(selected, i),
            child: Container(
              height: 72,
              padding: const EdgeInsets.symmetric(horizontal: 15),
              decoration: BoxDecoration(
                color: const Color(0xFF171717),
                borderRadius: BorderRadius.circular(10),
              ),
              child: Row(
                crossAxisAlignment: CrossAxisAlignment.center,
                children: [
                  Container(
                    width: 20,
                    height: 20,
                    decoration: BoxDecoration(
                      borderRadius: BorderRadius.circular(4),
                      border: Border.all(
                        color: selected.contains(i)
                            ? AppColors.primary
                            : const Color(0xFF737373),
                        width: 2,
                      ),
                      color: selected.contains(i)
                          ? AppColors.primary
                          : const Color(0xFF101010),
                    ),
                    alignment: Alignment.center,
                    child: selected.contains(i)
                        ? const Icon(
                            Icons.check,
                            size: 16,
                            color: Color(0xFF1D1D1D),
                          )
                        : null,
                  ),
                  20.gap,
                  Expanded(
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
                    ),
                  ),
                ],
              ),
            ),
          ),
          10.vgap,
        ],
      ],
    );
  }
}
