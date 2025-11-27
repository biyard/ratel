import 'package:ratel/exports.dart';

class MainController extends BaseController {
  final userApi = Get.find<UserApi>();
  final feedApi = Get.find<FeedsApi>();

  @override
  void onInit() {
    super.onInit();
    getUser();
  }

  void getUser() async {
    showLoading();
    final item = await userApi.getUserInfoV2();
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

  final Rx<UserV2Model> user = UserV2Model(
    pk: '',
    email: '',
    nickname: '',
    profileUrl: '',
    description: '',
    userType: 0,
    username: '',
    followersCount: 0,
    followingsCount: 0,
    theme: 0,
    point: 0,
    referralCode: null,
    phoneNumber: null,
    principal: null,
    evmAddress: null,
    teams: const [],
  ).obs;
}
