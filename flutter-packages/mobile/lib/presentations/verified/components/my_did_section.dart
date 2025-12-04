import 'package:ratel/exports.dart';
import 'package:ratel/presentations/verified/components/code_verify_dialog.dart';

class MyDidSection extends StatelessWidget {
  final UserAttributes attributes;
  final VerifiedController controller;

  const MyDidSection({
    super.key,
    required this.controller,
    required this.attributes,
  });

  String _genderLabel(String? gender) {
    final g = (gender ?? '').toLowerCase();
    switch (g) {
      case 'male':
        return 'Male';
      case 'female':
        return 'Female';
      default:
        return 'No Gender';
    }
  }

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);

    final genderText = _genderLabel(attributes.gender);
    final universityText = attributes.university?.isNotEmpty == true
        ? attributes.university!
        : 'No University';

    return Container(
      margin: const EdgeInsets.symmetric(horizontal: 10),
      padding: const EdgeInsets.fromLTRB(16, 14, 16, 14),
      decoration: BoxDecoration(
        color: const Color(0xFF191919),
        borderRadius: BorderRadius.circular(5),
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(
            'My DID',
            style: theme.textTheme.titleMedium?.copyWith(
              color: Colors.white,
              fontWeight: FontWeight.w600,
              fontSize: 18,
            ),
          ),
          14.vgap,
          _DidItemTile(
            icon: SvgPicture.asset(Assets.gender, width: 18, height: 18),
            title: genderText,
            subtitle: 'Gender',
          ),
          12.vgap,
          _DidItemTile(
            icon: SvgPicture.asset(Assets.university, width: 18, height: 18),
            title: universityText,
            subtitle: 'University',
          ),
          20.vgap,
          Center(
            child: TextButton(
              onPressed: () {
                CodeVerifyDialog.show(context: context, controller: controller);
              },
              child: Text(
                'Authorize',
                style: theme.textTheme.bodyMedium?.copyWith(
                  color: AppColors.primary,
                  fontWeight: FontWeight.w600,
                ),
              ),
            ),
          ),
        ],
      ),
    );
  }
}

class _DidItemTile extends StatelessWidget {
  final SvgPicture icon;
  final String title;
  final String subtitle;

  const _DidItemTile({
    required this.icon,
    required this.title,
    required this.subtitle,
  });

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);

    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 14, vertical: 14),
      decoration: BoxDecoration(
        color: const Color(0xFF181818),
        borderRadius: BorderRadius.circular(2),
        border: Border.all(color: AppColors.neutral800),
      ),
      child: Row(
        children: [
          icon,
          12.gap,
          Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text(
                title,
                style: theme.textTheme.bodyMedium?.copyWith(
                  color: AppColors.neutral500,
                  fontWeight: FontWeight.w500,
                  fontSize: 15,
                ),
              ),
              4.vgap,
              Text(
                subtitle,
                style: theme.textTheme.bodySmall?.copyWith(
                  color: AppColors.neutral400,
                  fontSize: 12,
                ),
              ),
            ],
          ),
        ],
      ),
    );
  }
}
