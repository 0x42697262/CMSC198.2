pub struct ReplayData {
    /// Game mode of the replay (0 = osu! Standard, 1 = Taiko, 2 = Catch the Beat, 3 = osu!mania)
    game_mode: u8,
    /// Version of the game when the replay was created (ex. 20131216)
    version: u32,
    /// osu! beatmap MD5 hash
    pub beatmap_hash: String,
    /// Player name
    pub player_name: String,
    /// osu! replay MD5 hash (includes certain properties of the replay)
    replay_hash: String,
    /// Number of 300s
    pub count300: u16,
    /// Number of 100s in standard, 150s in Taiko, 100s in CTB, 100s in mania
    pub count100: u16,
    /// Number of 50s in standard, small fruit in CTB, 50s in mania
    pub count50: u16,
    /// Number of Gekis in standard, Max 300s in mania
    count_geki: u16,
    /// Number of Katus in standard, 200s in mania
    count_katu: u16,
    /// Number of misses
    pub count_miss: u16,
    /// Total score displayed on the score report
    pub score: u32,
    /// Greatest combo displayed on the score report
    pub max_combo: u16,
    /// Perfect/full combo (1 = no misses and no slider breaks and no early finished sliders)
    pub perfect_combo: u8,
    // pub perfect_combo: bool,
    /// Mods used. See below for list of mod values.
    mods: u32,
    /// Life bar graph: comma separated pairs u/v, where u is the time in milliseconds into the song and v is a floating point value from 0 - 1 that represents the amount of life you have at the given time (0 = life bar is empty, 1= life bar is full)
    hp: String,
    /// Time stamp (Windows ticks)
    pub timestamp: u64,
    /// Length in bytes of compressed replay data
    replay_length: u32,
    /// Compressed replay data
    pub replay_data: Vec<u8>,
    /// Online Score ID
    online_score_id: u64,
    /// Additional mod information. Only present if [Target Practice](/wiki/Gameplay/Game_modifier/Target_Practice) is enabled.
    additional_mod_info: f64,
}

impl ReplayData {
    pub fn new(
        game_mode: u8,
        version: u32,
        beatmap_hash: String,
        player_name: String,
        replay_hash: String,
        count300: u16,
        count100: u16,
        count50: u16,
        count_geki: u16,
        count_katu: u16,
        count_miss: u16,
        score: u32,
        max_combo: u16,
        perfect_combo: u8,
        // perfect_combo: bool,
        mods: u32,
        hp: String,
        timestamp: u64,
        replay_length: u32,
        replay_data: Vec<u8>,
        online_score_id: u64,
        additional_mod_info: f64,
    ) -> Self {
        ReplayData {
            game_mode,
            version,
            beatmap_hash,
            player_name,
            replay_hash,
            count300,
            count100,
            count50,
            count_geki,
            count_katu,
            count_miss,
            score,
            max_combo,
            perfect_combo,
            mods,
            hp,
            timestamp,
            replay_length,
            replay_data,
            online_score_id,
            additional_mod_info,
        }
    }

    pub fn save() {}
}
