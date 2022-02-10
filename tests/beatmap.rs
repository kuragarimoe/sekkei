mod tests {
    use std::{env};
    use sekkei::parser::{
        beatmap::{
            BeatmapFile
        }
    };

    #[test]
    fn test_parser() {
        let path = env::current_dir().unwrap().to_str().unwrap().to_string() + "/tests/files/IMAGINARY LIKE THE JUSTICE.osu";
        let bm = BeatmapFile::from_file(&path);
        
        // format ver
        assert_eq!(bm.format_version, 14);

        println!("{}", bm.metadata.preview_time);
        println!("{}", bm.title);
        println!("{}", bm.artist);
        println!("{}", bm.difficulty.circle_size);
    }
}