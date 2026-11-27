

pub struct GraphicSettings {
    pub screen_width: i32,
    pub screen_height: i32,
}

impl GraphicSettings {

    pub fn default() -> Self {
        GraphicSettings {
            screen_width: 810,
            screen_height: 1080
        } 
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_config() {
        let config = GraphicSettings::default();
        assert_eq!(config.screen_width, 810);
        assert_eq!(config.screen_height, 1080);
    }
}


