use regex::Regex;
use std::{fs, path::PathBuf};

use crate::{
    constants,
    game::Gamemode,
    parser::beatmap::objects::{
        CurveType, HitObject, HitObjectExtra, HitSample, HitType, InheritedTimingPoint, SliderData,
        SliderObject, SliderObjectType, TimingPoint, TimingPointType, UninheritedTimingPoint,
    },
    util::Vector2,
};

use self::objects::SliderBody;

// exports
pub mod objects;

#[derive(Debug)]
pub struct BeatmapFile {
    // internal metadata
    pub format_version: i32,

    // overall beatmap information
    pub title: String,
    pub title_unicode: String,
    pub artist: String,
    pub artist_unicode: String,
    pub gamemode: Gamemode,

    // timings
    pub timing_points: Vec<TimingPoint>,
    pub uninherited_points: Vec<UninheritedTimingPoint>,
    pub inherited_points: Vec<InheritedTimingPoint>,

    // difficulty information
    pub difficulty_name: String,
    pub stack_leniency: f32,

    // objects
    pub hit_objects: Vec<HitObject>,

    // general metadata
    pub audio: AudioMetadata,
    pub difficulty: DifficultyMetadata,
    pub metadata: Metadata,
}

#[derive(Debug)]
pub struct Metadata {
    pub tags: Vec<String>,
    pub preview_time: i32,
}

#[derive(Debug)]
pub struct AudioMetadata {
    pub filename: String,
    pub lead_in: i32
}

