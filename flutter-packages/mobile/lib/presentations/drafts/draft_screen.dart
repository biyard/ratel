import 'package:flutter_slidable/flutter_slidable.dart';
import 'package:ratel/exports.dart';

class DraftScreen extends GetWidget<DraftController> {
  const DraftScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Layout<DraftController>(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Padding(
            padding: const EdgeInsets.fromLTRB(24, 20, 24, 20),
            child: Row(
              children: [
                InkWell(
                  onTap: controller.goBack,
                  child: SvgPicture.asset(Assets.back, width: 24, height: 24),
                ),
                20.gap,
                const Text(
                  'My Drafts',
                  style: TextStyle(
                    color: Colors.white,
                    fontSize: 24,
                    fontWeight: FontWeight.w700,
                    height: 1.2,
                  ),
                ),
              ],
            ),
          ),
          Container(
            width: double.infinity,
            height: 1,
            color: const Color(0xff464646),
          ),
          Padding(
            padding: const EdgeInsets.all(10.0),
            child: Obx(
              () => ListView.separated(
                primary: false,
                shrinkWrap: true,
                itemCount: controller.feeds.length,
                separatorBuilder: (_, __) => const SizedBox(height: 10),
                itemBuilder: (_, i) => DraftSlidableCard(
                  data: controller.feeds[i],
                  onTap: () => controller.openDraft(controller.feeds[i].feedId),
                  onDelete: () => {
                    showRemoveDraftModal(context, controller.feeds[i].feedId),
                  },
                ),
              ),
            ),
          ),
        ],
      ),
    );
  }
}

class DraftSlidableCard extends StatelessWidget {
  final FeedModel data;
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
      key: ValueKey('draft_${data.feedId}'),
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
      child: DraftCard(data: data, onTap: onTap),
    );
  }
}

class DraftCard extends StatelessWidget {
  final FeedModel data;
  final VoidCallback? onTap;
  final List<String>? tags;
  final bool hideThumb;

  const DraftCard({
    super.key,
    required this.data,
    this.onTap,
    this.tags,
    this.hideThumb = false,
  });

  @override
  Widget build(BuildContext context) {
    final showThumb =
        !hideThumb && (data.image != null && data.image!.isNotEmpty);

    return InkWell(
      onTap: onTap,
      borderRadius: BorderRadius.circular(16),
      child: RoundContainer(
        width: double.infinity,
        radius: 10,
        color: AppColors.neutral900,
        padding: const EdgeInsets.all(15),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            if (data.feedType.isNotEmpty) ...[
              Wrap(
                spacing: 4,
                runSpacing: 4,
                children: [DraftTagChip(text: data.feedType)],
              ),
              15.vgap,
            ],
            Row(
              crossAxisAlignment: CrossAxisAlignment.center,
              children: [
                if (showThumb) ...[DraftThumbnail(url: data.image!), 10.gap],
                Expanded(
                  child: Text(
                    data.title,
                    maxLines: 2,
                    overflow: TextOverflow.ellipsis,
                    style: const TextStyle(
                      color: Colors.white,
                      fontSize: 18,
                      height: 1.25,
                      fontWeight: FontWeight.w700,
                    ),
                  ),
                ),
              ],
            ),
            15.vgap,
            Row(
              mainAxisAlignment: MainAxisAlignment.end,
              children: [
                Text(
                  timeAgo(data.createdAt),
                  style: const TextStyle(
                    color: AppColors.neutral500,
                    fontSize: 12,
                    height: 1.2,
                    fontWeight: FontWeight.w500,
                  ),
                ),
              ],
            ),
          ],
        ),
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
      padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4.5),
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
          height: 1.1,
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

void showRemoveDraftModal(BuildContext ctx, int feedId) {
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
                  "Delete Draft",
                  style: TextStyle(
                    color: Colors.white,
                    fontWeight: FontWeight.w700,
                    fontSize: 24,
                    height: 1.33,
                  ),
                ),
                24.vgap,
                Text(
                  "Could you remove this draft? This action cannot be undone.",
                  style: TextStyle(
                    color: AppColors.neutral300,
                    fontWeight: FontWeight.w400,
                    fontSize: 12,
                    height: 1.33,
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
                        child: Padding(
                          padding: EdgeInsets.fromLTRB(20, 15, 20, 15),
                          child: Text(
                            "Cancel",
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
                              await controller.deleteDraft(feedId);
                              Navigator.pop(context);
                            },
                      child: RoundContainer(
                        width: 180,
                        height: 50,
                        color: AppColors.primary,
                        radius: 10,
                        child: Center(
                          child: Padding(
                            padding: EdgeInsets.fromLTRB(20, 15, 20, 15),
                            child: Text(
                              "Remove",
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
