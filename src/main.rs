mod name_print;


macro_rules! print{
    ($name:ident) => {
        println!("Hello {}", $name);
    }
}


fn main() {
 name_print::print("hichem\n");
 println!("Mod worked fine!\n");
 let a = "Hichem";
 print!(a);
 println!("Macro worked fine!\n");
}
