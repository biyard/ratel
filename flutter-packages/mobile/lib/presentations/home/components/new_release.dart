import 'package:ratel/exports.dart';

class NewRelease extends StatelessWidget {
  final List<FeedSummary> items;
  final void Function(int feedId, bool isBookmarked)? onBookmarkTap;
  const NewRelease({super.key, required this.items, this.onBookmarkTap});

  @override
  Widget build(BuildContext context) {
    if (items.isEmpty) return const SizedBox.shrink();
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text(
          HomeLocalization.newRelease,
          style: TextStyle(
            color: Colors.white,
            fontSize: 16,
            fontWeight: FontWeight.w700,
            height: 1.2,
          ),
        ),
        10.vgap,
        for (final it in items) ...[
          FeedBox(data: it, onBookmarkTap: onBookmarkTap),
          10.vgap,
        ],
      ],
    );
  }
}
