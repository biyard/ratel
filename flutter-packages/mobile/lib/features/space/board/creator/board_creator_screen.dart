import 'package:ratel/exports.dart';
import 'package:ratel/features/space/board/components/board_body_card.dart';
import 'package:ratel/features/space/board/components/board_detail_top_bar.dart';
import 'package:ratel/features/space/board/components/board_time_header.dart';
import 'package:ratel/features/space/board/components/board_title_and_author.dart';
import 'package:ratel/features/space/board/components/board_comment_input_bar.dart';

class BoardCreatorScreen extends GetWidget<BoardCreatorController> {
  const BoardCreatorScreen({super.key});
  @override
  Widget build(BuildContext context) {
    logger.d("Board hasMoreComents: ${controller.hasMoreComments}");
    return Layout<BoardCreatorController>(
      scrollable: false,
      child: Container(
        color: const Color(0xFF111111),
        child: SafeArea(
          bottom: true,
          child: Column(
            children: [
              Expanded(
                child: Obx(() {
                  if (controller.isLoading.value &&
                      controller.post.value == null) {
                    return const Center(
                      child: SizedBox(
                        width: 24,
                        height: 24,
                        child: CircularProgressIndicator(strokeWidth: 2),
                      ),
                    );
                  }
                  final post = controller.post.value;
                  if (post == null) {
                    return const SizedBox.shrink();
                  }
                  return Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      BoardDetailTopBar(
                        categoryName: post.categoryName,
                        onBackTap: () {
                          Get.rootDelegate.offNamed(
                            spaceWithPk(controller.spacePk),
                          );
                        },
                        onEditTap: () {
                          logger.d('BoardCreatorScreen: edit post ${post.pk}');
                        },
                      ),
                      Padding(
                        padding: const EdgeInsets.fromLTRB(16, 0, 16, 0),
                        child: Column(
                          children: [
                            BoardTitleAndAuthor(post: post),
                            15.vgap,
                            BoardTimeHeader(
                              timeZone: 'Asia/Seoul',
                              start: _fromTimestamp(post.startedAt),
                              end: _fromTimestamp(post.endedAt),
                            ),
                            15.vgap,
                          ],
                        ),
                      ),
                      Expanded(
                        child: SingleChildScrollView(
                          padding: const EdgeInsets.fromLTRB(16, 8, 16, 8),
                          child: Column(
                            crossAxisAlignment: CrossAxisAlignment.start,
                            children: [BoardBodyCard(post: post)],
                          ),
                        ),
                      ),
                    ],
                  );
                }),
              ),
              Obx(() {
                final post = controller.post.value;

                bool canComment = true;
                if (post != null) {
                  final hasSchedule = post.startedAt != 0 && post.endedAt != 0;
                  if (hasSchedule) {
                    final now = DateTime.now().millisecondsSinceEpoch;
                    final start = _fromTimestamp(
                      post.startedAt,
                    ).millisecondsSinceEpoch;
                    final end = _fromTimestamp(
                      post.endedAt,
                    ).millisecondsSinceEpoch;
                    canComment = now >= start && now <= end;
                  }
                }

                return BoardCommentInputBar(
                  user: controller.user.value,
                  onSubmit: (text) async => controller.addComment(text),
                  comments: controller.comments,
                  isLoading: controller.isLoadingComments.value,
                  hasMore: controller.hasMoreComments,
                  isLoadingMore: controller.isLoadingMoreComments.value,
                  canComment: canComment,
                  onLikeTap: (c) => controller.toggleLike(c),
                  onEdit: (c, newText) async =>
                      controller.updateComment(c, newText),
                  onDelete: (c) async => controller.deleteComment(c),
                  onLoadMore: () async {
                    await controller.loadMoreComments();
                    return controller.hasMoreComments;
                  },
                );
              }),
            ],
          ),
        ),
      ),
    );
  }
}

DateTime _fromTimestamp(int ts) {
  if (ts == 0) {
    return DateTime.fromMillisecondsSinceEpoch(0);
  }
  if (ts < 1000000000000) {
    return DateTime.fromMillisecondsSinceEpoch(ts * 1000);
  }
  return DateTime.fromMillisecondsSinceEpoch(ts);
}
