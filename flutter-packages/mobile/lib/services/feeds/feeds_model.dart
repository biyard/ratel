class PostCommentListResult {
  final List<PostCommentModel> items;
  final String? bookmark;

  const PostCommentListResult({required this.items, this.bookmark});
}

class FeedSummaryModel {
  final String pk;

  final int createdAt;
  final int updatedAt;

  String title;
  final String htmlContents;

  final int shares;
  int likes;
  int comments;

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

  FeedSummaryModel({
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

  factory FeedSummaryModel.fromJson(Map<String, dynamic> json) {
    final urlsJson = json['urls'] as List<dynamic>? ?? const [];

    return FeedSummaryModel(
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
  final List<FeedSummaryModel> items;
  final String? bookmark;

  const FeedV2ListResult({required this.items, this.bookmark});
}

class PostDetailPostModel {
  final String pk;
  final String sk;

  final int createdAt;
  final int updatedAt;

  String title;
  String htmlContents;

  final int postType;
  final int status;
  final int? visibility;

  final int shares;
  int likes;
  int comments;

  final String userPk;
  final String authorDisplayName;
  final String authorProfileUrl;
  final String authorUsername;
  final int authorType;

  final String? spacePk;
  final int? spaceType;
  final int? spaceVisibility;
  final int? booster;
  final int? rewards;

  final List<String> urls;

  PostDetailPostModel({
    required this.pk,
    required this.sk,
    required this.createdAt,
    required this.updatedAt,
    required this.title,
    required this.htmlContents,
    required this.postType,
    required this.status,
    required this.visibility,
    required this.shares,
    required this.likes,
    required this.comments,
    required this.userPk,
    required this.authorDisplayName,
    required this.authorProfileUrl,
    required this.authorUsername,
    required this.authorType,
    this.spacePk,
    this.spaceType,
    this.spaceVisibility,
    this.booster,
    this.rewards,
    this.urls = const [],
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

  factory PostDetailPostModel.fromJson(Map<String, dynamic> json) {
    final urlsJson = json['urls'] as List<dynamic>? ?? const [];

    return PostDetailPostModel(
      pk: _asString(json['pk']),
      sk: _asString(json['sk']),
      createdAt: _asInt(json['created_at']),
      updatedAt: _asInt(json['updated_at']),
      title: _asString(json['title']),
      htmlContents: _asString(json['html_contents']),
      postType: _asInt(json['post_type']),
      status: _asInt(json['status']),
      visibility: _asOptInt(json['visibility']),
      shares: _asInt(json['shares']),
      likes: _asInt(json['likes']),
      comments: _asInt(json['comments']),
      userPk: _asString(json['user_pk']),
      authorDisplayName: _asString(json['author_display_name']),
      authorProfileUrl: _asString(json['author_profile_url']),
      authorUsername: _asString(json['author_username']),
      authorType: _asInt(json['author_type']),
      spacePk: _asOptString(json['space_pk']),
      spaceType: _asOptInt(json['space_type']),
      spaceVisibility: _asOptInt(json['space_visibility']),
      booster: _asOptInt(json['booster']),
      rewards: _asOptInt(json['rewards']),
      urls: urlsJson.map((e) => e.toString()).toList(),
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'pk': pk,
      'sk': sk,
      'created_at': createdAt,
      'updated_at': updatedAt,
      'title': title,
      'html_contents': htmlContents,
      'post_type': postType,
      'status': status,
      'visibility': visibility,
      'shares': shares,
      'likes': likes,
      'comments': comments,
      'user_pk': userPk,
      'author_display_name': authorDisplayName,
      'author_profile_url': authorProfileUrl,
      'author_username': authorUsername,
      'author_type': authorType,
      'space_pk': spacePk,
      'space_type': spaceType,
      'space_visibility': spaceVisibility,
      'booster': booster,
      'rewards': rewards,
      'urls': urls,
    };
  }

  String? get mainImage => urls.isNotEmpty ? urls.first : null;
}

class PostCommentModel {
  final String pk;
  final String sk;

  final int updatedAt;

  final String content;

  int likes;
  int reports;
  int replies;

  final String? parentCommentSk;

  final String authorPk;
  final String authorDisplayName;
  final String authorUsername;
  final String authorProfileUrl;

  bool liked;
  bool isReport;

  PostCommentModel({
    required this.pk,
    required this.sk,
    required this.updatedAt,
    required this.content,
    required this.likes,
    required this.reports,
    required this.replies,
    this.parentCommentSk,
    required this.authorPk,
    required this.authorDisplayName,
    required this.authorUsername,
    required this.authorProfileUrl,
    required this.liked,
    required this.isReport,
  });

  factory PostCommentModel.fromJson(Map<String, dynamic> json) {
    return PostCommentModel(
      pk: PostDetailPostModel._asString(json['pk']),
      sk: PostDetailPostModel._asString(json['sk']),
      updatedAt: PostDetailPostModel._asInt(json['updated_at']),
      content: PostDetailPostModel._asString(json['content']),
      likes: PostDetailPostModel._asInt(json['likes']),
      reports: PostDetailPostModel._asInt(json['reports']),
      replies: PostDetailPostModel._asInt(json['replies']),
      parentCommentSk: PostDetailPostModel._asOptString(
        json['parent_comment_sk'],
      ),
      authorPk: PostDetailPostModel._asString(json['author_pk']),
      authorDisplayName: PostDetailPostModel._asString(
        json['author_display_name'],
      ),
      authorUsername: PostDetailPostModel._asString(json['author_username']),
      authorProfileUrl: PostDetailPostModel._asString(
        json['author_profile_url'],
      ),
      liked: json['liked'] as bool? ?? false,
      isReport: json['is_report'] as bool? ?? false,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'pk': pk,
      'sk': sk,
      'updated_at': updatedAt,
      'content': content,
      'likes': likes,
      'replies': replies,
      'parent_comment_sk': parentCommentSk,
      'author_pk': authorPk,
      'author_display_name': authorDisplayName,
      'author_username': authorUsername,
      'author_profile_url': authorProfileUrl,
      'liked': liked,
    };
  }
}

class PostArtworkMetadataModel {
  final String traitType;
  final String value;
  final String? displayType;

  const PostArtworkMetadataModel({
    required this.traitType,
    required this.value,
    this.displayType,
  });

  factory PostArtworkMetadataModel.fromJson(Map<String, dynamic> json) {
    return PostArtworkMetadataModel(
      traitType: PostDetailPostModel._asString(json['trait_type']),
      value: PostDetailPostModel._asString(json['value']),
      displayType: PostDetailPostModel._asOptString(json['display_type']),
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'trait_type': traitType,
      'value': value,
      'display_type': displayType,
    };
  }
}

class PostRepostModel {
  final String pk;
  final String sk;

  final String postPk;
  final String postTitle;
  final String postHtmlContents;

  final String authorPk;
  final String authorDisplayName;
  final String authorProfileUrl;

  const PostRepostModel({
    required this.pk,
    required this.sk,
    required this.postPk,
    required this.postTitle,
    required this.postHtmlContents,
    required this.authorPk,
    required this.authorDisplayName,
    required this.authorProfileUrl,
  });

  factory PostRepostModel.fromJson(Map<String, dynamic> json) {
    return PostRepostModel(
      pk: PostDetailPostModel._asString(json['pk']),
      sk: PostDetailPostModel._asString(json['sk']),
      postPk: PostDetailPostModel._asString(json['post_pk']),
      postTitle: PostDetailPostModel._asString(json['post_title']),
      postHtmlContents: PostDetailPostModel._asString(
        json['post_html_contents'],
      ),
      authorPk: PostDetailPostModel._asString(json['author_pk']),
      authorDisplayName: PostDetailPostModel._asString(
        json['author_display_name'],
      ),
      authorProfileUrl: PostDetailPostModel._asString(
        json['author_profile_url'],
      ),
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'pk': pk,
      'sk': sk,
      'post_pk': postPk,
      'post_title': postTitle,
      'post_html_contents': postHtmlContents,
      'author_pk': authorPk,
      'author_display_name': authorDisplayName,
      'author_profile_url': authorProfileUrl,
    };
  }
}

class FeedModel {
  final PostDetailPostModel post;
  final List<PostCommentModel> comments;
  final List<PostArtworkMetadataModel> artworkMetadata;
  final PostRepostModel? repost;
  final bool isLiked;
  final bool isReport;
  final int permissions;

  const FeedModel({
    required this.post,
    List<PostCommentModel>? comments,
    List<PostArtworkMetadataModel>? artworkMetadata,
    this.repost,
    required this.isLiked,
    required this.isReport,
    required this.permissions,
  }) : comments = comments ?? const [],
       artworkMetadata = artworkMetadata ?? const [];

  factory FeedModel.fromJson(Map<String, dynamic> json) {
    return FeedModel(
      post: PostDetailPostModel.fromJson(
        (json['post'] as Map<String, dynamic>? ?? const {}),
      ),
      comments: (json['comments'] as List<dynamic>?)
          ?.map((e) => PostCommentModel.fromJson(e as Map<String, dynamic>))
          .toList(),
      artworkMetadata: (json['artwork_metadata'] as List<dynamic>?)
          ?.map(
            (e) => PostArtworkMetadataModel.fromJson(e as Map<String, dynamic>),
          )
          .toList(),
      repost: json['repost'] != null
          ? PostRepostModel.fromJson(json['repost'] as Map<String, dynamic>)
          : null,
      isLiked: json['is_liked'] as bool? ?? false,
      isReport: json['is_report'] as bool? ?? false,
      permissions: PostDetailPostModel._asInt(json['permissions']),
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'post': post.toJson(),
      'comments': comments.map((e) => e.toJson()).toList(),
      'artwork_metadata': artworkMetadata.map((e) => e.toJson()).toList(),
      'repost': repost?.toJson(),
      'is_liked': isLiked,
      'permissions': permissions,
    };
  }
}

class LikeCommentResponse {
  final bool liked;

  const LikeCommentResponse({required this.liked});

  factory LikeCommentResponse.fromJson(Map<String, dynamic> json) {
    return LikeCommentResponse(liked: json['liked'] as bool? ?? false);
  }
}

class LikePostResponse {
  final bool like;

  LikePostResponse({required this.like});

  factory LikePostResponse.fromJson(Map<String, dynamic> json) {
    return LikePostResponse(like: json['like'] as bool? ?? false);
  }
}
