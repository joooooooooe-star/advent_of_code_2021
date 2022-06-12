mod binarydiagnostic;
mod dive;
mod giantsquid;
mod hydrothermalventure;
mod sonarsweep;

fn main() {
    hydrothermalventure();
}

#[allow(dead_code)]
fn sonarsweep() {
    let num = sonarsweep::depth_increase("inputs/sonarsweep_input.txt");
    println!("There are {} depth increases", num);
}

#[allow(dead_code)]
fn dive() {
    let num = dive::dive("inputs/dive_input.txt");
    println!("Product of position and depth is {}", num);
}

#[allow(dead_code)]
fn binarydiagnostic() {
    binarydiagnostic::binarydiagnostic("inputs/binarydiagnostic_input.txt", 12);
}

#[allow(dead_code)]
fn giantsquid() {
    giantsquid::play_bingo("inputs/giantsquid_input.txt");
}

#[allow(dead_code)]
fn hydrothermalventure() {
    hydrothermalventure::find_overlap("inputs/hydrothermalventure_input.txt")
}
