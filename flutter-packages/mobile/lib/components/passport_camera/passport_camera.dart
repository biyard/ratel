import 'package:camera/camera.dart';
import 'package:permission_handler/permission_handler.dart';
import 'package:ratel/exports.dart';

class PassportLiveCamera extends StatefulWidget {
  const PassportLiveCamera({super.key, this.onReady});

  final VoidCallback? onReady;

  @override
  State<PassportLiveCamera> createState() => _PassportLiveCameraState();
}

class _PassportLiveCameraState extends State<PassportLiveCamera> {
  CameraController? _controller;
  Future<void>? _init;
  String? _error;
  bool _notified = false;

  @override
  void initState() {
    super.initState();
    _setup();
  }

  Future<void> _setup() async {
    try {
      final p = await Permission.camera.request();
      if (!p.isGranted) {
        setState(() => _error = 'Camera permission denied');
        return;
      }

      final cams = await availableCameras();
      if (cams.isEmpty) {
        setState(() => _error = 'No camera available');
        return;
      }
      final back = cams.firstWhere(
        (c) => c.lensDirection == CameraLensDirection.back,
        orElse: () => cams.first,
      );

      final controller = CameraController(
        back,
        ResolutionPreset.medium,
        enableAudio: false,
      );
      final init = controller.initialize();

      setState(() {
        _controller = controller;
        _init = init;
      });

      await init;
      if (!mounted) return;
      if (!_notified) {
        _notified = true;
        widget.onReady?.call();
      }
      setState(() {});
    } catch (e) {
      setState(() => _error = 'Failed to init camera: $e');
    }
  }

  @override
  void dispose() {
    _controller?.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    if (_error != null) {
      return Center(
        child: Text(
          _error!,
          style: const TextStyle(color: AppColors.neutral400, fontSize: 12),
          textAlign: TextAlign.center,
        ),
      );
    }
    if (_controller == null || _init == null) {
      return const Center(child: CircularProgressIndicator());
    }
    return FutureBuilder(
      future: _init,
      builder: (context, snap) {
        if (snap.connectionState != ConnectionState.done) {
          return const Center(child: CircularProgressIndicator());
        }
        return CameraPreview(_controller!);
      },
    );
  }
}
