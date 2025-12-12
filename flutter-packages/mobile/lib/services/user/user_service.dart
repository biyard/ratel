import 'package:ratel/exports.dart';

class UserService extends GetxService {
  final Rx<UserModel> user = UserModel(
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

  static void init() {
    Get.put<UserService>(UserService());
    Get.put<UserApi>(UserApi());
  }

  Future<UserModel> getUser() async {
    final userApi = Get.find<UserApi>();
    final item = await userApi.getUserInfoV2();
    user(item);

    return item;
  }
}
