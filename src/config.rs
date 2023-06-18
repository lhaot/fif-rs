#[derive(Debug)]
pub struct Config {
    pub target: String,
    pub filenames: Vec<String>,
    pub flags: String,
    pub recursion: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            target: String::new(),
            filenames: Vec::new(),
            flags: String::new(),
            recursion: false,
        }
    }
}

impl Config {
    pub fn parse(args: Vec<String>) -> Self {
        if 3 > args.len() {
            panic!("not enough arguments")
        };

        let mut config = Config {
            target: args[1].clone(),
            filenames: Vec::new(),
            flags: String::new(),
            recursion: false,
        };

        for arg in &args[2..] {
            if arg.starts_with('-') {
                config.flags = arg.replace('-', "");
            } else {
                config.filenames.push(arg.clone());
            }
        };

        for flag in config.flags.chars() {
            match flag {
                'r' => config.recursion = true,
                _ => panic!("invalid flag argument: {}", flag),
            };
        };

        return config;
    }
}
