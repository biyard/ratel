import 'package:ratel/exports.dart';

class HomeController extends BaseController {
  Rx<ProfileModel> profile = ProfileModel(
    profile:
        'https://lh3.googleusercontent.com/a/ACg8ocIGf0gpB8MQdGkp5TXW1327nRpuPz70iy_hQY2NXNwanRXbFw=s96-c',
    nickname: 'Hyejin Choi',
  ).obs;
}
