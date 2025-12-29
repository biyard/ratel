import 'package:ratel/exports.dart';

class AccountMiddleware extends GetMiddleware {
  @override
  Future<GetNavConfig?> redirectDelegate(GetNavConfig route) async {
    final auth = AuthService();
    await auth.loadAccounts(refresh: true);

    if (auth.accounts.isEmpty) {
      return GetNavConfig.fromRoute(loginScreen);
    } else {
      return route;
    }
  }
}
