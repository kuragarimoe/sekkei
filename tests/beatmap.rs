mod tests {
    use sekkei::parser::beatmap::BeatmapFile;
    use std::env;

    #[test]
    fn test_parser() {
        let path = env::current_dir().unwrap().to_str().unwrap().to_string()
            + "/tests/files/kakushigoto.osu";
        let bm = BeatmapFile::from_file(&path);

        // format ver
        assert_eq!(bm.format_version, 14);

        println!("{}", bm.metadata.preview_time);
        println!("{}", bm.title);
        println!("{}", bm.artist);
        println!("{}", bm.difficulty.circle_size);

        // print bm to file
        std::fs::write("./test.osu_dec", format!("{:#?}", bm));
    }
}
