#![allow(unused_variables)] // ignore unused variables

type File = String;

fn open(f: &mut File) -> bool {
    true // always success
}

fn close(f: &mut File) -> bool {
    true // always success
}

#[allow(dead_code)] // ignore unused function
fn read(f: &mut File, save_to: &mut Vec<u8>) -> ! {
    unimplemented!() // crash program
}

fn main() {
    let mut f1 = File::from("f1.txt");
    open(&mut f1);
    // read(f1, vec1[]);
    close(&mut f1);
}
