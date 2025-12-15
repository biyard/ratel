import 'package:ratel/exports.dart';

class MainController extends BaseController {
  final userService = Get.find<UserService>();
  final feedApi = Get.find<FeedsApi>();

  void getUser() async {
    showLoading();
    final item = await userService.getUser();
    user(item);

    hideLoading();
  }

  Future<void> createPost() async {
    final postPk = await feedApi.createPost();
    logger.d("postPk: $postPk");
    if (postPk.isNotEmpty) {
      Get.rootDelegate.toNamed(createPostScreen, arguments: {'postPk': postPk});
    }
  }

  Rx<UserModel> get user => userService.user;
}
