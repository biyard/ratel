import 'package:ratel/exports.dart';
import 'package:ratel/features/space/boards/components/board_category_bar.dart';
import 'package:ratel/features/space/boards/components/board_post_card.dart';

class BoardsCreatorScreen extends GetWidget<BoardsCreatorController> {
  const BoardsCreatorScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Layout<BoardsCreatorController>(
      scrollable: false,
      child: Obx(() {
        if (controller.isLoading.value && controller.posts.isEmpty) {
          return const Center(
            child: SizedBox(
              width: 24,
              height: 24,
              child: CircularProgressIndicator(strokeWidth: 2),
            ),
          );
        }

        return Column(
          children: [
            BoardsCategoryBar(
              categories: controller.categories,
              selectedCategory: controller.selectedCategory,
              onCategorySelected: controller.onCategorySelected,
            ),
            10.vgap,
            Expanded(
              child: RefreshIndicator(
                onRefresh: controller.refreshAll,
                color: AppColors.primary,
                backgroundColor: AppColors.bg,
                child: ListView.builder(
                  itemCount:
                      controller.posts.length + (controller.hasMore ? 1 : 0),
                  itemBuilder: (context, index) {
                    if (index >= controller.posts.length) {
                      controller.loadMore();
                      return const Padding(
                        padding: EdgeInsets.symmetric(vertical: 16),
                        child: Center(
                          child: SizedBox(
                            width: 20,
                            height: 20,
                            child: CircularProgressIndicator(strokeWidth: 1.6),
                          ),
                        ),
                      );
                    }

                    final post = controller.posts[index];
                    return BoardPostCard(
                      post: post,
                      onTap: () {
                        final route = AppRoutes.spacePostWithPk(
                          controller.spacePk,
                          post.pk,
                        );
                        logger.d('Navigate to board detail: $route');
                        Get.rootDelegate.toNamed(route);
                      },
                    );
                  },
                ),
              ),
            ),
          ],
        );
      }),
    );
  }
}
