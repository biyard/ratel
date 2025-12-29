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
    return Layout<AccountController>(
      scrollable: false,
      child: Container(
        color: const Color(0xFF1D1D1D),
        padding: const EdgeInsets.only(top: 20),
        child: Column(
          children: [
            const Padding(
              padding: EdgeInsets.only(top: 20),
              child: SignupLogo(),
            ),
            25.vgap,

            Expanded(
              child: Obx(() {
                final items = controller.accounts;

                if (items.isEmpty) {
                  return const SizedBox.shrink();
                }

                return Column(
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
                    Expanded(
                      child: SingleChildScrollView(
                        padding: const EdgeInsets.symmetric(horizontal: 20),
                        child: Column(
                          children: items
                              .map(
                                (a) => Padding(
                                  padding: const EdgeInsets.only(bottom: 12),
                                  child: AccountTile(
                                    displayName: a.displayName,
                                    username: a.username,
                                    profileUrl: a.profileUrl,
                                    onTap: () => controller.onSelectAccount(a),
                                  ),
                                ),
                              )
                              .toList(),
                        ),
                      ),
                    ),
                  ],
                );
              }),
            ),

            Padding(
              padding: const EdgeInsets.symmetric(horizontal: 20),
              child: Obx(() {
                final hasAccounts = controller.accounts.isNotEmpty;

                return Column(
                  children: [
                    if (hasAccounts) ...[20.vgap, const OrDivider()],
                    20.vgap,
                    AddAnotherAccountButton(
                      onTap: controller.onAddAnotherAccount,
                    ),
                    const SizedBox(height: 28),
                  ],
                );
              }),
            ),

            Padding(
              padding: const EdgeInsets.only(bottom: 21),
              child: SignupHint(onSignup: controller.onSignup),
            ),
          ],
        ),
      ),
    );
  }
}
