import 'package:ratel/exports.dart';

class PollProgressBar extends StatelessWidget {
  const PollProgressBar({
    super.key,
    required this.total,
    required this.currentIndex,
    required this.maxReached,
  });

  final int total;
  final int currentIndex;
  final int maxReached;

  @override
  Widget build(BuildContext context) {
    if (total <= 0) return const SizedBox.shrink();

    return Row(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: List.generate(total, (i) {
        final clickable = i <= maxReached;
        final isBlue = clickable;
        return Expanded(
          child: Padding(
            padding: EdgeInsets.only(right: i == total - 1 ? 0 : 10),
            child: Container(
              height: 8,
              decoration: BoxDecoration(
                color: isBlue
                    ? const Color(0x803B82F6)
                    : const Color(0xFF262626),
                borderRadius: BorderRadius.circular(100),
              ),
            ),
          ),
        );
      }),
    );
  }
}
