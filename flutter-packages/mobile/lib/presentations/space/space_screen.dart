import 'package:flutter/rendering.dart';
import 'package:ratel/exports.dart';
import 'package:ratel/presentations/space/components/space_header_section.dart';
import 'package:ratel/presentations/space/components/space_stats_section.dart';
import 'package:ratel/presentations/space/components/space_tab_bar.dart';
import 'package:ratel/presentations/space/components/space_title_and_author_section.dart';
import 'package:ratel/presentations/space/components/space_top_bar.dart';
import 'package:ratel/presentations/space/components/space_more_bottom_sheet.dart';

class SpaceScreen extends GetWidget<SpaceController> {
  const SpaceScreen({super.key});

  static const double _collapseThreshold = 56.0;
  static const double _expandThreshold = 36.0;

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

  bool _onAnyScroll(ScrollNotification n) {
    return controller.handleHeaderByScroll(n);
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
                onMore: canReport
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

                final header = Padding(
                  padding: const EdgeInsets.symmetric(horizontal: 16),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      SpaceMetaSection(space: space),
                      20.vgap,
                      SizedBox(
                        width: double.infinity,
                        height: 0.5,
                        child: const ColoredBox(color: Color(0xff464646)),
                      ),
                    ],
                  ),
                );

                return Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    _SlideUpCollapsible(
                      collapsed: collapsed,
                      duration: const Duration(milliseconds: 220),
                      slideUpPx: 28,
                      child: header,
                    ),
                    _AnimatedGap(
                      visible: !collapsed,
                      height: 20,
                      duration: const Duration(milliseconds: 220),
                    ),
                    Padding(
                      padding: const EdgeInsets.symmetric(horizontal: 16),
                      child: SpaceTabBar(controller: controller),
                    ),
                    20.vgap,
                    Expanded(
                      child: Obx(() {
                        final pk = controller.spacePk;
                        if (pk.isEmpty) return const SizedBox.shrink();

                        final base = controller.baseRoute;
                        final current = controller.currentRoute;

                        return NotificationListener<ScrollNotification>(
                          onNotification: _onAnyScroll,
                          child: Listener(
                            behavior: HitTestBehavior.translucent,
                            onPointerMove: (e) =>
                                controller.handlePointerMove(e.delta.dy),
                            onPointerUp: (_) => controller.resetPointer(),
                            onPointerCancel: (_) => controller.resetPointer(),
                            child: Padding(
                              padding: const EdgeInsets.symmetric(
                                horizontal: 16,
                              ),
                              child: GetRouterOutlet(
                                anchorRoute: base,
                                initialRoute: current,
                              ),
                            ),
                          ),
                        );
                      }),
                    ),
                  ],
                );
              }),
            ),
          ],
        ),
      ),
    );
  }
}

class _SlideUpCollapsible extends StatelessWidget {
  const _SlideUpCollapsible({
    required this.collapsed,
    required this.child,
    this.duration = const Duration(milliseconds: 220),
    this.curve = Curves.easeOutCubic,
    this.slideUpPx = 24,
  });

  final bool collapsed;
  final Widget child;
  final Duration duration;
  final Curve curve;
  final double slideUpPx;

  @override
  Widget build(BuildContext context) {
    final target = collapsed ? 1.0 : 0.0;

    return TweenAnimationBuilder<double>(
      tween: Tween<double>(end: target),
      duration: duration,
      curve: curve,
      builder: (context, t, _) {
        final visible = (1.0 - t).clamp(0.0, 1.0);
        return ClipRect(
          child: Align(
            alignment: Alignment.topCenter,
            heightFactor: visible,
            child: Opacity(
              opacity: visible,
              child: Transform.translate(
                offset: Offset(0, -slideUpPx * t),
                child: child,
              ),
            ),
          ),
        );
      },
    );
  }
}

class _AnimatedGap extends StatelessWidget {
  const _AnimatedGap({
    required this.visible,
    required this.height,
    this.duration = const Duration(milliseconds: 220),
    this.curve = Curves.easeOutCubic,
  });

  final bool visible;
  final double height;
  final Duration duration;
  final Curve curve;

  @override
  Widget build(BuildContext context) {
    return TweenAnimationBuilder<double>(
      tween: Tween<double>(end: visible ? height : 0.0),
      duration: duration,
      curve: curve,
      builder: (_, h, __) => SizedBox(height: h),
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
