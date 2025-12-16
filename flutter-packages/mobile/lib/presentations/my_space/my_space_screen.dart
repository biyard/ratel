import 'package:ratel/exports.dart';
import 'package:ratel/presentations/my_space/components/my_space_list_item.dart';

class MySpaceScreen extends GetWidget<MySpaceController> {
  const MySpaceScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Layout<MySpaceController>(
      enableSafeArea: false,
      scrollable: false,
      child: SafeArea(
        bottom: false,
        child: _MySpaceList(controller: controller),
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
    final bottomPad = MediaQuery.of(context).padding.bottom;

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
          color: AppColors.primary,
          backgroundColor: AppColors.bg,
          child: ListView(
            physics: const AlwaysScrollableScrollPhysics(),
            padding: EdgeInsets.fromLTRB(0, 10, 0, bottomPad + 10),
            children: [
              const Header(title: 'My Spaces'),
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

      final hasMore = controller.hasMore;
      final itemCount = 1 + items.length + (hasMore ? 1 : 0);

      return RefreshIndicator(
        onRefresh: controller.refreshSpaces,
        color: AppColors.primary,
        backgroundColor: AppColors.bg,
        child: ListView.separated(
          physics: const AlwaysScrollableScrollPhysics(),
          padding: EdgeInsets.fromLTRB(0, 10, 0, bottomPad + 10),
          itemCount: itemCount,
          separatorBuilder: (_, index) {
            if (index == 0) return 10.vgap;

            final listIndex = index - 1;
            if (hasMore && listIndex == items.length - 1) {
              return const SizedBox(height: 8);
            }

            return Column(
              children: [
                10.vgap,
                const Padding(
                  padding: EdgeInsets.fromLTRB(10, 0, 10, 0),
                  child: SizedBox(
                    width: double.infinity,
                    height: 0.5,
                    child: ColoredBox(color: Color(0xff464646)),
                  ),
                ),
                10.vgap,
              ],
            );
          },
          itemBuilder: (context, index) {
            if (index == 0) {
              return Column(
                children: [
                  const Header(title: 'My Spaces'),
                  15.vgap,
                ],
              );
            }

            final listIndex = index - 1;

            if (hasMore && listIndex == items.length) {
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
            }

            final item = items[listIndex];

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
