import 'package:ratel/exports.dart';
import 'package:ratel/presentations/boosting/components/exchange_panel.dart';

import 'package:ratel/presentations/boosting/components/reward_banner.dart';
import 'package:ratel/presentations/boosting/components/reward_chart_card.dart';
import 'package:ratel/presentations/boosting/components/share_card.dart';

class BoostingScreen extends GetWidget<BoostingController> {
  const BoostingScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Layout<BoostingController>(
      scrollable: true,
      child: Obx(() {
        final reward = controller.reward.value;
        return Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Padding(
              padding: const EdgeInsets.fromLTRB(16, 16, 16, 16),
              child: Row(
                children: [
                  InkWell(
                    onTap: controller.goBack,
                    child: SvgPicture.asset(Assets.back, width: 24, height: 24),
                  ),
                  const SizedBox(width: 12),
                  const Expanded(
                    child: Text(
                      'My Rewards',
                      textAlign: TextAlign.center,
                      style: TextStyle(
                        color: Colors.white,
                        fontSize: 22,
                        fontWeight: FontWeight.w800,
                        height: 1.2,
                      ),
                    ),
                  ),
                  IconButton(
                    onPressed: () {},
                    icon: const Icon(Icons.more_vert, color: Colors.white),
                  ),
                ],
              ),
            ),

            const Padding(
              padding: EdgeInsets.fromLTRB(15, 10, 15, 20),
              child: RewardBanner(additionalPoints: 4000),
            ),

            Padding(
              padding: const EdgeInsets.fromLTRB(15, 0, 15, 0),
              child: ShareCard(totalPoints: reward.points),
            ),

            20.vgap,

            Padding(
              padding: const EdgeInsets.fromLTRB(15, 0, 15, 0),
              child: ExchangePanel(totalPoints: reward.points),
            ),

            30.vgap,

            const Padding(
              padding: EdgeInsets.fromLTRB(15, 0, 15, 8),
              child: Text(
                'Chart',
                style: TextStyle(
                  color: Colors.white,
                  fontWeight: FontWeight.w700,
                  fontSize: 16,
                  height: 1.2,
                ),
              ),
            ),

            const Padding(
              padding: EdgeInsets.fromLTRB(16, 0, 16, 24),
              child: RewardChartCard(),
            ),
          ],
        );
      }),
    );
  }
}
