import 'package:ratel/exports.dart';

class BookmarkScreen extends GetWidget<BookmarkController> {
  const BookmarkScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Layout<BookmarkController>(
      child: Padding(
        padding: EdgeInsets.all(20),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                InkWell(
                  onTap: controller.goBack,
                  child: SvgPicture.asset(Assets.back, width: 32, height: 32),
                ),
                20.gap,
                Text(
                  BookmarkLocalization.bookmarks,
                  style: TextStyle(
                    color: Colors.white,
                    fontSize: 24,
                    fontWeight: FontWeight.w700,
                    height: 1.3,
                  ),
                ),
              ],
            ),
            20.vgap,
            Obx(() {
              final items = controller.bookmarkedFeeds.value;
              if (items.isEmpty) {
                return const _BookmarkEmpty();
              }
              return Padding(
                padding: const EdgeInsets.fromLTRB(0, 10, 0, 10),
                child: ListView.separated(
                  shrinkWrap: true,
                  physics: const NeverScrollableScrollPhysics(),
                  itemCount: items.length,
                  separatorBuilder: (_, __) => 10.vgap,
                  itemBuilder: (_, i) => FeedBox(
                    data: items[i],
                    onBookmarkTap: (feedId, isBookmarked) async {
                      await controller.removebookmark(feedId);
                    },
                  ),
                ),
              );
            }),
          ],
        ),
      ),
    );
  }
}

class _BookmarkEmpty extends StatelessWidget {
  const _BookmarkEmpty();

  @override
  Widget build(BuildContext context) {
    return Container(
      alignment: Alignment.center,
      padding: const EdgeInsets.symmetric(vertical: 60),
      child: Text(
        BookmarkLocalization.bookmarkError,
        style: TextStyle(color: AppColors.neutral500, fontSize: 14),
      ),
    );
  }
}
