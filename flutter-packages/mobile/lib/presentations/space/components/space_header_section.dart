import 'package:ratel/exports.dart';

class SpaceHeaderSection extends StatelessWidget {
  const SpaceHeaderSection({super.key, required this.space});

  final SpaceModel space;

  @override
  Widget build(BuildContext context) {
    final rewards = space.rewards;
    final hasRewards = rewards > 0;

    return Wrap(
      spacing: 10,
      runSpacing: 7,
      children: [
        _BadgePrimary(
          icon: SvgPicture.asset(Assets.palace, width: 14, height: 14),
          label: 'Space',
        ),
        if (hasRewards)
          _BadgePrimary(
            icon: SvgPicture.asset(Assets.reward, width: 14, height: 14),
            label: 'Rewards',
          ),
      ],
    );
  }
}

class _BadgePrimary extends StatelessWidget {
  const _BadgePrimary({required this.label, this.icon});

  final String label;
  final SvgPicture? icon;

  @override
  Widget build(BuildContext context) {
    return Container(
      height: 25,
      padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 5.5),
      decoration: BoxDecoration(
        color: AppColors.primary,
        borderRadius: BorderRadius.circular(4),
      ),
      child: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          if (icon != null) ...[icon!, 4.gap],
          Text(
            label,
            style: const TextStyle(
              fontWeight: FontWeight.w600,
              fontSize: 12,
              height: 16 / 12,
              color: Color(0xFF262626),
            ),
          ),
        ],
      ),
    );
  }
}
