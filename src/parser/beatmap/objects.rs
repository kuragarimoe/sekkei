use std::{
    convert::TryFrom,
    str::FromStr
};

use crate::util::Vector2;

#[derive(Debug)]
pub struct HitObject {
    pub x: f32,
    pub y: f32,
    pub position: Vector2,
    pub end_position: Vector2,
    pub start_time: f32,
    pub end_time: f32,
    pub hit_sound: i32,
    pub hit_type: i32,
    pub stack_height: i32,
    pub slider_data: Option<SliderData>,
    pub slider_objects: Option<Vec<SliderObject>>,
    pub extra_data: Option<HitObjectExtra>
}

impl Clone for HitObject {
    fn clone(&self) -> Self {
        HitObject {
            x: self.x,
            y: self.y,
            position: self.position,
            end_position: self.end_position,
            start_time: self.start_time,
            end_time: self.end_time,
            hit_sound: self.hit_sound,
            hit_type: self.hit_type,
            stack_height: self.stack_height,
            slider_data: self.slider_data.clone(),
            slider_objects: self.slider_objects.clone(),
            extra_data: self.extra_data.clone(),
        }
    }
}

#[derive(Debug)]
pub struct HitObjectExtra {
    pub hit_sample: HitSample,
}

impl Clone for HitObjectExtra {
    fn clone(&self) -> Self {
        HitObjectExtra {
            hit_sample: self.hit_sample.clone(),
        }
    }
}

#[derive(Debug)]
pub struct HitSample {
    pub normal_set: i32,
    pub additional_set: i32,
    pub index: i32,
    pub volume: i32,
    pub file_name: String,
}

impl Default for HitSample {
    fn default() -> Self {
        HitSample {
            normal_set: 0,
            additional_set: 0,
            index: 0,
            volume: 0,
            file_name: "".to_string(),
        }
    }
}

impl Clone for HitSample {
    fn clone(&self) -> Self {
        HitSample {
            normal_set: self.normal_set,
            additional_set: self.additional_set,
            index: self.index,
            volume: self.volume,
            file_name: self.file_name.clone(),
        }
    }
}

#[derive(Debug)]
pub enum HitSound {
    Normal,
    Whistle,
    Finish,
    Clap,
}

impl TryFrom<i32> for HitSound {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == HitSound::Normal as i32 => Ok(HitSound::Normal),
            x if x == HitSound::Whistle as i32 => Ok(HitSound::Whistle),
            x if x == HitSound::Finish as i32 => Ok(HitSound::Finish),
            x if x == HitSound::Clap as i32 => Ok(HitSound::Clap),
            _ => Err(()),
        }
    }
}

impl FromStr for HitSound {
    type Err = ();

