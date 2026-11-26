use engine::{graphics::GraphicSettings, run}; 


struct MetaData {
    title: &'static str
}

impl MetaData {
    
    fn new(title: &'static str) -> Self {
        Self {
            title
        } 
    }

}


fn main() {
    
    const GAME_TITLE: &str = "Space Raider";
    let meta_data = MetaData::new(GAME_TITLE);
   
    let graphic_settings = GraphicSettings::default();

    run(meta_data.title, graphic_settings);

}


