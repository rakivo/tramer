use tramer::tramer;

#[tramer("millis")]
fn profile_millis() {
    std::thread::sleep(std::time::Duration::from_millis(150));
}

#[tramer("nanos")]
fn profile_nanos() {
    std::thread::sleep(std::time::Duration::from_micros(300));
}

#[tramer("secs")]
fn profile_seconds() {
    std::thread::sleep(std::time::Duration::from_millis(250));
}

fn main() {
    profile_nanos();
    profile_millis();
    profile_seconds();
}
