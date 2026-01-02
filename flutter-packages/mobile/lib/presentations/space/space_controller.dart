import 'package:ratel/exports.dart';

class SpaceController extends BaseController {
  final ReportApi reportApi = Get.find<ReportApi>();
  final SpaceService _spaceService = Get.find<SpaceService>();

  late final String spacePk;

  final tabs = <SpaceTab>[].obs;
  final currentTab = Rx<SpaceTab>(SpaceTab.summary());
  final selectedPollSk = RxnString();

  Worker? _spaceWorker;

  Rxn<SpaceModel> get spaceRx => _spaceService.spaceOf(spacePk);
  SpaceModel? get space => spaceRx.value;
  RxBool get isLoading => _spaceService.isLoadingOf(spacePk);

  final isHeaderCollapsed = false.obs;

  static const double collapseThresholdPx = 56.0;
  static const double dragThresholdPx = 56.0;

  static const double scrollExpandThresholdPx = 72.0;
  static const double scrollCollapseThresholdPx = 72.0;

  double _dragSum = 0.0;
  int _lastScrollMs = 0;

  double _expandDragSum = 0.0;
  double _collapseDragSum = 0.0;

  @override
  void onInit() {
    super.onInit();

    final rawPk = Get.parameters['spacePk']!;
    spacePk = Uri.decodeComponent(rawPk);

    _spaceWorker = ever<SpaceModel?>(spaceRx, (space) {
      if (space == null) return;

      if (space.spaceType == SpaceType.poll) {
        tabs.assignAll(_buildTabsForPollSpace());
      } else if (space.spaceType == SpaceType.deliberation) {
        tabs.assignAll(_buildTabsForDeliberationSpace());
      } else {
        tabs.clear();
      }

      if (tabs.isNotEmpty && !tabs.any((t) => t.id == currentTab.value.id)) {
        currentTab.value = tabs.first;
      }
    });

    _spaceService.loadSpace(spacePk);
  }

  @override
  void onClose() {
    _spaceWorker?.dispose();
    super.onClose();
  }

  Future<void> reportSpace({required String spacePk}) async {
    final rx = spaceRx;
    final current = rx.value;
    if (current == null) return;

    try {
      await reportApi.reportSpace(spacePk: spacePk);
      Biyard.info('Reported successfully.');

      current.isReport = true;
      current.reports = (current.reports ?? 0) + 1;

      rx.refresh();
    } catch (e) {
      logger.e('reportSpace error: $e');
      Biyard.error('Report Failed', 'Failed to report. Please try again.');
    }
  }

  String _buildDefaultPollSk() {
    final idx = spacePk.indexOf('#');
    if (idx == -1) {
      return '';
    }
    final suffix = spacePk.substring(idx + 1);
    return 'SPACE_POLL#$suffix';
  }

  void onTabSelected(SpaceTab tab, {String? pollSk}) {
    currentTab.value = tab;

    if (tab.id == 'poll' || tab.id == 'analyze') {
      selectedPollSk.value = pollSk?.isNotEmpty == true
          ? pollSk
          : _buildDefaultPollSk();
    } else {
      selectedPollSk.value = null;
    }
  }

  String get baseRoute => '/space/${Uri.encodeComponent(spacePk)}';
  String get currentRoute => '$baseRoute${currentTab.value.route}';

  void markScrollActivity() {
    _lastScrollMs = DateTime.now().millisecondsSinceEpoch;
  }

  void handlePointerMove(double dy) {
    final now = DateTime.now().millisecondsSinceEpoch;
    if (now - _lastScrollMs < 140) return;

    _dragSum += dy;

    if (_dragSum <= -dragThresholdPx) {
      if (!isHeaderCollapsed.value) isHeaderCollapsed.value = true;
      _dragSum = 0;
      return;
    }

    if (_dragSum >= dragThresholdPx) {
      if (isHeaderCollapsed.value) isHeaderCollapsed.value = false;
      _dragSum = 0;
      return;
    }
  }

  void resetPointer() {
    _dragSum = 0.0;
  }

  void resetScrollDragSums() {
    _expandDragSum = 0.0;
    _collapseDragSum = 0.0;
  }

  bool handleHeaderByScroll(ScrollNotification n) {
    markScrollActivity();

    if (n.metrics.axis != Axis.vertical) return false;

    if (n is ScrollEndNotification) {
      resetScrollDragSums();
      return false;
    }

    if (n is ScrollUpdateNotification) {
      final delta = n.scrollDelta ?? 0.0;

      if (n.dragDetails == null) return false;

      final collapsedNow = isHeaderCollapsed.value;

      if (!collapsedNow) {
        if (delta > 0) {
          _collapseDragSum += delta;
        } else {
          _collapseDragSum = 0.0;
        }

        if (_collapseDragSum >= scrollCollapseThresholdPx ||
            n.metrics.pixels > collapseThresholdPx) {
          isHeaderCollapsed.value = true;
          resetScrollDragSums();
        }

        return false;
      }

      final minExtent = n.metrics.minScrollExtent;
      final atTop = n.metrics.pixels <= (minExtent + 0.5);

      if (!atTop) {
        _expandDragSum = 0.0;
        return false;
      }

      if (delta < 0) {
        _expandDragSum += -delta;
      } else {
        _expandDragSum = 0.0;
      }

      if (_expandDragSum >= scrollExpandThresholdPx) {
        isHeaderCollapsed.value = false;
        resetScrollDragSums();
      }

      return false;
    }

    return false;
  }

  List<SpaceTab> _buildTabsForPollSpace() {
    final baseTabs = <SpaceTab>[
      SpaceTab(id: 'overview', label: 'Overview', route: '/overview'),
      SpaceTab(id: 'poll', label: 'Polls', route: '/poll'),
    ];

    final isAdmin = space?.isAdmin ?? false;

    if (isAdmin) {
      baseTabs.addAll([
        SpaceTab(id: 'analyze', label: 'Analyze', route: '/analyze'),
        // SpaceTab(id: 'setting', label: 'Settings', route: '/setting'),
      ]);
    }

    return baseTabs;
  }

  List<SpaceTab> _buildTabsForDeliberationSpace() {
    final baseTabs = <SpaceTab>[
      SpaceTab(id: 'overview', label: 'Overview', route: '/overview'),
      SpaceTab(id: 'file', label: 'Files', route: '/file'),
      SpaceTab(id: 'poll', label: 'Polls', route: '/polls'),
      SpaceTab(id: 'board', label: 'Boards', route: '/boards'),
    ];

    final isAdmin = space?.isAdmin ?? false;

    if (isAdmin) {
      baseTabs.addAll([
        SpaceTab(id: 'analyze', label: 'Analyze', route: '/analyzes'),
      ]);
    }

    return baseTabs;
  }
}

class SpaceTab {
  final String id;
  final String label;
  final String route;

  SpaceTab({required this.id, required this.label, required this.route});

  factory SpaceTab.summary() =>
      SpaceTab(id: 'summary', label: 'Summary', route: '/summary');
}
