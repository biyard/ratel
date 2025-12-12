import 'package:ratel/exports.dart';

class SubjectiveAnswersView extends StatelessWidget {
  final PollSummary summary;

  const SubjectiveAnswersView({super.key, required this.summary});

  Map<String, int> _extractAnswers() {
    if (summary is ShortAnswerSummary) {
      return (summary as ShortAnswerSummary).answers;
    } else if (summary is SubjectiveSummary) {
      return (summary as SubjectiveSummary).answers;
    }
    return const {};
  }

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final answers = _extractAnswers();
    if (answers.isEmpty) {
      return const SizedBox.shrink();
    }

    final entries = answers.entries.toList()
      ..sort((a, b) => b.value.compareTo(a.value));

    return Column(
      children: entries.map((e) {
        return Container(
          width: double.infinity,
          margin: const EdgeInsets.symmetric(vertical: 4),
          padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 10),
          decoration: BoxDecoration(
            borderRadius: BorderRadius.circular(8),
            color: Color(0xff1a1a1a),
            border: Border.all(color: Color(0xff464646), width: 1),
          ),
          child: Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              Expanded(
                child: Text(
                  e.key,
                  style: theme.textTheme.bodyMedium?.copyWith(
                    color: Colors.white,
                    fontSize: 14,
                  ),
                  overflow: TextOverflow.ellipsis,
                ),
              ),
              10.gap,
              Text(
                '(${e.value})',
                style: theme.textTheme.bodySmall?.copyWith(
                  color: Colors.white,
                  fontSize: 14,
                ),
              ),
            ],
          ),
        );
      }).toList(),
    );
  }
}
