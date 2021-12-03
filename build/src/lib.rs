use std::{env, fs, path::PathBuf};

pub fn build() {
    let mut main = String::from("#[warn(clippy::pedantic)]\n\n");

    let days = fs::read_dir("src").unwrap().count();

    let mut mods = Vec::new();
    let mut structs = Vec::new();

    let year = env::current_dir()
        .unwrap()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    for n in 1..days {
        mods.push(format!(
            // lol
            // lmao
            "#[path = \"../../../../../{}/src/day{:02}.rs\"] mod day{:02};\n",
            year, n, n
        ));
        structs.push(format!("day{:02}::Day{:02},\n", n, n));
    }

    for m in mods {
        main.push_str(&m);
    }

    main.push_str("\naoc_shared::main!(\n");

    for s in structs {
        main.push_str(&s);
    }

    main.push_str(");\n");

    let out = PathBuf::from(env::var("OUT_DIR").unwrap()).join("main.rs");

    fs::write(out, main).unwrap();
}
