import 'package:ratel/exports.dart';

class OrDivider extends StatelessWidget {
  const OrDivider({super.key});

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.fromLTRB(15, 0, 15, 0),
      child: Row(
        children: [
          const Expanded(
            child: Divider(color: Color(0xFF262626), thickness: 1, height: 1),
          ),
          20.gap,
          Text(
            'OR',
            style: TextStyle(
              fontWeight: FontWeight.w500,
              color: Color(0xff8c8c8c),
              fontSize: 15,
              height: 22.5 / 15,
            ),
          ),
          20.gap,
          const Expanded(
            child: Divider(color: Color(0xFF262626), thickness: 1, height: 1),
          ),
        ],
      ),
    );
  }
}
