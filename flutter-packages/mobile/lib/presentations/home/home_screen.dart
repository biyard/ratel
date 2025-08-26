import 'package:ratel/exports.dart';
import 'package:ratel/presentations/home/components/matched_feed.dart';
import 'package:ratel/presentations/home/components/new_release.dart';
import 'package:ratel/presentations/home/components/top_space.dart';

class HomeScreen extends GetWidget<HomeController> {
  const HomeScreen({super.key});

  @override
  Widget build(BuildContext context) {
    final bottomPad = MediaQuery.of(context).padding.bottom;

    return Layout<HomeController>(
      child: Padding(
        padding: EdgeInsets.fromLTRB(14, 10, 14, bottomPad),
        child: Column(
          children: [
            Obx(() => TopSpace(items: controller.topSpaces.value)),
            30.vgap,
            Obx(
              () => MatchedFeed(
                items: controller.matchedFeeds.value,
                onBookmarkTap: (feedId, isBookmarked) async {
                  if (isBookmarked) {
                    await controller.removebookmark(feedId);
                  } else {
                    await controller.addBookmark(feedId);
                  }
                },
              ),
            ),
            30.vgap,
            Obx(
              () => NewRelease(
                items: controller.newFeeds.value,
                onBookmarkTap: (feedId, isBookmarked) async {
                  if (isBookmarked) {
                    await controller.removebookmark(feedId);
                  } else {
                    await controller.addBookmark(feedId);
                  }
                },
              ),
            ),
          ],
        ),
      ),
    );
  }
}
