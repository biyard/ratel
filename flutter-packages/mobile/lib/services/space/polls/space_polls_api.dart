import 'package:ratel/exports.dart';

class SpacePollsApi extends GetConnect {
  final apiEndpoint = Config.apiEndpoint;

  SpacePollsApi() {
    httpClient.baseUrl = apiEndpoint;
    httpClient.timeout = const Duration(seconds: 10);

    httpClient.addRequestModifier<void>((req) async {
      final authApi = Get.find<AuthApi>();
      final cookie = await authApi.cookieHeaderAsync();
      logger.d('${req.method} ${req.url} | SpacePollsApi Cookie: $cookie');
      if (cookie?.isNotEmpty == true) {
        req.headers['Cookie'] = cookie!;
      }
      return req;
    });
  }

  Future<PollListResult> listPolls(String spacePk, {String? bookmark}) async {
    final encodedSpacePk = Uri.encodeComponent(spacePk);
    final resp = await get<Map<String, dynamic>>(
      '/v3/spaces/$encodedSpacePk/polls',
      query: bookmark == null ? null : {'bookmark': bookmark},
    );

    if (!resp.isOk || resp.body == null) {
      throw Exception(
        'Failed to list polls for $spacePk: ${resp.statusCode} ${resp.statusText}',
      );
    }

    return PollListResult.fromJson(resp.body!);
  }

  Future<PollModel> getPoll(String spacePk, String pollSk) async {
    final encodedSpacePk = Uri.encodeComponent(spacePk);
    final encodedPollSk = Uri.encodeComponent(pollSk);
    final resp = await get<Map<String, dynamic>>(
      '/v3/spaces/$encodedSpacePk/polls/$encodedPollSk',
    );

    if (!resp.isOk || resp.body == null) {
      throw Exception(
        'Failed to get poll $pollSk in $spacePk: ${resp.statusCode} ${resp.statusText}',
      );
    }

    return PollModel.fromJson(resp.body!);
  }

  Future<RespondPollResult> respondPoll(
    String spacePk,
    String pollSk,
    List<Answer> answers,
  ) async {
    final encodedSpacePk = Uri.encodeComponent(spacePk);
    final encodedPollSk = Uri.encodeComponent(pollSk);

    final body = {'answers': answers.map((a) => a.toJson()).toList()};

    final resp = await post<Map<String, dynamic>>(
      '/v3/spaces/$encodedSpacePk/polls/$encodedPollSk/responses',
      body,
    );

    if (!resp.isOk || resp.body == null) {
      throw Exception(
        'Failed to respond poll $pollSk in $spacePk: ${resp.statusCode} ${resp.statusText}',
      );
    }

    return RespondPollResult.fromJson(resp.body!);
  }

  Future<PollResult> getPollResult(String spacePk, String pollSk) async {
    final encodedSpacePk = Uri.encodeComponent(spacePk);
    final encodedPollSk = Uri.encodeComponent(pollSk);
    final resp = await get<Map<String, dynamic>>(
      '/v3/spaces/$encodedSpacePk/polls/$encodedPollSk/results',
    );

    if (!resp.isOk || resp.body == null) {
      throw Exception(
        'Failed to get poll result $pollSk in $spacePk: ${resp.statusCode} ${resp.statusText}',
      );
    }

    return PollResult.fromJson(resp.body!);
  }
}
