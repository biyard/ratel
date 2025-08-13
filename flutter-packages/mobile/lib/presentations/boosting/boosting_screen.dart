import 'package:ratel/exports.dart';
import 'dart:math' as math;

class BoostingScreen extends GetWidget<BoostingController> {
  const BoostingScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Layout<BoostingController>(
      child: SingleChildScrollView(
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            // Header
            Padding(
              padding: const EdgeInsets.fromLTRB(24, 20, 24, 20),
              child: Row(
                children: [
                  InkWell(
                    onTap: controller.goBack,
                    child: SvgPicture.asset(Assets.back, width: 24, height: 24),
                  ),
                  20.gap,
                  const Text(
                    'Boosting Points',
                    style: TextStyle(
                      color: Colors.white,
                      fontSize: 24,
                      fontWeight: FontWeight.w700,
                      height: 1.2,
                    ),
                  ),
                ],
              ),
            ),
            Container(
              height: 1,
              width: double.infinity,
              color: const Color(0xff464646),
            ),
            8.vgap,

            // List
            Obx(() {
              final items = controller.boostings;
              if (items.isEmpty) {
                return const Padding(
                  padding: EdgeInsets.symmetric(vertical: 40),
                  child: Center(
                    child: Text(
                      'No history',
                      style: TextStyle(color: AppColors.neutral500),
                    ),
                  ),
                );
              }

              return ListView.separated(
                shrinkWrap: true,
                physics: const NeverScrollableScrollPhysics(),
                padding: const EdgeInsets.symmetric(
                  horizontal: 14,
                  vertical: 6,
                ),
                itemCount: items.length,
                separatorBuilder: (_, __) => const SizedBox(height: 8),
                itemBuilder: (_, i) => BoostingTile(
                  model: items[i],
                  isFirst: i == 0,
                  onExchange: () {}, // TODO: hook up action
                ),
              );
            }),
          ],
        ),
      ),
    );
  }
}

class BoostingTile extends StatelessWidget {
  const BoostingTile({
    super.key,
    required this.model,
    required this.isFirst,
    required this.onExchange,
  });

  final BoostingModel model;
  final bool isFirst;
  final VoidCallback onExchange;

  @override
  Widget build(BuildContext context) {
    final dt = DateTime.fromMillisecondsSinceEpoch(model.updatedAt * 1000);
    final month = monthLabel(dt.month);
    final updatedText =
        'Last updated: ${dt.year}-${_2(dt.month)}-${_2(dt.day)} ${_2(dt.hour)}:${_2(dt.minute)}';

    return Padding(
      padding: const EdgeInsets.all(20.0),
      child: Column(
        children: [
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              MonthLabel(label: month),
              Text(
                updatedText,
                style: const TextStyle(
                  color: Color(0xffaeaeae),
                  fontSize: 11,
                  fontWeight: FontWeight.w500,
                  height: 1.3,
                ),
              ),
            ],
          ),
          10.vgap,

          isFirst
              ? Center(child: Points(points: model.points))
              : Row(
                  mainAxisAlignment: MainAxisAlignment.spaceBetween,
                  crossAxisAlignment: CrossAxisAlignment.center,
                  children: [
                    Padding(
                      padding: const EdgeInsets.only(left: 20),
                      child: Points(points: model.points),
                    ),

                    if (model.exchanged) ...[
                      const ExchangedBadge(),
                      Padding(
                        padding: const EdgeInsets.only(right: 20),
                        child: Ratels(value: model.ratels),
                      ),
                    ] else ...[
                      const SizedBox.shrink(),
                      ExchangeButton(onPressed: onExchange),
                    ],
                  ],
                ),
        ],
      ),
    );
  }
}

class MonthLabel extends StatelessWidget {
  const MonthLabel({super.key, required this.label});
  final String label;

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 3),
      decoration: BoxDecoration(
        color: Colors.white,
        borderRadius: BorderRadius.circular(4),
        border: Border.all(color: AppColors.neutral700),
      ),
      child: Text(
        label,
        style: const TextStyle(
          color: AppColors.neutral800,
          fontSize: 11,
          fontWeight: FontWeight.w500,
          height: 1.2,
        ),
      ),
    );
  }
}

class Points extends StatelessWidget {
  const Points({super.key, required this.points});
  final int points;

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.center,
      mainAxisAlignment: MainAxisAlignment.center,
      children: [
        Text(
          '${points}',
          style: const TextStyle(
            color: Colors.white,
            fontSize: 24,
            fontWeight: FontWeight.w700,
            height: 1.2,
          ),
        ),
        10.vgap,
        const Text(
          'Points',
          style: TextStyle(
            color: Colors.white,
            fontSize: 14,
            fontWeight: FontWeight.w600,
            height: 1.2,
          ),
        ),
      ],
    );
  }
}

class ExchangedBadge extends StatelessWidget {
  const ExchangedBadge({super.key});

  @override
  Widget build(BuildContext context) {
    return Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        SvgPicture.asset(Assets.exchange),
        SizedBox(height: 5),
        Text(
          'Exchanged',
          style: TextStyle(
            color: Colors.white,
            fontSize: 11,
            fontWeight: FontWeight.w500,
            height: 1.2,
          ),
        ),
      ],
    );
  }
}

class Ratels extends StatelessWidget {
  const Ratels({super.key, required this.value});
  final int value;

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.center,
      mainAxisAlignment: MainAxisAlignment.center,
      children: [
        Text(
          '$value',
          style: const TextStyle(
            color: Colors.white,
            fontSize: 24,
            fontWeight: FontWeight.w700,
            height: 1.2,
          ),
        ),
        10.vgap,
        const Text(
          'Ratels',
          style: TextStyle(
            color: Colors.white,
            fontSize: 14,
            fontWeight: FontWeight.w600,
            height: 1.2,
          ),
        ),
      ],
    );
  }
}

class ExchangeButton extends StatelessWidget {
  const ExchangeButton({super.key, required this.onPressed});
  final VoidCallback onPressed;

  @override
  Widget build(BuildContext context) {
    return ElevatedButton(
      onPressed: onPressed,
      style: ElevatedButton.styleFrom(
        backgroundColor: AppColors.primary,
        foregroundColor: AppColors.neutral800,
        padding: const EdgeInsets.symmetric(horizontal: 20, vertical: 10),
        shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(5)),
      ),
      child: const Text(
        'EXCHANGE',
        style: TextStyle(
          fontWeight: FontWeight.w800,
          fontSize: 12,
          color: Color(0xff000203),
          height: 1.2,
        ),
      ),
    );
  }
}

String monthLabel(int m) {
  const names = [
    'January',
    'February',
    'March',
    'April',
    'May',
    'June',
    'July',
    'August',
    'September',
    'October',
    'November',
    'December',
  ];
  return names[math.max(1, m) - 1];
}

String _2(int n) => n.toString().padLeft(2, '0');
