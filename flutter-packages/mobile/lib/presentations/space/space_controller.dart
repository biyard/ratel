import 'package:ratel/exports.dart';

class SpaceController extends BaseController {
  final SpaceService _spaceService = Get.find<SpaceService>();

  late final String spacePk;

  final tabs = <SpaceTab>[].obs;
  final currentTab = Rx<SpaceTab>(SpaceTab.summary());

  Rxn<SpaceModel> get spaceRx => _spaceService.spaceOf(spacePk);

  SpaceModel? get space => spaceRx.value;

  RxBool get isLoading => _spaceService.isLoadingOf(spacePk);

  @override
  void onInit() {
    super.onInit();

    final rawPk = Get.parameters['spacePk']!;
    spacePk = Uri.decodeComponent(rawPk);
    _spaceService.loadSpace(spacePk);

    tabs.assignAll(_buildTabsForSpace());
  }

  void onTabSelected(SpaceTab tab) {
    currentTab.value = tab;

    final base = '/space/${Uri.encodeComponent(spacePk)}';
    Get.rootDelegate.toNamed('$base${tab.route}');
  }

  List<SpaceTab> _buildTabsForSpace() {
    return [
      SpaceTab(id: 'summary', label: 'Summary', route: '/summary'),
      SpaceTab(id: 'delib', label: 'Deliberation', route: '/deliberation'),
      SpaceTab(id: 'elearning', label: 'E-Learning', route: '/elearning'),
      SpaceTab(id: 'poll', label: 'Poll', route: '/poll'),
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
