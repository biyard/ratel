import 'package:ratel/components/header/main_header.dart';
import 'package:ratel/exports.dart';

class HomeScreen extends GetWidget<HomeController> {
  const HomeScreen({super.key});

  @override
  Widget build(BuildContext context) {
    final bottomPad = MediaQuery.of(context).padding.bottom;

    return Layout<HomeController>(
      enableSafeArea: false,
      scrollable: false,
      child: Obx(
        () => RefreshIndicator(
          onRefresh: controller.loadInitial,
          color: AppColors.primary,
          backgroundColor: AppColors.bg,
          child: ListView.separated(
            controller: controller.scrollController,
            physics: const AlwaysScrollableScrollPhysics(),
            padding: EdgeInsets.fromLTRB(0, 0, 0, bottomPad + 10),
            itemCount:
                1 +
                controller.feeds.length +
                (controller.hasMore.value ? 1 : 0),
            separatorBuilder: (_, index) {
              if (index == 0) return const SizedBox.shrink();
              return 8.vgap;
            },
            itemBuilder: (context, index) {
              if (index == 0) {
                return const MainHeader();
              }

              final feedIndex = index - 1;

              if (feedIndex >= controller.feeds.length) {
                return _buildLoadMoreIndicator();
              }

              final feed = controller.feeds[feedIndex];
              return FeedCard(
                feed: feed,
                onLikeTap: () {
                  controller.toggleLikePost(feed);
                },
                onBookmarkTap: () {},
                onTap: () async {
                  logger.d("feed tapped: ${feed.pk} ${feed.spacePk}");
                  if (feed.spacePk != null) {
                    logger.d("space pk: ${feed.spacePk}");
                    controller.feedsService.fetchDetail(
                      feed.pk,
                      forceRefresh: true,
                    );

                    Get.rootDelegate.toNamed(spaceWithPk(feed.spacePk!));
                  } else {
                    logger.d("feed pk: ${feed.pk}");
                    Get.rootDelegate.toNamed(postWithPk(feed.pk));
                  }
                },
              );
            },
          ),
        ),
      ),
    );
  }

  Widget _buildLoadMoreIndicator() {
    if (!controller.hasMore.value) {
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
