import 'dart:async';
import 'package:camera/camera.dart';
import 'package:flutter_image_compress/flutter_image_compress.dart';
import 'package:ratel/exports.dart';

class MedicalCapture extends StatefulWidget {
  const MedicalCapture({
    super.key,
    required this.onPrev,
    required this.onParsed,
  });
  final VoidCallback onPrev;
  final Future<void> Function(MedicalInfo info) onParsed;

  @override
  State<MedicalCapture> createState() => _MedicalCaptureState();
}

class _MedicalCaptureState extends State<MedicalCapture> {
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
      if (controller.value.isStreamingImages) {
        await controller.stopImageStream();
      }
      if (!controller.value.isPreviewPaused) {
        await controller.pausePreview();
      }
      final xfile = await controller.takePicture();
      final raw = await xfile.readAsBytes();
      final bytes = await compressJpeg(raw);
      final presign = await _api.getPresigned();
      await _api.putToS3(presign.url, bytes);

      logger.d("presigned key: ${presign.key}");
      final info = await _api.uploadMedicalKeys([presign.key]);
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
        if (_controller?.value.isPreviewPaused == true) {
          await _controller?.resumePreview();
        }
      } catch (_) {}
    }
  }

  Future<void> _onManualScan() async {
    if (_busy) return;
    try {
      if (_controller?.value.isStreamingImages == true) {
        await _controller!.stopImageStream();
      }
    } catch (_) {}
    await _captureAndSend();
  }

  @override
  void dispose() {
    _controller?.stopImageStream().catchError((_) {});
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.fromLTRB(20, 0, 20, 50),
      child: Column(
        mainAxisAlignment: MainAxisAlignment.start,
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
              'Scan your Medical Check-up Certificate',
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
                  "Certificates must clearly display the certificate number, hospital name, and your name/date of birth to be verified.",
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
                    color: Color(0xFF2E2D37),
                    borderRadius: BorderRadius.circular(6),
                  ),
                  clipBehavior: Clip.antiAlias,
                  child: _showCamera
                      ? PassportCameraBox(onReady: _onCameraReady)
                      : const ColoredBox(color: Color(0xFF2E2D37)),
                ),
                30.vgap,
                SizedBox(
                  width: double.infinity,
                  height: 44,
                  child: ElevatedButton.icon(
                    onPressed: _busy ? null : _onManualScan,
                    label: Text(
                      _busy ? 'Uploading...' : 'Scan',
                      style: TextStyle(
                        color: AppColors.neutral900,
                        fontWeight: FontWeight.bold,
                      ),
                    ),
                    style: ElevatedButton.styleFrom(
                      backgroundColor: AppColors.primary,
                      shape: RoundedRectangleBorder(
                        borderRadius: BorderRadius.circular(6),
                      ),
                    ),
                  ),
                ),
                12.vgap,
                if (_error != null)
                  Text(
                    _error!,
                    style: const TextStyle(color: Colors.red, fontSize: 12),
                  ),
                8.vgap,
                if (_busy)
                  const Center(
                    child: SizedBox(
                      width: 20,
                      height: 20,
                      child: CircularProgressIndicator(strokeWidth: 2),
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
