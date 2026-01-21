import 'package:ratel/exports.dart';

class OrDivider extends StatelessWidget {
  const OrDivider({super.key});

  @override
  Widget build(BuildContext context) {
    return Row(
      children: [
        Expanded(child: Divider(color: Color(0xFF262626), height: 1)),
        20.gap,
        Text(
          'OR',
          style: TextStyle(
            fontSize: 15,
            color: Color(0xFF8C8C8C),
            fontWeight: FontWeight.w500,
            height: 22.5 / 15,
          ),
        ),
        20.gap,
        Expanded(child: Divider(color: Color(0xFF262626), height: 1)),
      ],
    );
  }
}
