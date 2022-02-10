use regex::Regex;
use std::{fs, path::PathBuf};

use crate::{
    parser::beatmap::objects::{FollowPoint, HitObject, HitObjectExtra, HitSample, HitType, SliderData, CurveType},
    game::Gamemode,
};

use self::objects::SliderBody;

// exports
pub mod objects;

pub struct BeatmapFile {
    // internal metadata
    pub format_version: i32,

    // overall beatmap information
    pub title: String,
    pub title_unicode: String,
    pub artist: String,
    pub artist_unicode: String,
    pub gamemode: Gamemode,

    // difficulty information
    pub difficulty_name: String,

    // objects
    pub hit_objects: Vec<HitObject>,

    // general metadata
    pub audio: AudioMetadata,
    pub difficulty: DifficultyMetadata,
    pub metadata: Metadata,
}

pub struct Metadata {
    pub tags: Vec<String>,
    pub preview_time: i32,
}

pub struct AudioMetadata {
    pub filename: String,
}

pub struct DifficultyMetadata {
    pub hp_drain: f32,
    pub circle_size: f32,
    pub overall_difficulty: f32,
    pub approach_rate: f32,
    pub slider_multiplier: f32,
    pub slider_tickrate: f32,
}

impl Default for BeatmapFile {
    fn default() -> Self {
        // empty beatmap
        BeatmapFile {
            format_version: 0,

            // map metadata
            title: "".to_string(),
            title_unicode: "".to_string(),
            artist: "".to_string(),
            artist_unicode: "".to_string(),
            gamemode: Gamemode::Standard,

            // difficulty metadata
            difficulty_name: "".to_string(),

            // objects
            hit_objects: vec![],

            // general metadata
            audio: AudioMetadata {
                filename: "".to_string(),
            },
            difficulty: DifficultyMetadata {
                hp_drain: 0.0,
                circle_size: 0.0,
                overall_difficulty: 0.0,
                approach_rate: 0.0,
                slider_multiplier: 0.0,
                slider_tickrate: 0.0,
            },
            metadata: Metadata {
                tags: vec![],
                preview_time: 0,
            },
        }
    }
}

impl BeatmapFile {
    pub fn from_file(path: &str) -> BeatmapFile {
        BeatmapFile::from_str(&fs::read_to_string(&path).unwrap())
    }

    pub fn from_pathbuf(path: PathBuf) -> BeatmapFile {
        BeatmapFile::from_str(&fs::read_to_string(path).unwrap())
    }

