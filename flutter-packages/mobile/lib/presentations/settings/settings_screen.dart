import 'package:ratel/exports.dart';

class SettingsScreen extends GetWidget<SettingsController> {
  const SettingsScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Layout<SettingsController>(
      scrollable: false,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          AppTopBar(
            padding: const EdgeInsets.fromLTRB(20, 20, 20, 10),
            onBack: () => {controller.goBack()},
            title: "My Profile",
          ),

          Padding(
            padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 0),
            child: Column(
              children: [
                SettingItem(
                  icon: const Icon(Icons.logout, color: Colors.white70),
                  title: 'Logout',
                  subtitle: 'Sign out of your account',
                  onTap: controller.logout,
                ),

                Container(
                  width: double.infinity,
                  height: 1,
                  color: Color(0xff2d2d2d),
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }
}
