import 'package:ratel/exports.dart';
import 'package:loading_indicator/loading_indicator.dart';

const List<Color> _kDefaultRainbowColors = const [
  Colors.red,
  Colors.orange,
  Colors.yellow,
  Colors.green,
  Colors.blue,
  Colors.indigo,
  Colors.purple,
];

class BiyardIndicator extends ProgressIndicator {
  const BiyardIndicator({
    super.key,
    super.value,
    super.backgroundColor,
    super.color,
    super.valueColor,
    super.semanticsLabel,
    super.semanticsValue,
  });

  @override
  State<BiyardIndicator> createState() => _BiyardIndicatorState();
}

class _BiyardIndicatorState extends State<BiyardIndicator> {
  @override
  Widget build(BuildContext context) {
    // return const LoadingIndicator(
    //   indicatorType: Indicator.ballPulseSync,
    //   colors: _kDefaultRainbowColors,
    // );
    return Container(
      color: Colors.grey.withOpacity(0.3),
      child: const Row(
        mainAxisAlignment: MainAxisAlignment.center,
        mainAxisSize: MainAxisSize.max,
        children: [
          SizedBox(
            width: 100,
            height: double.infinity,
            child: Column(
              mainAxisAlignment: MainAxisAlignment.center,
              mainAxisSize: MainAxisSize.max,
              crossAxisAlignment: CrossAxisAlignment.center,
              children: const [
                LoadingIndicator(
                  indicatorType: Indicator.ballPulseSync,
                  colors: _kDefaultRainbowColors,
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }
}

class BiyardProgressIndicator extends ProgressIndicator {
  const BiyardProgressIndicator({
    super.key,
    super.value,
    super.backgroundColor,
    super.color,
    super.valueColor,
    super.semanticsLabel,
    super.semanticsValue,
  });

  @override
  State<BiyardProgressIndicator> createState() =>
      _BiyardProgressIndicatorState();
}

class _BiyardProgressIndicatorState extends State<BiyardProgressIndicator> {
  @override
  Widget build(BuildContext context) {
    return const LoadingIndicator(
      indicatorType: Indicator.ballPulseSync,
      colors: _kDefaultRainbowColors,
    );
  }
}
