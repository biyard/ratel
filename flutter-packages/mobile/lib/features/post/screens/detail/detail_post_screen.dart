import 'package:ratel/exports.dart';
import 'package:ratel/features/post/screens/detail/components/detail_comment_bar.dart';
import 'package:ratel/features/post/screens/detail/components/detail_scroll_content.dart';
import 'package:ratel/features/post/screens/detail/components/detail_top_bar.dart';

class DetailPostScreen extends GetWidget<DetailPostController> {
  const DetailPostScreen({super.key});

  @override
  Widget build(BuildContext context) {
    final bottomInset = MediaQuery.of(context).padding.bottom;

    return Layout<DetailPostController>(
      scrollable: false,
      child: Container(
        color: const Color(0xFF1D1D1D),
        child: Column(
          children: [
            const DetailTopBar(),
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

                return DetailScrollContent(post: model.post);
              }),
            ),
            Obx(() {
              return DetailCommentBar(
                bottomInset: bottomInset,
                comments: controller.feed.value?.comments ?? [],
              );
            }),
          ],
        ),
      ),
    );
  }
}
