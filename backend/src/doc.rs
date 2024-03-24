use crate::{
    api::{
        admin::{self, users::ChangePassword, LoginRequest, LoginResponse},
        map::{
            self, FetchedEntity, NewCommentRequest, NewEntityRequest, SearchRequest, ViewRequest,
        },
        root::{self, BootstrapResponse, HealthCheckResponse},
        ErrorResponse,
    },
    models::{
        access_token::{AccessToken, NewOrUpdateAccessToken, PermissionPolicy, Permissions},
        category::{Category, NewCategory, UpdateCategory},
        comment::{Comment, ListedComment, NewComment, PublicComment, UpdateComment},
        entity::{
            Entity, ListedEntity, NewEntity, PublicEntity, UnprocessedLocation, UpdateEntity,
        },
        entity_cache::CachedEntity,
        family::{Family, Field, FieldType, Form, NewOrUpdateFamily},
        tag::{NewOrUpdateTag, Tag},
        user::{NewUser, User},
    },
};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        root::health_check,
        root::boostrap,
        // map
        map::view_request,
        map::search_request,
        map::fetch_entity,
        map::new_comment,
        map::new_entity,
        // admin
        admin::login,
        // admin::users
        admin::users::list,
        admin::users::new,
        admin::users::get,
        admin::users::change_self_password,
        admin::users::change_password,
        admin::users::delete,
        // admin::access_tokens
        admin::access_tokens::list,
        admin::access_tokens::new,
        admin::access_tokens::get,
        admin::access_tokens::update,
        admin::access_tokens::delete,
        // admin::families
        admin::families::list,
        admin::families::new,
        admin::families::get,
        admin::families::update,
        admin::families::delete,
        // admin::categories
        admin::categories::list,
        admin::categories::new,
        admin::categories::get,
        admin::categories::update,
        admin::categories::delete,
        // admin::tags
        admin::tags::list,
        admin::tags::new,
        admin::tags::get,
        admin::tags::update,
        admin::tags::delete,
        // admin::entities
        admin::entities::pending,
        admin::entities::search,
        admin::entities::new,
        admin::entities::get,
        admin::entities::update,
        admin::entities::delete,
        admin::entities::get_comments,
        admin::entities::register_parent,
        admin::entities::remove_parent,
        // admin::comments
        admin::comments::pending,
        admin::comments::new,
        admin::comments::get,
        admin::comments::update,
        admin::comments::delete,
    ),
    components(schemas(
        // general
        ErrorResponse,
        // root
        HealthCheckResponse,
        BootstrapResponse,
        // families
        Family,
        NewOrUpdateFamily,
        Form,
        Field,
        FieldType,
        // categories
        Category,
        NewCategory,
        UpdateCategory,
        // tags
        Tag,
        NewOrUpdateTag,
        // entities
        Entity,
        NewEntity,
        UpdateEntity,
        ListedEntity,
        PublicEntity,
        CachedEntity,
        UnprocessedLocation,
        // comments
        Comment,
        NewComment,
        UpdateComment,
        ListedComment,
        PublicComment,
        // access_tokens
        AccessToken,
        NewOrUpdateAccessToken,
        Permissions,
        PermissionPolicy,
        // users
        NewUser,
        User,
        ChangePassword,
        LoginRequest,
        LoginResponse,
        // map
        ViewRequest,
        SearchRequest,
        NewCommentRequest,
        NewEntityRequest,
        FetchedEntity,
    ))
)]
pub struct ApiDoc {}
