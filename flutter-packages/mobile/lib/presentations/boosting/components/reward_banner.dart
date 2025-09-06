import 'package:ratel/exports.dart';

class RewardBanner extends StatelessWidget {
  const RewardBanner({super.key, required this.additionalPoints});
  final int additionalPoints;

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.fromLTRB(20, 10, 20, 10),
      decoration: BoxDecoration(
        borderRadius: BorderRadius.circular(14),
        gradient: LinearGradient(
          begin: Alignment.topLeft,
          end: Alignment.bottomRight,
          colors: [Color(0xFF381D96), Color(0xFF381D96).withAlpha(127)],
        ),
      ),
      child: Row(
        mainAxisAlignment: MainAxisAlignment.start,
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Container(
            width: 36,
            height: 36,
            decoration: const BoxDecoration(
              shape: BoxShape.circle,
              color: AppColors.indigo950,
            ),
            child: Center(
              child: SvgPicture.asset(Assets.solarStar, width: 21, height: 21),
            ),
          ),
          10.gap,
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Row(
                  children: [
                    SvgPicture.asset(
                      Assets.add,
                      width: 20,
                      height: 20,
                      color: AppColors.primary,
                    ),
                    Text(
                      '${comma(additionalPoints)} P',
                      style: const TextStyle(
                        color: AppColors.primary,
                        fontWeight: FontWeight.w700,
                        fontSize: 20,
                        height: 1.2,
                      ),
                    ),
                  ],
                ),
                10.vgap,
                const Text(
                  "Nice Work! You've earned a new achievement badge!",
                  style: TextStyle(
                    color: Colors.white,
                    fontWeight: FontWeight.w600,
                    fontSize: 14,
                    height: 1.3,
                  ),
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }
}