#[derive(Debug)]
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
            stack_leniency: 0.0,

            // objects
            hit_objects: vec![],

            // timings
            timing_points: vec![],
            uninherited_points: vec![],
            inherited_points: vec![],

            // general metadata
            audio: AudioMetadata {
                filename: "".to_string(),
                lead_in: 0
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
        let kvp_regex = Regex::new(r"(\w+):\s*(.*)").unwrap();

        // empty bm
        let mut beatmap = BeatmapFile::default();
        let mut timing_point = UninheritedTimingPoint {
            time: 0.0,
            beat_length: 0.0,
            time_signature: 4,
        };

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
                            "AudioLeadIn" => beatmap.audio.lead_in = value.clone().parse().unwrap_or(0),
                            "PreviewTime" => {
                                beatmap.metadata.preview_time = value.clone().parse().unwrap()
                            }
                            "Mode" => beatmap.gamemode = value.clone().parse().unwrap(),
                            "StackLeniency" => beatmap.stack_leniency = value.clone().parse().unwrap(),
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
                            "HPDrainRate" => {
                                beatmap.difficulty.hp_drain = value.clone().parse().unwrap()
                            }
                            "CircleSize" => {
                                beatmap.difficulty.circle_size = value.clone().parse().unwrap()
                            }
                            "OverallDifficulty" => {
                                beatmap.difficulty.overall_difficulty =
                                    value.clone().parse().unwrap()
                            }
                            "ApproachRate" => {
                                beatmap.difficulty.approach_rate = value.clone().parse().unwrap()
                            }
                            "SliderMultiplier" => {
                                beatmap.difficulty.slider_multiplier =
                                    value.clone().parse().unwrap()
                            }
                            "SliderTickRate" => {
                                beatmap.difficulty.slider_tickrate = value.clone().parse().unwrap()
                            }
                            _ => continue,
                        }
                    }
                }

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

                "TimingPoints" => {
                    let values: Vec<String> = s.clone().split(",").map(|s| s.to_string()).collect();

                    if (values.len() as i32) < 2 {
                        continue;
                    }

                    if let Some(_val) = values.get(values.len() - 1) {
                        let mut time: f32 = values[0].parse().unwrap_or(0.0);

                        if beatmap.format_version < 5 {
                            time = time + 24.0;
                        }

                        let beat_length: f32 = values[1].parse().unwrap_or(0.0);
                        let mut time_signature = 4;

                        let mut timing_change = false;
                        let change = values[2].parse::<i32>().unwrap_or(0);
                        if values.len() >= 3 {
                            if change == 0 {
                                time_signature = 4;
                            } else {
                                time_signature = values[2].parse().unwrap_or(0);
                            }
                        }

                        if values.len() >= 7 {
                            let timing_change_num = values[6].parse().unwrap_or(0);
                            timing_change = timing_change_num == 1;
                        }

                        if timing_change == true {
                            beatmap.uninherited_points.push(UninheritedTimingPoint {
                                time: time,
                                beat_length: beat_length,
                                time_signature: time_signature,
                            });

                            beatmap.timing_points.push(TimingPoint {
                                time: time,
                                beat_length: beat_length,
                                time_signature: time_signature,
                                speed_multiplier: if beat_length < 0.0 {
                                    100.0 / (-beat_length)
                                } else {
                                    1.0
                                },
                                point_type: TimingPointType::Uninherited,
                            });
                        } else {
                            beatmap.inherited_points.push(InheritedTimingPoint {
                                time: time,
                                speed_multiplier: if beat_length < 0.0 {
                                    100.0 / (-beat_length)
                                } else {
                                    1.0
                                },
                                inherited_from: timing_point.clone(),
                            });

                            beatmap.timing_points.push(TimingPoint {
                                time: time,
                                beat_length: beat_length,
                                time_signature: time_signature,
                                speed_multiplier: if beat_length < 0.0 {
                                    100.0 / (-beat_length)
                                } else {
                                    1.0
                                },
                                point_type: TimingPointType::Inherited,
                            });
                        }
                    }
                }

                "HitObjects" => {
                    // oh no
                    let values: Vec<String> = s.clone().split(",").map(|s| s.to_string()).collect();
                    let mut base = HitObject {
                        x: values[0].parse().unwrap_or(0.0),
                        y: values[1].parse().unwrap_or(0.0),
                        position: Vector2::new(
                            values[0].parse().unwrap_or(0.0),
                            values[1].parse().unwrap_or(0.0),
                        ),
                        end_position: Vector2::new(0.0, 0.0),
                        start_time: values[2].parse().unwrap_or(0.0),
                        end_time: 0.0,
                        hit_sound: values[4].parse().unwrap_or(0),
                        hit_type: values[3].parse().unwrap_or(1),
                        stack_height: 0,
                        slider_data: None,
                        slider_objects: None,
                        extra_data: None,
                    };

                    if let Some(val) = values.get(values.len() - 1) {
                        if val.contains(":") {
                            if (base.hit_type & HitType::Hold as i32) != 0 {
                                // mania hold
                                let t: Vec<String> =
                                    val.clone().split(":").map(|s| s.to_string()).collect();

                                // set extra data
                                base.end_time = t[0].parse().unwrap_or(0.0);
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
                            base_points: vec![],
                            slider_points: vec![Vector2::new(0.0, 0.0)],
                            slider_body: SliderBody {
                                body: vec![],
                                length: vec![0.0],
                            },
                        };

                        for point in slider_split {
                            if point.contains(":") {
                                // sliderpoint
                                let point_data: Vec<String> =
                                    point.split(":").map(|s| s.to_string()).collect();

                                slider_base.base_points.push(Vector2::new(
                                    point_data[0].parse().unwrap_or(0.0),
                                    point_data[1].parse().unwrap_or(0.0),
                                ));
                                slider_base.slider_points.push(
                                    Vector2::new(
                                        point_data[0].parse().unwrap_or(0.0),
                                        point_data[1].parse().unwrap_or(0.0),
                                    ) - Vector2::new(base.x, base.y),
                                );
                            }
                        }

                        // check if point is linear
                        if slider_base.slider_points.len() == 3 // length is at least 3
                            && slider_base.curve_type as i32 == CurveType::PerfectCurve as i32
                        // is a perfect curve
                        {
                            // precision check
                            let point1 = &slider_base.slider_points[0];
                            let point2 = &slider_base.slider_points[1];
                            let point3 = &slider_base.slider_points[2];

                            let is_linear = f32::abs(
                                0.0 - ((point2.y - point1.y) * (point3.x - point1.x)
                                    - (point2.x - point1.x) * (point3.y - point1.y)),
                            ) <= 0.001;

                            if is_linear == true {
                                // this is linear
                                slider_base.curve_type = CurveType::Linear;
                            }
                        }

                        let mut slider_length = 0.0;
                        let mut repeat_count: i32 = values[6].parse().unwrap();
                        repeat_count = i32::max(0, repeat_count - 1);

                        if values.len() > 7 {
                            // slider length
                            slider_length = values[7].parse().unwrap_or(0.0);
                        }

                        // handle slider body
                        let expected_distance = f32::max(0.0, slider_length);
                        let mut slider_start = 0;
                        let mut slider_end = 0;

                        for (i, _slider_point) in
                            slider_base.slider_points.clone().into_iter().enumerate()
                        {
                            slider_end += 1;

                            if i == &slider_base.slider_points.len() - 1
                                || (slider_base.slider_points[i].x
                                    == slider_base.slider_points[i + 1].x
                                    && slider_base.slider_points[i].y
                                        == slider_base.slider_points[i + 1].y)
                            {
                                // get a specific vector
                                let sub_path =
                                    slider_base.slider_points[slider_start..slider_end].to_vec();
                                let mut approximated_path;

                                // approximate subpath
                                // TODO: impl partialeq for curvetype
                                if slider_base.curve_type as i32 == CurveType::Linear as i32 {
                                    approximated_path = sub_path.clone();
                                } else if slider_base.curve_type as i32
                                    == CurveType::PerfectCurve as i32
                                {
                                    if slider_base.slider_points.len() != 3 || sub_path.len() != 3 {
                                        approximated_path =
                                            BeatmapFile::approximate_bezier(&sub_path);
                                    } else {
                                        approximated_path =
                                            BeatmapFile::approximate_perfect_curve(&sub_path);

                                        if approximated_path.len() == 0 {
                                            approximated_path =
                                                BeatmapFile::approximate_bezier(&sub_path);
                                        }
                                    }
                                } else if slider_base.curve_type as i32 == CurveType::Catmull as i32
                                {
                                    approximated_path = BeatmapFile::approximate_catmull(&sub_path)
                                } else {
                                    approximated_path = BeatmapFile::approximate_bezier(&sub_path);
                                }

                                // add to slider body
                                for point in &approximated_path {
                                    // if (this.calculatedPath.length === 0 || this.calculatedPath[this.calculatedPath.length - 1].x !== t.x || this.calculatedPath[this.calculatedPath.length - 1].y !== t.y) {
                                    if &slider_base.slider_body.body.len() == &0
                                        || &slider_base.slider_body.body
                                            [slider_base.slider_body.body.len() - 1]
                                            .x
                                            != &point.x
                                        || &slider_base.slider_body.body
                                            [slider_base.slider_body.body.len() - 1]
                                            .y
                                            != &point.y
                                    {
                                        &slider_base.slider_body.body.push(*point);
                                    }
                                }

                                slider_start = slider_end
                            }
                        }

                        // calculate path length
                        let mut length = 0.0;
                        for i in 0..slider_base.slider_body.body.len() - 1 {
                            let point = slider_base.slider_body.body[i];
                            let difference =
                                *slider_base.slider_body.body.get(i + 1).unwrap() - point;
                            let diff = difference.len();

                            if (expected_distance - length) < diff {
                                // :desolate:
                                slider_base.slider_body.body[i + 1] =
                                    point + point.scale((expected_distance - length) / diff);

                                // calculate drain
                                let index = i + 2;
                                let drain_amount = index
                                    ..(slider_base.slider_body.body.len()
                                        - (slider_base.slider_body.body.len() - 2 - i));
                                slider_base.slider_body.body.drain(drain_amount);

                                length = expected_distance;
                                slider_base.slider_body.length.push(length);
                                break;
                            }

                            length += diff;
                            slider_base.slider_body.length.push(length);
                        }

                        let cloned_body = slider_base.slider_body.body.clone();
                        let cloned_len = slider_base.slider_body.length.clone();
                        if length < expected_distance && cloned_body.len() > 1 {
                            let difference = *slider_base
                                .slider_body
                                .body
                                .get(cloned_body.len() - 1)
                                .unwrap()
                                - *slider_base
                                    .slider_body
                                    .body
                                    .get(cloned_body.len() - 2)
                                    .unwrap();
                            let diff = difference.len();

                            if diff <= 0.0 {
                                // do nothing
                            } else {
                                slider_base.slider_body.body[cloned_body.len() - 1] =
                                    slider_base.slider_body.body[cloned_body.len() - 1]
                                        + difference.scale((expected_distance - length) / diff);
                                slider_base.slider_body.length[cloned_len.len() - 1] =
                                    expected_distance;
                            }
                        }

                        // slider body obtained
                        base.slider_data = Some(slider_base);

                        // handle slider data //
                        let distance = f32::min(f32::max(1.0, 0.0), 1.0) * expected_distance;
                        let mut distance_index = cloned_len
                            .iter()
                            .position(|&f| f == distance)
                            .unwrap_or(usize::MAX); // unsafe as fuck, but i cant use -1.

                        // calculate ending position
                        let mut returned = false;
                        if distance_index == usize::MAX {
                            for i in 0..cloned_len.len() {
                                if cloned_len[i] > distance {
                                    distance_index = i;
                                    returned = true;
                                    break;
                                }
                            }

                            if returned == false {
                                distance_index = cloned_len.len() - 1;
                            }
                        }

                        // interpolate slider vertices
                        let mut vertice = Vector2::new(0.0, 0.0);

                        if distance_index <= 0 {
                            vertice = cloned_body[0];
                        } else if distance_index >= cloned_body.len() {
                            vertice = cloned_body[cloned_body.len() - 1];
                        } else {
                            let start = cloned_body[distance_index - 1];
                            let end = cloned_body[distance_index];

                            let distance_start = cloned_len[distance_index - 1];
                            let distance_end = cloned_len[distance_index];

                            if f32::abs(distance_start - distance_end)
                                <= constants::PRECISION_LENIENCE
                            {
                                vertice = start
                            } else {
                                let scale =
                                    (distance - distance_start) / (distance_end - distance_start);
                                vertice = start + ((end - start).scale(scale));
                            }
                        }

                        // calculate and setend position
                        base.end_position = base.position + vertice;

                        // calculate slider timing data
                        let timing_point = beatmap.get_timing_point(base.start_time);
                        let scoring_distance = 100.0
                            * beatmap.difficulty.slider_multiplier
                            * timing_point.speed_multiplier;
                        let velocity = scoring_distance / timing_point.beat_length;
                        let span_count = repeat_count + 1;
                        let tick_distance =
                            scoring_distance / beatmap.difficulty.slider_tickrate * 1.0;
                        let end_time =
                            base.start_time + (span_count as f32 * expected_distance / velocity);
                        let duration = end_time - base.start_time;

                        base.end_time = end_time;

                        // create slider hitobjects
                        let mut hitobjects = vec![];

                        // slider head & end
                        hitobjects.push(SliderObject {
                            x: base.position.x,
                            y: base.position.y,
                            position: base.position,
                            start_time: base.start_time,
                            span_index: 0,
                            repeat_index: 0,
                            span_start_time: 0.0,
                            slider_object_type: SliderObjectType::SliderHead,
                        });

                        // create slider ticks
                        let length = f32::min(100000.0, expected_distance);
                        let certified_tick_distance =
                            f32::min(f32::max(tick_distance, 0.0), length);

                        if certified_tick_distance == 0.0 {
                            // do nothing
                        } else {
                            let min_distance_from_end = velocity * 10.0;
                            let span_duration = duration / span_count as f32;

                            for span in 0..span_count {
                                let span_start = base.start_time + span as f32 * span_duration;
                                let reversed = span % 2 == 1;

                                let mut d = tick_distance;
                                while d < length - min_distance_from_end {
                                    if d > length - min_distance_from_end {
                                        break;
                                    }
                                    let progress = d / length;
                                    let time_progress = if reversed == true {
                                        1.0 - progress
                                    } else {
                                        progress
                                    };

                                    // calculate tick position
                                    let distance =
                                        f32::min(f32::max(progress, 0.0), 1.0) * expected_distance;
                                    let mut index = cloned_len
                                        .iter()
                                        .position(|&f| f == distance)
                                        .unwrap_or(usize::MAX);
                                    let mut returned = false;
                                    if index == usize::MAX {
                                        for i in 0..cloned_len.len() {
                                            if cloned_len[i] > distance {
                                                index = i;
                                                returned = true;
                                                break;
                                            }
                                        }

                                        if returned == false {
                                            index = cloned_len.len() - 1;
                                        }
                                    }
                                    // interpolate slider vertices
                                    let mut vertice = Vector2::new(0.0, 0.0);

                                    if index <= 0 {
                                        vertice = cloned_body[0];
                                    } else if index >= cloned_body.len() {
                                        vertice = cloned_body[cloned_body.len() - 1];
                                    } else {
                                        let start = cloned_body[index - 1];
                                        let end = cloned_body[index];

                                        let distance_start = cloned_len[index - 1];
                                        let distance_end = cloned_len[index];

                                        if f32::abs(distance_start - distance_end)
                                            <= constants::PRECISION_LENIENCE
                                        {
                                            vertice = start
                                        } else {
                                            let scale = (distance - distance_start)
                                                / (distance_end - distance_start);
                                            vertice = start + ((end - start).scale(scale));
                                        }
                                    }
                                    let tick_position = base.position + vertice;

                                    hitobjects.push(SliderObject {
                                        x: tick_position.x,
                                        y: tick_position.y,
                                        position: tick_position,
                                        start_time: span_start + time_progress * span_duration,
                                        span_index: span,
                                        repeat_index: 0,
                                        span_start_time: span_start,
                                        slider_object_type: SliderObjectType::SliderTick,
                                    });

                                    d += tick_distance;
                                }
                            }
                        }

                        // parse repeat points
                        let mut repeat = 1.0;
                        let mut repeat_index = 0;
                        while repeat_index < repeat_count {
                            let distance =
                                f32::min(f32::max(repeat % 2.0, 0.0), 1.0) * expected_distance;
                            let mut index = cloned_len
                                .iter()
                                .position(|&f| f == distance)
                                .unwrap_or(usize::MAX);
                            let mut returned = false;
                            if index == usize::MAX {
                                for i in 0..cloned_len.len() {
                                    if cloned_len[i] > distance {
                                        index = i;
                                        returned = true;
                                        break;
                                    }
                                }

                                if returned == false {
                                    index = cloned_len.len() - 1;
                                }
                            }
                            // interpolate slider vertices
                            let mut vertice = Vector2::new(0.0, 0.0);

                            if index <= 0 {
                                vertice = cloned_body[0];
                            } else if index >= cloned_body.len() {
                                vertice = cloned_body[cloned_body.len() - 1];
                            } else {
                                let start = cloned_body[index - 1];
                                let end = cloned_body[index];

                                let distance_start = cloned_len[index - 1];
                                let distance_end = cloned_len[index];

                                if f32::abs(distance_start - distance_end)
                                    <= constants::PRECISION_LENIENCE
                                {
                                    vertice = start
                                } else {
                                    let scale = (distance - distance_start)
                                        / (distance_end - distance_start);
                                    vertice = start + ((end - start).scale(scale));
                                }
                            }
                            
                            let repeat_position = base.position + vertice;
                            let span_duration = duration / span_count as f32;

                            hitobjects.push(SliderObject {
                                x: repeat_position.x,
                                y: repeat_position.y,
                                position: repeat_position,
                                start_time: base.start_time + (repeat * span_duration),
                                span_index: 0,
                                repeat_index: repeat_index,
                                span_start_time: span_duration,
                                slider_object_type: SliderObjectType::SliderTick,
                            });

                            repeat_index += 1;
                            repeat += 1.0;
                        }

                        // add sliderend
                        hitobjects.push(SliderObject {
                            x: base.end_position.x,
                            y: base.end_position.y,
                            position: base.end_position,
                            start_time: f32::max(base.start_time + duration / 2.0, base.end_time - constants::LEGACY_TICK_OFFSET),
                            span_index: 0,
                            repeat_index: 0,
                            span_start_time: 0.0,
                            slider_object_type: SliderObjectType::SliderEnd,
                        });

                        hitobjects.sort_by(|a, b| a.start_time.partial_cmp(&b.start_time).unwrap());
                        base.slider_objects = Some(hitobjects);
                    }

                    // spinner
                    if base.hit_type & (HitType::Spinner as i32) != 0 {
                        base.end_time = values[5].parse().unwrap_or(0.0);

                        if let Some(s) = base.extra_data {
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
                _ => continue,
            }
        }

        // apply stacking
        let stack_distance = 3.0;
        let mut time_preempt = 600.0;
        let end_index = beatmap.hit_objects.len() - 1;

        if beatmap.difficulty.approach_rate > 5.0 {
            time_preempt = 1200.0 + (450.0 - 1200.0) * (beatmap.difficulty.approach_rate - 5.0) / 5.0;
        } else if beatmap.difficulty.approach_rate < 5.0 {
            time_preempt = 1200.0 + (1200.0 - 1800.0) * (5.0 - beatmap.difficulty.approach_rate) / 5.0;
        } else {
            time_preempt = 1200.0;
        }

        let mut extended_end = beatmap.hit_objects.len() - 1;

        if end_index < beatmap.hit_objects.len() - 1 {
            let mut i = end_index;
            while i >= 0 {
                let stack_base = i;

                let mut j = stack_base + 1;
                while j < beatmap.hit_objects.len() {
                    let stack_base_object = &beatmap.hit_objects[stack_base];

                    if stack_base_object.hit_type as i32 == HitType::Spinner as i32 {
                        break;
                    }

                    let stack_j = &beatmap.hit_objects[j];

                    if stack_j.hit_type as i32 == HitType::Spinner as i32 {
                        break;
                    }

                    let end_time = if stack_base_object.hit_type as i32 == HitType::Slider as i32 {
                        stack_base_object.end_time
                    } else {
                        stack_base_object.start_time
                    };
                    let stack_threshold = time_preempt * beatmap.stack_leniency;

                    if stack_j.start_time - end_time > stack_threshold {
                        break;
                    }

                    let endpos_distance_check = if stack_base_object.hit_type as i32 == HitType::Slider as i32 {
                        stack_base_object.end_position.distance(stack_j.position) < stack_distance
                    } else {
                        false
                    };

                    if stack_base_object.position.distance(stack_j.position) < stack_distance || endpos_distance_check {
                        beatmap.hit_objects[j].stack_height = 0;
                    }
                    
                    j += 1;
                }

                if stack_base > extended_end {
                    extended_end = stack_base;

                    if extended_end == beatmap.hit_objects.len() - 1 {
                        break;
                    }
                }

                i -= 1;
            }
        }

        let mut extended_start = 0;
        let mut i2 = extended_end;
        while i2 > 0 {
            let mut n = i2;

            if beatmap.hit_objects[i2].stack_height != 0 || beatmap.hit_objects[i2].hit_type as i32 == HitType::Slider as i32 {
                i2 -= 1;
                continue;
            }

            let stack_threshold = time_preempt * beatmap.stack_leniency;

            if beatmap.hit_objects[i2].hit_type as i32 == HitType::Normal as i32 {
                while n >= 0 {
                    n -= 1;

                    if beatmap.hit_objects[n].hit_type as i32 == HitType::Spinner as i32 {
                        continue;
                    }

                    let end_time = if beatmap.hit_objects[n].hit_type as i32 == HitType::Normal as i32 {
                        beatmap.hit_objects[n].start_time
                    } else {
                        beatmap.hit_objects[n].end_time
                    };

                    if &beatmap.hit_objects[i2].start_time - end_time > stack_threshold {
                        break;
                    }
                    
                    if n < extended_start {
                        beatmap.hit_objects[n].stack_height = 0;
                        extended_start = n;
                    }

                    let endpos_distance_check = if beatmap.hit_objects[n].hit_type as i32 == HitType::Slider as i32 {
                        beatmap.hit_objects[n].end_position.distance(beatmap.hit_objects[i2].position) < stack_distance
                    } else {
                        false
                    };
                    if endpos_distance_check {
                        let offset = beatmap.hit_objects[i2].stack_height - beatmap.hit_objects[n].stack_height + 1;
                        let mut j = n + 1;
                        while j <= i2 + 1 {
                            let stack_j = &beatmap.hit_objects[j];
                            if beatmap.hit_objects[n].end_position.distance(stack_j.position) < stack_distance {
                                beatmap.hit_objects[j].stack_height -= offset;
                            }
                        }

                        break;
                    }

                    if beatmap.hit_objects[n].position.distance(beatmap.hit_objects[i2].position) < stack_distance {
                        beatmap.hit_objects[n].stack_height = beatmap.hit_objects[i2].stack_height + 1;
                    }
                }
            } else if beatmap.hit_objects[i2].hit_type as i32 == HitType::Slider as i32 {
                while n >= 0 {
                    n -= 1;

                    let stack_n = &beatmap.hit_objects[n];

                    if stack_n.hit_type as i32 == HitType::Spinner as i32 {
                        continue;
                    }

                    if &beatmap.hit_objects[i2].start_time - stack_n.start_time > stack_threshold {
                        break;
                    }

                    let stack_n_endpos = if stack_n.hit_type as i32 == HitType::Normal as i32 {
                        stack_n.position
                    } else {
                        stack_n.end_position
                    };

                    if stack_n_endpos.distance(beatmap.hit_objects[i2].position) < stack_distance {
                        beatmap.hit_objects[n].stack_height = beatmap.hit_objects[i2].stack_height + 1;
                    }
                }
            }

            i2 -= 1;
        }

        beatmap // return beatmap
    }

    pub fn get_timing_point(&self, time: f32) -> TimingPoint {
        // sort timing points
        let mut timing_points = self.timing_points.clone();
        timing_points.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());

        let mut current_index = 0;
        let mut current_point = &timing_points[0];

        for timing_point in &timing_points {
            if timing_point.time > time {
                if current_index == 0 {
                    break;
                }

                current_index = current_index - 1;
                current_point = timing_point;
                break;
            }

            current_index += 1;
        }

        if current_point.time < time {
            return timing_points[0];
        }

        timing_points[current_index]
    }

    pub fn get_uninherited_timing_point(&self, time: f32) -> UninheritedTimingPoint {
        // sort timing points
        let mut timing_points = self.uninherited_points.clone();
        timing_points.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());

        let mut current_index = 0;
        let mut current_point = &timing_points[0];

        for timing_point in &timing_points {
            if timing_point.time > time {
                if current_index == 0 {
                    break;
                }

                current_index = current_index - 1;
                current_point = timing_point;
                break;
            }

            current_index += 1;
        }

        if current_point.time < time {
            return timing_points[0];
        }

        timing_points[current_index]
    }

    pub fn get_inherited_timing_point(&self, time: f32) -> InheritedTimingPoint {
        // sort timing points
        let mut timing_points = self.inherited_points.clone();
        timing_points.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());

        let mut current_index = 0;
        let mut current_point = &timing_points[0];

        for timing_point in &timing_points {
            if timing_point.time > time {
                if current_index == 0 {
                    break;
                }

                current_index = current_index - 1;
                current_point = timing_point;
                break;
            }

            current_index += 1;
        }

        if current_point.time < time {
            return timing_points[0];
        }

        timing_points[current_index]
    }

    pub fn parse_hitsample(val: &str) -> HitSample {
        let t: Vec<String> = val.clone().split(":").map(|s| s.to_string()).collect();

        HitSample {
            normal_set: t[0].parse().unwrap_or(0),
            additional_set: t[1].parse().unwrap_or(0),
            index: t[2].parse().unwrap_or(0),
            volume: t[3].parse().unwrap_or(0),
            file_name: t[4].clone(),
        }
    }

    pub fn approximate_bezier(sub_points: &Vec<Vector2>) -> Vec<Vector2> {
        let mut approximated_path = vec![];

        if sub_points.len() == 0 {
            // nothing, just return nothing
            return approximated_path;
        }

        let mut subdiv_buffer1 = vec![];
        let mut subdiv_buffer2 = vec![];

        for _i in 0..sub_points.len() {
            subdiv_buffer1.push(Vector2::new(0.0, 0.0));
        }

        for _i in 0..((sub_points.len() * 2) - 1) {
            subdiv_buffer2.push(Vector2::new(0.0, 0.0));
        }

        let mut to_flatten = vec![];
        let mut free_buffers = vec![];
        let deep_copy = sub_points.clone();

        // copy the base slider points to avoid overriding original ones
        to_flatten.push(deep_copy);

        while to_flatten.len() > 0 {
            let mut parent = to_flatten.pop().unwrap();
            let mut flat_enough = true;

            // are the control points we're using flat enough?
            for i in 1..parent.len() - 1 {
                let scale = parent[i].scale(2.0);
                let subtract = parent[i - 1] - scale;
                let sum = subtract + parent[i + 1];

                if sum.len().powf(2.0)
                    > constants::BEZIER_TOLERANCE * constants::BEZIER_TOLERANCE * 4.0
                {
                    flat_enough = false;
                    break;
                }
            }

            if flat_enough == true {
                // subdivide
                let approxmid_points = &mut subdiv_buffer1;
                for i in 0..sub_points.len() {
                    approxmid_points[i] = parent[i];
                }

                for i in 0..sub_points.len() {
                    subdiv_buffer2[i] = approxmid_points[0];
                    approxmid_points[parent.len() - i - 1] = approxmid_points[parent.len() - i - 1];

                    for j in 0..((sub_points.len() - i) - 1) {
                        approxmid_points[j] =
                            (approxmid_points[j] + approxmid_points[j + 1]).div(2.0);
                    }
                }

                // reuse 2nd buffer for next iteration
                for i in 0..(sub_points.len() - 1) {
                    subdiv_buffer2[sub_points.len() + i] = subdiv_buffer1[i + 1];
                }

                approximated_path.push(parent[0]);

                for i in 1..(sub_points.len() - 1) {
                    let index = 2 * i;
                    let vector = (subdiv_buffer2[index - 1]
                        + (Vector2::new(2.0, 2.0).mul(subdiv_buffer2[index])
                            + subdiv_buffer2[index + 1]))
                        .scale(0.25);
                    approximated_path.push(vector);
                }

                // push
                free_buffers.push(parent);
                continue;
            }

            // no, it is not.
            // further flatten the curve to get a close enough approximation
            // we might not yet have a flat approximation, ` we'd need to subdivide a bare array
            let mut right_child = vec![];
            if free_buffers.len() > 0 {
                right_child = free_buffers.pop().unwrap();
            } else {
                for _i in 0..sub_points.len() {
                    right_child.push(Vector2::new(0.0, 0.0));
                }
            }

            // subdivide
            let mid_points = &mut subdiv_buffer1;
            for i in 0..sub_points.len() {
                mid_points[i] = parent[i].clone();
            }

            for i in 0..sub_points.len() {
                subdiv_buffer2[i] = mid_points[0];
                right_child[sub_points.len() - i - 1] = mid_points[sub_points.len() - i - 1];

                for j in 0..(sub_points.len() - i - 1) {
                    mid_points[j] = (mid_points[j] + mid_points[j + 1]).div(2.0);
                }
            }

            for i in 0..sub_points.len() {
                parent[i] = subdiv_buffer2[i];
            }

            to_flatten.push(right_child);
            to_flatten.push(parent);
        }

        approximated_path.push(sub_points[sub_points.len() - 1]);
        approximated_path
    }

    pub fn approximate_perfect_curve(sub_points: &Vec<Vector2>) -> Vec<Vector2> {
        let mut approximated_path = vec![];

        // clone points to avoid overriding original points
        let point1 = sub_points[0].clone();
        let point2 = sub_points[1].clone();
        let point3 = sub_points[2].clone();

        // squared point lengths
        let point1_sq = (point2 - point3).len().powf(2.0);
        let point2_sq = (point1 - point3).len().powf(2.0);
        let point3_sq = (point1 - point2).len().powf(2.0);

        if f32::abs(point1_sq - 0.0) <= constants::PRECISION_LENIENCE
            || f32::abs(point2_sq - 0.0) <= constants::PRECISION_LENIENCE
            || f32::abs(point3_sq - 0.0) <= constants::PRECISION_LENIENCE
        {
            return vec![];
        }

        let point1_s = point1_sq * (point2_sq + point3_sq - point1_sq);
        let point2_s = point2_sq * (point1_sq + point3_sq - point2_sq);
        let point3_s = point3_sq * (point1_sq + point2_sq - point3_sq);

        let point_sum = point1_s + point2_s + point3_s;

        if f32::abs(point_sum - 0.0) <= constants::PRECISION_LENIENCE {
            return vec![];
        }

        // handle mathematics
        let point_center =
            (point1.scale(point1_s) + point2.scale(point2_s) + point3.scale(point3_s))
                .div(point_sum);
        let d_a = point1 - point_center;
        let d_c = point3 - point_center;
        let rad = d_a.len();

        let theta_start = f32::atan2(d_a.y, d_a.x);
        let mut theta_end = f32::atan2(d_c.y, d_c.x);

        while theta_end < theta_start {
            theta_end += 2.0 * std::f32::consts::PI;
        }

        let mut dir = 1.0;
        let mut theta_range = theta_end - theta_start;

        let mut ortho_atoc = point3 - point1;
        ortho_atoc = Vector2::new(ortho_atoc.y, -ortho_atoc.x);

        if ortho_atoc.dot(point2 - point1) < 0.0 {
            dir = -1.0;
            theta_range = 2.0 * std::f32::consts::PI - theta_range;
        }

        let points = if 2.0 * rad <= 0.1 {
            2
        } else {
            f32::max(
                2.0,
                (theta_range / (2.0 * f32::acos(1.0 - 0.1 / rad))).ceil(),
            ) as usize
        };

        for i in 0..points {
            let fraction = i as f32 / (points - 1) as f32;
            let theta = theta_start + dir * fraction * theta_range;

            approximated_path.push(Vector2::new(f32::cos(theta), f32::sin(theta)).scale(rad));
        }

        approximated_path
    }

    pub fn approximate_catmull(sub_points: &Vec<Vector2>) -> Vec<Vector2> {
        let mut approximated_path = vec![];

        for i in 0..(sub_points.len() - 1) {
            let vec1 = if i > 0 {
                sub_points[i - 1]
            } else {
                sub_points[i]
            };
            let vec2 = sub_points[i];
            let vec3 = if i < (sub_points.len() - 1) {
                sub_points[i + 1]
            } else {
                vec2 + vec2 - vec1
            };
            let vec4 = if i < (sub_points.len() - 2) {
                sub_points[i + 2]
            } else {
                vec3 + vec3 - vec2
            };

            for c in 0..constants::CATMULL_DETAIL {
                let t = c as f32 / constants::CATMULL_DETAIL as f32;
                let t2 = t * t;
                let t3 = t * t2;

                let t_b = (c + 1) as f32 / constants::CATMULL_DETAIL as f32;
                let t2_b = t_b * t_b;
                let t3_b = t_b * t2_b;

                let p1 = Vector2::new(
                    0.5 * (2.0 * vec2.x + (-vec1.x + vec3.x) * t + (2.0 * vec1.x - 5.0 * vec2.x + 4.0 * vec3.x - vec4.x) * t2 + (-vec1.x + 3.0 * vec2.x - 3.0 * vec3.x + vec4.x) * t3),
                    0.5 * (2.0 * vec2.y + (-vec1.y + vec3.y) * t + (2.0 * vec1.y - 5.0 * vec2.y + 4.0 * vec3.y - vec4.y) * t2 + (-vec1.y + 3.0 * vec2.y - 3.0 * vec3.y + vec4.y) * t3)
                );

                let p2 = Vector2::new(
                    0.5 * (2.0 * vec2.x + (-vec1.x + vec3.x) * t_b + (2.0 * vec1.x - 5.0 * vec2.x + 4.0 * vec3.x - vec4.x) * t2_b + (-vec1.x + 3.0 * vec2.x - 3.0 * vec3.x + vec4.x) * t3_b),
                    0.5 * (2.0 * vec2.y + (-vec1.y + vec3.y) * t_b + (2.0 * vec1.y - 5.0 * vec2.y + 4.0 * vec3.y - vec4.y) * t2_b + (-vec1.y + 3.0 * vec2.y - 3.0 * vec3.y + vec4.y) * t3_b)
                );

                approximated_path.push(p1);
                approximated_path.push(p2);
            }
        }

        approximated_path
    }
}
