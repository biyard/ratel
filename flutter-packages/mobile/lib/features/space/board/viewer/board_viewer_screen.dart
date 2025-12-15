import 'package:ratel/exports.dart';
import 'package:ratel/features/space/board/components/board_body_card.dart';
import 'package:ratel/features/space/board/components/board_detail_top_bar.dart';
import 'package:ratel/features/space/board/components/board_time_header.dart';
import 'package:ratel/features/space/board/components/board_title_and_author.dart';
import 'package:ratel/features/space/board/components/board_comment_input_bar.dart';
import 'package:ratel/features/post/screens/detail/components/post_more_bottom_sheet.dart';

class BoardViewerScreen extends GetWidget<BoardViewerController> {
  const BoardViewerScreen({super.key});

  Future<void> _openBoardActionSheet(
    BuildContext context, {
    required String spacePk,
    required String spacePostPk,
    required bool isReported,
  }) async {
    if (isReported) return;

    await showModalBottomSheet(
      context: context,
      backgroundColor: const Color(0xFF191919),
      shape: const RoundedRectangleBorder(
        borderRadius: BorderRadius.vertical(top: Radius.circular(20)),
      ),
      builder: (_) {
        return PostMoreBottomSheet(
          onUpdate: null,
          onDelete: null,
          onReport: () async {
            Navigator.pop(context);
            await controller.reportSpacePost(
              spacePk: spacePk,
              spacePostPk: spacePostPk,
            );
          },
        );
      },
    );
  }

  @override
  Widget build(BuildContext context) {
    return Layout<BoardViewerController>(
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

                  final canReport = !(post.isReport ?? false);

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
                        onMoreTap: canReport
                            ? () => _openBoardActionSheet(
                                context,
                                spacePk: controller.spacePk,
                                spacePostPk: post.pk,
                                isReported: post.isReport ?? false,
                              )
                            : null,
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
                  user: controller.user,
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
                  onReport: (comment) async {
                    await controller.reportSpaceComment(
                      spacePostPk: controller.post.value?.pk ?? "",
                      commentSk: comment.sk,
                    );
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
