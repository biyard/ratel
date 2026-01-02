import 'package:ratel/exports.dart';

class PollTimeHeader extends StatelessWidget {
  const PollTimeHeader({
    super.key,
    required this.timeZone,
    required this.start,
    required this.end,
  });

  final String timeZone;
  final DateTime start;
  final DateTime end;

  static const _months = [
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

  String _mon(DateTime d) => _months[(d.month - 1).clamp(0, 11)];

  String _rangeText(DateTime s, DateTime e) {
    final sameYear = s.year == e.year;
    final sameMonth = sameYear && s.month == e.month;

    if (sameMonth) {
      return '${_mon(s)} ${s.day}-${e.day}, ${s.year}';
    }
    if (sameYear) {
      return '${_mon(s)} ${s.day}-${_mon(e)} ${e.day}, ${s.year}';
    }
    return '${_mon(s)} ${s.day}, ${s.year}-${_mon(e)} ${e.day}, ${e.year}';
  }

  int _daysLeft() {
    final now = DateTime.now();
    final diff = end.difference(now);
    if (diff.isNegative) return 0;
    final days = (diff.inSeconds / 86400).ceil();
    return days < 0 ? 0 : days;
  }

  @override
  Widget build(BuildContext context) {
    final text = '${_daysLeft()} days left, ${_rangeText(start, end)}';

    return Text(
      text,
      style: const TextStyle(
        fontFamily: 'Raleway',
        fontWeight: FontWeight.w600,
        fontSize: 13,
        height: 20 / 13,
        color: Color(0xFF8C8C8C),
      ),
    );
  }
}
