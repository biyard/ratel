import 'package:camera/camera.dart';
import 'package:flutter_image_compress/flutter_image_compress.dart';
import 'package:ratel/exports.dart';
import 'passport_box.dart';

class StepCapture extends StatefulWidget {
  const StepCapture({super.key, required this.onParsed});
  final void Function(PassportInfo info) onParsed;

  @override
  State<StepCapture> createState() => _StepCaptureState();
}

class _StepCaptureState extends State<StepCapture> {
  final _api = DocumentsApi();
  CameraController? _controller;
  bool _busy = false;
  String? _error;
  bool _showCamera = true;

  void _onCameraReady(CameraController controller) {
    _controller = controller;
  }

  Future<Uint8List> compressJpeg(Uint8List src) async {
    final out = await FlutterImageCompress.compressWithList(
      src,
      quality: 85,
      minWidth: 1600,
      minHeight: 1600,
      format: CompressFormat.jpeg,
    );
    return Uint8List.fromList(out);
  }

  Future<void> _captureAndSend() async {
    if (_busy) return;
    final controller = _controller;
    if (controller == null || !controller.value.isInitialized) {
      setState(() => _error = 'Camera is not ready. Please try again.');
      return;
    }

    setState(() {
      _busy = true;
      _error = null;
    });

    try {
      await controller.pausePreview();
      final xfile = await controller.takePicture();
      final raw = await xfile.readAsBytes();
      final bytes = await compressJpeg(raw);

      final presign = await _api.getPresigned();
      await _api.putToS3(presign.url, bytes);
      final info = await _api.uploadPassportKey(presign.key);

      if (!mounted) return;
      setState(() {
        _busy = false;
        _showCamera = false;
      });
      widget.onParsed(info);
    } catch (e) {
      if (!mounted) return;
      setState(() {
        _busy = false;
        _error = 'Upload failed: $e';
      });
      try {
        await _controller?.resumePreview();
      } catch (_) {}
    }
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
            child: _showCamera
                ? PassportCameraBox(onReady: _onCameraReady)
                : const ColoredBox(color: Color(0xFF2E2D37)),
          ),
          12.vgap,
          if (_error != null)
            Text(
              _error!,
              style: const TextStyle(color: Colors.red, fontSize: 12),
            ),
          8.vgap,
          SizedBox(
            width: double.infinity,
            height: 44,
            child: ElevatedButton(
              onPressed: _busy ? null : _captureAndSend,
              child: _busy
                  ? const SizedBox(
                      width: 20,
                      height: 20,
                      child: CircularProgressIndicator(strokeWidth: 2),
                    )
                  : const Text('Scan passport'),
            ),
          ),
        ],
      ),
    );
  }
}
