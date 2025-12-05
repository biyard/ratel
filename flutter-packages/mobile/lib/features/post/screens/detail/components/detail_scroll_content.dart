import 'package:flutter_html/flutter_html.dart';
import 'package:ratel/exports.dart';

class DetailScrollContent extends StatelessWidget {
  const DetailScrollContent({
    super.key,
    required this.post,
    required this.isLiked,
    required this.isLiking,
    required this.onToggleLike,
  });

  final PostDetailPostModel post;

  final bool isLiked;
  final bool isLiking;
  final VoidCallback onToggleLike;

  String normalizeHtmlColors(String html) {
    final fixed = html.replaceAllMapped(
      RegExp(r'color:\s*var\(--theme-text-color,\s*(#[0-9a-fA-F]{6})\s*\)'),
      (m) => 'color: ${m.group(1)}',
    );
    return fixed;
  }

  String _relativeTime(int millis) {
    final dt = DateTime.fromMillisecondsSinceEpoch(
      millis,
      isUtc: true,
    ).toLocal();
    final now = DateTime.now();
    final diff = now.difference(dt);

    if (diff.inDays >= 7) {
      final w = (diff.inDays / 7).floor();
      return '${w}w ago';
    }
    if (diff.inDays >= 1) {
      return '${diff.inDays}d ago';
    }
    if (diff.inHours >= 1) {
      return '${diff.inHours}h ago';
    }
    if (diff.inMinutes >= 1) {
      return '${diff.inMinutes}m ago';
    }
    return 'now';
  }

  @override
  Widget build(BuildContext context) {
    final fixedHtml = normalizeHtmlColors(post.htmlContents);
    logger.d('htmlContents = $fixedHtml');

    return Padding(
      padding: const EdgeInsets.symmetric(horizontal: 15),
      child: SingleChildScrollView(
        padding: const EdgeInsets.only(bottom: 40),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            _DetailHeaderSection(
              post: post,
              relativeTime: _relativeTime(post.createdAt),
              isLiked: isLiked,
              isLiking: isLiking,
              onToggleLike: onToggleLike,
            ),
            20.vgap,
            Html(
              data: fixedHtml,
              style: {
                'p': Style(
                  fontSize: FontSize(15),
                  lineHeight: LineHeight(1.6),
                  fontFamily: 'Raleway',
                  color: Colors.white,
                ),
              },
            ),
            40.vgap,
          ],
        ),
      ),
    );
  }
}

class _DetailHeaderSection extends StatelessWidget {
  const _DetailHeaderSection({
    required this.post,
    required this.relativeTime,
    required this.isLiked,
    required this.isLiking,
    required this.onToggleLike,
  });

  final PostDetailPostModel post;
  final String relativeTime;

  final bool isLiked;
  final bool isLiking;
  final VoidCallback onToggleLike;

  @override
  Widget build(BuildContext context) {
    final textTheme = Theme.of(context).textTheme;

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text(
          post.title,
          style: textTheme.titleLarge?.copyWith(
            fontWeight: FontWeight.w700,
            fontSize: 24,
            height: 1.33,
            letterSpacing: 0.5,
            color: Colors.white,
          ),
        ),
        20.vgap,
        Row(
          children: [
            Profile(
              profileImageUrl: post.authorProfileUrl.isNotEmpty
                  ? post.authorProfileUrl
                  : defaultProfileImage,
              displayName: post.authorDisplayName,
            ),
            const Spacer(),
            Text(
              relativeTime,
              style: textTheme.bodySmall?.copyWith(
                fontSize: 12,
                color: const Color(0xFF737373),
              ),
            ),
          ],
        ),
        20.vgap,
        _DetailStatsRow(
          likes: post.likes,
          comments: post.comments,
          rewards: post.rewards ?? 0,
          reposts: post.shares,
          isLiked: isLiked,
          isLiking: isLiking,
          onToggleLike: onToggleLike,
        ),
      ],
    );
  }
}

class _DetailStatsRow extends StatelessWidget {
  const _DetailStatsRow({
    required this.likes,
    required this.comments,
    required this.rewards,
    required this.reposts,
    required this.isLiked,
    required this.isLiking,
    required this.onToggleLike,
  });

  final int likes;
  final int comments;
  final int rewards;
  final int reposts;

  final bool isLiked;
  final bool isLiking;
  final VoidCallback onToggleLike;

  @override
  Widget build(BuildContext context) {
    return Row(
      children: [
        _DetailStatItem(
          icon: SvgPicture.asset(
            Assets.thumbs,
            width: 20,
            height: 20,
            colorFilter: ColorFilter.mode(
              isLiked ? AppColors.primary : const Color(0xFF737373),
              BlendMode.srcIn,
            ),
          ),
          label: likes.toString(),
          onTap: isLiking ? null : onToggleLike,
        ),
        20.gap,
        _DetailStatItem(
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
    );
  }
}

class _DetailStatItem extends StatelessWidget {
  const _DetailStatItem({required this.icon, required this.label, this.onTap});

  final Widget icon;
  final String label;
  final VoidCallback? onTap;

  @override
  Widget build(BuildContext context) {
    final textTheme = Theme.of(context).textTheme;

    final content = Row(
      children: [
        icon,
        5.gap,
        Text(
          label,
          style: textTheme.bodySmall?.copyWith(
            fontSize: 14,
            fontWeight: FontWeight.w600,
            color: Colors.white,
          ),
        ),
      ],
    );

    if (onTap == null) {
      return content;
    }

    return GestureDetector(
      behavior: HitTestBehavior.opaque,
      onTap: onTap,
      child: content,
    );
  }
}
