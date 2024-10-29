use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

use crate::panel::Card;


pub struct MyAssetsPlugin;

impl Plugin for MyAssetsPlugin{
    fn build(&self, app: &mut App) {
        app .init_collection::<MyAssets>();
      
    }
}

#[derive(AssetCollection, Resource)]
pub struct MyAssets {
// atlas
    #[asset(texture_atlas_layout(tile_size_x =77 , tile_size_y = 75, columns =8 , rows = 1, padding_x = 0, padding_y = 0, offset_x = 0, offset_y = 0))]
    pub peashoote_layout: Handle<TextureAtlasLayout>,
    #[asset(texture_atlas_layout(tile_size_x =73, tile_size_y = 75, columns =8 , rows = 1, padding_x = 0, padding_y = 0, offset_x = 0, offset_y = 0))]
    pub sunflower_layout: Handle<TextureAtlasLayout>,
// background
    #[asset(path = "map0.jpg")]
    pub bg: Handle<Image>,
// bar&card
    #[asset(path = "./bar/bar5.png")]
    pub bar: Handle<Image>,
    #[asset(path = "Cards/card_2.png")]
    pub sunflower_card: Handle<Image>,
    #[asset(path = "Cards/card_1.png")]
    pub peashoote_card: Handle<Image>,
// plant
    #[asset(path = "Plants/plant_9.png")]
    pub peashooter: Handle<Image>,
    #[asset(path = "Plants/plant_0.png")]
    pub sunflower: Handle<Image>,
    
}

impl MyAssets {
    pub fn type_of(&self,card_type: Card) -> (Handle<Image>,Handle<TextureAtlasLayout>){
        match card_type {
            Card::peashooter=>(self.peashooter.clone(),self.peashoote_layout.clone()),
            Card::sunflower=>(self.sunflower.clone(),self.sunflower_layout.clone()),
        }
    }
}