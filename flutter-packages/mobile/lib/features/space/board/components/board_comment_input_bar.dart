import 'package:ratel/exports.dart';
import 'package:ratel/features/space/board/components/board_comment_sheet.dart';

typedef CommentLikeTap<T> = void Function(T comment);
typedef CommentEditTap<T> = Future<void> Function(T comment, String newContent);
typedef CommentDeleteTap<T> = Future<void> Function(T comment);

Future<void> showBoardCommentsBottomSheet({
  required UserV2Model user,
  required BuildContext context,
  required List<SpacePostCommentModel> comments,
  required bool isLoading,
  required bool hasMore,
  required bool isLoadingMore,
  required Future<void> Function(String text) onSend,
  CommentLikeTap<SpacePostCommentModel>? onLikeTap,
  CommentEditTap<SpacePostCommentModel>? onEdit,
  CommentDeleteTap<SpacePostCommentModel>? onDelete,
  Future<bool> Function()? onLoadMore,
  bool canComment = true,
}) {
  return showModalBottomSheet(
    context: context,
    isScrollControlled: true,
    backgroundColor: Colors.transparent,
    builder: (sheetContext) {
      return DraggableScrollableSheet(
        initialChildSize: 0.7,
        minChildSize: 0.4,
        maxChildSize: 0.95,
        expand: false,
        builder: (context, scrollController) {
          return BoardCommentsSheet(
            user: user,
            comments: comments,
            isLoading: isLoading,
            hasMore: hasMore,
            isLoadingMore: isLoadingMore,
            scrollController: scrollController,
            onSend: onSend,
            onLikeTap: onLikeTap,
            onEdit: onEdit,
            onDelete: onDelete,
            onLoadMore: onLoadMore,
            canComment: canComment,
          );
        },
      );
    },
  );
}

class BoardCommentInputBar extends StatelessWidget {
  final UserV2Model user;
  final Future<void> Function(String text) onSubmit;
  final List<SpacePostCommentModel> comments;
  final bool isLoading;
  final bool hasMore;
  final bool isLoadingMore;
  final bool canComment;
  final CommentLikeTap<SpacePostCommentModel>? onLikeTap;
  final CommentEditTap<SpacePostCommentModel>? onEdit;
  final CommentDeleteTap<SpacePostCommentModel>? onDelete;
  final Future<bool> Function()? onLoadMore;

  const BoardCommentInputBar({
    super.key,
    required this.user,
    required this.onSubmit,
    this.comments = const [],
    this.isLoading = false,
    this.hasMore = false,
    this.isLoadingMore = false,
    this.canComment = true,
    this.onLikeTap,
    this.onEdit,
    this.onDelete,
    this.onLoadMore,
  });

  @override
  Widget build(BuildContext context) {
    return SafeArea(
      top: false,
      child: Padding(
        padding: const EdgeInsets.fromLTRB(12, 4, 12, 8),
        child: SizedBox(
          height: 44,
          child: GestureDetector(
            behavior: HitTestBehavior.opaque,
            onTap: () {
              showBoardCommentsBottomSheet(
                context: context,
                user: user,
                comments: comments,
                isLoading: isLoading,
                hasMore: hasMore,
                isLoadingMore: isLoadingMore,
                onSend: onSubmit,
                onLikeTap: onLikeTap,
                onEdit: onEdit,
                onDelete: onDelete,
                onLoadMore: onLoadMore,
                canComment: canComment,
              );
            },
            child: Container(
              padding: const EdgeInsets.symmetric(horizontal: 12),
              decoration: BoxDecoration(
                color: const Color(0xFF191919),
                borderRadius: BorderRadius.circular(22),
                border: Border.all(color: const Color(0xFF262626), width: 1),
              ),
              child: Row(
                children: [
                  SvgPicture.asset(Assets.roundBubble, width: 20, height: 20),
                  8.gap,
                  const Expanded(
                    child: Text(
                      'Add a comment',
                      overflow: TextOverflow.ellipsis,
                      style: TextStyle(color: Color(0xFF404040), fontSize: 16),
                    ),
                  ),
                  8.gap,
                  const Icon(
                    Icons.send_rounded,
                    size: 18,
                    color: AppColors.neutral500,
                  ),
                ],
              ),
            ),
          ),
        ),
      ),
    );
  }
}
