import 'package:http_parser/http_parser.dart';
import 'package:ratel/exports.dart';
import 'dart:convert';
import 'package:http/http.dart' as http;

class DriveApi extends GetConnect {
  static const String base = 'https://www.googleapis.com/drive/v3';

  static void init() {
    Get.put<DriveApi>(DriveApi());
  }

  Future<FileList?> listFiles(String accessToken) async {
    final String fileName = Config.env;
    logger.d("list files: $accessToken");
    String uri = "$base/files?q=${fileName}";
    final response = await get(
      uri,
      headers: {'Authorization': 'Bearer $accessToken'},
      query: {'spaces': 'appDataFolder'},
      decoder: (map) => FileList.fromJson(map),
    );

    logger.d("file responses: $response");

    if (response.statusCode == 200) {
      return response.body;
    } else {
      throw Exception('Failed to query files');
    }
  }

  Future<String?> getFile(String accessToken, String fileId) async {
    String uri = "$base/files/$fileId?alt=media";
    final response = await get(
      uri,
      headers: {'Authorization': 'Bearer $accessToken'},
      decoder: (map) => map as String,
    );

    if (response.statusCode == 200) {
      return response.body;
    } else {
      throw Exception('Failed to get file: ${response.statusText}');
    }
  }

  Future<File> uploadFile(String accessToken, String content) async {
    final String fileName = Config.env;
    final url = Uri.parse(
      'https://www.googleapis.com/upload/drive/v3/files'
      '?uploadType=multipart',
    );

    final metadata = {
      'name': fileName,
      'parents': ['appDataFolder'],
    };

    final request = http.MultipartRequest('POST', url)
      ..headers['Authorization'] = 'Bearer $accessToken'
      ..files.add(
        http.MultipartFile.fromString(
          'metadata',
          json.encode(metadata),
          filename: 'metadata.json',
          contentType: MediaType('application', 'json'),
        ),
      )
      ..files.add(
        http.MultipartFile.fromString(
          'media',
          content,
          filename: '$fileName.txt',
          contentType: MediaType('text', 'plain'),
        ),
      );

    final streamedResponse = await request.send();
    final response = await http.Response.fromStream(streamedResponse);

    if (response.statusCode == 200) {
      final data = json.decode(response.body);
      return File.fromJson(data);
    } else {
      throw Exception(
        'Error uploading file: ${response.statusCode} ${response.body}',
      );
    }
  }
}
