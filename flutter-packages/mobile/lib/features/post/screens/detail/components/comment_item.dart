import 'package:ratel/exports.dart';
import 'package:ratel/features/post/screens/detail/components/reply_item.dart';

class CommentItem extends StatelessWidget {
  const CommentItem({
    super.key,
    required this.comment,
    required this.onReply,
    required this.repliesOf,
    required this.isRepliesLoadingOf,
    required this.isLikingCommentOf,
    required this.isCommentLiked,
    required this.onToggleLikeComment,
  });

  final PostCommentModel comment;
  final void Function(PostCommentModel) onReply;

  final List<PostCommentModel> Function(String commentSk) repliesOf;
  final bool Function(String commentSk) isRepliesLoadingOf;

  final bool Function(String commentSk) isLikingCommentOf;
  final bool Function(String commentSk, {bool fallback}) isCommentLiked;
  final Future<void> Function(String commentSk) onToggleLikeComment;

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

  String _plainContent(String raw) {
    final noTags = raw.replaceAll(RegExp(r'<[^>]*>'), '');
    return noTags.trim();
  }

  @override
  Widget build(BuildContext context) {
    final textTheme = Theme.of(context).textTheme;
    final content = _plainContent(comment.content);
    final timeText = _relativeTime(comment.updatedAt);

    final replies = repliesOf(comment.sk);
    final isRepliesLoading = isRepliesLoadingOf(comment.sk);
    final isLiking = isLikingCommentOf(comment.sk);
    final liked = isCommentLiked(comment.sk, fallback: comment.liked == true);

    final child = Column(
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
            20.gap,
            Row(
              children: [
                SvgPicture.asset(
                  Assets.roundBubble,
                  width: 20,
                  height: 20,
                  colorFilter: const ColorFilter.mode(
                    Color(0xFF737373),
                    BlendMode.srcIn,
                  ),
                ),
                5.gap,
                Text(
                  comment.replies.toString(),
                  style: textTheme.bodySmall?.copyWith(
                    fontSize: 15,
                    color: Colors.white,
                  ),
                ),
              ],
            ),
          ],
        ),
        if (isRepliesLoading) ...[
          12.vgap,
          const SizedBox(
            width: 16,
            height: 16,
            child: CircularProgressIndicator(strokeWidth: 2),
          ),
        ] else if (replies.isNotEmpty) ...[
          12.vgap,
          Column(
            children: replies
                .map(
                  (r) => Padding(
                    padding: const EdgeInsets.only(left: 34),
                    child: ReplyItem(
                      comment: r,
                      isCommentLiked: isCommentLiked,
                      isLikingCommentOf: isLikingCommentOf,
                      onToggleLikeComment: onToggleLikeComment,
                    ),
                  ),
                )
                .toList(),
          ),
        ],
      ],
    );

    return Dismissible(
      key: ValueKey('comment-${comment.sk}'),
      direction: DismissDirection.endToStart,
      confirmDismiss: (direction) async {
        onReply(comment);
        return false;
      },
      background: Align(
        alignment: Alignment.centerRight,
        child: Container(
          width: 80,
          decoration: const BoxDecoration(
            color: Color(0xFF2563EB),
            borderRadius: BorderRadius.only(
              topRight: Radius.circular(8),
              bottomRight: Radius.circular(8),
            ),
          ),
          child: const Center(
            child: Icon(
              Icons.subdirectory_arrow_left_rounded,
              color: Colors.white,
              size: 24,
            ),
          ),
        ),
      ),
      child: child,
    );
  }
}
