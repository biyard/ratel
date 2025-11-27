import 'package:ratel/exports.dart';
import 'package:ratel/components/feed_card/v2/feed_card_v2.dart';

class HomeScreen extends GetWidget<HomeController> {
  const HomeScreen({super.key});

  @override
  Widget build(BuildContext context) {
    final bottomPad = MediaQuery.of(context).padding.bottom;

    return Layout<HomeController>(
      scrollable: false,
      child: Obx(
        () => RefreshIndicator(
          onRefresh: controller.loadInitial,
          color: AppColors.primary,
          backgroundColor: AppColors.bg,
          child: ListView.separated(
            controller: controller.scrollController,
            padding: EdgeInsets.fromLTRB(0, 10, 0, bottomPad + 10),
            itemCount:
                controller.feeds.length + (controller.hasMore.value ? 1 : 0),
            separatorBuilder: (_, __) => 8.vgap,
            itemBuilder: (context, index) {
              if (index >= controller.feeds.length) {
                return _buildLoadMoreIndicator();
              }

              final feed = controller.feeds[index];
              return FeedCardV2(feed: feed, onBookmarkTap: () => {});
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
