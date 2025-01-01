use crate::posts::handlers::{
    __path_create_post, __path_get_post, __path_get_posts, __path_put_post, __path_remove_post,
};
use crate::users::handlers::{__path_create_user, __path_get_users};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(
    get_posts,
    get_post,
    create_post,
    put_post,
    remove_post,
    get_users,
    create_user
))]
pub struct ApiDoc;
