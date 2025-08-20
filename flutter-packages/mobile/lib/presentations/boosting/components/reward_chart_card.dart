import 'dart:math' as math;
import 'package:ratel/exports.dart';

//FIXME: fix to query api
class RewardChartCard extends StatefulWidget {
  const RewardChartCard({super.key});

  @override
  State<RewardChartCard> createState() => _RewardChartCardState();
}

enum _ChartRange { day, week, month }

class _RewardChartCardState extends State<RewardChartCard> {
  _ChartRange range = _ChartRange.week;

  List<double> get _values {
    switch (range) {
      case _ChartRange.day:
        return [0.2, 0.4, 0.3, 0.6, 0.5, 0.7, 0.9];
      case _ChartRange.week:
        return [0.92, 0.58, 0.35, 0.62, 0.88];
      case _ChartRange.month:
        return List<double>.generate(12, (i) => math.max(0.15, (i + 1) / 12));
    }
  }

  List<String> _xLabels(DateTime now) {
    switch (range) {
      case _ChartRange.day:
        const d = ['M', 'T', 'W', 'T', 'F', 'S', 'S'];
        return d;
      case _ChartRange.week:
        return const ['1-7', '-14', '-21', '-27', '-31'];
      case _ChartRange.month:
        return const [
          'JAN',
          'FEB',
          'MAR',
          'APR',
          'MAY',
          'JUN',
          'JUL',
          'AUG',
          'SEP',
          'OCT',
          'NOV',
          'DEC',
        ];
    }
  }

  @override
  Widget build(BuildContext context) {
    final now = DateTime.now();
    final month = _monthShort(now.month).toUpperCase();
    final labels = _xLabels(now);

    return Container(
      padding: const EdgeInsets.fromLTRB(20, 18, 20, 18),
      decoration: BoxDecoration(
        color: AppColors.neutral900,
        borderRadius: BorderRadius.circular(10),
      ),
      child: Column(
        children: [
          Container(
            decoration: BoxDecoration(
              color: Colors.transparent,
              borderRadius: BorderRadius.circular(10),
              border: Border.all(color: Colors.white.withAlpha(25), width: 1),
            ),
            padding: const EdgeInsets.all(4),
            child: Row(
              mainAxisSize: MainAxisSize.min,
              children: [
                _seg('Day', _ChartRange.day),
                Container(
                  width: 1,
                  height: 20,
                  color: Colors.white.withAlpha(25),
                ),
                _seg('Week', _ChartRange.week),
                Container(
                  width: 1,
                  height: 20,
                  color: Colors.white.withAlpha(25),
                ),
                _seg('Month', _ChartRange.month),
              ],
            ),
          ),
          const SizedBox(height: 14),

          Text(
            range == _ChartRange.week
                ? '$month 1-31'
                : _titleFor(range, now, month),
            style: const TextStyle(
              color: Colors.white,
              fontWeight: FontWeight.w600,
              fontSize: 15,
            ),
          ),
          10.vgap,

          SizedBox(
            height: 190,
            child: Row(
              crossAxisAlignment: CrossAxisAlignment.end,
              children: [
                SizedBox(
                  width: 36,
                  child: Column(
                    mainAxisAlignment: MainAxisAlignment.spaceBetween,
                    children: [
                      YLabel(text: '100%'),
                      YLabel(text: '60%'),
                      YLabel(text: '20%'),
                      0.vgap,
                    ],
                  ),
                ),
                15.gap,

                Expanded(
                  child: LayoutBuilder(
                    builder: (ctx, c) {
                      final gap = (range == _ChartRange.month) ? 10.0 : 20.0;
                      final n = _values.length;
                      final barW = (c.maxWidth - (n - 1) * gap) / n;

                      final bool isMonth = range == _ChartRange.month;
                      final double labelFont = (isMonth && barW < 26) ? 10 : 14;

                      return Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          Expanded(
                            child: Row(
                              crossAxisAlignment: CrossAxisAlignment.end,
                              children: List.generate(n, (i) {
                                final h = _values[i].clamp(0.0, 1.0);
                                return Padding(
                                  padding: EdgeInsets.only(
                                    right: i == n - 1 ? 0 : gap,
                                  ),
                                  child: Container(
                                    width: barW,
                                    height: 150 * h,
                                    decoration: BoxDecoration(
                                      borderRadius: BorderRadius.circular(8),
                                      gradient: LinearGradient(
                                        begin: Alignment.topCenter,
                                        end: Alignment.bottomCenter,
                                        colors: [
                                          AppColors.primary,
                                          AppColors.primary.withAlpha(127),
                                          Color(0xff171717).withAlpha(127),
                                        ],
                                        stops: [0.0, 0.7, 1.0],
                                      ),
                                    ),
                                  ),
                                );
                              }),
                            ),
                          ),

                          const SizedBox(height: 8),

                          Row(
                            crossAxisAlignment: CrossAxisAlignment.start,
                            children: List.generate(n, (i) {
                              final raw = labels[i];
                              final String label = (isMonth && barW < 18)
                                  ? raw.substring(0, 1)
                                  : raw;

                              return Padding(
                                padding: EdgeInsets.only(
                                  right: i == n - 1 ? 0 : gap,
                                ),
                                child: SizedBox(
                                  width: barW,
                                  child: FittedBox(
                                    fit: BoxFit.scaleDown,
                                    child: Text(
                                      label,
                                      maxLines: 1,
                                      softWrap: false,
                                      overflow: TextOverflow.visible,
                                      textAlign: TextAlign.center,
                                      style: TextStyle(
                                        color: AppColors.neutral400,
                                        fontWeight: FontWeight.w400,
                                        fontSize: labelFont,
                                        height: 1.2,
                                      ),
                                    ),
                                  ),
                                ),
                              );
                            }),
                          ),
                        ],
                      );
                    },
                  ),
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }

  Widget _seg(String label, _ChartRange r) {
    final selected = range == r;
    return GestureDetector(
      onTap: () => setState(() => range = r),
      child: AnimatedContainer(
        duration: const Duration(milliseconds: 160),
        padding: const EdgeInsets.symmetric(horizontal: 14, vertical: 6),
        decoration: BoxDecoration(
          color: Colors.transparent,
          borderRadius: BorderRadius.circular(8),
        ),
        child: Text(
          label,
          style: TextStyle(
            color: selected ? AppColors.primary : Colors.white,
            fontWeight: FontWeight.w600,
            fontSize: 14,
            height: 1.3,
          ),
        ),
      ),
    );
  }

  static String _titleFor(_ChartRange r, DateTime now, String month) {
    switch (r) {
      case _ChartRange.day:
        return '${now.year}-${_2(now.month)}-${_2(now.day)}';
      case _ChartRange.week:
        return '$month 1-31';
      case _ChartRange.month:
        return '${now.year}';
    }
  }

  static String _monthShort(int m) {
    const s = [
      'Jan',
      'Feb',
      'Mar',
      'Apr',
      'May',
      'Jun',
      'Jul',
      'Aug',
      'Sep',
      'Oct',
      'Nov',
      'Dec',
    ];
    return s[math.max(1, m) - 1];
  }
}

String _2(int n) => n.toString().padLeft(2, '0');

class YLabel extends StatelessWidget {
  final String text;
  const YLabel({super.key, required this.text});

  @override
  Widget build(BuildContext context) {
    return Text(
      text,
      style: const TextStyle(
        color: AppColors.neutral400,
        fontSize: 14,
        fontWeight: FontWeight.w400,
        height: 1.3,
      ),
    );
  }
}
