

pub struct GraphicSettings {
    pub screen_width: i32,
    pub screen_height: i32,
}

impl GraphicSettings {

    pub fn new(screen_width: i32, screen_height: i32) -> Self {
        GraphicSettings {
            screen_width,
            screen_height
        } 
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn screen_dimensions() {
        let config = GraphicSettings::new(1920, 1080);
        assert_eq!(config.screen_width, 1920);
        assert_eq!(config.screen_height, 1080);
    }

}



