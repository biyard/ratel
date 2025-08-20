import 'package:ratel/exports.dart';

class ShareCard extends StatelessWidget {
  const ShareCard({super.key, required this.totalPoints});

  final int totalPoints;

  static const int _pointPerToken = 1000;
  static const int _monthPoolT = 1000000;

  @override
  Widget build(BuildContext context) {
    final myT = totalPoints ~/ _pointPerToken;
    final percent = (myT / _monthPoolT).clamp(0, 1.0) * 100;

    return Container(
      padding: const EdgeInsets.fromLTRB(10, 15, 10, 15),
      decoration: BoxDecoration(
        color: AppColors.neutral900,
        borderRadius: BorderRadius.circular(10),
      ),
      child: Column(
        children: [
          Row(
            mainAxisAlignment: MainAxisAlignment.start,
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Expanded(
                child: share(
                  'Your share',
                  comma(totalPoints),
                  myT.toString(),
                  percent.toStringAsFixed(2).toString(),
                  alignEnd: false,
                ),
              ),
              15.gap,
              Expanded(child: monthPool()),
            ],
          ),
          20.vgap,

          Stack(
            alignment: Alignment.centerLeft,
            children: [
              Container(
                height: 36,
                decoration: BoxDecoration(
                  color: AppColors.neutral800,
                  borderRadius: BorderRadius.circular(10),
                ),
              ),

              TweenAnimationBuilder<double>(
                key: ValueKey(totalPoints),
                duration: const Duration(milliseconds: 700),
                curve: Curves.easeOutCubic,
                tween: Tween(begin: 0, end: (percent / 100).clamp(0.02, 1.0)),
                builder: (context, t, child) {
                  return ClipRRect(
                    borderRadius: BorderRadius.circular(10),
                    child: Align(
                      alignment: Alignment.centerLeft,
                      child: FractionallySizedBox(widthFactor: t, child: child),
                    ),
                  );
                },
                child: Container(
                  height: 28,
                  margin: const EdgeInsets.symmetric(vertical: 4),
                  decoration: BoxDecoration(
                    borderRadius: BorderRadius.circular(50),
                    gradient: const LinearGradient(
                      begin: Alignment.centerLeft,
                      end: Alignment.centerRight,
                      colors: [AppColors.primary, AppColors.primary],
                    ),
                  ),
                ),
              ),

              Positioned(
                left: 6,
                right: 56,
                child: Text(
                  'Yours ${percent.toStringAsFixed(2)}%',
                  maxLines: 1,
                  overflow: TextOverflow.ellipsis,
                  style: const TextStyle(
                    color: Colors.white,
                    fontSize: 14,
                    fontWeight: FontWeight.w700,
                    height: 1.1,
                  ),
                ),
              ),

              const Positioned(
                right: 12,
                child: Text(
                  '100%',
                  style: TextStyle(
                    color: Colors.white,
                    fontSize: 14,
                    fontWeight: FontWeight.w700,
                  ),
                ),
              ),
            ],
          ),
        ],
      ),
    );
  }

  Widget monthPool() {
    return Column(
      mainAxisAlignment: MainAxisAlignment.end,
      crossAxisAlignment: CrossAxisAlignment.end,
      children: [
        Text(
          "This month's pool",
          style: const TextStyle(
            color: AppColors.neutral400,
            fontSize: 14,
            fontWeight: FontWeight.w600,
            height: 1.1,
          ),
        ),
        8.vgap,
        Row(
          mainAxisAlignment: MainAxisAlignment.end,
          crossAxisAlignment: CrossAxisAlignment.center,
          children: [
            Text(
              "/ 1M",
              style: const TextStyle(
                color: AppColors.neutral400,
                fontSize: 20,
                fontWeight: FontWeight.w700,
              ),
            ),
            5.gap,
            Text(
              "T (100%)",
              style: const TextStyle(
                color: AppColors.neutral400,
                fontSize: 15,
                fontWeight: FontWeight.w700,
              ),
            ),
          ],
        ),
      ],
    );
  }

  Widget share(
    String k,
    String v,
    String token,
    String percent, {
    String? sub,
    bool alignEnd = false,
  }) {
    final cross = alignEnd ? CrossAxisAlignment.end : CrossAxisAlignment.start;
    return Column(
      crossAxisAlignment: cross,
      children: [
        Text(
          k,
          style: const TextStyle(
            color: AppColors.neutral400,
            fontSize: 14,
            fontWeight: FontWeight.w600,
            height: 1.1,
          ),
        ),
        8.vgap,
        Row(
          mainAxisAlignment: MainAxisAlignment.start,
          crossAxisAlignment: CrossAxisAlignment.end,
          children: [
            Text(
              v,
              style: const TextStyle(
                color: Colors.white,
                fontSize: 20,
                fontWeight: FontWeight.w700,
              ),
            ),
            3.gap,
            Text(
              "P",
              style: const TextStyle(
                color: Colors.white,
                fontSize: 15,
                fontWeight: FontWeight.w500,
              ),
            ),
          ],
        ),
        8.vgap,
        Row(
          children: [
            SvgPicture.asset(Assets.exchange2, width: 20, height: 20),
            4.gap,
            Text(
              "$token T",
              style: const TextStyle(
                color: Colors.white,
                fontSize: 15,
                fontWeight: FontWeight.w500,
              ),
            ),
            3.gap,
            Text(
              "($percent %)",
              style: const TextStyle(
                color: Colors.white,
                fontSize: 12,
                fontWeight: FontWeight.w500,
              ),
            ),
          ],
        ),
      ],
    );
  }
}
