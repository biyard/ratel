import 'package:ratel/exports.dart';

class IndustryApi extends GetConnect {
  final apiEndpoint = Config.apiEndpoint;

  IndustryApi() {
    httpClient.addRequestModifier<void>((req) async {
      final authApi = Get.find<AuthApi>();
      final cookie = await authApi.cookieHeaderAsync();
      logger.d('${req.method} ${req.url} | IndustryApi Cookie: $cookie');
      if (cookie?.isNotEmpty == true) req.headers['Cookie'] = cookie!;
      return req;
    });
  }

  Future<List<IndustryModel>> getIndustries() async {
    final uri = Uri.parse(apiEndpoint).resolve('/v2/industries');

    final headers = <String, String>{'Content-Type': 'application/json'};
    final res = await get(uri.toString(), headers: headers);

    if (!res.isOk) return [];

    final List<IndustryModel> industries = [];

    for (var i = 0; i < res.body.length; i++) {
      industries.add(
        IndustryModel(
          id: int.parse(res.body[i]["id"].toString()),
          label: res.body[i]["name"],
        ),
      );
    }

    logger.d("industries: ${industries}");

    return industries;
  }

  Future<dynamic> selectTopics(List<int> topicIds) async {
    final uri = Uri.parse(apiEndpoint).resolve('/v2/industries/select-topics');
    final headers = <String, String>{'Content-Type': 'application/json'};
    final body = {'topics': topicIds};

    final res = await post(uri.toString(), body, headers: headers);

    logger.d('response body: ${res.isOk} ');

    if (!res.isOk) return null;

    logger.d('response body: ${res.body}');

    return res.body;
  }
}
