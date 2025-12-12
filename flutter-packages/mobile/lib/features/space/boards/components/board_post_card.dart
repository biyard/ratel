import 'package:flutter_html/flutter_html.dart';
import 'package:ratel/exports.dart';

class BoardPostCard extends StatelessWidget {
  final SpacePostModel post;
  final VoidCallback? onTap;

  const BoardPostCard({super.key, required this.post, this.onTap});

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final previewUrl = _resolvePreviewImageUrl(post);
    final createdAt = DateTime.fromMillisecondsSinceEpoch(
      post.createdAt,
      isUtc: false,
    );
    final relative = _formatRelativeTime(createdAt);
    final profileImageUrl = (post.authorProfileUrl.isNotEmpty
        ? post.authorProfileUrl
        : defaultProfileImage);

    return GestureDetector(
      behavior: HitTestBehavior.opaque,
      onTap: onTap,
      child: Container(
        margin: const EdgeInsets.symmetric(vertical: 5),
        padding: const EdgeInsets.symmetric(vertical: 20),
        decoration: BoxDecoration(
          color: const Color(0xff191919),
          borderRadius: BorderRadius.circular(10),
        ),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Padding(
              padding: const EdgeInsets.symmetric(horizontal: 20),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Row(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Expanded(
                        child: Text(
                          post.title,
                          style: theme.textTheme.titleMedium?.copyWith(
                            fontWeight: FontWeight.w600,
                            color: theme.colorScheme.onSurface,
                          ),
                          maxLines: 2,
                          overflow: TextOverflow.ellipsis,
                        ),
                      ),
                      10.gap,
                      Wrap(
                        spacing: 6,
                        runSpacing: 4,
                        children: [
                          _SmallTag(
                            label: '${post.numberOfComments} Response',
                            bgColor: AppColors.neutral800,
                            fgColor: AppColors.neutral300,
                          ),
                          if (post.categoryName.isNotEmpty)
                            _SmallTag(
                              label: post.categoryName,
                              bgColor: AppColors.neutral800,
                              fgColor: AppColors.neutral300,
                            ),
                        ],
                      ),
                    ],
                  ),
                ],
              ),
            ),
            if (post.htmlContents.isNotEmpty || previewUrl != null) ...[
              10.vgap,
              Padding(
                padding: const EdgeInsets.symmetric(horizontal: 20),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    if (post.htmlContents.isNotEmpty)
                      Html(
                        data: post.htmlContents,
                        style: {
                          '*': Style.fromTextStyle(
                            theme.textTheme.bodyMedium!.copyWith(
                              color: AppColors.neutral300,
                              height: 1.4,
                            ),
                          ),
                        },
                      ),
                    if (previewUrl != null) ...[
                      10.vgap,
                      ClipRRect(
                        borderRadius: BorderRadius.circular(10),
                        child: AspectRatio(
                          aspectRatio: 16 / 9,
                          child: Image.network(previewUrl, fit: BoxFit.cover),
                        ),
                      ),
                    ],
                  ],
                ),
              ),
            ],
            10.vgap,
            Padding(
              padding: const EdgeInsets.symmetric(horizontal: 20),
              child: Row(
                children: [
                  Expanded(
                    child: Profile(
                      profileImageUrl: profileImageUrl,
                      displayName: post.authorDisplayName,
                    ),
                  ),
                  const Spacer(),
                  Text(
                    relative,
                    style: theme.textTheme.bodySmall?.copyWith(
                      color: AppColors.neutral400,
                    ),
                  ),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }

  String? _resolvePreviewImageUrl(SpacePostModel post) {
    for (final url in post.urls) {
      if (url.isNotEmpty) {
        return url;
      }
    }
    return null;
  }

  String _formatRelativeTime(DateTime time) {
    final now = DateTime.now();
    final diff = now.difference(time);

    if (diff.inDays >= 7) return '${(diff.inDays / 7).floor()}w ago';
    if (diff.inDays >= 1) return '${diff.inDays}d ago';
    if (diff.inHours >= 1) return '${diff.inHours}h ago';
    if (diff.inMinutes >= 1) return '${diff.inMinutes}m ago';
    return 'now';
  }
}

class _SmallTag extends StatelessWidget {
  final String label;
  final Color bgColor;
  final Color fgColor;

  const _SmallTag({
    required this.label,
    required this.bgColor,
    required this.fgColor,
  });

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);

    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 3),
      decoration: BoxDecoration(
        color: bgColor,
        borderRadius: BorderRadius.circular(6),
        border: Border.all(color: fgColor.withOpacity(0.25), width: 1),
      ),
      child: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          Text(
            label,
            style: theme.textTheme.labelSmall?.copyWith(
              color: fgColor,
              fontWeight: FontWeight.w500,
            ),
          ),
        ],
      ),
    );
  }
}
