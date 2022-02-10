use regex::Regex;
use std::{fs, path::PathBuf};

use crate::{
    util::Vector2,
    game::Gamemode,
    constants,
    parser::beatmap::objects::{
        CurveType, HitObject, HitObjectExtra, HitSample, HitType, SliderData,
    },
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

    // difficulty information
    pub difficulty_name: String,

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
        let kvp_regex = Regex::new(r"(\w+):\s*(.*)").unwrap();

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
                            }
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
                            base_points: vec![],
                            slider_points: vec![ Vector2::new(0.0, 0.0) ],
                            slider_body: SliderBody {
                                body: vec![],
                                length: vec![ 0.0 ],
                            },
                        };

                        for point in slider_split {
                            if point.contains(":") {
                                // sliderpoint
                                let point_data: Vec<String> =
                                    point.split(":").map(|s| s.to_string()).collect();

                                slider_base.base_points.push(Vector2::new(point_data[0].parse().unwrap_or(0.0) ,point_data[1].parse().unwrap_or(0.0)));
                                slider_base.slider_points.push(Vector2::new(point_data[0].parse().unwrap_or(0.0),point_data[1].parse().unwrap_or(0.0)) - Vector2::new(base.x, base.y));
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

                        for (i, slider_point) in
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
                                } else if slider_base.curve_type as i32 == CurveType::PerfectCurve as i32 {
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
                                } else if slider_base.curve_type as i32 == CurveType::Catmull as i32 {
                                    approximated_path = BeatmapFile::approximate_catmull(&sub_path)
                                } else {
                                    approximated_path = BeatmapFile::approximate_bezier(&sub_path);
                                }

                                // add to slider body
                                for point in &approximated_path {
                                    // if (this.calculatedPath.length === 0 || this.calculatedPath[this.calculatedPath.length - 1].x !== t.x || this.calculatedPath[this.calculatedPath.length - 1].y !== t.y) {
                                    if &slider_base.slider_body.body.len() == &0
                                        || &slider_base.slider_body.body[slider_base.slider_body.body.len() - 1].x
                                            != &point.x
                                        || &slider_base.slider_body.body[slider_base.slider_body.body.len() - 1].y
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
                                slider_base.slider_body.body.remove(i + 2);

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

                        // handle slider itself, now that we have the body
                        // TODO: just that, gotta handle timings beforehand, though
                        base.slider_data = Some(slider_base);
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

        for i in 0..sub_points.len() {
            subdiv_buffer1.push(Vector2::new(0.0, 0.0));
        }

        for i in 0..((sub_points.len() * 2) - 1) {
            subdiv_buffer2.push(Vector2::new(0.0, 0.0));
        }

        let mut to_flatten = vec![];
        let mut free_buffers = vec![];
        let deep_copy = sub_points.clone();

        // copy the base slider points to avoid overriding original ones
        to_flatten.push(deep_copy);

        dbg!(sub_points.len());
        while to_flatten.len() > 0 {
            let mut parent = to_flatten.pop().unwrap();
            let mut flat_enough = true;

            // are the control points we're using flat enough?
            for i in 1..parent.len() - 1 {
                let scale = parent[i].scale(2.0);
                let subtract = parent[i - 1] - scale;
                let sum = subtract + parent[i + 1];

                if sum.len().powf(2.0) > constants::BEZIER_TOLERANCE * constants::BEZIER_TOLERANCE * 4.0 {
                    flat_enough = false;
                    break;
                }
            }

            if flat_enough == true {
                // subdivide
                let mut approxmid_points = &mut subdiv_buffer1;
                for i in 0..sub_points.len() {
                    approxmid_points[i] = parent[i];
                }

                for i in 0..sub_points.len() {
                    subdiv_buffer2[i] = approxmid_points[0];
                    approxmid_points[parent.len() - i - 1] =
                        approxmid_points[parent.len() - i - 1];

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
                    let vector = (subdiv_buffer2[index - 1] + (Vector2::new(2.0, 2.0).mul(subdiv_buffer2[index]) + subdiv_buffer2[index + 1])).scale(0.25);
                    approximated_path.push(vector);
                }

                // push
                free_buffers.push(parent);
                continue;
            }

            // no, it is not.
            // further flatten the curve to get a close enough approximation
            // we might not yet have a flat approximation, so we'd need to subdivide a bare array
            let mut right_child = vec![];
            if free_buffers.len() > 0 {
                right_child = free_buffers.pop().unwrap();
            } else {
                for i in 0..sub_points.len() {
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
        dbg!(sub_points.len());

        approximated_path.push(sub_points[sub_points.len() - 1]);
        approximated_path
    }

    pub fn approximate_perfect_curve(sub_points: &Vec<Vector2>) -> Vec<Vector2> {
        let mut approximated_path = vec![];

        // approximate perfect curve
        approximated_path = vec![];

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
            f32::max(2.0, (theta_range / (2.0 * f32::acos(1.0 - 0.1 / rad))).ceil()) as usize
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

        approximated_path
    }
}
