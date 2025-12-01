import 'package:ratel/exports.dart';

class SpaceController extends BaseController {
  final spaceApi = Get.find<SpaceApi>();
  final space = Rxn<SpaceModel>();

  late final String spacePk;

  final tabs = <SpaceTab>[].obs;
  final currentTab = Rx<SpaceTab>(SpaceTab.summary());

  @override
  void onInit() {
    super.onInit();

    spacePk = Get.parameters['spacePk']!;
    _loadSpace();

    tabs.assignAll(_buildTabsForSpace());
  }

  void onTabSelected(SpaceTab tab) {
    currentTab.value = tab;

    final base = '/space/$spacePk';
    Get.rootDelegate.toNamed('$base${tab.route}');
  }

  Future<void> _loadSpace() async {
    try {
      final result = await spaceApi.getSpace(spacePk);

      if (result == null) {
        logger.e('Failed to load space: null response for $spacePk');
        return;
      }

      logger.d(
        "Loaded space: ${result.pk}, ${result.authorUsername}, ${result.isAdmin}",
      );

      space.value = result;
    } catch (e, s) {
      logger.e('Failed to load space $spacePk: $e', stackTrace: s);
    }
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
