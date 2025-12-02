use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generate_day.rs");
    println!("AAAA {}", dest_path.display());
    let generated_match_branches = fs::read_dir(Path::new("./src/days/")).unwrap().map(|path| {
        let mut s = path.unwrap().file_name().into_string().expect("should be a string");
        (0..3).for_each(|_| { s.pop(); });
        format!("{} => Ok(Day {{ part_one: Box::new(days::{}::PartOne {{}}), part_two: Box::new(days::{}::PartTwo {{}}) }}),", &s[3..], s, s)
    }).collect::<Vec<_>>().join("\n           ");
    println!("{}", generated_match_branches);
    fs::write(
        &dest_path,
        format!("mod days;
        
        pub fn get_day(n: usize) -> Result<Day, ()> {{
            match n {{
                {}
                _ => Err(()),
            }}
        }}
        ", generated_match_branches)
    ).unwrap();
    
    let dest_dir_path = Path::new(&out_dir).join("days");
    fs::create_dir(dest_dir_path);

    fs::read_dir(Path::new("./src/days/")).unwrap().for_each(|path| {
        let location = path.unwrap();
        let name = location.file_name().into_string().expect("should be a string");
        fs::copy(location.path(), Path::new(&out_dir).join("days").join(name));
    });
    
    let dest_path = Path::new(&out_dir).join("days").join("mod.rs");
    let generated_uses = fs::read_dir(Path::new("./src/days/")).unwrap().map(|path| {
        let mut s = path.unwrap().file_name().into_string().expect("should be a string");
        (0..3).for_each(|_| { s.pop(); });
        format!("pub mod {};", s)
    }).collect::<Vec<_>>().join("\n");
    println!("{}", generated_uses);
    fs::write(
        &dest_path,
        format!("{}
        ", generated_uses)
    ).unwrap();
}