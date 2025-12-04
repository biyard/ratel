import 'package:ratel/exports.dart';

class CommentItem extends StatelessWidget {
  const CommentItem({
    super.key,
    required this.comment,
    required this.isLikingCommentOf,
    required this.isCommentLiked,
    required this.onToggleLikeComment,
  });

  final PostCommentModel comment;

  final bool Function(String commentSk) isLikingCommentOf;
  final bool Function(String commentSk, {bool fallback}) isCommentLiked;
  final Future<void> Function(String commentSk) onToggleLikeComment;

  String _relativeTime(int secs) {
    final dt = DateTime.fromMillisecondsSinceEpoch(
      secs * 1000,
      isUtc: true,
    ).toLocal();
    final now = DateTime.now();
    final diff = now.difference(dt);

    if (diff.inDays >= 7) {
      final w = (diff.inDays / 7).floor();
      return '${w}w ago';
    }
    if (diff.inDays >= 1) return '${diff.inDays}d ago';
    if (diff.inHours >= 1) return '${diff.inHours}h ago';
    if (diff.inMinutes >= 1) return '${diff.inMinutes}m ago';
    return 'now';
  }

  String _plainContent(String raw) {
    final noTags = raw.replaceAll(RegExp(r'<[^>]*>'), '');
    return noTags.trim();
  }

  @override
  Widget build(BuildContext context) {
    return Obx(() {
      final textTheme = Theme.of(context).textTheme;
      final content = _plainContent(comment.content);
      final timeText = _relativeTime(comment.updatedAt);

      final isLiking = isLikingCommentOf(comment.sk);
      final liked = isCommentLiked(comment.sk, fallback: comment.liked == true);

      return Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            crossAxisAlignment: CrossAxisAlignment.center,
            children: [
              RoundContainer(
                width: 24,
                height: 24,
                radius: 118.5,
                imageUrl: comment.authorProfileUrl.isNotEmpty
                    ? comment.authorProfileUrl
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
                          comment.authorDisplayName,
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
                      timeText,
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
          4.vgap,
          Text(
            content,
            style: textTheme.bodyMedium?.copyWith(
              fontSize: 15,
              height: 24 / 15,
              letterSpacing: 0.5,
              color: Colors.white,
            ),
          ),
          10.vgap,
          Row(
            children: [
              GestureDetector(
                behavior: HitTestBehavior.opaque,
                onTap: isLiking ? null : () => onToggleLikeComment(comment.sk),
                child: Row(
                  children: [
                    if (isLiking)
                      const SizedBox(
                        width: 16,
                        height: 16,
                        child: CircularProgressIndicator(strokeWidth: 2),
                      )
                    else
                      SvgPicture.asset(
                        Assets.thumbs,
                        width: 20,
                        height: 20,
                        colorFilter: ColorFilter.mode(
                          liked ? AppColors.primary : const Color(0xFF737373),
                          BlendMode.srcIn,
                        ),
                      ),
                    5.gap,
                    Text(
                      comment.likes.toString(),
                      style: textTheme.bodySmall?.copyWith(
                        fontSize: 15,
                        color: Colors.white,
                      ),
                    ),
                  ],
                ),
              ),
            ],
          ),
        ],
      );
    });
  }
}
