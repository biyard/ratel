import 'package:ratel/exports.dart';

class ExchangePanel extends StatelessWidget {
  const ExchangePanel({super.key, required this.totalPoints});
  final int totalPoints;

  static const int _pointPerToken = 1000;

  @override
  Widget build(BuildContext context) {
    final myT = totalPoints ~/ _pointPerToken;

    return Container(
      padding: const EdgeInsets.fromLTRB(10, 15, 10, 15),
      decoration: BoxDecoration(
        color: AppColors.neutral900,
        borderRadius: BorderRadius.circular(10),
      ),
      child: Row(
        mainAxisAlignment: MainAxisAlignment.spaceBetween,
        children: [
          Row(
            mainAxisAlignment: MainAxisAlignment.start,
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              SvgPicture.asset(Assets.rewardCoin),
              5.gap,
              Row(
                mainAxisAlignment: MainAxisAlignment.center,
                crossAxisAlignment: CrossAxisAlignment.center,
                children: [
                  Text(
                    comma(totalPoints),
                    style: const TextStyle(
                      color: Colors.white,
                      fontWeight: FontWeight.w700,
                      fontSize: 16,
                    ),
                  ),
                  3.gap,
                  Text(
                    "P",
                    style: const TextStyle(
                      color: Colors.white,
                      fontWeight: FontWeight.w500,
                      fontSize: 15,
                    ),
                  ),
                ],
              ),
            ],
          ),
          Expanded(
            child: Column(
              children: [
                RoundContainer(
                  radius: 12,
                  color: AppColors.neutral800,
                  child: Padding(
                    padding: EdgeInsets.all(10),
                    child: SvgPicture.asset(
                      Assets.exchange2,
                      width: 24,
                      height: 24,
                      color: AppColors.primary,
                    ),
                  ),
                ),
                4.vgap,
                const Text(
                  'Exchange',
                  style: TextStyle(
                    color: Colors.white,
                    fontSize: 12,
                    fontWeight: FontWeight.w600,
                    height: 1.2,
                  ),
                ),
              ],
            ),
          ),
          Row(
            mainAxisAlignment: MainAxisAlignment.start,
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Row(
                mainAxisAlignment: MainAxisAlignment.center,
                crossAxisAlignment: CrossAxisAlignment.center,
                children: [
                  Text(
                    comma(myT),
                    style: const TextStyle(
                      color: Colors.white,
                      fontWeight: FontWeight.w700,
                      fontSize: 16,
                    ),
                  ),
                  3.gap,
                  Text(
                    "T",
                    style: const TextStyle(
                      color: Colors.white,
                      fontWeight: FontWeight.w500,
                      fontSize: 15,
                    ),
                  ),
                ],
              ),
              5.gap,
              SvgPicture.asset(Assets.botCoin),
            ],
          ),
        ],
      ),
    );
  }
}
