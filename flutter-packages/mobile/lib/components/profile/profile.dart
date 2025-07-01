import 'package:ratel/exports.dart';

class Profile extends StatelessWidget {
  Profile({super.key, this.width, this.height, this.profile});

  final String? profile;
  double? width = 30;
  double? height = 30;

  @override
  Widget build(BuildContext context) {
    return (profile != "" && profile != null)
        ? ClipOval(
            child: Image.network(
              profile!,
              width: width,
              height: height,
              fit: BoxFit.cover,
            ),
          )
        : RoundContainer(
            color: AppColors.neutral400,
            width: width,
            height: height,
            radius: 100,
          );
  }
}
