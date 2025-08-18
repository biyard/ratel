import 'package:ratel/exports.dart';

class MainController extends BaseController {
  final userApi = Get.find<UserApi>();

  @override
  void onInit() {
    super.onInit();
    getUser();
  }

  void getUser() async {
    showLoading();
    final item = await userApi.getUserInfo();
    user(item);

    hideLoading();
  }

  Rx<UserModel> user = UserModel(
    id: 0,
    profileUrl: '',
    nickname: '',
    username: '',
    points: 0,
    followersCount: 0,
    followingsCount: 0,

    teams: [],
  ).obs;
}
