import 'package:ratel/exports.dart';

class ReplyItem extends StatelessWidget {
  const ReplyItem({
    super.key,
    required this.comment,
    required this.isCommentLiked,
    required this.isLikingCommentOf,
    required this.onToggleLikeComment,
  });

  final PostCommentModel comment;
  final bool Function(String commentSk, {bool fallback}) isCommentLiked;
  final bool Function(String commentSk) isLikingCommentOf;
  final Future<void> Function(String commentSk) onToggleLikeComment;

  String _plainContent(String raw) {
    final noTags = raw.replaceAll(RegExp(r'<[^>]*>'), '');
    return noTags.trim();
  }

  String _relativeTime(int millis) {
    final dt = DateTime.fromMillisecondsSinceEpoch(
      millis * 1000,
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

  @override
  Widget build(BuildContext context) {
    final textTheme = Theme.of(context).textTheme;
    final content = _plainContent(comment.content);
    final timeText = _relativeTime(comment.updatedAt);

    final isLiking = isLikingCommentOf(comment.sk);
    final liked = isCommentLiked(comment.sk, fallback: comment.liked == true);

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Row(
          children: [
            RoundContainer(
              width: 20,
              height: 20,
              radius: 118.5,
              imageUrl: comment.authorProfileUrl.isNotEmpty
                  ? comment.authorProfileUrl
                  : defaultProfileImage,
              color: null,
              alignment: Alignment.center,
              child: null,
            ),
            8.gap,
            Expanded(
              child: Row(
                children: [
                  Text(
                    comment.authorDisplayName,
                    style: textTheme.bodySmall?.copyWith(
                      fontWeight: FontWeight.w500,
                      fontSize: 14,
                      color: Colors.white,
                      letterSpacing: 0.5,
                    ),
                  ),
                  4.gap,
                  SvgPicture.asset(Assets.badge, width: 16, height: 16),
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
          style: textTheme.bodySmall?.copyWith(
            fontSize: 14,
            height: 20 / 14,
            letterSpacing: 0.5,
            color: Colors.white,
          ),
        ),
        8.vgap,
        Row(
          children: [
            GestureDetector(
              behavior: HitTestBehavior.opaque,
              onTap: isLiking ? null : () => onToggleLikeComment(comment.sk),
              child: Row(
                children: [
                  if (isLiking)
                    const SizedBox(
                      width: 14,
                      height: 14,
                      child: CircularProgressIndicator(strokeWidth: 2),
                    )
                  else
                    SvgPicture.asset(
                      Assets.thumbs,
                      width: 18,
                      height: 18,
                      colorFilter: ColorFilter.mode(
                        liked ? AppColors.primary : const Color(0xFF737373),
                        BlendMode.srcIn,
                      ),
                    ),
                  4.gap,
                  Text(
                    comment.likes.toString(),
                    style: textTheme.bodySmall?.copyWith(
                      fontSize: 13,
                      color: Colors.white,
                    ),
                  ),
                ],
              ),
            ),
          ],
        ),
        8.vgap,
      ],
    );
  }
}
