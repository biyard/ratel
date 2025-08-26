import 'package:ratel/exports.dart';

class NewRelease extends StatelessWidget {
  final List<FeedSummary> items;
  const NewRelease({super.key, required this.items});

  @override
  Widget build(BuildContext context) {
    if (items.isEmpty) return const SizedBox.shrink();
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text(
          HomeLocalization.newRelease,
          style: TextStyle(
            color: Colors.white,
            fontSize: 16,
            fontWeight: FontWeight.w700,
            height: 1.2,
          ),
        ),
        10.vgap,
        for (final it in items) ...[FeedBox(data: it), 10.vgap],
      ],
    );
  }
}