    fn from_str(input: &str) -> Result<HitSound, Self::Err> {
        match input {
            "Normal" => Ok(HitSound::Normal),
            "Whistle" => Ok(HitSound::Whistle),
            "Finish" => Ok(HitSound::Finish),
            "Clap" => Ok(HitSound::Clap),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub enum HitType {
    Normal = 1 << 0,
    Slider = 1 << 1,
    NewCombo = 1 << 2,
    Spinner = 1 << 3,
    ComboSkip1 = 1 << 4,
    ComboSkip2 = 1 << 5,
    ComboSkip3 = 1 << 6,
    Hold = 1 << 7,
}

impl HitType {
    fn from_i32(v: i32) -> Result<Self, ()> {
        match v {
            x if x == HitType::Normal as i32 => Ok(HitType::Normal),
            x if x == HitType::Slider as i32 => Ok(HitType::Slider),
            x if x == HitType::NewCombo as i32 => Ok(HitType::NewCombo),
            x if x == HitType::Spinner as i32 => Ok(HitType::Spinner),
            x if x == HitType::ComboSkip1 as i32 => Ok(HitType::ComboSkip1),
            x if x == HitType::ComboSkip2 as i32 => Ok(HitType::ComboSkip2),
            x if x == HitType::ComboSkip3 as i32 => Ok(HitType::ComboSkip3),
            x if x == HitType::Hold as i32 => Ok(HitType::Hold),
            _ => Err(()),
        }
    }
}

impl FromStr for HitType {
    type Err = ();

    fn from_str(input: &str) -> Result<HitType, Self::Err> {
        match input {
            "Normal" => Ok(HitType::Normal),
            "Slider" => Ok(HitType::Slider),
            "NewCombo" => Ok(HitType::NewCombo),
            "Spinner" => Ok(HitType::Spinner),
            "ComboSkip1" => Ok(HitType::ComboSkip1),
            "ComboSkip2" => Ok(HitType::ComboSkip2),
            "ComboSkip3" => Ok(HitType::ComboSkip3),
            "Hold" => Ok(HitType::Hold),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub enum CurveType {
    Catmull = 1,
    Bezier = 2,
    Linear = 3,
    PerfectCurve = 4,
}

impl FromStr for CurveType {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "C" => Ok(CurveType::Catmull),
            "B" => Ok(CurveType::Bezier),
            "L" => Ok(CurveType::Linear),
            "P" => Ok(CurveType::PerfectCurve),
            _ => Err(()),
        }
    }
}

impl Copy for CurveType {}

impl Clone for CurveType {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Debug)]
pub struct TimingPoint {
    pub time: f32,
    pub beat_length: f32,
    pub time_signature: i32,
    pub speed_multiplier: f32,
    pub point_type: TimingPointType
}

impl Copy for TimingPoint {}

impl Clone for TimingPoint {
    fn clone(&self) -> Self {
        TimingPoint {
            time: self.time,
            beat_length: self.beat_length,
            time_signature: self.time_signature,
            speed_multiplier: self.speed_multiplier,
            point_type: self.point_type
        }
    }
}

#[derive(Debug)]
pub enum TimingPointType {
    Uninherited,
    Inherited
}

impl Copy for TimingPointType {}

impl Clone for TimingPointType {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Debug)]
pub struct UninheritedTimingPoint {
    pub time: f32,
    pub beat_length: f32,
    pub time_signature: i32
}

impl Copy for UninheritedTimingPoint {}

impl Clone for UninheritedTimingPoint {
    fn clone(&self) -> Self {
        UninheritedTimingPoint {
            time: self.time,
            beat_length: self.beat_length,
            time_signature: self.time_signature
        }
    }
}

#[derive(Debug)]
pub struct InheritedTimingPoint {
    pub time: f32,
    pub speed_multiplier: f32,
    pub inherited_from: UninheritedTimingPoint
}

impl Copy for InheritedTimingPoint {}

impl Clone for InheritedTimingPoint {
    fn clone(&self) -> Self {
        InheritedTimingPoint {
            time: self.time,
            speed_multiplier: self.speed_multiplier,
            inherited_from: self.inherited_from.clone(),
        }
    }
}

#[derive(Debug)]
pub struct SliderData {
    pub curve_type: CurveType,
    pub base_points: Vec<Vector2>,
    pub slider_points: Vec<Vector2>,
    pub slider_body: SliderBody,
}

impl Clone for SliderData {
    fn clone(&self) -> Self {
        SliderData {
            curve_type: self.curve_type,
            base_points: self.base_points.clone(),
            slider_points: self.slider_points.clone(),
            slider_body: self.slider_body.clone(),
        }
    }
}

#[derive(Debug)]
pub struct SliderBody {
    pub body: Vec<Vector2>,
    pub length: Vec<f32>,
}

impl Clone for SliderBody {
    fn clone(&self) -> Self {
        SliderBody {
            body: self.body.clone(),
            length: self.length.clone(),
        }
    }
}

#[derive(Debug)]
pub struct SliderObject {
    pub x: f32,
    pub y: f32,
    pub position: Vector2,
    pub start_time: f32,
    pub span_index: i32,
    pub repeat_index: i32,
    pub span_start_time: f32,
    pub slider_object_type: SliderObjectType
}

impl Copy for SliderObject {}

impl Clone for SliderObject {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Debug)]
pub enum SliderObjectType {
    SliderHead,
    SliderTick,
    SliderRepeat,
    SliderEnd,
}

impl Copy for SliderObjectType {}

impl Clone for SliderObjectType {
    fn clone(&self) -> Self {
        *self
    }
}