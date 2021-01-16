/// SCORE DATA ///
pub struct Score {
    // General Score Data
    pub score_id: i64,
    //pub mode: Gamemode,
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

// for score calculation
pub struct PartialScore {
	//pub mode: Gamemode,
	pub mods: i64,
    pub max_combo: i32,
}