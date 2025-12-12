import 'package:ratel/exports.dart';

class ReportApi extends GetConnect {
  final apiEndpoint = Config.apiEndpoint;

  ReportApi() {
    httpClient.addRequestModifier<void>((req) async {
      final authApi = Get.find<AuthApi>();
      final cookie = await authApi.cookieHeaderAsync();
      logger.d('${req.method} ${req.url} | FeedsApi Cookie: $cookie');
      if (cookie?.isNotEmpty == true) req.headers['Cookie'] = cookie!;
      return req;
    });
  }

  Future<ReportContentResponse?> _sendReport(Map<String, dynamic> body) async {
    final uri = Uri.parse(apiEndpoint).resolve('/v3/reports');

    final headers = <String, String>{'Content-Type': 'application/json'};
    final res = await post(uri.toString(), body, headers: headers);

    if (!res.isOk) {
      logger.e('report failed status=${res.statusCode} body=${res.body}');
      return null;
    }

    final json = res.body;
    if (json is! Map<String, dynamic>) {
      logger.e('report invalid body=${res.body}');
      return null;
    }

    return ReportContentResponse.fromJson(Map<String, dynamic>.from(json));
  }

  Future<ReportContentResponse?> reportPost({required String postPk}) {
    return _sendReport({'post_pk': postPk});
  }

  Future<ReportContentResponse?> reportSpace({required String spacePk}) {
    return _sendReport({'space_pk': spacePk});
  }

  Future<ReportContentResponse?> reportSpacePost({
    required String spacePk,
    required String spacePostPk,
  }) {
    return _sendReport({'space_pk': spacePk, 'space_post_pk': spacePostPk});
  }

  Future<ReportContentResponse?> reportSpacePostComment({
    required String spacePostPk,
    required String commentSk,
  }) {
    return _sendReport({'space_post_pk': spacePostPk, 'comment_sk': commentSk});
  }
}
