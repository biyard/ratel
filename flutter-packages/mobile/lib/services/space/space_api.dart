import 'package:ratel/exports.dart';

class SpaceApi extends GetConnect {
  final apiEndpoint = Config.apiEndpoint;

  SpaceApi() {
    httpClient.addRequestModifier<void>((req) async {
      final authApi = Get.find<AuthApi>();
      final cookie = await authApi.cookieHeaderAsync();
      logger.d('${req.method} ${req.url} | FeedsApi Cookie: $cookie');
      if (cookie?.isNotEmpty == true) req.headers['Cookie'] = cookie!;
      return req;
    });
  }

  Future<MySpaceModel> getMySpaces() async {
    final uri = Uri.parse(apiEndpoint).resolve('/v2/my-spaces');
    final headers = <String, String>{'Content-Type': 'application/json'};
    final res = await get(uri.toString(), headers: headers);

    logger.d("my spaces res: ${res.body}");

    if (!res.isOk || res.body == null)
      return MySpaceModel(spaces: [], boostings: []);

    int _i(dynamic v) {
      if (v == null) return 0;
      final s = '$v'.trim();
      if (s.isEmpty || s.toLowerCase() == 'null') return 0;
      return int.tryParse(s) ?? 0;
    }

    String _s(dynamic v) {
      if (v == null) return '';
      final s = '$v';
      return s.toLowerCase() == 'null' ? '' : s;
    }

    final mySpace = <SpaceSummary>[];
    final boostings = <SpaceSummary>[];

    final spacesJson = res.body['spaces'] as List? ?? const [];
    for (final space in spacesJson) {
      mySpace.add(
        SpaceSummary(
          id: _i(space['id']),
          createdAt: _i(space['created_at']),
          updatedAt: _i(space['updated_at']),
          feedId: _i(space['feed_id']),
          title: _s(space['title']),
          htmlContents: _s(space['html_contents']),
          imageUrl: _s(space['image_url']),
          isBookmarked: false,
          authorUrl: '',
          authorName: '',
          likes: 0,
          rewards: 0,
          comments: 0,
        ),
      );
    }

    final boostingsJson = res.body['boostings'] as List? ?? const [];
    for (final space in boostingsJson) {
      boostings.add(
        SpaceSummary(
          id: _i(space['id']),
          createdAt: _i(space['created_at']),
          updatedAt: _i(space['updated_at']),
          feedId: _i(space['feed_id']),
          title: _s(space['title']),
          htmlContents: _s(space['html_contents']),
          imageUrl: _s(space['image_url']),
          isBookmarked: false,
          authorUrl: '',
          authorName: '',
          likes: 0,
          rewards: 0,
          comments: 0,
        ),
      );
    }

    return MySpaceModel(spaces: mySpace, boostings: boostings);
  }

  Future<SpaceModel?> getSpace(String spacePk) async {
    final encodedPk = Uri.encodeComponent(spacePk);
    final uri = Uri.parse(apiEndpoint).resolve('/v3/spaces/$encodedPk');

    final headers = <String, String>{'Content-Type': 'application/json'};
    final res = await get(uri.toString(), headers: headers);

    logger.d(
      'getSpace($spacePk) res: status=${res.statusCode}, body=${res.body}',
    );

    if (!res.isOk || res.body == null) {
      return null;
    }

    try {
      final body = res.body;

      if (body is Map<String, dynamic>) {
        return SpaceModel.fromJson(body);
      }

      if (body is Map) {
        return SpaceModel.fromJson(Map<String, dynamic>.from(body));
      }

      logger.e('Unexpected getSpace response type: ${body.runtimeType}');
      return null;
    } catch (e, s) {
      logger.e('Failed to parse SpaceModel from getSpace: $e\n$s');
      return null;
    }
  }

  Future<dynamic> setComment(
    int feedId,
    int userId,
    String htmlContents,
  ) async {
    final uri = Uri.parse(apiEndpoint).resolve('/v1/feeds');

    final headers = <String, String>{'Content-Type': 'application/json'};
    final body = {
      'comment': {
        'html_contents': htmlContents,
        'user_id': userId,
        'parent_id': feedId,
      },
    };

    final res = await post(uri.toString(), body, headers: headers);

    if (!res.isOk) return null;

    logger.d('response body: ${res.body}');

    return res.isOk;
  }

  Future<dynamic> responseAnswer(
    int spaceId,
    int surveyId,
    List<Answer> answers,
  ) async {
    final uri = Uri.parse(
      apiEndpoint,
    ).resolve('/v1/spaces/${spaceId}/responses');

    final headers = <String, String>{'Content-Type': 'application/json'};
    final body = {
      'respond_answer': {
        'answers': answers.map((e) => e.toJson()).toList(),
        'survey_type': 2,
        'survey_id_param': surveyId,
      },
    };

    final res = await post(uri.toString(), body, headers: headers);

    if (!res.isOk) return null;

    logger.d('response body: ${res.body}');

    return res.isOk;
  }
}
