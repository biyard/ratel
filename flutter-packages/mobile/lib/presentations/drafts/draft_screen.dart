import 'package:flutter_slidable/flutter_slidable.dart';
import 'package:ratel/exports.dart';

class DraftScreen extends GetWidget<DraftController> {
  const DraftScreen({super.key});

  @override
  Widget build(BuildContext context) {
    final bottomPad = MediaQuery.of(context).padding.bottom;

    return Layout<DraftController>(
      scrollable: false,
      child: Obx(
        () => RefreshIndicator(
          onRefresh: () => controller.listFeeds(),
          color: AppColors.primary,
          backgroundColor: AppColors.bg,
          child: ListView.separated(
            padding: EdgeInsets.fromLTRB(0, 0, 0, bottomPad + 10),
            itemCount: controller.feeds.length + 1,
            separatorBuilder: (_, __) => const SizedBox(height: 10),
            itemBuilder: (context, index) {
              if (index == 0) {
                return Padding(
                  padding: const EdgeInsets.all(20.0),
                  child: AppTopBar(
                    onBack: () => Get.back(),
                    title: DraftLocalization.draftMyDraft,
                  ),
                );
              }

              final i = index - 1;
              return DraftSlidableCard(
                data: controller.feeds[i],
                onTap: () => controller.openDraft(controller.feeds[i].pk),
                onDelete: () {
                  showRemoveDraftModal(context, controller.feeds[i].pk);
                },
              );
            },
          ),
        ),
      ),
    );
  }
}

class DraftSlidableCard extends StatelessWidget {
  final FeedV2SummaryModel data;
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
  final FeedV2SummaryModel data;
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
    final allTags = tags ?? const <String>[];
    final visibleTags = allTags.length > 4
        ? allTags.sublist(0, 4)
        : allTags.toList();
    final extraCount = allTags.length > 4 ? allTags.length - 4 : 0;

    return InkWell(
      onTap: onTap,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          RoundContainer(
            width: double.infinity,
            radius: 0,
            color: Colors.transparent,
            padding: const EdgeInsets.all(15),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Row(
                  crossAxisAlignment: CrossAxisAlignment.center,
                  children: [
                    Expanded(
                      child: Wrap(
                        spacing: 4,
                        runSpacing: 4,
                        children: [
                          ...visibleTags
                              .map((t) => DraftTagChip(text: t))
                              .toList(),
                          if (extraCount > 0)
                            DraftTagChip(text: '+$extraCount'),
                        ],
                      ),
                    ),
                    if (onDelete != null) ...[
                      10.gap,
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
                15.vgap,
                Text(
                  data.title,
                  maxLines: 2,
                  overflow: TextOverflow.ellipsis,
                  style: const TextStyle(
                    color: Colors.white,
                    fontSize: 18,
                    height: 27 / 18,
                    fontWeight: FontWeight.w700,
                  ),
                ),
                15.vgap,
                Row(
                  mainAxisAlignment: MainAxisAlignment.end,
                  children: [
                    Text(
                      timeAgo(data.createdAt),
                      style: const TextStyle(
                        color: Color(0xFF737373),
                        fontSize: 12,
                        height: 1.33,
                        fontWeight: FontWeight.w500,
                      ),
                    ),
                    30.gap,
                  ],
                ),
              ],
            ),
          ),
          Container(height: 1, color: const Color(0xFF2D2D2D)),
        ],
      ),
    );
  }
}

class DraftTagChip extends StatelessWidget {
  final String text;
  const DraftTagChip({super.key, required this.text});

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 3),
      decoration: BoxDecoration(
        color: const Color(0xFF262626),
        borderRadius: BorderRadius.circular(4),
      ),
      child: Text(
        text,
        style: const TextStyle(
          color: Colors.white,
          fontSize: 12,
          fontWeight: FontWeight.w600,
          height: 1.33,
        ),
      ),
    );
  }
}

class DraftThumbnail extends StatelessWidget {
  final String url;
  const DraftThumbnail({super.key, required this.url});

  @override
  Widget build(BuildContext context) {
    return ClipRRect(
      borderRadius: BorderRadius.circular(8),
      child: SizedBox(
        width: 54,
        height: 54,
        child: Image.network(
          url,
          fit: BoxFit.cover,
          errorBuilder: (_, __, ___) {
            return RoundContainer(
              color: AppColors.neutral500,
              radius: 8,
              width: 54,
              height: 54,
            );
          },
        ),
      ),
    );
  }
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
