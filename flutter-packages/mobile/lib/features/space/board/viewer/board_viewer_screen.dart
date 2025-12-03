import 'package:ratel/exports.dart';
import 'package:ratel/features/space/board/components/board_body_card.dart';
import 'package:ratel/features/space/board/components/board_detail_top_bar.dart';
import 'package:ratel/features/space/board/components/board_title_and_author.dart';

class BoardViewerScreen extends GetWidget<BoardViewerController> {
  const BoardViewerScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Layout<BoardViewerController>(
      scrollable: false,
      child: Container(
        color: const Color(0xFF111111),
        child: SafeArea(
          bottom: false,
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
                        onBackTap: () => {
                          Get.rootDelegate.offNamed(
                            spaceWithPk(controller.spacePk),
                          ),
                        },
                        onEditTap: () {
                          logger.d('BoardViewerScreen: edit post ${post.pk}');
                        },
                      ),
                      Expanded(
                        child: SingleChildScrollView(
                          padding: const EdgeInsets.fromLTRB(16, 8, 16, 8),
                          child: Column(
                            crossAxisAlignment: CrossAxisAlignment.start,
                            children: [
                              BoardTitleAndAuthor(post: post),
                              15.vgap,
                              BoardBodyCard(post: post),
                            ],
                          ),
                        ),
                      ),
                    ],
                  );
                }),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
