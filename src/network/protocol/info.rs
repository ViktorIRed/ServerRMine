pub struct Info;


impl Info {
    pub const CURRENT_PROTOCOL: u8 = 18;

    pub const LOGIN_PACKET: u8 = 0x82;
	pub const LOGIN_STATUS_PACKET: u8 = 0x83;

	pub const MESSAGE_PACKET: u8 = 0x85;
	pub const SET_TIME_PACKET: u8 = 0x86;
	pub const START_GAME_PACKET: u8 = 0x87;
	pub const ADD_MOB_PACKET: u8 = 0x88;
	pub const ADD_PLAYER_PACKET: u8 = 0x89;
	pub const REMOVE_PLAYER_PACKET: u8 = 0x8a;

	pub const ADD_ENTITY_PACKET: u8 = 0x8c;
	pub const REMOVE_ENTITY_PACKET: u8 = 0x8d;
	pub const ADD_ITEM_ENTITY_PACKET: u8 = 0x8e;
	pub const TAKE_ITEM_ENTITY_PACKET: u8 = 0x8f;
	pub const MOVE_ENTITY_PACKET: u8 = 0x90;

	pub const ROTATE_HEAD_PACKET: u8 = 0x94;
	pub const MOVE_PLAYER_PACKET: u8 = 0x95;
	//pub const PLACE_BLOCK_PACKET: u8 = 0x96;
	pub const REMOVE_BLOCK_PACKET: u8 = 0x97;
	pub const UPDATE_BLOCK_PACKET: u8 = 0x98;
	pub const ADD_PAINTING_PACKET: u8 = 0x99;
	pub const EXPLODE_PACKET: u8 = 0x9a;
	pub const LEVEL_EVENT_PACKET: u8 = 0x9b;
	pub const TILE_EVENT_PACKET: u8 = 0x9c;
	pub const ENTITY_EVENT_PACKET: u8 = 0x9d;

	pub const PLAYER_EQUIPMENT_PACKET: u8 = 0xa0;
	pub const PLAYER_ARMOR_EQUIPMENT_PACKET: u8 = 0xa1;
	pub const INTERACT_PACKET: u8 = 0xa2;
	pub const USE_ITEM_PACKET: u8 = 0xa3;
	pub const PLAYER_ACTION_PACKET: u8 = 0xa4;

	pub const HURT_ARMOR_PACKET: u8 = 0xa6;
	pub const SET_ENTITY_DATA_PACKET: u8 = 0xa7;
	pub const SET_ENTITY_MOTION_PACKET: u8 = 0xa8;
	//pub const SET_ENTITY_LINK_PACKET: u8 = 0xa9;
	pub const SET_HEALTH_PACKET: u8 = 0xaa;
	pub const SET_SPAWN_POSITION_PACKET: u8 = 0xab;
	pub const ANIMATE_PACKET: u8 = 0xac;
	pub const RESPAWN_PACKET: u8 = 0xad;
	pub const SEND_INVENTORY_PACKET: u8 = 0xae;
	pub const DROP_ITEM_PACKET: u8 = 0xaf;
	pub const CONTAINER_OPEN_PACKET: u8 = 0xb0;
	pub const CONTAINER_CLOSE_PACKET: u8 = 0xb1;
	pub const CONTAINER_SET_SLOT_PACKET: u8 = 0xb2;
	pub const CONTAINER_SET_DATA_PACKET: u8 = 0xb3;
	pub const CONTAINER_SET_CONTENT_PACKET: u8 = 0xb4;
	//pub const CONTAINER_ACK_PACKET: u8 = 0xb5;
	pub const CHAT_PACKET: u8 = 0xb6;
	pub const ADVENTURE_SETTINGS_PACKET: u8 = 0xb7;
	pub const ENTITY_DATA_PACKET: u8 = 0xb8;
	//pub const PLAYER_INPUT_PACKET: u8 = 0xb9;
	pub const FULL_CHUNK_DATA_PACKET: u8 = 0xba;
	pub const UNLOAD_CHUNK_PACKET: u8 = 0xbb;
}