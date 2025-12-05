import 'package:ratel/exports.dart';

class FeedCardV2 extends StatelessWidget {
  const FeedCardV2({
    super.key,
    required this.feed,
    this.onBookmarkTap,
    this.onTap,
    this.onEditTap,
    this.onMoreTap,
    this.onJoinSpaceTap,
  });

  final FeedV2SummaryModel feed;
  final VoidCallback? onBookmarkTap;
  final VoidCallback? onTap;
  final VoidCallback? onEditTap;
  final VoidCallback? onMoreTap;
  final VoidCallback? onJoinSpaceTap;

  bool get hasSpace => feed.spacePk != null && feed.spacePk!.isNotEmpty;

  @override
  Widget build(BuildContext context) {
    final imageUrl = feed.mainImage;
    final bodyText = _plainTextFromHtml(feed.htmlContents);

    final profileImageUrl = (feed.authorProfileUrl.isNotEmpty
        ? feed.authorProfileUrl
        : defaultProfileImage);

    final top = Container(
      color: const Color(0xff171717),
      child: Padding(
        padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 20),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                if (hasSpace) ...[_spaceChip('Space')],
                8.gap,
                Text(
                  feed.title,
                  maxLines: 1,
                  overflow: TextOverflow.ellipsis,
                  style: const TextStyle(
                    fontFamily: 'Raleway',
                    color: Colors.white,
                    fontWeight: FontWeight.w700,
                    fontSize: 18,
                    height: 24 / 18,
                  ),
                ),
              ],
            ),
            10.vgap,
            Row(
              children: [
                RoundContainer(
                  width: 24,
                  height: 24,
                  radius: 100,
                  color: AppColors.neutral500,
                  child: ClipRRect(
                    borderRadius: BorderRadius.circular(100),
                    child: Image.network(profileImageUrl, fit: BoxFit.cover),
                  ),
                ),
                8.gap,
                Expanded(
                  child: Row(
                    children: [
                      Flexible(
                        child: Text(
                          feed.authorDisplayName,
                          overflow: TextOverflow.ellipsis,
                          style: const TextStyle(
                            fontFamily: 'Raleway',
                            color: Colors.white,
                            fontWeight: FontWeight.w700,
                            fontSize: 14,
                            height: 20 / 14,
                          ),
                        ),
                      ),
                    ],
                  ),
                ),
                const SizedBox(width: 8),
                Text(
                  timeAgo(feed.createdAt),
                  style: const TextStyle(
                    fontFamily: 'Inter',
                    color: AppColors.neutral500,
                    fontWeight: FontWeight.w500,
                    fontSize: 12,
                    height: 16 / 12,
                  ),
                ),
              ],
            ),
            10.vgap,
            if (bodyText.isNotEmpty)
              Text(
                bodyText,
                maxLines: 5,
                overflow: TextOverflow.ellipsis,
                style: const TextStyle(
                  fontFamily: 'Raleway',
                  color: Colors.white,
                  fontWeight: FontWeight.w400,
                  fontSize: 15,
                  height: 22 / 15,
                ),
              ),
            if (imageUrl != null && imageUrl.isNotEmpty) ...[
              10.vgap,
              ClipRRect(
                borderRadius: BorderRadius.circular(10),
                child: AspectRatio(
                  aspectRatio: 367 / 206,
                  child: Image.network(imageUrl, fit: BoxFit.cover),
                ),
              ),
            ],

            // if (hasSpace) ...[
            //   20.vgap,
            //   Align(
            //     alignment: Alignment.centerLeft,
            //     child: TextButton(
            //       onPressed: onJoinSpaceTap,
            //       style: TextButton.styleFrom(
            //         backgroundColor: AppColors.primary,
            //         shape: RoundedRectangleBorder(
            //           borderRadius: BorderRadius.circular(24),
            //         ),
            //         padding: EdgeInsets.zero,
            //       ),
            //       child: Padding(
            //         padding: const EdgeInsets.fromLTRB(20, 12, 20, 12),
            //         child: const Text(
            //           'Join Space',
            //           style: TextStyle(
            //             fontFamily: 'Raleway',
            //             color: Colors.black,
            //             fontWeight: FontWeight.w700,
            //             fontSize: 14,
            //           ),
            //         ),
            //       ),
            //     ),
            //   ),
            // ],
          ],
        ),
      ),
    );

    final bottom = Container(
      decoration: const BoxDecoration(
        color: Color(0xff171717),
        border: Border(top: BorderSide(color: Color(0xFF262626), width: 1)),
      ),
      child: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          Row(
            children: [
              _bottomMetric(
                icon: SvgPicture.asset(Assets.thumbs, width: 20, height: 20),
                value: feed.likes,
              ),
              _bottomMetric(
                icon: SvgPicture.asset(
                  Assets.roundBubble,
                  width: 20,
                  height: 20,
                ),
                value: feed.comments,
              ),
            ],
          ),
        ],
      ),
    );

    final content = Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        Container(
          decoration: const BoxDecoration(
            color: AppColors.panelBg,
            // border: Border(
            //   top: BorderSide(color: Color(0xFF2D2D2D), width: 0.5),
            // ),
          ),
          child: top,
        ),
        bottom,
      ],
    );

    if (onTap == null) {
      return content;
    }

    return InkWell(
      onTap: onTap,
      child: ClipRRect(borderRadius: BorderRadius.circular(0), child: content),
    );
  }

  Widget _bottomMetric({
    required SvgPicture icon,
    required int value,
    VoidCallback? onTap,
  }) {
    final child = Padding(
      padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 12),
      child: Row(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          icon,
          const SizedBox(width: 6),
          Text(
            value.toString(),
            style: const TextStyle(
              fontFamily: 'Inter',
              color: Colors.white,
              fontWeight: FontWeight.w500,
              fontSize: 15,
              height: 20 / 15,
            ),
          ),
        ],
      ),
    );

    return Expanded(
      child: onTap == null ? child : InkWell(onTap: onTap, child: child),
    );
  }

  Widget _spaceChip(String label) {
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 6, vertical: 3),
      decoration: BoxDecoration(
        color: AppColors.primary,
        borderRadius: BorderRadius.circular(4),
      ),
      child: Row(
        children: [
          SvgPicture.asset(Assets.palace, width: 14, height: 14),
          4.gap,
          Text(
            label,
            style: const TextStyle(
              fontFamily: 'Raleway',
              color: Colors.black,
              fontWeight: FontWeight.w600,
              fontSize: 11,
            ),
          ),
        ],
      ),
    );
  }

  String _plainTextFromHtml(String html) {
    if (html.isEmpty) return '';
    final noTags = html.replaceAll(RegExp(r'<[^>]+>'), '');
    return noTags.trim();
  }
}
