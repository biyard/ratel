import 'package:ratel/exports.dart';

class Header extends StatelessWidget {
  const Header({super.key});

  @override
  Widget build(BuildContext context) {
    return Container(
      color: AppColors.neutral800,
      height: 48,
      width: double.infinity,
      child: Padding(
        padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 4),
        child: Stack(
          alignment: Alignment.center,
          children: [
            Center(child: Assets.logoImage),

            Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                Row(
                  children: [
                    Assets.bellImage,
                    const SizedBox(width: 10),
                    Assets.searchImage,
                  ],
                ),
                InkWell(
                  onTap: () {
                    showDialog(
                      context: context,
                      builder: (BuildContext context) {
                        return LoginModal();
                      },
                    );
                  },
                  child: Text(
                    "Sign In",
                    style: TextStyle(
                      color: AppColors.neutral500,
                      fontSize: 15,
                      fontWeight: FontWeight.w700,
                      height: 18 / 15,
                    ),
                  ),
                ),
                //FIXME: fix to query my profile
                // Profile(
                //   profile:
                //       "https://lh3.googleusercontent.com/a/ACg8ocIGf0gpB8MQdGkp5TXW1327nRpuPz70iy_hQY2NXNwanRXbFw=s96-c",
                // ),
              ],
            ),
          ],
        ),
      ),
    );
  }
}
