import 'package:ratel/exports.dart';

class HomeScreen extends GetWidget<HomeController> {
  const HomeScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Layout<HomeController>(
      child: Padding(
        padding: const EdgeInsets.symmetric(horizontal: 14, vertical: 10),
        child: ListView.separated(
          primary: false,
          shrinkWrap: true,
          itemCount: controller.feeds.length,
          separatorBuilder: (_, __) => const SizedBox(height: 14),
          itemBuilder: (_, i) => _FeedCard(data: controller.feeds[i]),
        ),
      ),
    );
  }
}

class _FeedCard extends StatelessWidget {
  const _FeedCard({required this.data});
  final FeedData data;

  @override
  Widget build(BuildContext context) {
    final hasSpace = data.spaceIds.isNotEmpty;
    final hasRewards = data.rewards != null;

    return Container(
      decoration: BoxDecoration(
        color: AppColors.bg,
        borderRadius: BorderRadius.circular(14),
        border: Border.all(color: AppColors.neutral700),
      ),
      child: Padding(
        padding: const EdgeInsets.fromLTRB(10, 20, 10, 10),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                if (hasSpace)
                  _Badge(
                    icon: SvgPicture.asset(
                      Assets.palace,
                      width: 14,
                      height: 14,
                    ),
                    label: 'SPACE',
                    fg: AppColors.neutral800,
                    bg: AppColors.primary,
                  ),
                if (hasSpace) 10.gap,
                if (hasRewards)
                  _Badge(
                    icon: SvgPicture.asset(Assets.coin, width: 14, height: 14),
                    label: 'REWARDS',
                    fg: AppColors.neutral800,
                    bg: AppColors.primary,
                  ),
                if (hasRewards) 10.gap,
                _Badge(
                  icon: null,
                  label: data.feedType,
                  fg: Colors.white,
                  bg: Colors.transparent,
                  border: AppColors.neutral700,
                ),
                const Spacer(),
                SvgPicture.asset(Assets.bookmark, width: 20, height: 20),
                10.gap,
                SvgPicture.asset(Assets.edit1, width: 20, height: 20),
                10.gap,
                SvgPicture.asset(Assets.extra, width: 20, height: 20),
              ],
            ),
            10.vgap,
            Row(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                _thumb(""),
                12.gap,
                Flexible(
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text(
                        data.title,
                        maxLines: 1,
                        overflow: TextOverflow.ellipsis,
                        style: const TextStyle(
                          color: Colors.white,
                          fontSize: 14,
                          fontWeight: FontWeight.w600,
                          height: 1.3,
                        ),
                      ),
                      10.vgap,
                      Row(
                        children: [
                          _avatar(data.authorUrl),
                          10.gap,
                          const Text(
                            'Author',
                            style: TextStyle(
                              color: Colors.white,
                              fontWeight: FontWeight.w700,
                              fontSize: 12,
                              height: 1.3,
                            ),
                          ),
                          10.gap,
                          SvgPicture.asset(Assets.badge, width: 20, height: 20),
                          const Spacer(),
                          Text(
                            _timeAgo(data.createdAt),
                            style: const TextStyle(
                              color: Colors.white,
                              fontWeight: FontWeight.w400,
                              fontSize: 12,
                              height: 1.3,
                            ),
                          ),
                        ],
                      ),
                      10.vgap,
                      Text(
                        data.description,
                        maxLines: 1,
                        overflow: TextOverflow.ellipsis,
                        style: const TextStyle(
                          color: Colors.white,
                          fontWeight: FontWeight.w400,
                          fontSize: 12,
                          height: 1.3,
                        ),
                      ),
                    ],
                  ),
                ),
              ],
            ),
            10.vgap,
            Container(
              height: 1,
              width: double.infinity,
              color: AppColors.neutral800,
            ),
            8.vgap,
            Row(
              mainAxisAlignment: MainAxisAlignment.center,
              crossAxisAlignment: CrossAxisAlignment.center,
              children: [
                _Stat(
                  icon: SvgPicture.asset(Assets.thumbs, width: 13, height: 13),
                  label: '${data.likes}',
                ),
                10.gap,
                _Stat(
                  icon: SvgPicture.asset(Assets.chat, width: 13, height: 13),
                  label: '${data.comments}',
                ),
                if (data.rewards != null) ...[
                  10.gap,
                  _Stat(
                    icon: SvgPicture.asset(
                      Assets.reward,
                      width: 13,
                      height: 13,
                    ),
                    label: '${data.rewards}',
                  ),
                ],
                10.gap,
                _Stat(
                  icon: SvgPicture.asset(Assets.feed, width: 13, height: 13),
                  label: '${data.reposts}',
                ),
              ],
            ),
          ],
        ),
      ),
    );
  }

  Widget _thumb(String url) {
    const size = 78.0;
    if (url.isEmpty) {
      return Container(
        width: size,
        height: size,
        decoration: BoxDecoration(
          color: AppColors.neutral700,
          borderRadius: BorderRadius.circular(10),
        ),
        child: const Icon(Icons.image, color: AppColors.neutral400),
      );
    }
    return ClipRRect(
      borderRadius: BorderRadius.circular(10),
      child: Image.network(url, width: size, height: size, fit: BoxFit.cover),
    );
  }

  Widget _avatar(String url) {
    const s = 22.0;
    if (url.isEmpty) {
      return Container(
        width: s,
        height: s,
        decoration: const BoxDecoration(
          color: AppColors.neutral600,
          shape: BoxShape.circle,
        ),
      );
    }
    return ClipOval(
      child: Image.network(url, width: s, height: s, fit: BoxFit.cover),
    );
  }
}

class _Badge extends StatelessWidget {
  const _Badge({
    required this.icon,
    required this.label,
    required this.fg,
    required this.bg,
    this.border,
  });

  final SvgPicture? icon;
  final String label;
  final Color fg;
  final Color bg;
  final Color? border;

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 9, vertical: 6),
      decoration: BoxDecoration(
        color: bg,
        borderRadius: BorderRadius.circular(4),
        border: border == null ? null : Border.all(color: border!),
      ),
      child: Center(
        child: Row(
          mainAxisSize: MainAxisSize.min,
          children: [
            if (icon != null) ...[icon!, const SizedBox(width: 6)],
            Text(
              label,
              style: TextStyle(
                color: fg,
                fontSize: 11,
                fontWeight: FontWeight.w700,
              ),
            ),
          ],
        ),
      ),
    );
  }
}

class _Stat extends StatelessWidget {
  const _Stat({required this.icon, required this.label});
  final SvgPicture? icon;
  final String label;

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: 76,
      child: Center(
        child: Row(
          mainAxisAlignment: MainAxisAlignment.center,
          crossAxisAlignment: CrossAxisAlignment.center,
          children: [
            if (icon != null) ...[icon!, 5.gap],
            Text(
              label,
              style: const TextStyle(
                color: Colors.white,
                fontSize: 12,
                fontWeight: FontWeight.w400,
                height: 1.3,
              ),
            ),
          ],
        ),
      ),
    );
  }
}

String _timeAgo(int epochSec) {
  final now = DateTime.now();
  final dt = DateTime.fromMillisecondsSinceEpoch(epochSec * 1000);
  final diff = now.difference(dt);
  if (diff.inDays >= 7) return '${(diff.inDays / 7).floor()}w ago';
  if (diff.inDays >= 1) return '${diff.inDays}d ago';
  if (diff.inHours >= 1) return '${diff.inHours}h ago';
  if (diff.inMinutes >= 1) return '${diff.inMinutes}m ago';
  return 'now';
}