    pub fn from_str(map_string: &str) -> BeatmapFile {
        // begin parse
        let lines = map_string.lines();
        let mut section = "";

        // regex
        let section_regex = Regex::new(r"\[(\w+)\]").unwrap();
        let kvp_regex = Regex::new(r"(\w+):\s?(.*)").unwrap();

        // empty bm
        let mut beatmap = BeatmapFile::default();

        // iterate through
        for s in lines {
            if s.starts_with("//") {
                // ignore comments
                continue;
            }

            if s.starts_with("osu file format v") {
                // version
                beatmap.format_version = s[17..].parse().unwrap();
            }

            if section_regex.is_match(&s) {
                section = &s[1..&s.len() - 1];
                continue;
            }

            match section {
                "General" => {
                    // general section
                    for cap in kvp_regex.captures_iter(&s) {
                        // read value
                        let value = &cap[2];
                        match &cap[1] {
                            "AudioFilename" => beatmap.audio.filename = value.clone().to_string(),
                            "PreviewTime" => {
                                beatmap.metadata.preview_time = value.clone().parse().unwrap()
                            },
                            "Mode" => beatmap.gamemode = value.clone().parse().unwrap(),
                            _ => continue,
                        }
                    }
                }

                "Difficulty" => {
                    // difficulty section
                    for cap in kvp_regex.captures_iter(&s) {
                        // read value
                        let value = &cap[2];
                        match &cap[1] {
                            "HPDrainRate" => beatmap.difficulty.hp_drain = value.clone().parse().unwrap(),
                            "CircleSize" => beatmap.difficulty.circle_size = value.clone().parse().unwrap(),
                            "OverallDifficulty" => beatmap.difficulty.overall_difficulty = value.clone().parse().unwrap(),
                            "ApproachRate" => beatmap.difficulty.approach_rate = value.clone().parse().unwrap(),
                            "SliderMultiplier" => beatmap.difficulty.slider_multiplier = value.clone().parse().unwrap(),
                            "SliderTickRate" => beatmap.difficulty.slider_tickrate = value.clone().parse().unwrap(),
                            _ => continue,
                        }
                    }}

                "Metadata" => {
                    for cap in kvp_regex.captures_iter(&s) {
                        // read value
                        let value = &cap[2];
                        match &cap[1] {
                            "Title" => beatmap.title = value.clone().to_string(),
                            "TitleUnicode" => beatmap.title_unicode = value.clone().to_string(),

                            "Artist" => beatmap.artist = value.clone().to_string(),
                            "ArtistUnicode" => beatmap.artist_unicode = value.clone().to_string(),

                            "Version" => beatmap.difficulty_name = value.clone().to_string(),
                            "Tags" => {
                                beatmap.metadata.tags = value
                                    .clone()
                                    .split_whitespace()
                                    .map(|s| s.to_string())
                                    .collect()
                            }
                            _ => continue,
                        }
                    }
                }
                
                "HitObjects" => {
                    // oh no
                    let values: Vec<String> = s.clone().split(",").map(|s| s.to_string()).collect();
                    let mut base = HitObject {
                        x: values[0].parse().unwrap_or(0.0),
                        y: values[1].parse().unwrap_or(0.0),
                        start_time: values[2].parse().unwrap_or(0),
                        end_time: 0,
                        hit_sound: values[4].parse().unwrap_or(0),
                        hit_type: values[3].parse().unwrap_or(1),
                        slider_data: None,
                        extra_data: None,
                    };

                    if let Some(val) = values.get(values.len() - 1) {
                        if val.contains(":") {
                            if (base.hit_type & HitType::Hold as i32) != 0 {
                                // mania hold
                                let t: Vec<String> =
                                    val.clone().split(":").map(|s| s.to_string()).collect();

                                // set extra data
                                base.end_time = t[0].parse().unwrap_or(0);
                                base.extra_data = Some(HitObjectExtra {
                                    hit_sample: HitSample {
                                        normal_set: t[1].parse().unwrap_or(0),
                                        additional_set: t[2].parse().unwrap_or(0),
                                        index: t[3].parse().unwrap_or(0),
                                        volume: t[4].parse().unwrap_or(0),
                                        file_name: t[5].clone(),
                                    },
                                });
                            } else {
                                // normal hitcircle
                                let t: Vec<String> =
                                    val.clone().split(":").map(|s| s.to_string()).collect();

                                // set extra data
                                base.extra_data = Some(HitObjectExtra {
                                    hit_sample: HitSample {
                                        normal_set: t[0].parse().unwrap_or(0),
                                        additional_set: t[1].parse().unwrap_or(0),
                                        index: t[2].parse().unwrap_or(0),
                                        volume: t[3].parse().unwrap_or(0),
                                        file_name: t[4].clone(),
                                    },
                                });
                            }
                        }
                    }

                    // slider information
                    if base.hit_type & (HitType::Slider as i32) != 0 {
                        let slider_data = values.get(5).unwrap(); // has to have slider stuff
                        let slider_split: Vec<String> =
                            slider_data.split("|").map(|s| s.to_string()).collect();

                        // setup base
                        let mut slider_base = SliderData {
                            curve_type: slider_split[0].parse().unwrap_or(CurveType::Catmull),
                            slider_points: vec![],
                            slider_body: SliderBody {}
                        };

                        for point in slider_split {
                            if point.contains(":") {
                                // sliderpoint
                                let point_data: Vec<String> =
                                point.split(":").map(|s| s.to_string()).collect();

                                slider_base.slider_points.push(FollowPoint {
                                    x: point_data[0].parse().unwrap_or(0.0) - base.x,
                                    y: point_data[1].parse().unwrap_or(0.0) - base.y,
                                })
                            }
                        }

                        // check if point is linear
                        if slider_base.slider_points.len() == 3 // length is at least 3
                            && slider_base.curve_type as i32 == CurveType::PerfectCurve as i32// is a perfect curve
                        {
                            // precision check
                            let point1 = &slider_base.slider_points[0];
                            let point2 = &slider_base.slider_points[1];
                            let point3 = &slider_base.slider_points[2];

                            let isLinear = f32::abs(
                                0.0 - ((point2.y - point1.y) * (point3.x - point1.x) - (point2.x - point1.x) * (point3.y - point1.y))
                            ) <= 0.001;

                            if isLinear == true {
                                // this is linear
                                slider_base.curve_type = CurveType::Linear; 
                            }
                        }

                        let mut slider_length = 0.0;
                        let mut repeat_count: i32 = values[6].parse().unwrap();
                        repeat_count = i32::max(0, repeat_count - 1);

                        if values.len() > 7 {
                            // slider length
                            slider_length = values[8].parse().unwrap_or(0.0);
                        }

                        // handle slider body
                        let expected_distance = f64::max(0, slider_length);
                        let mut slider_start = 0;
                        let mut slider_end = 0;

                        let mut i = 0;
                        for slider_point in slider_base.slider_points {
                            slider_end += 1;

                            if i == slider_base.slider_points.len() - 1
                                || (slider_base.slider_points[i].x == slider_base.slider_points[i + 1].x && slider_base.slider_points[i].y == slider_base.slider_points[i + 1].y)
                            {
                                // get a specific vector
                                // TODO: implement Clone for FollowPoint, as this just outright fails to compile as a result
                                let sub_path = slider_base.slider_points[slider_start..slider_end].to_vec();
                                let approximated_path;

                                // approximate subpath
                                // TODO: this is a bit hacky.
                                if slider_base.curve_type as i32 == CurveType::Linear as i32 {
                                    approximated_path = sub_path;
                                } else if slider_base.curve_type as i32 == CurveType::PerfectCurve as i32 {
                                    if slider_base.slider_points.len() != 3 || sub_path.len() != 3 {
                                        approximated_path = approximate_bezier(sub_path);
                                    }
                                    
                                    // approximate perfect curve
                                    approximated_path = vec![];
                                    
                                    let point1 = sub_path[0];
                                    let point2 = sub_path[1];
                                    let point3 = sub_path[2];

                                    // TODO: replace followpoint with vector 2 and add a fuckton of methods to subtract them.
                                    let point1Sq = f32::powf(f32::sqrt((((point2.x - point3.x) * (point2.x - point3.x) * ((point2.x - point3.x) * (point2.x - point3.x)) + ((point2.y - point3.y) * (point2.y - point3.y)) * ((point2.y - point3.y) * (point2.y - point3.y)))), 2.0);
                                    let point2Sq = f32::powf(f32::sqrt((((point2.x - point3.x) * (point2.x - point3.x) * ((point2.x - point3.x) * (point2.x - point3.x)) + ((point2.y - point3.y) * (point2.y - point3.y)) * ((point2.y - point3.y) * (point2.y - point3.y)))), 2.0);
                                    let point3Sq = f32::powf(f32::sqrt((((point2.x - point3.x) * (point2.x - point3.x) * ((point2.x - point3.x) * (point2.x - point3.x)) + ((point2.y - point3.y) * (point2.y - point3.y)) * ((point2.y - point3.y) * (point2.y - point3.y)))), 2.0);

                                    // TODO: the rest of this!
                                } else if slider_base.curve_type as i32 == CurveType::Catmull as i32 {
                                    // TODO: approximate catmull
                                } else {
                                    approximated_path = sub_path;
                                }

                            }

                            // increment index
                            i += 1;
                        }
                    }

                    // spinner
                    if base.hit_type & (HitType::Spinner as i32) != 0 {
                        base.end_time = values[5].parse().unwrap_or(0);

                        if let Some(mut s) = base.extra_data {
                            base.extra_data = Some(s);
                        } else {
                            if let Some(val) = values.get(values.len() - 1) {
                                if val.contains(":") {
                                    // set extra data
                                    base.extra_data = Some(HitObjectExtra {
                                        hit_sample: BeatmapFile::parse_hitsample(val),
                                    });
                                } else {
                                    base.extra_data = Some(HitObjectExtra {
                                        hit_sample: HitSample::default(),
                                    });
                                }
                            }
                        }
                    }

                    // push hitobject
                    beatmap.hit_objects.push(base);
                }
                

                /*"TimingPoints" => {
                    let values: Vec<String> = s.clone().split(",").map(|s| s.to_string()).collect();

                    let time = 
                },*/

                _ => continue,
            }
        }

        beatmap // return beatmap
    }

    pub fn parse_hitsample(val: & str) -> HitSample {
        let t: Vec<String> = val.clone().split(":").map(|s| s.to_string()).collect();

        HitSample {
            normal_set: t[0].parse().unwrap_or(0),
            additional_set: t[1].parse().unwrap_or(0),
            index: t[2].parse().unwrap_or(0),
            volume: t[3].parse().unwrap_or(0),
            file_name: t[4].clone(),
        }
    }
}

pub fn approximate_bezier(sub_points: Vec<FollowPoint>) -> Vec<FollowPoint> {
    let mut approximated_path = vec![];

    if sub_points.len() == 0 {
        // nothing, just return nothing
        return approximated_path
    }

    

    approximated_path
}