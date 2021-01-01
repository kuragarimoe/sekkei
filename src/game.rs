/// SCORE DATA ///
pub struct Score {
    // General Score Data
    pub score_id: i64,
    pub mode: Gamemode,
    pub user_id: i64,
    pub beatmap_id: i32,
    pub mods: i64,

    // Score
    pub score: i32,
    pub max_combo: i32,
    pub count_300: i32,
    pub count_100: i32,
    pub count_50: i32,
    pub count_geki: i32,
    pub count_katu: i32,
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
    Mania2K = 1 << 28,
    
    /// FOR BETTER HANDLING ///
    UnrankedKeys = Mods::Mania1K | Mods::Mania2K | Mods::Mania3K | Mods::Mania9K | Mods::ManiaCoOp
}

pub enum Gamemode {
    Standard,
    Taiko,
    Catch,
    Mania,
}
