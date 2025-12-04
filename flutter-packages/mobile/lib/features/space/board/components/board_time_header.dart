import 'package:intl/intl.dart';
import 'package:ratel/exports.dart';

class BoardTimeHeader extends StatelessWidget {
  const BoardTimeHeader({
    super.key,
    required this.timeZone,
    required this.start,
    required this.end,
  });

  final String timeZone;
  final DateTime start;
  final DateTime end;

  @override
  Widget build(BuildContext context) {
    final fmt = DateFormat('MMM d, yyyy, hh:mm a');

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text(
          timeZone,
          style: const TextStyle(
            fontFamily: 'Inter',
            fontSize: 12,
            color: Color(0xFF9CA3AF),
          ),
        ),
        4.vgap,
        Row(
          children: [
            Text(
              fmt.format(start),
              style: const TextStyle(
                fontFamily: 'Inter',
                fontSize: 13,
                color: Colors.white,
              ),
            ),
            6.gap,
            const Text(
              '->',
              style: TextStyle(
                fontFamily: 'Inter',
                fontSize: 13,
                color: Color(0xFF9CA3AF),
              ),
            ),
            6.gap,
            Text(
              fmt.format(end),
              style: const TextStyle(
                fontFamily: 'Inter',
                fontSize: 13,
                color: Colors.white,
              ),
            ),
          ],
        ),
      ],
    );
  }
}
