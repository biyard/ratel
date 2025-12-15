import 'package:flutter_slidable/flutter_slidable.dart';
import 'package:ratel/exports.dart';

class DraftScreen extends GetWidget<DraftController> {
  const DraftScreen({super.key});

  @override
  Widget build(BuildContext context) {
    final bottomPad = MediaQuery.of(context).padding.bottom;

    return Layout<DraftController>(
      scrollable: false,
      child: Obx(() {
        final feeds = controller.feeds;
        final itemCount = feeds.length + 1;

        return RefreshIndicator(
          onRefresh: () => controller.listFeeds(),
          color: AppColors.primary,
          backgroundColor: AppColors.bg,
          child: ListView.separated(
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
                return AppTopBar(
                  padding: const EdgeInsets.fromLTRB(20, 20, 20, 10),
                  onBack: () => Get.back(),
                  title: DraftLocalization.draftMyDraft,
                );
              }

              final i = index - 1;
              final draft = feeds[i];

              return DraftSlidableCard(
                data: draft,
                onTap: () => controller.openDraft(draft.pk),
                onDelete: () {
                  showRemoveDraftModal(context, draft.pk);
                },
              );
            },
          ),
        );
      }),
    );
  }
}

class DraftSlidableCard extends StatelessWidget {
  final FeedSummaryModel data;
  final VoidCallback? onTap;
  final VoidCallback? onDelete;

  const DraftSlidableCard({
    super.key,
    required this.data,
    this.onTap,
    this.onDelete,
  });

  @override
  Widget build(BuildContext context) {
    return Slidable(
      key: ValueKey('draft_${data.pk}'),
      groupTag: 'drafts',
      closeOnScroll: true,
      endActionPane: ActionPane(
        motion: const DrawerMotion(),
        extentRatio: 0.22,
        children: [
          CustomSlidableAction(
            onPressed: (_) => onDelete?.call(),
            backgroundColor: const Color(0xFFEF4444),
            padding: EdgeInsets.zero,
            child: SvgPicture.asset(Assets.delete2, width: 28, height: 28),
          ),
        ],
      ),
      child: DraftCard(data: data, onTap: onTap, onDelete: onDelete),
    );
  }
}

class DraftCard extends StatelessWidget {
  final FeedSummaryModel data;
  final VoidCallback? onTap;
  final VoidCallback? onDelete;
  final List<String>? tags;
  final bool hideThumb;

  const DraftCard({
    super.key,
    required this.data,
    this.onTap,
    this.onDelete,
    this.tags,
    this.hideThumb = false,
  });

  @override
  Widget build(BuildContext context) {
    final bodyText = _plainTextFromHtml(data.htmlContents);
    final profileImageUrl = data.authorProfileUrl.isNotEmpty
        ? data.authorProfileUrl
        : defaultProfileImage;

    return InkWell(
      borderRadius: BorderRadius.circular(10),
      onTap: onTap,
      child: Container(
        decoration: BoxDecoration(color: const Color(0xFF171717)),
        padding: const EdgeInsets.fromLTRB(15, 14, 15, 14),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Expanded(
                  child: Text(
                    '(Draft) ${data.title}',
                    maxLines: 1,
                    overflow: TextOverflow.ellipsis,
                    style: const TextStyle(
                      color: Colors.white,
                      fontSize: 16,
                      height: 22 / 16,
                      fontWeight: FontWeight.w700,
                    ),
                  ),
                ),
                if (onDelete != null) ...[
                  8.gap,
                  InkWell(
                    onTap: onDelete,
                    borderRadius: BorderRadius.circular(20),
                    child: SvgPicture.asset(
                      Assets.trash,
                      width: 20,
                      height: 20,
                    ),
                  ),
                ],
              ],
            ),
            12.vgap,
            Row(
              children: [
                Expanded(
                  child: Profile(
                    displayName: data.authorDisplayName,
                    profileImageUrl: profileImageUrl,
                  ),
                ),
                Text(
                  timeAgo((data.createdAt / 1000).toInt()),
                  style: const TextStyle(
                    color: Color(0xFF737373),
                    fontSize: 12,
                    height: 1.33,
                    fontWeight: FontWeight.w500,
                  ),
                ),
              ],
            ),
            if (bodyText.isNotEmpty) ...[
              12.vgap,
              Text(
                bodyText,
                maxLines: 2,
                overflow: TextOverflow.ellipsis,
                style: const TextStyle(
                  color: Colors.white,
                  fontSize: 14,
                  height: 20 / 14,
                  fontWeight: FontWeight.w400,
                ),
              ),
            ],
          ],
        ),
      ),
    );
  }
}

String _plainTextFromHtml(String html) {
  if (html.isEmpty) return '';
  final noTags = html.replaceAll(RegExp(r'<[^>]+>'), '');
  return noTags.trim();
}

void showRemoveDraftModal(BuildContext ctx, String feedPk) {
  final controller = Get.find<DraftController>();

  showDialog(
    context: ctx,
    builder: (BuildContext context) {
      return AlertDialog(
        backgroundColor: AppColors.bg,
        surfaceTintColor: AppColors.bg,
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(20.0),
        ),
        content: FittedBox(
          fit: BoxFit.cover,
          child: SizedBox(
            width: 350,
            child: Column(
              children: [
                Text(
                  DraftLocalization.draftDeleteDraft,
                  style: const TextStyle(
                    color: Colors.white,
                    fontWeight: FontWeight.w700,
                    fontSize: 24,
                    height: 32 / 24,
                  ),
                ),
                24.vgap,
                Text(
                  DraftLocalization.draftDeleteDraftDescription,
                  textAlign: TextAlign.center,
                  style: TextStyle(
                    color: AppColors.neutral300,
                    fontWeight: FontWeight.w400,
                    fontSize: 12,
                    height: 22 / 15,
                  ),
                ),
                35.vgap,
                Row(
                  mainAxisAlignment: MainAxisAlignment.center,
                  crossAxisAlignment: CrossAxisAlignment.center,
                  children: [
                    InkWell(
                      onTap: () => Navigator.pop(context),
                      child: RoundContainer(
                        width: 95,
                        height: 50,
                        color: Colors.transparent,
                        radius: 10,
                        child: const Padding(
                          padding: EdgeInsets.fromLTRB(20, 15, 20, 15),
                          child: Text(
                            'Cancel',
                            style: TextStyle(
                              color: AppColors.neutral300,
                              fontWeight: FontWeight.w700,
                              fontSize: 16,
                            ),
                          ),
                        ),
                      ),
                    ),
                    10.gap,
                    InkWell(
                      onTap: controller.isBusy.value
                          ? null
                          : () async {
                              await controller.deleteDraft(feedPk);
                              Navigator.pop(context);
                            },
                      child: RoundContainer(
                        width: 206,
                        height: 50,
                        color: AppColors.primary,
                        radius: 10,
                        child: const Center(
                          child: Padding(
                            padding: EdgeInsets.fromLTRB(20, 15, 20, 15),
                            child: Text(
                              'Delete',
                              style: TextStyle(
                                color: AppColors.bg,
                                fontWeight: FontWeight.w700,
                                fontSize: 16,
                              ),
                            ),
                          ),
                        ),
                      ),
                    ),
                  ],
                ),
              ],
            ),
          ),
        ),
      );
    },
  );
}
