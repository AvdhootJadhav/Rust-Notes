#[derive(Debug, Clone, Copy)]
pub enum ShirtColor {
    Red,
    Blue,
}

pub struct Inventory {
    pub shirts: Vec<ShirtColor>
}

impl Inventory {
    
    pub fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    pub fn most_stocked(&self) -> ShirtColor {
        let mut red = 0;
        let mut blue = 0;

        for shirt in &self.shirts {
            match shirt {
                ShirtColor::Blue => blue+=1,
                ShirtColor::Red => red+=1
            }
        }

        if red > blue {
            ShirtColor::Red
        }
        else{
            ShirtColor::Blue
        }
    }
}