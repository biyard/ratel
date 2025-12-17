import 'package:ratel/exports.dart';
import 'package:ratel/features/post/screens/detail/components/comment_more_bottom_sheet.dart';

class CommentItem extends StatelessWidget {
  const CommentItem({
    super.key,
    required this.comment,
    required this.isLikingCommentOf,
    required this.isCommentLiked,
    required this.onToggleLikeComment,
    this.isReported = false,
    this.onReport,
  });

  final PostCommentModel comment;

  final bool Function(String commentSk) isLikingCommentOf;
  final bool Function(String commentSk, {bool fallback}) isCommentLiked;
  final Future<void> Function(String commentSk) onToggleLikeComment;

  final bool isReported;
  final Future<void> Function(String commentSk)? onReport;

  String _plainContent(String raw) {
    final noTags = raw.replaceAll(RegExp(r'<[^>]*>'), '');
    return noTags.trim();
  }

  Future<void> _openMoreSheet(BuildContext context) async {
    if (isReported || onReport == null) return;

    await showModalBottomSheet(
      context: context,
      backgroundColor: const Color(0xFF191919),
      shape: const RoundedRectangleBorder(
        borderRadius: BorderRadius.vertical(top: Radius.circular(20)),
      ),
      builder: (_) {
        return CommentMoreBottomSheet(
          onReport: () async {
            Navigator.pop(context);
            await onReport!(comment.sk);
          },
        );
      },
    );
  }

  @override
  Widget build(BuildContext context) {
    return Obx(() {
      final textTheme = Theme.of(context).textTheme;
      final content = _plainContent(comment.content);
      final time = fromTimestampToDate(comment.updatedAt);
      final timeText = formatRelativeTime(time);

      final isLiking = isLikingCommentOf(comment.sk);
      final liked = isCommentLiked(comment.sk, fallback: comment.liked == true);

      return Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            crossAxisAlignment: CrossAxisAlignment.center,
            children: [
              Expanded(
                child: Profile(
                  profileImageUrl: comment.authorProfileUrl.isNotEmpty
                      ? comment.authorProfileUrl
                      : defaultProfileImage,
                  displayName: comment.authorDisplayName,
                ),
              ),
              const Spacer(),
              Row(
                mainAxisSize: MainAxisSize.min,
                children: [
                  Text(
                    timeText,
                    style: textTheme.bodySmall?.copyWith(
                      fontSize: 12,
                      color: const Color(0xFF737373),
                    ),
                  ),
                  if (!isReported && onReport != null) ...[
                    8.gap,
                    GestureDetector(
                      onTap: () => _openMoreSheet(context),
                      child: SvgPicture.asset(
                        Assets.extra,
                        width: 20,
                        height: 20,
                      ),
                    ),
                  ],
                ],
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
