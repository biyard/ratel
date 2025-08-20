import 'package:ratel/exports.dart';

class SidePanel extends StatelessWidget {
  const SidePanel({
    super.key,
    required this.user,
    required this.width,
    required this.onClose,
  });

  final UserModel user;
  final double width;
  final VoidCallback onClose;

  Future<void> openThemeSheet(BuildContext context) async {
    await showModalBottomSheet(
      context: context,
      useRootNavigator: true,
      isScrollControlled: true,
      useSafeArea: false,
      showDragHandle: false,
      backgroundColor: Colors.transparent,
      builder: (_) => const StyledSheet(title: 'Theme', child: ThemeSheet()),
    );
  }

  Future<void> openAccountsSheet(BuildContext context) async {
    await showModalBottomSheet(
      context: context,
      useRootNavigator: true,
      isScrollControlled: true,
      useSafeArea: false,
      showDragHandle: false,
      backgroundColor: Colors.transparent,
      builder: (_) => StyledSheet(
        title: 'Accounts',
        child: AccountsSheet(teams: user.teams),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    return Material(
      color: AppColors.panelBg,
      elevation: 12,
      borderRadius: const BorderRadius.only(
        topRight: Radius.circular(10),
        bottomRight: Radius.circular(10),
      ),
      child: SizedBox(
        width: width,
        height: MediaQuery.of(context).size.height,
        child: Padding(
          padding: const EdgeInsets.all(15),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Row(
                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                children: [
                  (user.profileUrl != "")
                      ? ClipRRect(
                          borderRadius: BorderRadius.circular(100),
                          child: Image.network(
                            user.profileUrl,
                            width: 35,
                            height: 35,
                          ),
                        )
                      : Container(
                          width: 35,
                          height: 35,
                          decoration: const BoxDecoration(
                            color: AppColors.neutral700,
                            shape: BoxShape.circle,
                          ),
                        ),
                  Row(
                    children: [
                      Image.asset(Assets.logo, width: 25, height: 25),
                      const SizedBox(width: 10),
                      InkWell(
                        onTap: () async {
                          final rootCtx = Get.context ?? context;
                          onClose();
                          await Future.delayed(
                            const Duration(milliseconds: 320),
                          );
                          if (rootCtx != null) await openAccountsSheet(rootCtx);
                        },
                        child: SvgPicture.asset(
                          Assets.option,
                          width: 24,
                          height: 24,
                        ),
                      ),
                    ],
                  ),
                ],
              ),
              const SizedBox(height: 20),

              Row(
                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                children: [
                  Text(
                    user.nickname,
                    overflow: TextOverflow.ellipsis,
                    style: const TextStyle(
                      color: Colors.white,
                      fontSize: 12,
                      fontWeight: FontWeight.w600,
                    ),
                  ),
                  Row(
                    children: [
                      Text(
                        '\$${user.points}',
                        style: const TextStyle(
                          color: Colors.white,
                          fontSize: 12,
                          fontWeight: FontWeight.w600,
                        ),
                      ),
                      const SizedBox(width: 2),
                      const Text(
                        'Ratels',
                        style: TextStyle(
                          color: Color(0xffd4d4d4),
                          fontSize: 12,
                          fontWeight: FontWeight.w600,
                        ),
                      ),
                    ],
                  ),
                ],
              ),
              Text(
                user.username,
                style: const TextStyle(
                  color: Color(0xffd4d4d4),
                  fontSize: 11,
                  fontWeight: FontWeight.w500,
                ),
              ),
              const SizedBox(height: 10),
              Row(
                children: [
                  Text(
                    "${user.followingsCount} ",
                    style: const TextStyle(
                      color: Colors.white,
                      fontWeight: FontWeight.w600,
                      fontSize: 12,
                    ),
                  ),
                  const Text(
                    'Following',
                    style: TextStyle(
                      color: Color(0xffd4d4d4),
                      fontWeight: FontWeight.w500,
                      fontSize: 11,
                    ),
                  ),
                  const SizedBox(width: 10),
                  Text(
                    "${user.followersCount} ",
                    style: const TextStyle(
                      color: Colors.white,
                      fontWeight: FontWeight.w600,
                      fontSize: 12,
                    ),
                  ),
                  const Text(
                    'Followers',
                    style: TextStyle(
                      color: Color(0xffd4d4d4),
                      fontWeight: FontWeight.w500,
                      fontSize: 11,
                    ),
                  ),
                ],
              ),
              const SizedBox(height: 15),
              Container(color: const Color(0xffd4d4d4), height: 0.3),

              Expanded(
                child: ListView(
                  padding: const EdgeInsets.symmetric(
                    vertical: 20,
                    horizontal: 12,
                  ),
                  children: [
                    MenuItem(
                      icon: SvgPicture.asset(
                        Assets.editContent,
                        width: 20,
                        height: 20,
                      ),
                      label: 'Drafts',
                      onTap: () {
                        onClose();
                        Get.rootDelegate.offAndToNamed(AppRoutes.draftScreen);
                      },
                    ),
                    MenuItem(
                      icon: SvgPicture.asset(
                        Assets.folder,
                        width: 20,
                        height: 20,
                      ),
                      label: 'Posts',
                      onTap: () {
                        onClose();
                        Get.rootDelegate.offAndToNamed(AppRoutes.postScreen);
                      },
                    ),
                    MenuItem(
                      icon: SvgPicture.asset(
                        Assets.verification,
                        width: 20,
                        height: 20,
                      ),
                      label: 'Verified Credential',
                      onTap: () {
                        onClose();
                        Get.rootDelegate.offAndToNamed(
                          AppRoutes.verifiedScreen,
                        );
                      },
                    ),
                    MenuItem(
                      icon: SvgPicture.asset(
                        Assets.star,
                        width: 20,
                        height: 20,
                      ),
                      label: 'Boosting Points',
                      onTap: () {
                        onClose();
                        Get.rootDelegate.offAndToNamed(
                          AppRoutes.boostingScreen,
                        );
                      },
                    ),
                  ],
                ),
              ),
              Container(color: const Color(0xffd4d4d4), height: 0.3),

              Padding(
                padding: const EdgeInsets.fromLTRB(0, 30, 15, 15),
                child: InkWell(
                  onTap: () async {
                    final rootCtx = Get.context ?? context;
                    onClose();
                    await Future.delayed(const Duration(milliseconds: 320));
                    if (rootCtx != null) await openThemeSheet(rootCtx);
                  },
                  child: SvgPicture.asset(Assets.dark, width: 24, height: 24),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}

class MenuItem extends StatelessWidget {
  const MenuItem({
    super.key,
    required this.icon,
    required this.label,
    this.onTap,
  });
  final SvgPicture icon;
  final String label;
  final VoidCallback? onTap;

  @override
  Widget build(BuildContext context) {
    return ListTile(
      leading: icon,
      title: Text(
        label,
        style: const TextStyle(
          color: Color(0xffd4d4d4),
          fontWeight: FontWeight.w600,
          fontSize: 14,
        ),
      ),
      onTap: onTap,
      contentPadding: const EdgeInsets.symmetric(horizontal: 1, vertical: 2),
      horizontalTitleGap: 4,
      dense: true,
    );
  }
}

class ThemeSheet extends StatefulWidget {
  const ThemeSheet({super.key});

  @override
  State<ThemeSheet> createState() => _ThemeSheetState();
}

class _ThemeSheetState extends State<ThemeSheet> {
  late ThemeMode _selected;

  @override
  void initState() {
    super.initState();
    _selected = Get.isDarkMode ? ThemeMode.dark : ThemeMode.light;
  }

  void _select(ThemeMode v) {
    setState(() => _selected = v);
    Get.changeThemeMode(v);
  }

  Widget _row(String label, ThemeMode mode) {
    return ListTile(
      onTap: () => _select(mode),
      contentPadding: EdgeInsets.zero,
      horizontalTitleGap: 0,
      minVerticalPadding: 0,
      visualDensity: const VisualDensity(horizontal: 0, vertical: -2),
      title: Text(
        label,
        style: const TextStyle(
          color: Colors.white,
          fontWeight: FontWeight.w600,
          fontSize: 16,
          height: 1.2,
        ),
      ),
      trailing: Radio<ThemeMode>(
        value: mode,
        groupValue: _selected,
        onChanged: (v) {
          if (v != null) _select(v);
        },
        activeColor: AppColors.primary,
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.fromLTRB(16, 8, 16, 16),
      child: Column(
        mainAxisSize: MainAxisSize.min,
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          const Text(
            'Theme',
            style: TextStyle(
              color: Colors.white,
              fontSize: 18,
              fontWeight: FontWeight.w700,
              height: 1.2,
            ),
          ),
          12.vgap,
          const Divider(height: 1, color: AppColors.neutral700),
          30.vgap,

          _row('Dark', ThemeMode.dark),
          _row('Light', ThemeMode.light),
          _row('System-wide setting', ThemeMode.system),
        ],
      ),
    );
  }
}

class AccountsSheet extends StatefulWidget {
  const AccountsSheet({super.key, required this.teams});
  final List<Team> teams;

  @override
  State<AccountsSheet> createState() => _AccountsSheetState();
}

class _AccountsSheetState extends State<AccountsSheet> {
  int _selected = 0;

  Widget _accountTile({
    required int index,
    required String name,
    required String sub,
  }) {
    final isSel = _selected == index;
    return ListTile(
      onTap: () => setState(() => _selected = index),
      contentPadding: EdgeInsets.zero,
      minVerticalPadding: 0,
      horizontalTitleGap: 8,
      visualDensity: const VisualDensity(horizontal: 0, vertical: -2),
      dense: true,
      leading: const CircleAvatar(
        radius: 14,
        backgroundColor: AppColors.neutral600,
      ),
      title: Text(
        name,
        style: const TextStyle(
          color: Colors.white,
          fontWeight: FontWeight.w600,
          fontSize: 14,
          height: 1.2,
        ),
      ),
      subtitle: Text(
        sub,
        style: const TextStyle(
          color: AppColors.neutral400,
          fontSize: 12,
          height: 1.2,
        ),
      ),
      trailing: Icon(
        isSel ? Icons.radio_button_checked : Icons.radio_button_unchecked,
        color: isSel ? AppColors.primary : AppColors.neutral600,
      ),
    );
  }

  Widget _actionButton(String label, {VoidCallback? onTap}) {
    return InkWell(
      onTap: onTap,
      child: Container(
        height: 44,
        alignment: Alignment.center,
        decoration: BoxDecoration(
          color: Colors.transparent,
          borderRadius: BorderRadius.circular(50),
          border: Border.all(color: const Color(0xff464646), width: 1),
        ),
        child: Text(
          label,
          style: const TextStyle(
            color: Colors.white,
            fontWeight: FontWeight.w400,
            fontSize: 14,
            height: 1.1,
          ),
        ),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    final tiles = widget.teams
        .map(
          (t) => _accountTile(
            index: t.id,
            name: t.nickname,
            sub: '@${t.username}',
          ),
        )
        .toList();

    return Padding(
      padding: const EdgeInsets.fromLTRB(16, 8, 16, 16),
      child: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          _SheetHeader(title: 'Accounts'),
          const SizedBox(height: 8),
          ...tiles,
          const SizedBox(height: 10),
          _actionButton('Create a new Account', onTap: () {}),
          const SizedBox(height: 8),
          _actionButton('Add on existing account', onTap: () {}),
          const SizedBox(height: 6),
        ],
      ),
    );
  }
}

class StyledSheet extends StatelessWidget {
  const StyledSheet({
    super.key,
    required this.child,
    this.title,
    this.backgroundColor = AppColors.panelBg,
    this.radius = const Radius.circular(16),
    this.handle = true,
    this.handleColor = const Color(0xFF6B6B6D),
    this.handleWidth = 44,
    this.handleHeight = 5,
    this.horizontal = 16,
    this.top = 6,
    this.bottom = 16,
  });

  final Widget child;
  final String? title;

  final Color backgroundColor;
  final Radius radius;

  final bool handle;
  final Color handleColor;
  final double handleWidth;
  final double handleHeight;

  final double horizontal;
  final double top;
  final double bottom;

  @override
  Widget build(BuildContext context) {
    final bottomInset = MediaQuery.of(context).padding.bottom;

    return Material(
      color: backgroundColor,
      shape: RoundedRectangleBorder(
        borderRadius: BorderRadius.vertical(top: radius),
      ),
      clipBehavior: Clip.antiAlias,
      child: SafeArea(
        top: false,
        bottom: false,
        child: Padding(
          padding: EdgeInsets.fromLTRB(horizontal, top, horizontal, bottom),
          child: Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              if (handle)
                Container(
                  width: handleWidth,
                  height: handleHeight,
                  margin: const EdgeInsets.only(bottom: 12),
                  decoration: BoxDecoration(
                    color: handleColor,
                    borderRadius: BorderRadius.circular(999),
                  ),
                ),
              if (title != null) const SizedBox(height: 6),
              child,
              SizedBox(height: bottomInset),
            ],
          ),
        ),
      ),
    );
  }
}

class _SheetHeader extends StatelessWidget {
  const _SheetHeader({required this.title});
  final String title;

  @override
  Widget build(BuildContext context) {
    return Container(
      width: double.infinity,
      padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 10),
      decoration: const BoxDecoration(
        border: Border(
          bottom: BorderSide(color: AppColors.neutral700, width: 1),
        ),
      ),
      child: Text(
        title,
        style: const TextStyle(
          color: Colors.white,
          fontSize: 18,
          fontWeight: FontWeight.w700,
        ),
      ),
    );
  }
}
