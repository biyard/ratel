import 'package:ratel/exports.dart';
import 'package:ratel/features/onboarding/screens/account/components/account_tile.dart';
import 'package:ratel/features/onboarding/screens/account/components/add_another_account_button.dart';
import 'package:ratel/features/onboarding/screens/account/components/signup_hint.dart';
import 'package:ratel/features/onboarding/screens/signup/components/or_divider.dart';
import 'package:ratel/features/onboarding/screens/signup/components/signup_logo.dart';

class AccountScreen extends GetWidget<AccountController> {
  const AccountScreen({super.key});

  @override
  Widget build(BuildContext context) {
    const pageBg = Color(0xFF1D1D1D);
    const panelBg = Color(0xFF171717);

    return Layout<AccountController>(
      scrollable: false,
      enableSafeArea: false,
      style: const LayoutStyle(background: pageBg),
      child: Container(
        color: pageBg,
        child: SafeArea(
          bottom: false,
          child: Column(
            children: [
              const Padding(
                padding: EdgeInsets.only(top: 20),
                child: SignupLogo(),
              ),
              25.vgap,

              Expanded(
                child: Container(
                  width: double.infinity,
                  decoration: const BoxDecoration(
                    color: panelBg,
                    borderRadius: BorderRadius.vertical(
                      top: Radius.circular(24),
                    ),
                  ),
                  child: Column(
                    children: [
                      Expanded(
                        child: Obx(() {
                          final items = controller.accounts;

                          if (items.isEmpty) {
                            return const SizedBox.shrink();
                          }

                          return SingleChildScrollView(
                            physics: const BouncingScrollPhysics(),
                            padding: const EdgeInsets.fromLTRB(20, 30, 20, 0),
                            child: Column(
                              children: [
                                const Text(
                                  'Log in',
                                  style: TextStyle(
                                    fontSize: 24,
                                    fontWeight: FontWeight.w800,
                                    color: Colors.white,
                                    height: 32 / 24,
                                  ),
                                ),
                                20.vgap,
                                ...items.map(
                                  (a) => Padding(
                                    padding: const EdgeInsets.only(bottom: 12),
                                    child: AccountTile(
                                      displayName: a.displayName,
                                      username: a.username,
                                      profileUrl: a.profileUrl,
                                      onTap: () =>
                                          controller.onSelectAccount(a),
                                    ),
                                  ),
                                ),
                                20.vgap,
                                const OrDivider(),
                                20.vgap,
                                AddAnotherAccountButton(
                                  onTap: controller.onAddAnotherAccount,
                                ),
                                30.vgap,
                              ],
                            ),
                          );
                        }),
                      ),

                      SafeArea(
                        top: false,
                        child: Padding(
                          padding: const EdgeInsets.only(bottom: 21),
                          child: SignupHint(onSignup: controller.onSignup),
                        ),
                      ),
                    ],
                  ),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
