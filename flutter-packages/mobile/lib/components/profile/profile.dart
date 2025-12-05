import 'package:ratel/exports.dart';

class Profile extends StatelessWidget {
  const Profile({
    super.key,
    required this.profileImageUrl,
    required this.displayName,
  });

  final String profileImageUrl;
  final String displayName;

  @override
  Widget build(BuildContext context) {
    return Row(
      mainAxisSize: MainAxisSize.min,
      children: [
        RoundContainer(
          width: 24,
          height: 24,
          radius: 100,
          color: AppColors.neutral500,
          child: ClipRRect(
            borderRadius: BorderRadius.circular(100),
            child: Image.network(profileImageUrl, fit: BoxFit.cover),
          ),
        ),
        8.gap,
        Flexible(
          child: Text(
            displayName,
            maxLines: 1,
            overflow: TextOverflow.ellipsis,
            style: const TextStyle(
              fontFamily: 'Raleway',
              color: Colors.white,
              fontWeight: FontWeight.w700,
              fontSize: 14,
              height: 20 / 14,
            ),
          ),
        ),
      ],
    );
  }
}
