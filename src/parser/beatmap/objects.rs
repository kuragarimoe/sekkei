use std::{
    convert::TryFrom,
    str::FromStr
};

use crate::util::Vector2;

#[derive(Debug)]
pub struct HitObject {
    pub x: f32,
    pub y: f32,
    pub start_time: i32,
    pub end_time: i32,
    pub hit_sound: i32,
    pub hit_type: i32,
    pub slider_data: Option<SliderData>,
    pub extra_data: Option<HitObjectExtra>,
}

#[derive(Debug)]
pub struct HitObjectExtra {
    pub hit_sample: HitSample,
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
pub struct TimingPoint {}

#[derive(Debug)]
pub struct DifficultyTimingPoint {}

#[derive(Debug)]
pub struct SliderData {
    pub curve_type: CurveType,
    pub base_points: Vec<Vector2>,
    pub slider_points: Vec<Vector2>,
    pub slider_body: SliderBody,
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