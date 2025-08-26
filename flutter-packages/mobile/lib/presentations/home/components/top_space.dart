import 'package:ratel/exports.dart';
import 'package:ratel/presentations/home/components/app_card.dart';

class TopSpace extends StatelessWidget {
  final List<SpaceSummary> items;
  const TopSpace({super.key, required this.items});

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const Text(
          'Top Spaces',
          style: TextStyle(
            color: Colors.white,
            fontSize: 16,
            fontWeight: FontWeight.w700,
            height: 1.2,
          ),
        ),
        const SizedBox(height: 10),
        TopSpacesCarousel(items: items),
      ],
    );
  }
}

class TopSpacesCarousel extends StatelessWidget {
  final List<SpaceSummary> items;
  const TopSpacesCarousel({super.key, required this.items});

  @override
  Widget build(BuildContext context) {
    final controller = PageController(viewportFraction: 0.86);
    return SizedBox(
      height: 250,
      child: PageView.builder(
        controller: controller,
        padEnds: false,
        itemCount: items.length,
        itemBuilder: (_, i) => Padding(
          padding: const EdgeInsets.only(right: 14),
          child: SpaceSummaryCard(data: items[i]),
        ),
      ),
    );
  }
}

class SpaceSummaryCard extends StatelessWidget {
  final SpaceSummary data;
  const SpaceSummaryCard({super.key, required this.data});

  @override
  Widget build(BuildContext context) {
    return AppCard(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Padding(
            padding: const EdgeInsets.fromLTRB(15, 0, 15, 0),
            child: Column(
              children: [
                Row(
                  children: [
                    YellowChip(
                      icon: SvgPicture.asset(
                        Assets.palace,
                        width: 14,
                        height: 14,
                      ),
                      label: 'Space',
                    ),
                    10.gap,
                    YellowChip(
                      icon: SvgPicture.asset(
                        Assets.coin,
                        width: 14,
                        height: 14,
                      ),
                      label: 'Rewards',
                    ),
                    Spacer(),
                    SvgPicture.asset(Assets.bookmark, width: 20, height: 20),
                  ],
                ),
                15.vgap,

                Row(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    if (data.imageUrl.isNotEmpty || data.imageUrl != "") ...[
                      ClipRRect(
                        borderRadius: BorderRadius.circular(12),
                        child: RoundContainer(
                          width: 80,
                          height: 80,
                          radius: 8,
                          color: AppColors.neutral500,
                          child: Image.network(
                            data.imageUrl,
                            fit: BoxFit.cover,
                          ),
                        ),
                      ),
                      10.gap,
                    ],
                    Expanded(
                      child: Text(
                        data.title,
                        maxLines: 3,
                        overflow: TextOverflow.ellipsis,
                        style: TextStyle(
                          fontWeight: FontWeight.w700,
                          fontSize: 18,
                          color: Colors.white,
                          height: 1.2,
                        ),
                      ),
                    ),
                  ],
                ),
                15.vgap,

                Row(
                  children: [
                    Row(
                      children: [
                        ClipRRect(
                          borderRadius: BorderRadius.circular(100),
                          child: RoundContainer(
                            width: 20,
                            height: 20,
                            radius: 100,
                            color: AppColors.neutral500,
                            child: (data.authorUrl != "")
                                ? Image.network(
                                    data.authorUrl,
                                    fit: BoxFit.cover,
                                  )
                                : Container(),
                          ),
                        ),
                        4.gap,
                        Text(
                          data.authorName,
                          style: TextStyle(
                            fontWeight: FontWeight.w700,
                            color: Colors.white,
                            fontSize: 12,
                            height: 1.2,
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
                      ],
                    ),
                    const Spacer(),
                    Text(
                      timeAgo(data.updatedAt),
                      style: TextStyle(
                        fontWeight: FontWeight.w500,
                        color: AppColors.neutral500,
                        fontSize: 12,
                        height: 1.2,
                      ),
                    ),
                  ],
                ),
                15.vgap,
              ],
            ),
          ),
          const Divider(color: AppColors.neutral800, height: 1),

          Padding(
            padding: const EdgeInsets.fromLTRB(35, 15, 35, 0),
            child: Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              crossAxisAlignment: CrossAxisAlignment.center,
              children: [
                iconText(
                  SvgPicture.asset(Assets.thumbs, width: 20, height: 20),
                  compact(data.likes),
                ),
                iconText(
                  SvgPicture.asset(
                    Assets.rewardCoin,
                    width: 20,
                    height: 20,
                    color: AppColors.neutral500,
                  ),
                  compact(data.comments),
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

class YellowChip extends StatelessWidget {
  final SvgPicture icon;
  final String label;
  const YellowChip({super.key, required this.icon, required this.label});

  @override
  Widget build(BuildContext context) {
    return Container(
      height: 28,
      padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4.5),
      decoration: BoxDecoration(
        color: AppColors.primary,
        borderRadius: BorderRadius.circular(4),
      ),
      child: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          icon,
          4.gap,
          Text(
            label,
            style: const TextStyle(
              color: AppColors.bg,
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
