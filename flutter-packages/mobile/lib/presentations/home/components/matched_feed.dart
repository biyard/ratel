import 'package:ratel/exports.dart';
import 'package:ratel/presentations/home/components/app_card.dart';

class MatchedFeed extends StatelessWidget {
  final List<FeedSummary> items;
  const MatchedFeed({super.key, required this.items});

  @override
  Widget build(BuildContext context) {
    if (items.isEmpty) return const SizedBox.shrink();
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text(
          HomeLocalization.aiMatched,
          style: TextStyle(
            color: Colors.white,
            fontSize: 16,
            fontWeight: FontWeight.w700,
            height: 1.2,
          ),
        ),
        10.vgap,
        for (final it in items) ...[MatchedFeedCard(data: it), 10.vgap],
      ],
    );
  }
}

class MatchedFeedCard extends StatelessWidget {
  final FeedSummary data;
  const MatchedFeedCard({super.key, required this.data});

  @override
  Widget build(BuildContext context) {
    logger.d("author url: ${data.authorUrl}");
    return AppCard(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Padding(
            padding: const EdgeInsets.fromLTRB(15, 0, 15, 0),
            child: Row(
              children: [
                MatchedChip(),
                const Spacer(),
                SvgPicture.asset(Assets.bookmark, width: 20, height: 20),
              ],
            ),
          ),
          10.vgap,

          Text(
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
          10.vgap,

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
                            data.authorUrl != "" &&
                            !data.authorUrl.contains("test"))
                        ? Image.network(data.authorUrl, fit: BoxFit.cover)
                        : Container(),
                  ),
                ),
                4.gap,

                SizedBox(
                  width: 100,
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
                4.gap,
                SvgPicture.asset(
                  Assets.badge,
                  width: 16,
                  height: 16,
                  colorFilter: ColorFilter.mode(
                    AppColors.primary,
                    BlendMode.dstIn,
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
            padding: const EdgeInsets.fromLTRB(15, 15, 15, 0),
            child: Row(
              children: [
                iconText(
                  SvgPicture.asset(Assets.thumbs, width: 20, height: 20),
                  compact(data.likes),
                ),
                20.gap,
                iconText(
                  SvgPicture.asset(Assets.roundBubble, width: 20, height: 20),
                  compact(data.comments),
                ),
                const Spacer(),
                iconText(
                  SvgPicture.asset(
                    Assets.rewardCoin,
                    width: 20,
                    height: 20,
                    colorFilter: ColorFilter.mode(
                      AppColors.primary,
                      BlendMode.dstIn,
                    ),
                  ),
                  '${comma(data.rewards ?? 0)} P',
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
        5.gap,
        Text(
          text,
          style: TextStyle(
            fontSize: 15,
            fontWeight: FontWeight.w400,
            color: Colors.white,
          ),
        ),
      ],
    );
  }
}

class MatchedChip extends StatelessWidget {
  const MatchedChip({super.key});

  @override
  Widget build(BuildContext context) {
    return Container(
      height: 24,
      padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4.5),
      decoration: BoxDecoration(
        color: AppColors.neutral800,
        borderRadius: BorderRadius.circular(4),
      ),
      child: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          SvgPicture.asset(
            Assets.bot,
            width: 14,
            height: 14,
            colorFilter: const ColorFilter.mode(
              AppColors.primary,
              BlendMode.srcIn,
            ),
          ),
          4.gap,
          Text(
            HomeLocalization.matched,
            style: TextStyle(
              color: Colors.white,
              fontSize: 12,
              fontWeight: FontWeight.w600,
              height: 1.2,
            ),
          ),
        ],
      ),
    );
  }
}
