import 'package:ratel/exports.dart';

class IntroMiddleware extends GetMiddleware {
  @override
  Future<GetNavConfig?> redirectDelegate(GetNavConfig route) async {
    final auth = AuthApi();
    final user = UserApi();
    await auth.init();
    final res = await user.getUserInfoV2();

    if (res.pk == "") {
      return route;
    }

    final ok = await auth.tryAutoSignIn();

    if (ok) {
      return GetNavConfig.fromRoute(AppRoutes.mainScreen);
    } else {
      return route;
    }
  }
}
