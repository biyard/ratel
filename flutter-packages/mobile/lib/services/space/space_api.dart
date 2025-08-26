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

    if (!res.isOk) return MySpaceModel(spaces: [], boostings: []);

    final List<SpaceSummary> mySpace = [];
    final List<SpaceSummary> boostings = [];

    for (var i = 0; i < res.body["spaces"].length; i++) {
      final space = res.body["spaces"][i];
      mySpace.add(
        SpaceSummary(
          id: int.parse(space["id"].toString()),
          createdAt: int.parse(space["created_at"].toString()),
          updatedAt: int.parse(space["updated_at"].toString()),
          feedId: int.parse(space["feed_id"].toString()),
          title: space["title"],
          htmlContents: space["html_contents"],
          imageUrl: space["image_url"],

          //FIXME: connect api this field
          authorUrl: "",
          authorName: "",
          likes: 0,
          rewards: 0,
          comments: 0,
        ),
      );
    }

    for (var i = 0; i < res.body["boostings"].length; i++) {
      final space = res.body["boostings"][i];
      boostings.add(
        SpaceSummary(
          id: int.parse(space["id"].toString()),
          createdAt: int.parse(space["created_at"].toString()),
          updatedAt: int.parse(space["updated_at"].toString()),
          feedId: int.parse(space["feed_id"].toString()),
          title: space["title"],
          htmlContents: space["html_contents"],
          imageUrl: space["image_url"],

          //FIXME: connect api this field
          authorUrl: "",
          authorName: "",
          likes: 0,
          rewards: 0,
          comments: 0,
        ),
      );
    }
    return MySpaceModel(spaces: mySpace, boostings: boostings);
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

  Future<SpaceModel> getSpaceById(int spaceId) async {
    final uri = Uri.parse(apiEndpoint).resolve('/v1/spaces/$spaceId');

    final headers = <String, String>{'Content-Type': 'application/json'};
    final res = await get(uri.toString(), headers: headers);

    if (!res.isOk) {
      return SpaceModel(
        id: 0,
        feedId: 0,
        title: "",
        htmlContents: "",
        files: [],
        discussions: [],
        elearnings: [],
        surveys: [],
        comments: [],
        userResponses: [],
      );
    }

    logger.d("space info: ${res.body["user_responses"]}");
    final item = res.body;

    final List<FileModel> files = [];
    final List<DiscussionModel> discussions = [];
    final List<ElearningModel> elearnings = [];
    final List<CommentModel> comments = [];
    final List<SurveyResponse> responses = [];

    for (var i = 0; i < item["user_responses"].length; i++) {
      final res = item["user_responses"][i];
      responses.add(
        SurveyResponse(
          id: int.parse(res["id"].toString()),
          createdAt: int.parse(res["created_at"].toString()),
          surveyId: int.parse(res["survey_id"].toString()),
        ),
      );
    }

    for (var i = 0; i < item["files"].length; i++) {
      final file = item["files"][i];
      files.add(
        FileModel(
          name: file["name"],
          size: file["size"],
          ext: file["ext"],
          url: file["url"],
        ),
      );
    }

    for (var i = 0; i < item["discussions"].length; i++) {
      final discussion = item["discussions"][i];
      discussions.add(
        DiscussionModel(
          id: int.parse(discussion["id"].toString()),
          startedAt: int.parse(discussion["started_at"].toString()),
          endedAt: int.parse(discussion["ended_at"].toString()),
          name: discussion["name"],
          record: discussion["record"] ?? "",
        ),
      );
    }

    for (var i = 0; i < item["elearnings"].length; i++) {
      final elearning = item["elearnings"][i];
      elearnings.add(
        ElearningModel(
          id: int.parse(elearning["id"].toString()),
          files: [
            FileModel(
              name: elearning["files"][0]["name"],
              size: elearning["files"][0]["size"],
              ext: elearning["files"][0]["ext"],
              url: elearning["files"][0]["url"],
            ),
          ],
        ),
      );
    }

    final List<SurveyModel> surveys = [];
    final rawSurveys = item["surveys"] as List? ?? const [];

    for (final s in rawSurveys) {
      final sj = Map<String, dynamic>.from(s as Map);

      final List<QuestionModel> questions =
          (sj['questions'] as List? ?? const [])
              .whereType<Map>()
              .map((q) => QuestionModel.fromJson(Map<String, dynamic>.from(q)))
              .toList();

      surveys.add(
        SurveyModel(
          id: int.tryParse('${sj["id"]}') ?? 0,
          status: projectStatusFrom(sj["status"]),
          startedAt: int.tryParse('${sj["started_at"] ?? 0}') ?? 0,
          endedAt: int.tryParse('${sj["ended_at"] ?? 0}') ?? 0,
          questions: questions,
          responseCount: int.tryParse('${sj["response_count"] ?? 0}') ?? 0,
        ),
      );
    }

    for (var i = 0; i < item["feed_comments"].length; i++) {
      final comment = item["feed_comments"][i];

      comments.add(
        CommentModel(
          id: int.parse((comment["id"] ?? 0).toString()),
          createdAt: int.parse((comment["created_at"] ?? 0).toString()),
          nickname: comment["author"][0]["nickname"] ?? "",
          comment: comment["html_contents"] ?? "",
          profileUrl: comment["author"][0]["profile_url"] ?? "",
        ),
      );
    }

    return SpaceModel(
      id: int.parse(item["id"].toString()),
      feedId: int.parse(item["feed_id"].toString()),
      title: item["title"] ?? "",
      htmlContents: item["html_contents"] ?? "",
      files: files,
      discussions: discussions,
      elearnings: elearnings,
      surveys: surveys,
      comments: comments,
      userResponses: responses,
    );
  }
}
