import 'package:flutter_html/flutter_html.dart';
import 'package:ratel/exports.dart';

class DetailScrollContent extends StatelessWidget {
  const DetailScrollContent({super.key, required this.post});

  final PostDetailPostModel post;

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
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          _DetailHeaderSection(
            post: post,
            relativeTime: _relativeTime(post.createdAt),
          ),
          20.vgap,
          Expanded(
            child: SingleChildScrollView(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
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
          ),
        ],
      ),
    );
  }
}

class _DetailHeaderSection extends StatelessWidget {
  const _DetailHeaderSection({required this.post, required this.relativeTime});

  final PostDetailPostModel post;
  final String relativeTime;

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
            RoundContainer(
              width: 24,
              height: 24,
              radius: 118.5,
              imageUrl: post.authorProfileUrl.isNotEmpty
                  ? post.authorProfileUrl
                  : defaultProfileImage,
              color: null,
              alignment: Alignment.center,
              child: null,
            ),
            10.gap,
            Expanded(
              child: Row(
                children: [
                  Row(
                    children: [
                      Text(
                        post.authorDisplayName,
                        style: textTheme.bodyMedium?.copyWith(
                          fontWeight: FontWeight.w500,
                          fontSize: 16,
                          height: 24 / 16,
                          letterSpacing: 0.5,
                          color: Colors.white,
                        ),
                      ),
                      4.gap,
                      SvgPicture.asset(Assets.badge, width: 20, height: 20),
                    ],
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
            ),
          ],
        ),
        20.vgap,
        _DetailStatsRow(
          likes: post.likes,
          comments: post.comments,
          rewards: post.rewards ?? 0,
          reposts: post.shares,
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
  });

  final int likes;
  final int comments;
  final int rewards;
  final int reposts;

  String _formatNumber(int v) {
    if (v >= 10000) {
      final k = (v / 1000).floor();
      return '${k}K';
    }
    return v.toString();
  }

  @override
  Widget build(BuildContext context) {
    return Row(
      children: [
        _DetailStatItem(
          icon: SvgPicture.asset(Assets.thumbs, width: 20, height: 20),
          label: likes.toString(),
        ),
        20.gap,
        _DetailStatItem(
          icon: SvgPicture.asset(Assets.roundBubble, width: 20, height: 20),
          label: comments.toString(),
        ),
        20.gap,
        _DetailStatItem(
          icon: SvgPicture.asset(Assets.reward, width: 20, height: 20),
          label: _formatNumber(rewards),
        ),
      ],
    );
  }
}

class _DetailStatItem extends StatelessWidget {
  const _DetailStatItem({required this.icon, required this.label});

  final SvgPicture icon;
  final String label;

  @override
  Widget build(BuildContext context) {
    final textTheme = Theme.of(context).textTheme;

    return Row(
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
  }
}
