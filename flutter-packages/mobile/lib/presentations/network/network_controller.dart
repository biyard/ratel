import 'package:ratel/exports.dart';

class NetworkController extends BaseController {
  @override
  void onInit() {
    listNetworks();
    super.onInit();
  }

  void listNetworks() async {
    final items = await networkApi.getNetworksByV1();
    logger.d(
      "items length: ${items.followers.length} ${items.followings.length}",
    );
    invitations(items.followings);
    suggestions(items.followers);
  }

  final networkApi = Get.find<NetworkApi>();

  RxList<NetworkModel> invitations = <NetworkModel>[].obs;
  RxList<NetworkModel> suggestions = <NetworkModel>[].obs;
}
