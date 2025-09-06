import 'package:ratel/exports.dart';

class ConnectionController extends BaseController {
  @override
  void onInit() {
    listNetworks();
    super.onInit();
  }

  void listNetworks() async {
    final items = await networkApi.getConnections();
    logger.d("items length: ${items.length}");
    networks(items);
  }

  final networkApi = Get.find<NetworkApi>();

  final query = ''.obs;

  RxList<NetworkModel> networks = <NetworkModel>[].obs;

  final followed = <int>{}.obs;

  List<NetworkModel> get filtered {
    final q = query.value.trim().toLowerCase();
    if (q.isEmpty) return networks;
    return networks
        .where(
          (n) =>
              n.nickname.toLowerCase().contains(q) ||
              n.description.toLowerCase().contains(q),
        )
        .toList();
  }

  bool get hasFollowed => followed.isNotEmpty;

  Future<void> onSearchChanged(String v) async {
    query.value = v;
    final items = await networkApi.getConnectionByKeyword(v);
    networks(items);
  }

  void toggleFollow(int userId) {
    if (followed.contains(userId)) {
      followed.remove(userId);
    } else {
      followed.add(userId);
    }
  }

  Future<void> next() async {
    final network = NetworkApi();
    final ids = followed.toList();

    try {
      final res = await network.connectionFollow(ids);

      if (res != null) {
        Get.rootDelegate.offNamed(AppRoutes.setupAttributeScreen);
      } else {
        Biyard.error(
          "Failed to follow user.",
          "Follow user is failed. Please try again later.",
        );
      }
    } finally {}
  }

  void goBack() => Get.rootDelegate.offNamed(AppRoutes.selectTopicScreen);
  void skip() => Get.rootDelegate.offNamed(AppRoutes.setupAttributeScreen);
}
