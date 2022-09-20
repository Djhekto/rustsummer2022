use std::fs;

fn main() {
    let paths = fs::read_dir("./").unwrap();
    let mut dirsize:u64 = 0;
    for path in paths{let file_name = path.unwrap().file_name().to_string_lossy().into_owned(); println!("{}",file_name);}
    let paths = fs::read_dir("./").unwrap();
    for path in paths{   let filez = path.unwrap().path().metadata().unwrap().len();   println!("{}",filez);
    dirsize+= filez;}
println!("{}",dirsize);
} 

/*{
   let paths = fs::read_dir("./").unwrap();
   let mut dirsize:u64 = 0;
    for path in paths{
      let path1 = path.copied();
   let file_name = path.unwrap().file_name().to_string_lossy().into_owned(); println!("{}",file_name);
//   println!("{}", path.unwrap().path().display()) ; 
   let filez = path.unwrap().path().metadata().unwrap().len(); //  println!("{}",filez);
   dirsize+= filez;
} */