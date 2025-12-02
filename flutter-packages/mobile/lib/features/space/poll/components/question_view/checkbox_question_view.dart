import 'package:ratel/exports.dart';
import 'package:ratel/features/space/poll/components/option_tile.dart';

class CheckboxQuestionView extends StatelessWidget {
  const CheckboxQuestionView({
    super.key,
    required this.question,
    required this.answer,
    required this.onChanged,
  });

  final CheckboxQuestionModel question;
  final CheckboxAnswer? answer;
  final ValueChanged<Answer> onChanged;

  @override
  Widget build(BuildContext context) {
    final selected = (answer?.answer ?? const <int>[]).toSet();

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        for (int i = 0; i < question.options.length; i++) ...[
          OptionTile(
            label: question.options[i],
            selected: selected.contains(i),
            onTap: () {
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
            },
          ),
          10.vgap,
        ],
      ],
    );
  }
}
