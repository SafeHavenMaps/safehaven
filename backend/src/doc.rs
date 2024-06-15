use crate::{
    api::{
        admin::{
            self, entities::AdminSearchRequest, AdminUserTokenClaims, LoginRequest, LoginResponse,
        },
        map::{
            self, FetchedEntity, NewCommentRequest, NewEntityRequest,
            SearchRequest as MapSearchRequest, ViewRequest,
        },
        root::{self, BootstrapResponse, SafeMode, StatusResponse},
        ErrorResponse,
    },
    models::{
        access_token::{
            AccessToken, AccessTokenStats, NewOrUpdateAccessToken, PermissionPolicy, Permissions,
        },
        category::{Category, NewOrUpdateCategory},
        comment::{
            AdminComment, AdminListedComment, AdminNewOrUpdateComment, PublicComment,
            PublicNewComment,
        },
        entity::{
            AdminEntity, AdminListedEntity, AdminNewOrUpdateEntity, PublicEntity,
            PublicListedEntity, PublicNewEntity, UnprocessedLocation,
        },
        entity_cache::{
            AdminCachedEntitiesWithPagination, AdminCachedEntity, Cluster, EntitiesAndClusters,
            ViewerCachedEntitiesWithPagination, ViewerCachedEntity,
        },
        family::{Family, Field, FieldType, Form, NewOrUpdateFamily},
        options::{
            CartographyClusterConfig, CartographyInitConfig, ConfigurationOption, GeneralOptions,
            SafeHavenOptions, SafeModeConfig,
        },
        statistics::HomePageStats,
        tag::{NewOrUpdateTag, Tag},
        user::{NewOrUpdatedUser, User},
    },
};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        root::status,
        root::boostrap,
        // map
        map::viewer_view_request,
        map::viewer_search_request,
        map::viewer_fetch_entity,
        map::viewer_new_comment,
        map::viewer_new_entity,
        // admin
        admin::admin_login,
        admin::admin_logout,
        admin::admin_login_check,
        // admin::options
        admin::options::admin_options_get,
        admin::options::admin_options_update,
        admin::options::admin_options_delete,
        // admin::users
        admin::users::admin_users_list,
        admin::users::admin_user_new,
        admin::users::admin_user_get,
        admin::users::admin_user_change_self_password,
        admin::users::admin_user_update,
        admin::users::admin_user_delete,
        // admin::access_tokens
        admin::access_tokens::admin_access_tokens_list,
        admin::access_tokens::admin_access_token_new,
        admin::access_tokens::admin_access_token_get,
        admin::access_tokens::admin_access_token_get_stats,
        admin::access_tokens::admin_access_token_update,
        admin::access_tokens::admin_access_token_delete,
        // admin::families
        admin::families::admin_families_list,
        admin::families::admin_family_new,
        admin::families::admin_family_get,
        admin::families::admin_family_update,
        admin::families::admin_family_delete,
        admin::families::admin_family_update_icon,
        admin::families::admin_family_delete_icon,
        // admin::categories
        admin::categories::admin_categories_list,
        admin::categories::admin_category_new,
        admin::categories::admin_category_get,
        admin::categories::admin_category_update,
        admin::categories::admin_category_delete,
        admin::categories::admin_category_update_icon,
        admin::categories::admin_category_delete_icon,
        // admin::tags
        admin::tags::admin_tags_list,
        admin::tags::admin_tag_new,
        admin::tags::admin_tag_get,
        admin::tags::admin_tag_update,
        admin::tags::admin_tag_delete,
        // admin::entities
        admin::entities::admin_entities_pending,
        admin::entities::admin_entities_search,
        admin::entities::admin_entity_new,
        admin::entities::admin_entity_get,
        admin::entities::admin_entity_update,
        admin::entities::admin_entity_delete,
        admin::entities::admin_entity_get_comments,
        admin::entities::admin_entity_register_parent,
        admin::entities::admin_entity_remove_parent,
        // admin::comments
        admin::comments::admin_comments_pending,
        admin::comments::admin_comment_new,
        admin::comments::admin_comment_get,
        admin::comments::admin_comment_update,
        admin::comments::admin_comment_delete,
        // admin::statistics
        admin::statistics::admin_home_stats,
        admin::statistics::admin_count_comments_entities
    ),
    components(schemas(
        // general
        ErrorResponse,
        // admin
        AdminUserTokenClaims,
        // stats
        HomePageStats,
        // root
        StatusResponse,
        SafeMode,
        BootstrapResponse,
        // options
        SafeHavenOptions,
        ConfigurationOption,
        GeneralOptions,
        SafeModeConfig,
        CartographyInitConfig,
        CartographyClusterConfig,
        // families
        Family,
        NewOrUpdateFamily,
        Form,
        Field,
        FieldType,
        // categories
        Category,
        NewOrUpdateCategory,
        // tags
        Tag,
        NewOrUpdateTag,
        // entities
        AdminEntity,
        AdminListedEntity,
        AdminNewOrUpdateEntity,
        PublicEntity,
        PublicListedEntity,
        PublicNewEntity,
        ViewerCachedEntity,
        ViewerCachedEntitiesWithPagination,
        AdminCachedEntitiesWithPagination,
        AdminCachedEntity,
        Cluster,
        EntitiesAndClusters,
        UnprocessedLocation,
        AdminSearchRequest,
        // comments
        AdminComment,
        PublicNewComment,
        AdminNewOrUpdateComment,
        AdminListedComment,
        PublicComment,
        // access_tokens
        AccessToken,
        AccessTokenStats,
        NewOrUpdateAccessToken,
        Permissions,
        PermissionPolicy,
        // users
        NewOrUpdatedUser,
        User,
        LoginRequest,
        LoginResponse,
        // map
        ViewRequest,
        MapSearchRequest,
        NewCommentRequest,
        NewEntityRequest,
        FetchedEntity,
    ))
)]
pub struct ApiDoc {}
