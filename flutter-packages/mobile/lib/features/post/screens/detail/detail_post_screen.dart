import 'package:ratel/exports.dart';
import 'package:ratel/features/post/screens/detail/components/delete_post_dialog.dart';
import 'package:ratel/features/post/screens/detail/components/detail_comment_bar.dart';
import 'package:ratel/features/post/screens/detail/components/detail_scroll_content.dart';
import 'package:ratel/features/post/screens/detail/components/detail_top_bar.dart';
import 'package:ratel/features/post/screens/detail/components/post_more_bottom_sheet.dart';

class DetailPostScreen extends GetWidget<DetailPostController> {
  const DetailPostScreen({super.key});

  Future<void> _openPostActionSheet(
    BuildContext context, {
    required String postPk,
  }) async {
    await showModalBottomSheet(
      context: context,
      backgroundColor: const Color(0xFF191919),
      shape: const RoundedRectangleBorder(
        borderRadius: BorderRadius.vertical(top: Radius.circular(20)),
      ),
      builder: (_) {
        return PostMoreBottomSheet(
          onUpdate: () {
            Navigator.pop(context);
            Get.rootDelegate.toNamed(
              createPostScreen,
              arguments: {'postPk': postPk},
            );
          },
          onDelete: () async {
            Navigator.pop(context);
            await _confirmDelete(context, postPk: postPk);
          },
        );
      },
    );
  }

  Future<void> _confirmDelete(
    BuildContext context, {
    required String postPk,
  }) async {
    final result =
        await showDialog<bool>(
          context: context,
          barrierDismissible: true,
          builder: (_) => const DeletePostDialog(),
        ) ??
        false;

    if (!result) return;

    await controller.deletePost(postPk: postPk);
  }

  @override
  Widget build(BuildContext context) {
    final bottomInset = MediaQuery.of(context).padding.bottom;

    return Layout<DetailPostController>(
      scrollable: false,
      child: Container(
        color: const Color(0xFF1D1D1D),
        child: Column(
          children: [
            Obx(() {
              final model = controller.feed.value;
              final postPk = model?.post.pk ?? '';

              return DetailTopBar(
                isCreator: model?.post.userPk == controller.user.value.pk,
                onBack: () => Get.back(),
                onExtra: postPk.isEmpty
                    ? () {}
                    : () => _openPostActionSheet(context, postPk: postPk),
              );
            }),
            Expanded(
              child: Obx(() {
                if (controller.isLoading.value &&
                    controller.feed.value == null) {
                  return const Center(
                    child: SizedBox(
                      width: 24,
                      height: 24,
                      child: CircularProgressIndicator(strokeWidth: 2),
                    ),
                  );
                }

                final model = controller.feed.value;
                if (model == null) {
                  return const SizedBox.shrink();
                }

                return Obx(
                  () => DetailScrollContent(
                    post: model.post,
                    isLiked: model.isLiked == true,
                    isLiking: controller.isLikingPost.value,
                    onToggleLike: controller.toggleLikePost,
                  ),
                );
              }),
            ),
            Obx(() {
              final model = controller.feed.value;
              if (model == null) {
                return const SizedBox.shrink();
              }

              return DetailCommentBar(
                bottomInset: bottomInset,
                comments: model.comments,
                onSendComment: controller.addComment,
                isLikingCommentOf: controller.isLikingCommentOf,
                isCommentLiked: controller.isCommentLiked,
                onToggleLikeComment: (commentSk) =>
                    controller.toggleLikeComment(commentSk: commentSk),
              );
            }),
          ],
        ),
      ),
    );
  }
}
