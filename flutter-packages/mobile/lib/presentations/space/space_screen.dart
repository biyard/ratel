import 'package:ratel/exports.dart';
import 'package:ratel/presentations/space/components/space_header_section.dart';
import 'package:ratel/presentations/space/components/space_stats_section.dart';
import 'package:ratel/presentations/space/components/space_tab_bar.dart';
import 'package:ratel/presentations/space/components/space_title_and_author_section.dart';
import 'package:ratel/presentations/space/components/space_top_bar.dart';
import 'package:ratel/presentations/space/components/space_more_bottom_sheet.dart';

class SpaceScreen extends GetWidget<SpaceController> {
  const SpaceScreen({super.key});

  Future<void> _openSpaceActionSheet(
    BuildContext context, {
    required String spacePk,
  }) async {
    await showModalBottomSheet(
      context: context,
      backgroundColor: const Color(0xFF191919),
      shape: const RoundedRectangleBorder(
        borderRadius: BorderRadius.vertical(top: Radius.circular(20)),
      ),
      builder: (_) {
        return SpaceMoreBottomSheet(
          onReport: () async {
            Navigator.pop(context);
            await controller.reportSpace(spacePk: spacePk);
          },
        );
      },
    );
  }

  @override
  Widget build(BuildContext context) {
    return Layout<SpaceController>(
      scrollable: false,
      child: Container(
        color: const Color(0xFF1D1D1D),
        child: Column(
          children: [
            Obx(() {
              final space = controller.space;
              final canReport = space != null && (space.isReport != true);

              return SpaceTopBar(
                onBack: () => Get.back(),
                showMore: canReport,
                onMore: canReport && space != null
                    ? () => _openSpaceActionSheet(context, spacePk: space.pk)
                    : null,
              );
            }),
            Expanded(
              child: Obx(() {
                final space = controller.space;
                final collapsed = controller.isHeaderCollapsed.value;

                if (space == null) {
                  return const Center(
                    child: SizedBox(
                      width: 24,
                      height: 24,
                      child: CircularProgressIndicator(strokeWidth: 2),
                    ),
                  );
                }

                return Listener(
                  onPointerMove: (event) =>
                      controller.handlePointerMove(event.delta.dy),
                  onPointerUp: (_) => controller.resetPointer(),
                  onPointerCancel: (_) => controller.resetPointer(),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      AnimatedSwitcher(
                        duration: const Duration(milliseconds: 220),
                        switchInCurve: Curves.easeOut,
                        switchOutCurve: Curves.easeIn,
                        child: collapsed
                            ? const SizedBox.shrink(
                                key: ValueKey('header_collapsed'),
                              )
                            : Padding(
                                key: const ValueKey('header_expanded'),
                                padding: const EdgeInsets.symmetric(
                                  horizontal: 16,
                                ),
                                child: Column(
                                  crossAxisAlignment: CrossAxisAlignment.start,
                                  children: [
                                    SpaceMetaSection(space: space),
                                    20.vgap,
                                    Container(
                                      width: double.infinity,
                                      height: 0.3,
                                      color: const Color(0xFFD4D4D4),
                                    ),
                                  ],
                                ),
                              ),
                      ),
                      if (!collapsed) 20.vgap,
                      Padding(
                        padding: const EdgeInsets.symmetric(horizontal: 16),
                        child: SpaceTabBar(controller: controller),
                      ),
                      20.vgap,
                      Expanded(
                        child: Obx(() {
                          final pk = controller.spacePk;
                          if (pk.isEmpty) {
                            return const SizedBox.shrink();
                          }

                          final base = controller.baseRoute;
                          final current = controller.currentRoute;

                          logger.d('Space content route: $current');

                          return Padding(
                            padding: const EdgeInsets.symmetric(horizontal: 16),
                            child: GetRouterOutlet(
                              anchorRoute: base,
                              initialRoute: current,
                            ),
                          );
                        }),
                      ),
                    ],
                  ),
                );
              }),
            ),
          ],
        ),
      ),
    );
  }
}

class SpaceMetaSection extends StatelessWidget {
  const SpaceMetaSection({super.key, required this.space});

  final SpaceModel space;

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        SpaceHeaderSection(space: space),
        10.vgap,
        SpaceTitleAndAuthorSection(space: space),
        20.vgap,
        SpaceStatsSection(space: space),
      ],
    );
  }
}
