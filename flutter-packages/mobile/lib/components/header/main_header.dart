import 'package:ratel/exports.dart';

class MainHeader extends StatelessWidget {
  const MainHeader({super.key});

  @override
  Widget build(BuildContext context) {
    final topPad = MediaQuery.of(context).padding.top;

    return Container(
      color: const Color(0xff171717),
      width: double.infinity,
      padding: EdgeInsets.fromLTRB(20, topPad + 12.5, 20, 12.5),
      alignment: Alignment.topLeft,
      child: SvgPicture.asset(Assets.mainLogo),
    );
  }
}
