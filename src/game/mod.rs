pub mod score;

use std::{
    convert::TryFrom, 
    str::FromStr
};

pub enum Gamemode {
    Standard,
    Taiko,
    Catch,
    Mania,
}

impl TryFrom<i32> for Gamemode {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x ==  Gamemode::Standard as i32 => Ok(Gamemode::Standard),
            x if x ==  Gamemode::Taiko as i32 => Ok(Gamemode::Taiko),
            x if x ==  Gamemode::Catch as i32 => Ok(Gamemode::Catch),
            x if x ==  Gamemode::Mania as i32 => Ok(Gamemode::Mania ),
            _ => Err(()),
        }
    }
}

impl FromStr for Gamemode {
    type Err = ();
    
    fn from_str(input: &str) -> Result<Gamemode, Self::Err> {
        match input {
            "Standard"  => Ok(Gamemode::Standard),
            "0"  => Ok(Gamemode::Standard),
            "Taiko"  => Ok(Gamemode::Taiko),
            "1"  => Ok(Gamemode::Taiko),
            "Catch"  => Ok(Gamemode::Catch),
            "2"  => Ok(Gamemode::Catch),
            "Mania" => Ok(Gamemode::Mania),
            "3" => Ok(Gamemode::Mania),
            _      => Err(()),
        }
    }
}