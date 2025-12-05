import 'package:ratel/exports.dart';

class SpaceController extends BaseController {
  final SpaceService _spaceService = Get.find<SpaceService>();

  late final String spacePk;

  final tabs = <SpaceTab>[].obs;
  final currentTab = Rx<SpaceTab>(SpaceTab.summary());
  final selectedPollSk = RxnString();

  Worker? _spaceWorker;

  Rxn<SpaceModel> get spaceRx => _spaceService.spaceOf(spacePk);
  SpaceModel? get space => spaceRx.value;
  RxBool get isLoading => _spaceService.isLoadingOf(spacePk);

  final RxBool isHeaderCollapsed = false.obs;
  double _gestureDelta = 0;

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

  void handlePointerMove(double dy) {
    const threshold = 24.0;
    _gestureDelta += dy;

    if (_gestureDelta <= -threshold && !isHeaderCollapsed.value) {
      isHeaderCollapsed.value = true;
      _gestureDelta = 0;
    } else if (_gestureDelta >= threshold && isHeaderCollapsed.value) {
      isHeaderCollapsed.value = false;
      _gestureDelta = 0;
    }
  }

  void resetPointer() {
    _gestureDelta = 0;
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
        SpaceTab(id: 'setting', label: 'Settings', route: '/setting'),
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
        SpaceTab(id: 'member', label: 'Members', route: '/member'),
        SpaceTab(id: 'panel', label: 'Panels', route: '/panel'),
        SpaceTab(id: 'analyze', label: 'Analyze', route: '/analyzes'),
        SpaceTab(id: 'setting', label: 'Settings', route: '/setting'),
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
