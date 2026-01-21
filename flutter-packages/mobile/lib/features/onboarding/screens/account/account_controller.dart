import 'package:ratel/exports.dart';

class AccountController extends BaseController {
  final AuthService auth = Get.find<AuthService>();

  RxList<AccountItemModel> get accounts => auth.accounts;
  RxnString get bookmark => auth.accountsBookmark;
  RxBool get isLoading => auth.accountsBusy;
  RxnString get error => auth.accountsError;

  @override
  void onInit() {
    super.onInit();
    fetchAccounts(refresh: true);
  }

  Future<void> fetchAccounts({bool refresh = false}) async {
    await auth.loadAccounts(refresh: refresh);
  }

  Future<void> onSelectAccount(AccountItemModel a) async {
    final ok = await auth.changeAccount(a);
    if (!ok) {
      Biyard.error("Change Account Failed", 'Failed to change account');
      return;
    }
    Get.rootDelegate.offNamed(AppRoutes.mainScreen);
    Biyard.info("Login Successed.");
  }

  void onAddAnotherAccount() {
    Get.rootDelegate.toNamed(loginScreen);
  }

  void onSignup() {
    Get.rootDelegate.toNamed(signupScreen);
  }
}
