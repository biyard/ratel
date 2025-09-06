import 'dart:io' as io;
import 'package:file_picker/file_picker.dart' as fp;
import 'package:flutter/material.dart';
import 'package:http/http.dart' as http;
import 'package:ratel/exports.dart';

class FileUploader extends StatefulWidget {
  const FileUploader({
    super.key,
    required this.assetApi,
    this.onUploadSuccess,
    this.child,
  });

  final AssetApi assetApi;
  final void Function(String url)? onUploadSuccess;
  final Widget? child;

  @override
  State<FileUploader> createState() => _FileUploaderState();
}

class _FileUploaderState extends State<FileUploader> {
  bool _loading = false;

  Future<void> _pickAndUpload() async {
    final result = await fp.FilePicker.platform.pickFiles(
      type: fp.FileType.image,
      allowMultiple: false,
      withData: true,
    );
    if (result == null || result.files.isEmpty) return;

    final file = result.files.first;
    final fileName = file.name;
    final bytes = file.bytes;
    final path = file.path;

    if (bytes == null && path == null) {
      return;
    }

    final type = fileTypeFromPath(fileName);
    if (type == FileType.none) {
      if (mounted) {
        ScaffoldMessenger.of(
          context,
        ).showSnackBar(const SnackBar(content: Text('Unsupported file type')));
      }
      return;
    }

    setState(() => _loading = true);
    try {
      final AssetPresignedUris res = await widget.assetApi.getPresignedUrl(
        type,
      );
      if (res.presignedUris.isEmpty || res.uris.isEmpty) {
        throw Exception('No presigned URL received');
      }
      final presignedUrl = res.presignedUris.first;
      final publicUrl = res.uris.first;

      final headers = {'Content-Type': type.contentType};
      late http.Response putRes;

      if (bytes != null) {
        putRes = await http.put(
          Uri.parse(presignedUrl),
          headers: headers,
          body: bytes,
        );
      } else if (path != null) {
        final bytesFromPath = await io.File(path).readAsBytes();
        putRes = await http.put(
          Uri.parse(presignedUrl),
          headers: headers,
          body: bytesFromPath,
        );
      } else {
        throw Exception("No file data available");
      }

      if (putRes.statusCode < 200 || putRes.statusCode >= 300) {
        throw Exception('File upload failed: ${putRes.statusCode}');
      }

      widget.onUploadSuccess?.call(publicUrl);
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(
          context,
        ).showSnackBar(SnackBar(content: Text('Upload failed: $e')));
      }
    } finally {
      if (mounted) setState(() => _loading = false);
    }
  }

  @override
  Widget build(BuildContext context) {
    final child =
        widget.child ??
        Container(
          padding: const EdgeInsets.all(16),
          decoration: BoxDecoration(
            borderRadius: BorderRadius.circular(12),
            color: Colors.white.withOpacity(0.06),
            border: Border.all(color: Colors.white24),
          ),
          child: const Text(
            'Tap to upload image',
            style: TextStyle(color: Colors.white),
          ),
        );

    return InkWell(
      onTap: _loading ? null : _pickAndUpload,
      child: Stack(
        alignment: Alignment.center,
        children: [
          Opacity(opacity: _loading ? 0.5 : 1, child: child),
          if (_loading)
            const Positioned.fill(
              child: Center(child: CircularProgressIndicator()),
            ),
        ],
      ),
    );
  }
}
