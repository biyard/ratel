import 'package:ratel/exports.dart';
import 'package:ratel/services/rust/rust_service.dart';

class AuthService extends GetxService {
  final RustService rust = Get.find<RustService>();
  final AnonymousService anonymous = Get.find<AnonymousService>();
  final ByFirebase firebase = Get.find<ByFirebase>();

  String? idToken;
  String provider = '';

  String? principal;
  String? privateKey;
  String? publicKey;
  String? pkcs8;

  String? email;
  String? nickname;
  String? profileUrl;

  bool neededSignup = false;

  final RxList<AccountItemModel> accounts = <AccountItemModel>[].obs;
  final RxnString accountsBookmark = RxnString();
  final RxBool accountsBusy = false.obs;
  final RxnString accountsError = RxnString();

  final RxnString currentUserPk = RxnString();
  final RxBool switchingAccount = false.obs;
  final RxnString switchError = RxnString();

  static void init() {
    Get.put<AuthService>(AuthService());
    Get.put<AuthApi>(AuthApi());
  }

  Future<void> bootstrapSession() async {
    final UserService userService = Get.find<UserService>();
    final AuthApi api = Get.find<AuthApi>();
    await api.init();
    try {
      final user = await userService.getUser();
      currentUserPk.value = user.pk;
    } catch (_) {}
  }

  Future<void> loadAccounts({bool refresh = false}) async {
    final AuthApi api = Get.find<AuthApi>();
    if (accountsBusy.value) return;
    accountsBusy.value = true;
    accountsError.value = null;

    try {
      final nextBookmark = refresh ? null : accountsBookmark.value;
      final res = await api.listAccounts(bookmark: nextBookmark);

      if (res == null) {
        accountsError.value = 'failed_to_list_accounts';
        return;
      }

      if (refresh) {
        accounts.assignAll(res.items);
      } else {
        accounts.addAll(res.items);
      }
      accountsBookmark.value = res.bookmark;
    } catch (e) {
      accountsError.value = e.toString();
    } finally {
      accountsBusy.value = false;
    }
  }

  Future<bool> changeAccount(AccountItemModel account) async {
    final AuthApi api = Get.find<AuthApi>();
    if (switchingAccount.value) return false;
    switchingAccount.value = true;
    switchError.value = null;

    try {
      final res = await api.changeAccount(account.userPk);
      if (res == null) {
        switchError.value = 'failed_to_change_account';
        return false;
      }

      currentUserPk.value = account.userPk;

      final idx = accounts.indexWhere((a) => a.userPk == account.userPk);
      if (idx != -1) {
        final updated = AccountItemModel(
          userPk: account.userPk,
          displayName: account.displayName,
          profileUrl: account.profileUrl,
          username: account.username,
          userType: account.userType,
          lastLoginAt: DateTime.now().millisecondsSinceEpoch ~/ 1000,
          revoked: account.revoked,
        );
        accounts[idx] = updated;
      }

      return true;
    } catch (e) {
      switchError.value = e.toString();
      return false;
    } finally {
      switchingAccount.value = false;
    }
  }

  bool get hasMoreAccounts => (accountsBookmark.value?.isNotEmpty ?? false);
}
