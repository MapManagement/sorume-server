use crate::api_models::*;
use crate::services::*;

use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        profile_service::new_profile,
        profile_service::get_profile,
        profile_service::update_profile,
        profile_service::delete_profile,
        private_message_service::new_private_message,
        private_message_service::get_private_message,
        private_message_service::get_private_chat_messages,
        private_message_service::update_private_message,
        private_message_service::delete_private_message,
        private_message_service::delete_private_chat_messages,
        group_chat_service::new_group_chat,
        group_chat_service::get_group_chat,
        group_chat_service::update_group_chat,
        group_chat_service::delete_group_chat,
        group_chat_message_service::new_group_chat_message,
        group_chat_message_service::get_all_group_chat_messages,
        group_chat_message_service::get_member_group_chat_messages,
        group_chat_message_service::update_group_chat_message,
        group_chat_message_service::delete_group_chat_message,
        group_chat_message_service::delete_all_group_chat_messages,
        group_chat_message_service::delete_profile_group_chat_messages,
        group_chat_members_service::new_group_chat_member,
        group_chat_members_service::get_all_group_chat_members,
        group_chat_members_service::delete_all_group_chat_members,
        group_chat_members_service::delete_single_group_chat_member
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
        group_chat_member_schema::PostGroupChatMember,
        group_chat_member_schema::GetGroupChatMember
    ))
)]
pub struct ApiDoc;
