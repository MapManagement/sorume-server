use crate::api_models::*;
use crate::services::*;

use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        profile_service::new_profile,
        profile_service::get_profile,
        profile_service::update_profile,
        profile_service::delete_profile
    ),
    components(schemas(
        profile_schema::PostProfile,
        profile_schema::GetProfile,
        profile_schema::PatchProfile,
        private_message_schema::PostPrivateMessage,
        private_message_schema::GetPrivateMessage,
        private_message_schema::PatchPrivateMessage,
        private_message_schema::DeletePostPrivateChat,
        group_chat_schema::PostGroupChat,
        group_chat_schema::GetGroupChat,
        group_chat_schema::PatchGroupChat,
        group_chat_message_schema::PostGroupChatMessage,
        group_chat_message_schema::GetGroupChatMessage,
        group_chat_message_schema::PatchGroupChatMessage,
        group_chat_member_schema::PostGroupChatMember
    ))
)]
pub struct ApiDoc;
