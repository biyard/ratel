import 'dart:async';
import 'dart:io';
import 'package:camera/camera.dart';
import 'package:flutter/material.dart';
import 'package:permission_handler/permission_handler.dart';

typedef CameraReady = void Function(CameraController controller);

class PassportCameraBox extends StatefulWidget {
  const PassportCameraBox({super.key, required this.onReady});
  final CameraReady onReady;

  @override
  State<PassportCameraBox> createState() => _PassportCameraBoxState();
}

class _PassportCameraBoxState extends State<PassportCameraBox> {
  CameraController? _controller;
  Future<void>? _initializeFuture;
  String? _error;

  @override
  void initState() {
    super.initState();
    _initCamera();
  }

  Future<void> _initCamera() async {
    try {
      final status = await Permission.camera.request();
      if (!status.isGranted) {
        if (!mounted) return;
        setState(() => _error = 'Camera permission denied');
        return;
      }

      final cameras = await availableCameras();
      if (cameras.isEmpty) {
        if (!mounted) return;
        setState(() => _error = 'No camera available');
        return;
      }

      final camera = cameras.firstWhere(
        (c) => c.lensDirection == CameraLensDirection.back,
        orElse: () => cameras.first,
      );

      final controller = CameraController(
        camera,
        ResolutionPreset.medium,
        enableAudio: false,
        imageFormatGroup: Platform.isAndroid
            ? ImageFormatGroup.nv21
            : ImageFormatGroup.bgra8888,
      );

      _initializeFuture = controller.initialize();
      await _initializeFuture;

      if (!mounted) {
        await controller.dispose();
        return;
      }

      _controller = controller;
      widget.onReady(controller);
      setState(() {});
    } catch (e) {
      if (!mounted) return;
      setState(() => _error = '$e');
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
        child: Text(_error!, style: const TextStyle(color: Colors.red)),
      );
    }
    if (_controller == null || _initializeFuture == null) {
      return const Center(child: CircularProgressIndicator());
    }
    return FutureBuilder(
      future: _initializeFuture,
      builder: (_, snap) {
        if (snap.connectionState != ConnectionState.done) {
          return const Center(child: CircularProgressIndicator());
        }
        return ClipRRect(
          borderRadius: BorderRadius.circular(6),
          child: CameraPreview(_controller!),
        );
      },
    );
  }
}
