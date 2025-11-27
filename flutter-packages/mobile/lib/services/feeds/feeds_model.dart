class FeedModel {
  final int feedId;
  final List<int> spaceIds;
  final String feedType;
  final String? image;
  final String title;
  final String description;
  final int authorId;
  final String authorUrl;
  final String authorName;
  final int createdAt;

  final int? rewards;
  final int likes;
  final int comments;
  final int reposts;

  const FeedModel({
    required this.feedId,
    required this.spaceIds,
    required this.feedType,
    required this.image,
    required this.title,
    required this.description,
    required this.authorId,
    required this.authorUrl,
    required this.authorName,
    required this.createdAt,

    required this.rewards,
    required this.likes,
    required this.comments,
    required this.reposts,
  });
}

class FeedSummary {
  final int feedId;
  final List<int> spaceIds;
  final String feedType;
  final String? image;
  final String title;
  final String description;
  final int authorId;
  final bool isBookmarked;
  final String authorUrl;
  final String authorName;
  final int createdAt;

  final int? rewards;
  final int likes;
  final int comments;
  final int reposts;

  const FeedSummary({
    required this.feedId,
    required this.spaceIds,
    required this.feedType,
    required this.image,
    required this.title,
    required this.description,
    required this.isBookmarked,
    required this.authorId,
    required this.authorUrl,
    required this.authorName,
    required this.createdAt,

    required this.rewards,
    required this.likes,
    required this.comments,
    required this.reposts,
  });
}

class FeedV2SummaryModel {
  final String pk;

  final int createdAt;
  final int updatedAt;

  final String title;
  final String htmlContents;

  final int shares;
  final int likes;
  final int comments;

  final String authorDisplayName;
  final String authorProfileUrl;
  final String authorUsername;
  final String authorPk;
  final int authorType;

  final String? spacePk;
  final String? spaceType;

  final int booster;
  final int? rewards;

  final List<String> urls;
  final bool liked;

  const FeedV2SummaryModel({
    required this.pk,
    required this.createdAt,
    required this.updatedAt,
    required this.title,
    required this.htmlContents,
    required this.shares,
    required this.likes,
    required this.comments,
    required this.authorDisplayName,
    required this.authorProfileUrl,
    required this.authorUsername,
    required this.authorPk,
    required this.authorType,
    this.spacePk,
    this.spaceType,
    required this.booster,
    this.rewards,
    this.urls = const [],
    this.liked = false,
  });

  static String _asString(dynamic v, {String fallback = ''}) {
    if (v == null) return fallback;
    return v.toString();
  }

  static String? _asOptString(dynamic v) {
    if (v == null) return null;
    return v.toString();
  }

  static int _asInt(dynamic v, {int fallback = 0}) {
    if (v == null) return fallback;
    if (v is int) return v;
    if (v is num) return v.toInt();
    return int.tryParse(v.toString()) ?? fallback;
  }

  static int? _asOptInt(dynamic v) {
    if (v == null) return null;
    if (v is int) return v;
    if (v is num) return v.toInt();
    return int.tryParse(v.toString());
  }

  factory FeedV2SummaryModel.fromJson(Map<String, dynamic> json) {
    final urlsJson = json['urls'] as List<dynamic>? ?? const [];

    return FeedV2SummaryModel(
      pk: _asString(json['pk']),
      createdAt: _asInt(json['created_at']),
      updatedAt: _asInt(json['updated_at']),
      title: _asString(json['title']),
      htmlContents: _asString(json['html_contents']),
      shares: _asInt(json['shares']),
      likes: _asInt(json['likes']),
      comments: _asInt(json['comments']),
      authorDisplayName: _asString(json['author_display_name']),
      authorProfileUrl: _asString(json['author_profile_url']),
      authorUsername: _asString(json['author_username']),
      authorPk: _asString(json['auth_pk']),
      authorType: _asInt(json['author_type']),
      spacePk: _asOptString(json['space_pk']),
      spaceType: _asOptString(json['space_type']),
      booster: _asInt(json['booster']),
      rewards: _asOptInt(json['rewards']),
      urls: urlsJson.map((e) => e.toString()).toList(),
      liked: json['liked'] as bool? ?? false,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'pk': pk,
      'created_at': createdAt,
      'updated_at': updatedAt,
      'title': title,
      'html_contents': htmlContents,
      'shares': shares,
      'likes': likes,
      'comments': comments,
      'author_display_name': authorDisplayName,
      'author_profile_url': authorProfileUrl,
      'author_username': authorUsername,
      'auth_pk': authorPk,
      'author_type': authorType,
      'space_pk': spacePk,
      'space_type': spaceType,
      'booster': booster,
      'rewards': rewards,
      'urls': urls,
      'liked': liked,
    };
  }

  String? get mainImage => urls.isNotEmpty ? urls.first : null;
}

class FeedV2ListResult {
  final List<FeedV2SummaryModel> items;
  final String? bookmark;

  const FeedV2ListResult({required this.items, this.bookmark});
}
