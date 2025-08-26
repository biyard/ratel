import 'package:ratel/exports.dart';
import 'package:ratel/presentations/home/components/app_card.dart';

class NewRelease extends StatelessWidget {
  final List<FeedSummary> items;
  const NewRelease({super.key, required this.items});

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
        for (final it in items) ...[NewReleaseCard(data: it), 10.vgap],
      ],
    );
  }
}

class NewReleaseCard extends StatelessWidget {
  final FeedSummary data;
  const NewReleaseCard({super.key, required this.data});

  @override
  Widget build(BuildContext context) {
    return AppCard(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Padding(
            padding: const EdgeInsets.fromLTRB(15, 0, 15, 0),
            child: Row(
              children: [
                DarkTagChip(text: data.feedType),
                Spacer(),
                SvgPicture.asset(Assets.bookmark, width: 20, height: 20),
              ],
            ),
          ),
          15.vgap,
          Padding(
            padding: const EdgeInsets.symmetric(horizontal: 15),
            child: Row(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                if (data.image != null && data.image!.isNotEmpty) ...[
                  ClipRRect(
                    borderRadius: BorderRadius.circular(8),
                    child: RoundContainer(
                      width: 54,
                      height: 54,
                      radius: 8,
                      color: AppColors.neutral500,
                      child: Image.network(data.image!, fit: BoxFit.cover),
                    ),
                  ),
                  10.gap,
                ],

                Expanded(
                  child: Text(
                    data.title,
                    maxLines: 2,
                    overflow: TextOverflow.ellipsis,
                    style: const TextStyle(
                      fontWeight: FontWeight.w700,
                      fontSize: 18,
                      color: Colors.white,
                      height: 1.2,
                    ),
                  ),
                ),
              ],
            ),
          ),
          15.vgap,

          Padding(
            padding: const EdgeInsets.symmetric(horizontal: 15),
            child: Row(
              children: [
                ClipRRect(
                  borderRadius: BorderRadius.circular(100),
                  child: RoundContainer(
                    width: 20,
                    height: 20,
                    radius: 100,
                    color: AppColors.neutral500,
                    child:
                        (data.authorUrl.isNotEmpty &&
                            !data.authorUrl.contains('test'))
                        ? Image.network(data.authorUrl, fit: BoxFit.cover)
                        : const SizedBox.shrink(),
                  ),
                ),
                6.gap,
                SizedBox(
                  width: 110,
                  child: Text(
                    data.authorName,
                    overflow: TextOverflow.ellipsis,
                    style: const TextStyle(
                      fontWeight: FontWeight.w700,
                      color: Colors.white,
                      fontSize: 12,
                      height: 1.2,
                    ),
                  ),
                ),
                6.gap,
                SvgPicture.asset(
                  Assets.badge,
                  width: 16,
                  height: 16,
                  colorFilter: const ColorFilter.mode(
                    AppColors.primary,
                    BlendMode.srcIn,
                  ),
                ),
                const Spacer(),
                Text(
                  timeAgo(data.createdAt),
                  style: const TextStyle(
                    fontWeight: FontWeight.w500,
                    color: AppColors.neutral500,
                    fontSize: 12,
                    height: 1.2,
                  ),
                ),
              ],
            ),
          ),
          20.vgap,

          const Divider(color: AppColors.neutral800, height: 1),

          Padding(
            padding: const EdgeInsets.fromLTRB(30, 0, 30, 0),
            child: Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              crossAxisAlignment: CrossAxisAlignment.center,
              children: [
                iconText(
                  SvgPicture.asset(Assets.thumbs, width: 20, height: 20),
                  compact(data.likes),
                ),
                iconText(
                  SvgPicture.asset(Assets.roundBubble, width: 20, height: 20),
                  compact(data.comments),
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }

  Widget iconText(SvgPicture icon, String text) {
    return Row(
      children: [
        icon,
        6.gap,
        Text(
          text,
          style: const TextStyle(
            fontSize: 15,
            fontWeight: FontWeight.w400,
            color: Colors.white,
          ),
        ),
      ],
    );
  }
}

class DarkTagChip extends StatelessWidget {
  final String text;
  const DarkTagChip({super.key, required this.text});

  @override
  Widget build(BuildContext context) {
    return Container(
      height: 24,
      padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 2.5),
      decoration: BoxDecoration(
        color: AppColors.neutral800,
        borderRadius: BorderRadius.circular(4),
      ),
      child: Text(
        text,
        style: const TextStyle(
          color: Colors.white,
          fontSize: 12,
          fontWeight: FontWeight.w600,
          height: 1.2,
        ),
      ),
    );
  }
}
