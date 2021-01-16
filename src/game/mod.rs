pub mod score;

use std::{
    convert::TryFrom, 
    str::FromStr
};

/// GAME MODE DATA ///

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

/// MOD DATA ///

enum Mods {
	NoMod = 0,
	NoFail = 1 << 0,
	Easy = 1 << 1,
	TouchDevice = 1 << 2,
	Hidden = 1 << 3,
	HardRock = 1 << 4,
	SuddenDeath = 1 << 5,
	DoubleTime = 1 << 6,
	Relax = 1 << 7,
	HalfTime = 1 << 8,
	Nightcore = 1 << 9,
	Flashlight = 1 << 10,
	Autoplay = 1 << 11,
	SpunOut = 1 << 12,
	Relax2 = 1 << 13,
	Perfect = 1 << 14,
	Mania4K = 1 << 15,
	Mania5K = 1 << 16,
	Mania6K = 1 << 17,
	Mania7K = 1 << 18,
	Mania8K = 1 << 19,
	FadeIn = 1 << 20,
	Random = 1 << 21,
	Cinema = 1 << 22,
	Target = 1 << 23,
	Mania9K = 1 << 24,
	ManiaCoOp = 1 << 25,
	Mania1K = 1 << 26,
	Mania3K = 1 << 27,
    Mania2K = 1 << 28
}

impl Mods {
    pub fn from_str(string: String) -> Mods {
        match string.as_str() {
            // Difficulty Reduction Mods
            "NF" => Mods::NoFail,
            "HT" => Mods::HalfTime,
            "EZ" => Mods::Easy,

            // Perfectionist Mods
            "SD" => Mods::SuddenDeath,
            "PF" => Mods::Perfect,

            // Difficulty Increase Mods
            "HR" => Mods::HardRock,
            "HD" => Mods::Hidden,
            "DT" => Mods::DoubleTime,
            "NC" => Mods::Nightcore,
            "FL" => Mods::Flashlight,

            // Extraneous Mods
            "RX" => Mods::Relax,
            "AP" => Mods::Relax2,
            "SO" => Mods::SpunOut,
            "AT" => Mods::Autoplay,
            "TD" => Mods::TouchDevice, // RIP TD Players.
            
            // No Mod
            "NM" => Mods::NoMod,
            _ => Mods::NoMod, // Default, if none available.
        }
    }
}