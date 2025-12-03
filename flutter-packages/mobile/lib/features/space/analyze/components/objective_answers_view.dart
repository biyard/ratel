import 'package:ratel/exports.dart';
import 'package:fl_chart/fl_chart.dart';

class ChoiceOptionCount {
  final String label;
  final int count;

  ChoiceOptionCount(this.label, this.count);
}

class ObjectiveAnswersView extends StatelessWidget {
  final QuestionModel question;
  final PollSummary summary;

  const ObjectiveAnswersView({
    super.key,
    required this.question,
    required this.summary,
  });

  List<ChoiceOptionCount> _extractOptions() {
    Map<int, int> counts;

    if (summary is SingleChoiceSummary) {
      counts = (summary as SingleChoiceSummary).answers;
    } else if (summary is MultipleChoiceSummary) {
      counts = (summary as MultipleChoiceSummary).answers;
    } else if (summary is CheckboxSummary) {
      counts = (summary as CheckboxSummary).answers;
    } else if (summary is DropdownSummary) {
      counts = (summary as DropdownSummary).answers;
    } else if (summary is LinearScaleSummary) {
      counts = (summary as LinearScaleSummary).answers;
    } else {
      return const [];
    }

    final entries = <ChoiceOptionCount>[];

    if (question is ChoiceQuestionModel) {
      final q = question as ChoiceQuestionModel;
      counts.forEach((key, value) {
        if (key >= 0 && key < q.options.length) {
          entries.add(ChoiceOptionCount(q.options[key], value));
        }
      });
    } else if (question is CheckboxQuestionModel) {
      final q = question as CheckboxQuestionModel;
      counts.forEach((key, value) {
        if (key >= 0 && key < q.options.length) {
          entries.add(ChoiceOptionCount(q.options[key], value));
        }
      });
    } else if (question is DropdownQuestionModel) {
      final q = question as DropdownQuestionModel;
      counts.forEach((key, value) {
        if (key >= 0 && key < q.options.length) {
          entries.add(ChoiceOptionCount(q.options[key], value));
        }
      });
    } else if (question is LinearScaleQuestionModel) {
      final q = question as LinearScaleQuestionModel;
      counts.forEach((key, value) {
        String label = key.toString();
        if (key == q.minValue && q.minLabel.isNotEmpty) {
          label = '$key (${q.minLabel})';
        } else if (key == q.maxValue && q.maxLabel.isNotEmpty) {
          label = '$key (${q.maxLabel})';
        }
        entries.add(ChoiceOptionCount(label, value));
      });
    }

    entries.sort((a, b) => b.count.compareTo(a.count));
    return entries;
  }

  static const _chartColors = [
    Color(0xFFF97316),
    Color(0xFF22c55e),
    Color(0xFF6366f1),
    Color(0xFF274c9d),
    Color(0xFF2959c1),
    Color(0xFF2853af),
  ];

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final options = _extractOptions();
    final total = summary.totalCount;

    if (options.isEmpty || total == 0) {
      return const SizedBox.shrink();
    }

    return Column(
      children: [
        Column(
          children: options.asMap().entries.map((entry) {
            final index = entry.key;
            final o = entry.value;
            final ratio = o.count / total;
            final barColor = _chartColors[index % _chartColors.length];

            return Padding(
              padding: const EdgeInsets.symmetric(vertical: 4),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Row(
                    mainAxisAlignment: MainAxisAlignment.spaceBetween,
                    children: [
                      Expanded(
                        child: Text(
                          o.label,
                          style: theme.textTheme.bodyMedium?.copyWith(
                            color: AppColors.neutral400,
                          ),
                          overflow: TextOverflow.ellipsis,
                        ),
                      ),
                      8.gap,
                      Text(
                        '${o.count} (${(ratio * 100).toStringAsFixed(1)}%)',
                        style: theme.textTheme.bodySmall?.copyWith(
                          color: AppColors.neutral400,
                        ),
                      ),
                    ],
                  ),
                  4.vgap,
                  ClipRRect(
                    borderRadius: BorderRadius.circular(999),
                    child: LinearProgressIndicator(
                      value: ratio,
                      minHeight: 8,
                      color: barColor,
                      backgroundColor: AppColors.neutral300,
                    ),
                  ),
                ],
              ),
            );
          }).toList(),
        ),
        15.vgap,
        SizedBox(
          height: 180,
          child: PieChart(
            PieChartData(
              sectionsSpace: 2,
              centerSpaceRadius: 0,
              sections: [
                for (var i = 0; i < options.length; i++)
                  PieChartSectionData(
                    color: _chartColors[i % _chartColors.length],
                    value: options[i].count.toDouble(),
                    title:
                        '${options[i].label}: ${(options[i].count / total * 100).toStringAsFixed(0)}%',
                    radius: 70,
                    titleStyle: theme.textTheme.labelSmall?.copyWith(
                      color: Colors.white,
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
