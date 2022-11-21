pub fn process() -> String {
    let target: u32 = 1000;
    let total: u32 = get_multotal(target);
    total.to_string()
}

fn get_multotal(target: u32) -> u32 {
    let mut multotal: u32 = 0;
    for i in 1..target {
        if i % 3 == 0 || i % 5 == 0 {
            multotal += i;
        }
    }
    multotal
}