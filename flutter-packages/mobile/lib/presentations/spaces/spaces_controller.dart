import 'package:flutter/rendering.dart';
import 'package:ratel/exports.dart';

class SpacesController extends BaseController {
  final scrollCtrl = ScrollController();
  final boostingHeaderKey = GlobalKey();
  final chipsKey = GlobalKey();
  final isBoostingSection = false.obs;

  final spaceApi = Get.find<SpaceApi>();

  @override
  void onInit() {
    super.onInit();
    listFeeds();
    scrollCtrl.addListener(_onScroll);
    WidgetsBinding.instance.addPostFrameCallback((_) {
      isBoostingSection.value = false;
    });
  }

  void listFeeds() async {
    showLoading();
    final items = await spaceApi.getMySpaces();
    logger.d("item length: ${items.boostings.length} ${items.spaces.length}");
    mySpaces(items.spaces);
    boostings(items.boostings);
    hideLoading();
  }

  RxList<SpaceSummary> mySpaces = <SpaceSummary>[].obs;
  RxList<SpaceSummary> boostings = <SpaceSummary>[].obs;

  @override
  void onClose() {
    scrollCtrl.dispose();
    super.onClose();
  }

  void _onScroll() {
    final dir = scrollCtrl.position.userScrollDirection;
    if (dir == ScrollDirection.reverse) {
      if (!isBoostingSection.value) isBoostingSection.value = true;
    } else if (dir == ScrollDirection.forward) {
      if (isBoostingSection.value) isBoostingSection.value = false;
    }
    if (scrollCtrl.position.atEdge && scrollCtrl.position.pixels <= 0) {
      if (isBoostingSection.value) isBoostingSection.value = false;
    }
  }

  void scrollToMySpaces() {
    if (isBoostingSection.value) isBoostingSection.value = false;

    if (scrollCtrl.hasClients) {
      scrollCtrl.animateTo(
        0,
        duration: const Duration(milliseconds: 320),
        curve: Curves.easeOutCubic,
      );
    }
  }

  void routingSpace(int spaceId) {
    Get.rootDelegate.offNamed(AppRoutes.spaceWithId(spaceId));
  }

  void scrollToBoosting() {
    if (!isBoostingSection.value) isBoostingSection.value = true;

    if (scrollCtrl.hasClients) {
      final target = scrollCtrl.position.maxScrollExtent * 0.6;
      scrollCtrl.animateTo(
        target,
        duration: const Duration(milliseconds: 320),
        curve: Curves.easeInOutCubic,
      );
    }
  }

  static String formatTime(int ts) {
    final now = DateTime.now().millisecondsSinceEpoch;
    final diff = (now - ts).clamp(0, 1 << 31);
    final m = diff ~/ (60 * 1000);
    if (m < 1) return 'now';
    if (m < 60) return '${m}m';
    final h = m ~/ 60;
    if (h < 24) return '${h}h';
    final d = h ~/ 24;
    return '${d}d';
  }

  static String boostLabel(int? t) {
    switch (t) {
      case 2:
        return 'x2';
      case 3:
        return 'x10';
      case 4:
        return 'x100';
      default:
        return '';
    }
  }

  static String kFormat(int n) =>
      n >= 1000 ? '${(n / 1000).toStringAsFixed(1)}K' : '$n';
}
