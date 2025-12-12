import 'package:ratel/exports.dart';

class LinearScaleQuestionView extends StatelessWidget {
  const LinearScaleQuestionView({
    super.key,
    required this.question,
    required this.answer,
    required this.onChanged,
    required this.readOnly,
  });

  final LinearScaleQuestionModel question;
  final LinearScaleAnswer? answer;
  final ValueChanged<Answer> onChanged;
  final bool readOnly;

  @override
  Widget build(BuildContext context) {
    final selected = answer?.answer;

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Row(
          mainAxisAlignment: MainAxisAlignment.spaceBetween,
          children: [
            Text(
              question.minLabel.isEmpty ? 'min' : question.minLabel,
              style: const TextStyle(
                fontFamily: 'Inter',
                fontSize: 13,
                color: Color(0xFF9CA3AF),
              ),
            ),
            Text(
              question.maxLabel.isEmpty ? 'max' : question.maxLabel,
              style: const TextStyle(
                fontFamily: 'Inter',
                fontSize: 13,
                color: Color(0xFF9CA3AF),
              ),
            ),
          ],
        ),
        10.vgap,
        Align(
          alignment: Alignment.center,
          child: SizedBox(
            width: 300,
            child: Wrap(
              alignment: WrapAlignment.center,
              crossAxisAlignment: WrapCrossAlignment.center,
              runAlignment: WrapAlignment.center,
              spacing: 5,
              runSpacing: 5,
              children: [
                for (int v = question.minValue; v <= question.maxValue; v++)
                  GestureDetector(
                    onTap: readOnly
                        ? null
                        : () => onChanged(LinearScaleAnswer(v)),
                    child: Container(
                      width: 34,
                      height: 34,
                      alignment: Alignment.center,
                      decoration: BoxDecoration(
                        color: selected == v
                            ? AppColors.primary
                            : AppColors.neutral800,
                        borderRadius: BorderRadius.circular(6),
                      ),
                      child: Text(
                        '$v',
                        style: TextStyle(
                          fontFamily: 'Inter',
                          fontSize: 14,
                          fontWeight: FontWeight.w500,
                          color: selected == v ? Colors.black : Colors.white,
                        ),
                      ),
                    ),
                  ),
              ],
            ),
          ),
        ),
      ],
    );
  }
}
