mod cb_packet_declare_recipes { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketDeclareRecipes ; impl Packet for CbPacketDeclareRecipes { type PacketIDType = i32 ; type PacketContent = PacketDeclareRecipesContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 102 } } pub struct PacketDeclareRecipesContent { recipes: PacketDeclareRecipesContentArray ,

 } impl PacketContent for PacketDeclareRecipesContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.recipes.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let recipes : PacketDeclareRecipesContentArray = PacketContent :: read ( reader ) ?;;

 Ok ( Self { recipes } ) } } pub type PacketDeclareRecipesContentArray = Vec <PacketDeclareRecipesContentArrayContent >; pub struct PacketDeclareRecipesContentArrayContent { data_type: String ,

recipe_id: String ,

data: PacketDeclareRecipesContentArrayContentContent ,

 } impl PacketContent for PacketDeclareRecipesContentArrayContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.data_type.write(writer)?;;

total_bytes += self.recipe_id.write(writer)?;;

total_bytes += self.data.switch_write(false,writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let data_type : String = PacketContent :: read ( reader ) ?;;

let recipe_id : String = PacketContent :: read ( reader ) ?;;

let data : PacketDeclareRecipesContentArrayContentContent = PacketSwitch::switch_read(&not_found,reader)?;;

 Ok ( Self { data_type, recipe_id, data } ) } } pub enum PacketDeclareRecipesContentArrayContentContent { /// This switch variant requires a value minecraft:crafting_special_firework_star in the compare_to field

 MinecraftCraftingSpecialFireworkStar (minecraft_data::common::data::Void ) ,

/// This switch variant requires a value minecraft:crafting_special_firework_star_fade in the compare_to field

 MinecraftCraftingSpecialFireworkStarFade (minecraft_data::common::data::Void ) ,

/// This switch variant requires a value minecraft:blasting in the compare_to field

 MinecraftBlasting (crate::protocol::types::minecraft_smelting_format::MinecraftSmeltingFormat ) ,

/// This switch variant requires a value minecraft:smithing in the compare_to field

 MinecraftSmithing {

 base: crate::protocol::types::ingredient::Ingredient ,

addition: crate::protocol::types::ingredient::Ingredient ,

result: crate::protocol::types::slot::Slot ,

 } ,

/// This switch variant requires a value minecraft:stonecutting in the compare_to field

 MinecraftStonecutting {

 group: String ,

ingredient: crate::protocol::types::ingredient::Ingredient ,

result: crate::protocol::types::slot::Slot ,

 } ,

/// This switch variant requires a value minecraft:crafting_special_shielddecoration in the compare_to field

 MinecraftCraftingSpecialShielddecoration (minecraft_data::common::data::Void ) ,

/// This switch variant requires a value minecraft:crafting_special_suspiciousstew in the compare_to field

 MinecraftCraftingSpecialSuspiciousstew (minecraft_data::common::data::Void ) ,

/// This switch variant requires a value minecraft:smoking in the compare_to field

 MinecraftSmoking (crate::protocol::types::minecraft_smelting_format::MinecraftSmeltingFormat ) ,

/// This switch variant requires a value minecraft:crafting_special_bookcloning in the compare_to field

 MinecraftCraftingSpecialBookcloning (minecraft_data::common::data::Void ) ,

/// This switch variant requires a value minecraft:crafting_special_mapextending in the compare_to field

 MinecraftCraftingSpecialMapextending (minecraft_data::common::data::Void ) ,

/// This switch variant requires a value minecraft:crafting_special_repairitem in the compare_to field

 MinecraftCraftingSpecialRepairitem (minecraft_data::common::data::Void ) ,

/// This switch variant requires a value minecraft:crafting_special_banneraddpattern in the compare_to field

 MinecraftCraftingSpecialBanneraddpattern (minecraft_data::common::data::Void ) ,

/// This switch variant requires a value minecraft:crafting_shapeless in the compare_to field

 MinecraftCraftingShapeless {

 group: String ,

ingredients: PacketDeclareRecipesContentArrayContentContentContentArray ,

result: crate::protocol::types::slot::Slot ,

 } ,

/// This switch variant requires a value minecraft:crafting_special_bannerduplicate in the compare_to field

 MinecraftCraftingSpecialBannerduplicate (minecraft_data::common::data::Void ) ,

/// This switch variant requires a value minecraft:campfire_cooking in the compare_to field

 MinecraftCampfireCooking (crate::protocol::types::minecraft_smelting_format::MinecraftSmeltingFormat ) ,

/// This switch variant requires a value minecraft:crafting_shaped in the compare_to field

 MinecraftCraftingShaped {

 width: minecraft_data::common::data::VarInt ,

height: minecraft_data::common::data::VarInt ,

group: String ,

ingredients: PacketDeclareRecipesContentArrayContentContentContentArray ,

result: crate::protocol::types::slot::Slot ,

 } ,

/// This switch variant requires a value minecraft:smelting in the compare_to field

 MinecraftSmelting (crate::protocol::types::minecraft_smelting_format::MinecraftSmeltingFormat ) ,

/// This switch variant requires a value minecraft:crafting_special_firework_rocket in the compare_to field

 MinecraftCraftingSpecialFireworkRocket (minecraft_data::common::data::Void ) ,

/// This switch variant requires a value minecraft:crafting_special_mapcloning in the compare_to field

 MinecraftCraftingSpecialMapcloning (minecraft_data::common::data::Void ) ,

/// This switch variant requires a value minecraft:crafting_special_armordye in the compare_to field

 MinecraftCraftingSpecialArmordye (minecraft_data::common::data::Void ) ,

/// This switch variant requires a value minecraft:crafting_special_shulkerboxcoloring in the compare_to field

 MinecraftCraftingSpecialShulkerboxcoloring (minecraft_data::common::data::Void ) ,

/// This switch variant requires a value minecraft:crafting_special_tippedarrow in the compare_to field

 MinecraftCraftingSpecialTippedarrow (minecraft_data::common::data::Void ) ,

 } impl PacketSwitch for PacketDeclareRecipesContentArrayContentContent { type CompareType = void ; fn switch_read < Reader : BufRead > ( compare_to : & Self :: CompareType , reader : & mut Reader ) -> std :: io :: Result < Self > where Self : Sized { todo ! ( ) } fn switch_write < Writer : Write > ( self , write_compare : bool , writer : & mut Writer ) -> std :: io :: Result < usize > where Self : Sized { todo ! ( ) } } pub type PacketDeclareRecipesContentArrayContentContentContentArray = Vec <crate::protocol::types::ingredient::Ingredient >;

pub type PacketDeclareRecipesContentArrayContentContentContentArray = Vec <PacketDeclareRecipesContentArrayContentContentContentArrayArray >; pub type PacketDeclareRecipesContentArrayContentContentContentArrayArray = Vec <crate::protocol::types::ingredient::Ingredient >;

 }

 pub use cb_packet_declare_recipes ::*;
