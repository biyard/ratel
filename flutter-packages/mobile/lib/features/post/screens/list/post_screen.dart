import 'package:ratel/exports.dart';

class PostScreen extends GetWidget<PostController> {
  const PostScreen({super.key});

  @override
  Widget build(BuildContext context) {
    final bottomPad = MediaQuery.of(context).padding.bottom;

    return Layout<PostController>(
      scrollable: false,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Padding(
            padding: const EdgeInsets.all(20.0),
            child: AppTopBar(onBack: () => Get.back(), title: "My Posts"),
          ),
          Expanded(
            child: Obx(
              () => RefreshIndicator(
                onRefresh: controller.loadInitial,
                color: AppColors.primary,
                backgroundColor: AppColors.bg,
                child: ListView.separated(
                  controller: controller.scrollController,
                  padding: EdgeInsets.fromLTRB(0, 0, 0, bottomPad + 10),
                  itemCount:
                      controller.feeds.length + (controller.hasMore ? 1 : 0),
                  separatorBuilder: (_, __) => 8.vgap,
                  itemBuilder: (context, index) {
                    if (index >= controller.feeds.length) {
                      return _buildLoadMoreIndicator(controller);
                    }

                    final feed = controller.feeds[index];
                    return FeedCard(
                      feed: feed,
                      onTap: () => {
                        logger.d("feed tapped: ${feed.pk} ${feed.spacePk}"),
                        if (feed.spacePk != null)
                          {
                            logger.d("space pk: ${feed.spacePk}"),
                            Get.rootDelegate.toNamed(
                              spaceWithPk(feed.spacePk!),
                            ),
                          }
                        else
                          {
                            logger.d("feed pk: ${feed.pk}"),
                            Get.rootDelegate.toNamed(postWithPk(feed.pk)),
                          },
                      },
                    );
                  },
                ),
              ),
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildLoadMoreIndicator(PostController controller) {
    if (!controller.hasMore) {
      return const SizedBox.shrink();
    }
    if (!controller.isLoadingMore.value) {
      return const SizedBox(height: 32);
    }
    return const Padding(
      padding: EdgeInsets.symmetric(vertical: 16),
      child: Center(
        child: SizedBox(
          width: 20,
          height: 20,
          child: CircularProgressIndicator(strokeWidth: 2),
        ),
      ),
    );
  }
}
