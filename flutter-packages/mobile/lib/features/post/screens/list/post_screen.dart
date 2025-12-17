import 'package:ratel/exports.dart';

class PostScreen extends GetWidget<PostController> {
  const PostScreen({super.key});

  @override
  Widget build(BuildContext context) {
    final bottomPad = MediaQuery.of(context).padding.bottom;

    return Layout<PostController>(
      scrollable: false,
      child: Obx(() {
        final hasMore = controller.hasMore;
        final feeds = controller.feeds;

        final itemCount = feeds.length + 1 + (hasMore ? 1 : 0);

        return RefreshIndicator(
          onRefresh: controller.loadInitial,
          color: AppColors.primary,
          backgroundColor: AppColors.bg,
          child: ListView.separated(
            controller: controller.scrollController,
            padding: EdgeInsets.fromLTRB(0, 0, 0, bottomPad + 10),
            itemCount: itemCount,
            separatorBuilder: (_, index) {
              if (index == 0) {
                return 4.vgap;
              }
              return 8.vgap;
            },
            itemBuilder: (context, index) {
              if (index == 0) {
                return Padding(
                  padding: const EdgeInsets.fromLTRB(20, 20, 20, 10),
                  child: AppTopBar(onBack: () => Get.back(), title: "My Posts"),
                );
              }

              final loadMoreIndex = itemCount - 1;
              if (hasMore && index == loadMoreIndex) {
                return _buildLoadMoreIndicator(controller);
              }

              final feedIndex = index - 1;
              final feed = feeds[feedIndex];

              logger.d("feed liked: ${feed.pk} ${feed.liked}");

              return FeedCard(
                feed: feed,
                onLikeTap: () => controller.toggleLikePost(feed),
                onTap: () {
                  logger.d("feed tapped: ${feed.pk} ${feed.spacePk}");
                  if (feed.spacePk != null) {
                    logger.d("space pk: ${feed.spacePk}");
                    Get.rootDelegate.toNamed(spaceWithPk(feed.spacePk!));
                  } else {
                    logger.d("feed pk: ${feed.pk}");
                    Get.rootDelegate.toNamed(postWithPk(feed.pk));
                  }
                },
              );
            },
          ),
        );
      }),
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
