import 'package:ratel/exports.dart';

class BoardCommentItem extends StatelessWidget {
  final UserV2Model user;
  final SpacePostCommentModel comment;
  final VoidCallback? onLikeTap;
  final VoidCallback? onMoreTap;

  final bool isEditing;
  final TextEditingController? editingController;
  final VoidCallback? onSaveEdit;
  final VoidCallback? onCancelEdit;

  const BoardCommentItem({
    super.key,
    required this.user,
    required this.comment,
    this.onLikeTap,
    this.onMoreTap,
    this.isEditing = false,
    this.editingController,
    this.onSaveEdit,
    this.onCancelEdit,
  });

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);

    final createdAt = _fromTimestamp(comment.createdAt);
    final relative = _formatRelativeTime(createdAt);

    final profileImageUrl = comment.authorProfileUrl.isNotEmpty
        ? comment.authorProfileUrl
        : defaultProfileImage;

    final plainContent = _stripHtml(comment.content).trim();

    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 10),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            children: [
              RoundContainer(
                width: 24,
                height: 24,
                radius: 100,
                color: AppColors.neutral600,
                child: ClipRRect(
                  borderRadius: BorderRadius.circular(100),
                  child: Image.network(profileImageUrl, fit: BoxFit.cover),
                ),
              ),
              10.gap,
              Expanded(
                child: Row(
                  children: [
                    Text(
                      comment.authorDisplayName,
                      style: const TextStyle(
                        fontFamily: 'Raleway',
                        fontSize: 16,
                        fontWeight: FontWeight.w500,
                        letterSpacing: 0.5,
                        color: Colors.white,
                      ),
                      overflow: TextOverflow.ellipsis,
                    ),
                    4.gap,
                    SvgPicture.asset(Assets.badge, width: 16, height: 16),
                    const Spacer(),
                    Text(
                      relative,
                      style: theme.textTheme.bodySmall?.copyWith(
                        fontFamily: 'Inter',
                        fontSize: 12,
                        fontWeight: FontWeight.w500,
                        color: AppColors.neutral500,
                      ),
                    ),
                    10.gap,
                    if (!isEditing && comment.authorPk == user.pk)
                      InkWell(
                        onTap: onMoreTap,
                        child: SvgPicture.asset(
                          Assets.extra,
                          width: 24,
                          height: 24,
                        ),
                      ),
                  ],
                ),
              ),
            ],
          ),
          6.vgap,
          Padding(
            padding: const EdgeInsets.only(left: 34, right: 4),
            child: isEditing && editingController != null
                ? Row(
                    crossAxisAlignment: CrossAxisAlignment.center,
                    children: [
                      Expanded(
                        child: TextField(
                          controller: editingController,
                          maxLines: null,
                          style: const TextStyle(
                            fontFamily: 'Raleway',
                            fontSize: 15,
                            fontWeight: FontWeight.w400,
                            height: 24 / 15,
                            letterSpacing: 0.5,
                            color: Colors.white,
                          ),
                          decoration: const InputDecoration(
                            isDense: true,
                            border: InputBorder.none,
                          ),
                          textInputAction: TextInputAction.done,
                          onSubmitted: (_) {
                            FocusScope.of(context).unfocus();
                          },
                        ),
                      ),
                      4.gap,
                      IconButton(
                        icon: const Icon(Icons.check, color: Colors.green),
                        onPressed: onSaveEdit,
                      ),
                      IconButton(
                        icon: const Icon(Icons.close, color: Colors.red),
                        onPressed: onCancelEdit,
                      ),
                    ],
                  )
                : Row(
                    mainAxisAlignment: MainAxisAlignment.start,
                    crossAxisAlignment: CrossAxisAlignment.center,
                    children: [
                      Text(
                        plainContent.isEmpty ? comment.content : plainContent,
                        style: const TextStyle(
                          fontFamily: 'Raleway',
                          fontSize: 15,
                          fontWeight: FontWeight.w400,
                          height: 24 / 15,
                          letterSpacing: 0.5,
                          color: Colors.white,
                        ),
                      ),
                      if (comment.createdAt != comment.updatedAt) ...[
                        4.gap,
                        Text(
                          "(Updated)",
                          style: const TextStyle(
                            fontFamily: 'Raleway',
                            fontSize: 11,
                            fontWeight: FontWeight.w300,
                            color: AppColors.neutral500,
                          ),
                        ),
                      ],
                    ],
                  ),
          ),
          8.vgap,
          Padding(
            padding: const EdgeInsets.only(left: 34),
            child: Row(
              children: [
                GestureDetector(
                  onTap: isEditing ? null : onLikeTap,
                  behavior: HitTestBehavior.opaque,
                  child: Row(
                    children: [
                      SvgPicture.asset(
                        Assets.thumbs,
                        width: 20,
                        height: 20,
                        colorFilter: ColorFilter.mode(
                          comment.liked
                              ? AppColors.primary
                              : const Color(0xFF737373),
                          BlendMode.srcIn,
                        ),
                      ),
                      4.gap,
                      Text(
                        comment.likes.toString(),
                        style: const TextStyle(
                          fontFamily: 'Raleway',
                          fontSize: 15,
                          fontWeight: FontWeight.w400,
                          color: Colors.white,
                        ),
                      ),
                    ],
                  ),
                ),
                // 20.gap,
                // Row(
                //   children: [
                //     const SizedBox(
                //       width: 20,
                //       height: 20,
                //       child: Icon(
                //         Icons.chat_bubble_outline,
                //         size: 16,
                //         color: Color(0xFF737373),
                //       ),
                //     ),
                //     4.gap,
                //     Text(
                //       comment.replies.toString(),
                //       style: const TextStyle(
                //         fontFamily: 'Raleway',
                //         fontSize: 15,
                //         fontWeight: FontWeight.w400,
                //         color: Colors.white,
                //       ),
                //     ),
                //   ],
                // ),
              ],
            ),
          ),
        ],
      ),
    );
  }
}

String _stripHtml(String text) {
  final brReg = RegExp(r'<br\s*/?>', caseSensitive: false);
  final withoutBr = text.replaceAll(brReg, '\n');
  final tagReg = RegExp(r'<[^>]+>', multiLine: true, caseSensitive: false);
  return withoutBr.replaceAll(tagReg, '');
}

DateTime _fromTimestamp(int ts) {
  if (ts < 1000000000000) {
    return DateTime.fromMillisecondsSinceEpoch(
      ts * 1000,
      isUtc: true,
    ).toLocal();
  } else {
    return DateTime.fromMillisecondsSinceEpoch(ts, isUtc: true).toLocal();
  }
}

String _formatRelativeTime(DateTime time) {
  final now = DateTime.now();
  final diff = now.difference(time);

  if (diff.inMinutes < 1) return 'now';
  if (diff.inMinutes < 60) return '${diff.inMinutes}m ago';
  if (diff.inHours < 24) return '${diff.inHours}h ago';
  if (diff.inDays < 7) return '${diff.inDays}d ago';
  final weeks = (diff.inDays / 7).floor();
  return '${weeks}w ago';
}
