import 'package:ratel/exports.dart';
import 'package:ratel/presentations/my_space/components/my_space_list_item.dart';

class MySpaceScreen extends GetWidget<MySpaceController> {
  const MySpaceScreen({super.key});

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);

    return Layout<MySpaceController>(
      scrollable: false,
      child: SafeArea(
        bottom: false,
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Padding(
              padding: const EdgeInsets.fromLTRB(10, 0, 10, 4),
              child: Text(
                'My Spaces',
                style: theme.textTheme.titleLarge?.copyWith(
                  color: Colors.white,
                  fontWeight: FontWeight.w700,
                  fontSize: 22,
                ),
              ),
            ),
            Padding(
              padding: const EdgeInsets.fromLTRB(10, 0, 10, 8),
              child: Text(
                'Spaces you’re participating in',
                style: theme.textTheme.bodySmall?.copyWith(
                  color: AppColors.neutral500,
                  fontSize: 13,
                ),
              ),
            ),
            4.vgap,
            Expanded(child: _MySpaceList(controller: controller)),
          ],
        ),
      ),
    );
  }
}

class _MySpaceList extends StatelessWidget {
  final MySpaceController controller;

  const _MySpaceList({required this.controller});

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);

    return Obx(() {
      final isFirstLoading =
          controller.isLoading.value &&
          controller.pendingSpaces.isEmpty &&
          controller.participatingSpaces.isEmpty;

      if (isFirstLoading) {
        return const Center(
          child: SizedBox(
            width: 24,
            height: 24,
            child: CircularProgressIndicator(strokeWidth: 2),
          ),
        );
      }

      final items = <MySpaceItem>[
        ...controller.participatingSpaces,
        ...controller.pendingSpaces,
      ];

      if (items.isEmpty) {
        return RefreshIndicator(
          onRefresh: controller.refreshSpaces,
          child: ListView(
            physics: const AlwaysScrollableScrollPhysics(),
            children: [
              100.vgap,
              Center(
                child: Text(
                  'No spaces yet.',
                  style: theme.textTheme.bodyMedium?.copyWith(
                    color: AppColors.neutral500,
                  ),
                ),
              ),
            ],
          ),
        );
      }

      return RefreshIndicator(
        onRefresh: controller.refreshSpaces,
        color: AppColors.primary,
        backgroundColor: AppColors.bg,
        child: ListView.separated(
          physics: const AlwaysScrollableScrollPhysics(),
          itemCount: items.length + (controller.hasMore ? 1 : 0),
          separatorBuilder: (_, index) {
            if (controller.hasMore && index == items.length - 1) {
              return const SizedBox(height: 8);
            }

            return Column(
              children: [
                10.vgap,
                Padding(
                  padding: const EdgeInsets.fromLTRB(10, 0, 10, 0),
                  child: Container(
                    width: double.infinity,
                    height: 1,
                    color: const Color(0xff464646),
                  ),
                ),
                10.vgap,
              ],
            );
          },
          itemBuilder: (context, index) {
            final hasMore = controller.hasMore;

            if (hasMore && index == items.length) {
              final isLoadingMore = controller.isLoadingMore.value;

              if (!isLoadingMore) {
                WidgetsBinding.instance.addPostFrameCallback((_) {
                  controller.loadMoreSpaces();
                });
              }

              if (isLoadingMore) {
                return const Padding(
                  padding: EdgeInsets.symmetric(vertical: 8),
                  child: Center(
                    child: SizedBox(
                      width: 20,
                      height: 20,
                      child: CircularProgressIndicator(strokeWidth: 2),
                    ),
                  ),
                );
              }

              return const SizedBox(height: 16);
            }

            final item = items[index];

            return MySpaceListItem(
              item: item,
              onTap: () {
                if (item.isClosed) {
                  Biyard.error('Closed Space', 'This space is closed.');
                  return;
                }
                Get.rootDelegate.toNamed(spaceWithPk(item.pk));
              },
            );
          },
        ),
      );
    });
  }
}
