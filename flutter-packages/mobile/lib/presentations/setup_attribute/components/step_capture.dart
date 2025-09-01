import 'dart:async';
import 'dart:io';
import 'package:camera/camera.dart';
import 'package:flutter_image_compress/flutter_image_compress.dart';
import 'package:google_mlkit_text_recognition/google_mlkit_text_recognition.dart';
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

  final _textRecognizer = TextRecognizer(script: TextRecognitionScript.latin);
  bool _autoTriggered = false;
  int _stableHits = 0;
  DateTime _lastFrameTs = DateTime.fromMillisecondsSinceEpoch(0);

  void _onCameraReady(CameraController controller) {
    _controller = controller;
    _startAutoDetect();
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
        if (_controller?.value.isPreviewPaused == true) {
          await _controller?.resumePreview();
        }
      } catch (_) {}
      _autoTriggered = false;
      _stableHits = 0;
      _startAutoDetect();
    }
  }

  void _startAutoDetect() {
    final c = _controller;
    if (c == null || !c.value.isInitialized) return;
    if (c.value.isStreamingImages) return;
    c.startImageStream(_onFrame);
  }

  Future<void> _onFrame(CameraImage image) async {
    if (_autoTriggered || _busy) return;

    final now = DateTime.now();
    if (now.difference(_lastFrameTs).inMilliseconds < 300) return;
    _lastFrameTs = now;

    try {
      if (!_enoughBrightness(image)) return;

      final input = _toMlKitInputImage(image, _controller!);
      final result = await _textRecognizer.processImage(input);
      final text = result.text.toUpperCase();

      final ok = _isMrzLike(text);
      if (ok) {
        _stableHits++;
      } else {
        _stableHits = 0;
      }

      if (_stableHits >= 1) {
        _autoTriggered = true;
        await _controller?.stopImageStream();
        await _captureAndSend();
      }
    } catch (_) {}
  }

  bool _isMrzLike(String raw) {
    final text = raw.replaceAll('\n', ' ').replaceAll(' ', '');
    final line1 = RegExp(r'(P<|ID<)[A-Z<]{2,3}[A-Z<]{2,}');
    final line2 = RegExp(r'[A-Z0-9<]{30,}');
    final chevrons = RegExp(r'<{3,}');
    final hasLine1 = line1.hasMatch(text);
    final hasLine2 = line2.hasMatch(text);
    final manyChevrons = chevrons.hasMatch(text);
    return (hasLine1 && (hasLine2 || manyChevrons)) ||
        (manyChevrons && text.length > 40);
  }

  bool _enoughBrightness(CameraImage img) {
    try {
      final y = img.planes.first.bytes;
      if (y.isEmpty) return true;
      int step = (y.length ~/ 5000).clamp(1, 50);
      int sum = 0, cnt = 0;
      for (int i = 0; i < y.length; i += step) {
        sum += y[i];
        cnt++;
      }
      final avg = sum / cnt;
      return avg > 25;
    } catch (_) {
      return true;
    }
  }

  InputImage _toMlKitInputImage(
    CameraImage image,
    CameraController controller,
  ) {
    final rotation =
        InputImageRotationValue.fromRawValue(
          controller.description.sensorOrientation,
        ) ??
        InputImageRotation.rotation0deg;

    if (Platform.isIOS) {
      final bytes = image.planes.first.bytes;
      return InputImage.fromBytes(
        bytes: bytes,
        metadata: InputImageMetadata(
          size: Size(image.width.toDouble(), image.height.toDouble()),
          rotation: rotation,
          format: InputImageFormat.bgra8888,
          bytesPerRow: image.planes.first.bytesPerRow,
        ),
      );
    } else {
      Uint8List nv21;
      if (image.planes.length == 1 &&
          image.format.group == ImageFormatGroup.nv21) {
        nv21 = image.planes.first.bytes;
      } else {
        nv21 = _yuv420ToNv21(image);
      }
      return InputImage.fromBytes(
        bytes: nv21,
        metadata: InputImageMetadata(
          size: Size(image.width.toDouble(), image.height.toDouble()),
          rotation: rotation,
          format: InputImageFormat.nv21,
          bytesPerRow: image.planes.first.bytesPerRow,
        ),
      );
    }
  }

  Uint8List _yuv420ToNv21(CameraImage image) {
    final width = image.width;
    final height = image.height;
    final yPlane = image.planes[0].bytes;
    final uPlane = image.planes[1].bytes;
    final vPlane = image.planes[2].bytes;
    final yRowStride = image.planes[0].bytesPerRow;
    final uvRowStride = image.planes[1].bytesPerRow;
    final uvPixelStride = image.planes[1].bytesPerPixel ?? 1;

    final out = Uint8List(width * height + 2 * (width ~/ 2) * (height ~/ 2));
    int outIndex = 0;

    for (int r = 0; r < height; r++) {
      final start = r * yRowStride;
      out.setRange(
        outIndex,
        outIndex + width,
        yPlane.sublist(start, start + width),
      );
      outIndex += width;
    }

    final uvHeight = height ~/ 2;
    final uvWidth = width ~/ 2;
    for (int r = 0; r < uvHeight; r++) {
      for (int c = 0; c < uvWidth; c++) {
        final uvIndex = r * uvRowStride + c * uvPixelStride;
        final v = vPlane[uvIndex];
        final u = uPlane[uvIndex];
        out[outIndex++] = v;
        out[outIndex++] = u;
      }
    }
    return out;
  }

  @override
  void dispose() {
    _controller?.stopImageStream().catchError((_) {});
    _textRecognizer.close();
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
    );
  }
}
