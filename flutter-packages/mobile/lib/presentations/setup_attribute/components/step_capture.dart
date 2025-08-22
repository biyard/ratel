import 'dart:async';
import 'package:ratel/exports.dart';

class StepCapture extends StatefulWidget {
  const StepCapture({
    super.key,
    required this.imageUrl,
    required this.onCapture,
  });

  final String imageUrl;
  final VoidCallback onCapture;

  @override
  State<StepCapture> createState() => _StepCaptureState();
}

class _StepCaptureState extends State<StepCapture> {
  Timer? _timer;
  int _secLeft = 10;
  bool _started = false;
  bool _done = false;

  void _startCountdown() {
    if (_started || _done || !mounted) return;
    setState(() {
      _started = true;
      _secLeft = 10;
    });
    _timer?.cancel();
    _timer = Timer.periodic(const Duration(seconds: 1), (t) {
      if (!mounted) return;
      setState(() => _secLeft--);
      if (_secLeft <= 0) {
        t.cancel();
        _done = true;
        widget.onCapture();
      }
    });
  }

  @override
  void dispose() {
    _timer?.cancel();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Expanded(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          const Text(
            "Could you observe the below icon in the front of your passport?\nOnly the passport having the below icon could be verified.",
            style: TextStyle(
              color: AppColors.neutral300,
              fontSize: 12,
              fontWeight: FontWeight.w400,
              height: 1.33,
            ),
          ),
          16.vgap,
          Container(
            width: double.infinity,
            height: 220,
            decoration: BoxDecoration(
              color: const Color(0xFF2E2D37),
              borderRadius: BorderRadius.circular(6),
            ),
            clipBehavior: Clip.antiAlias,
            child: PassportLiveCamera(onReady: _startCountdown),
          ),
        ],
      ),
    );
  }
}
