import 'package:ratel/exports.dart';

class AnalyzeHeader extends StatelessWidget {
  final PollModel poll;
  final PollResult result;

  const AnalyzeHeader({super.key, required this.poll, required this.result});

  String _formatDate(DateTime d) {
    final y = d.year.toString().padLeft(4, '0');
    final m = d.month.toString().padLeft(2, '0');
    final day = d.day.toString().padLeft(2, '0');
    return '$y.$m.$day';
  }

  @override
  Widget build(BuildContext context) {
    final participants = result.finalAnswers.isNotEmpty
        ? result.finalAnswers.length
        : result.sampleAnswers.length;

    final totalResponses = poll.userResponseCount;

    final startedAt = DateTime.fromMillisecondsSinceEpoch(
      poll.startedAt,
      isUtc: false,
    );
    final endedAt = DateTime.fromMillisecondsSinceEpoch(
      poll.endedAt,
      isUtc: false,
    );
    final now = DateTime.now();
    final remainingRaw = endedAt.difference(now).inDays;
    final remaining = remainingRaw < 0 ? 0 : remainingRaw;

    return Padding(
      padding: const EdgeInsets.fromLTRB(0, 0, 0, 12),
      child: Column(
        children: [
          Row(
            children: [
              Expanded(
                child: _HeaderNumberCard(
                  icon: Icons.group_outlined,
                  title: 'Participants',
                  value: '$participants',
                ),
              ),
              10.gap,
              Expanded(
                child: _HeaderNumberCard(
                  icon: Icons.show_chart_outlined,
                  title: 'Responses',
                  value: '$totalResponses',
                ),
              ),
            ],
          ),
          10.vgap,
          Row(
            children: [
              Expanded(
                child: _HeaderNumberCard(
                  icon: Icons.watch_later_outlined,
                  title: 'Remaining Days',
                  value: '$remaining Days',
                ),
              ),
              10.gap,
              Expanded(
                child: _HeaderPeriodCard(
                  icon: Icons.calendar_month_outlined,
                  title: 'Poll Period',
                  start: _formatDate(startedAt),
                  end: _formatDate(endedAt),
                ),
              ),
            ],
          ),
        ],
      ),
    );
  }
}

class _HeaderNumberCard extends StatelessWidget {
  final IconData icon;
  final String title;
  final String value;

  const _HeaderNumberCard({
    required this.icon,
    required this.title,
    required this.value,
  });

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);

    return Container(
      height: 100,
      decoration: BoxDecoration(
        color: const Color(0xFF171717),
        borderRadius: BorderRadius.circular(10),
      ),
      child: Padding(
        padding: const EdgeInsets.symmetric(horizontal: 4),
        child: Container(
          decoration: BoxDecoration(
            color: const Color(0xFF1A1A1A),
            borderRadius: BorderRadius.circular(10),
          ),
          child: Padding(
            padding: const EdgeInsets.symmetric(vertical: 15),
            child: Row(
              children: [
                Container(
                  width: 48,
                  height: 48,
                  decoration: BoxDecoration(
                    borderRadius: BorderRadius.circular(10),
                  ),
                  child: Icon(icon, size: 28, color: const Color(0xFFD4D4D4)),
                ),
                10.gap,
                Expanded(
                  child: Column(
                    mainAxisAlignment: MainAxisAlignment.center,
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text(
                        title,
                        style: theme.textTheme.labelMedium?.copyWith(
                          fontWeight: FontWeight.w600,
                          fontSize: 13,
                          color: Colors.white,
                        ),
                      ),
                      4.vgap,
                      Text(
                        value,
                        style: theme.textTheme.titleMedium?.copyWith(
                          fontWeight: FontWeight.w600,
                          fontSize: 18,
                          color: Colors.white,
                        ),
                      ),
                    ],
                  ),
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}

class _HeaderPeriodCard extends StatelessWidget {
  final IconData icon;
  final String title;
  final String start;
  final String end;

  const _HeaderPeriodCard({
    required this.icon,
    required this.title,
    required this.start,
    required this.end,
  });

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);

    return Container(
      height: 100,
      decoration: BoxDecoration(
        color: const Color(0xFF171717),
        borderRadius: BorderRadius.circular(10),
      ),
      child: Padding(
        padding: const EdgeInsets.symmetric(horizontal: 4),
        child: Container(
          decoration: BoxDecoration(
            color: const Color(0xFF1A1A1A),
            borderRadius: BorderRadius.circular(10),
          ),
          child: Padding(
            padding: const EdgeInsets.symmetric(vertical: 15),
            child: Row(
              children: [
                Container(
                  width: 48,
                  height: 48,
                  decoration: BoxDecoration(
                    borderRadius: BorderRadius.circular(10),
                  ),
                  child: Icon(icon, size: 28, color: const Color(0xFFD4D4D4)),
                ),
                10.gap,
                Expanded(
                  child: Column(
                    mainAxisAlignment: MainAxisAlignment.center,
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text(
                        title,
                        style: theme.textTheme.labelMedium?.copyWith(
                          fontWeight: FontWeight.w600,
                          fontSize: 13,
                          color: Colors.white,
                        ),
                      ),
                      4.vgap,
                      Text(
                        start,
                        style: theme.textTheme.titleMedium?.copyWith(
                          fontWeight: FontWeight.w600,
                          fontSize: 16,
                          color: Colors.white,
                        ),
                      ),
                      Text(
                        end,
                        style: theme.textTheme.titleMedium?.copyWith(
                          fontWeight: FontWeight.w600,
                          fontSize: 16,
                          color: Colors.white,
                        ),
                      ),
                    ],
                  ),
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}
