import 'package:ratel/exports.dart';

class SpaceController extends BaseController {
  final SpaceService _spaceService = Get.find<SpaceService>();

  late final String spacePk;

  final tabs = <SpaceTab>[].obs;
  final currentTab = Rx<SpaceTab>(SpaceTab.summary());
  Worker? _spaceWorker;

  Rxn<SpaceModel> get spaceRx => _spaceService.spaceOf(spacePk);

  SpaceModel? get space => spaceRx.value;

  RxBool get isLoading => _spaceService.isLoadingOf(spacePk);

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

  void onTabSelected(SpaceTab tab) {
    currentTab.value = tab;
  }

  String get baseRoute => '/space/${Uri.encodeComponent(spacePk)}';

  String get currentRoute => '$baseRoute${currentTab.value.route}';

  List<SpaceTab> _buildTabsForPollSpace() {
    return [
      SpaceTab(id: 'overview', label: 'Overview', route: '/overview'),
      SpaceTab(id: 'poll', label: 'Polls', route: '/poll'),
      SpaceTab(id: 'analyze', label: 'Analyze', route: '/analyze'),
      SpaceTab(id: 'setting', label: 'Settings', route: '/setting'),
    ];
  }

  List<SpaceTab> _buildTabsForDeliberationSpace() {
    return [
      SpaceTab(id: 'overview', label: 'Overview', route: '/overview'),
      SpaceTab(id: 'file', label: 'Files', route: '/file'),
      SpaceTab(id: 'poll', label: 'Polls', route: '/polls'),
      SpaceTab(id: 'board', label: 'Boards', route: '/board'),
      SpaceTab(id: 'member', label: 'Members', route: '/member'),
      SpaceTab(id: 'panel', label: 'Panels', route: '/panel'),
      SpaceTab(id: 'analyze', label: 'Analyze', route: '/analyze'),
      SpaceTab(id: 'setting', label: 'Settings', route: '/setting'),
    ];
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
