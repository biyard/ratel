import 'dart:async';
import 'package:ratel/exports.dart';

class StepCapture extends StatefulWidget {
  const StepCapture({
    super.key,
    required this.onPrev,
    required this.imageUrl,
    required this.onCapture,
  });

  final String imageUrl;
  final VoidCallback onPrev;
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
    return Padding(
      padding: const EdgeInsets.fromLTRB(20, 0, 20, 50),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          SizedBox(
            height: 70,
            child: Row(
              children: [
                InkWell(
                  onTap: widget.onPrev,
                  child: SvgPicture.asset(Assets.back, width: 16, height: 16),
                ),
                10.gap,
                const Text(
                  'Set up attribute',
                  style: TextStyle(
                    color: Colors.white,
                    fontSize: 14,
                    fontWeight: FontWeight.w600,
                    height: 1.2,
                  ),
                ),
              ],
            ),
          ),
          const Padding(
            padding: EdgeInsets.fromLTRB(4, 0, 4, 16),
            child: Text(
              'Passport',
              style: TextStyle(
                color: Colors.white,
                fontSize: 32,
                fontWeight: FontWeight.w800,
                height: 1.1,
              ),
            ),
          ),
          Expanded(
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
                ClipRRect(
                  borderRadius: BorderRadius.circular(6),
                  child: Container(
                    width: double.infinity,
                    height: 220,
                    color: const Color(0xFF2E2D37),
                    child: PassportLiveCamera(onReady: _startCountdown),
                  ),
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }
}
