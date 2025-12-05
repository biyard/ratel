import 'package:ratel/exports.dart';

class SpaceStatsSection extends StatelessWidget {
  const SpaceStatsSection({super.key, required this.space});

  final SpaceModel space;

  @override
  Widget build(BuildContext context) {
    final likes = space.likes;
    final comments = space.comments;

    return Container(
      padding: EdgeInsets.zero,
      decoration: BoxDecoration(
        border: Border.all(color: Colors.transparent, width: 0),
      ),
      child: Row(
        mainAxisAlignment: MainAxisAlignment.start,
        children: [
          _StatItem(
            icon: SvgPicture.asset(
              Assets.thumbs,
              width: 20,
              height: 20,
              colorFilter: ColorFilter.mode(
                const Color(0xFF737373),
                BlendMode.srcIn,
              ),
            ),
            label: likes.toString(),
          ),
          20.gap,
          _StatItem(
            icon: SvgPicture.asset(
              Assets.roundBubble,
              width: 20,
              height: 20,
              colorFilter: const ColorFilter.mode(
                Color(0xFF737373),
                BlendMode.srcIn,
              ),
            ),
            label: comments.toString(),
          ),
        ],
      ),
    );
  }
}

class _StatItem extends StatelessWidget {
  const _StatItem({required this.icon, required this.label});

  final SvgPicture icon;
  final String label;

  @override
  Widget build(BuildContext context) {
    return Row(
      children: [
        icon,
        5.gap,
        Text(
          label,
          style: const TextStyle(
            fontFamily: 'Inter',
            fontWeight: FontWeight.w600,
            fontSize: 14,
            height: 20 / 14,
            color: Colors.white,
          ),
        ),
      ],
    );
  }
}
