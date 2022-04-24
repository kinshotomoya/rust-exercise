fn main() {
    let test: Test = Default::default();
    let api: RedisStationLabelResponse = Default::default();
    let res = test_fn().unwrap_or_default();
    print!("{:?}", res);
}

fn test_fn() -> Result<Vec<RedisStationLabelResponse>, Box<dyn std::error::Error>> {
    let api: RedisStationLabelResponse = Default::default();
    Ok(vec![api])
}

#[derive(Debug, Default)]
struct Test {
    one: i32,
    two: i32
}

#[derive(Debug, Default)]
pub struct RedisStationLabelResponse {
    pub code: String,
    pub station_name: String,
    pub distance: i32
}

impl RedisStationLabelResponse {
    pub fn new() -> Self {
        RedisStationLabelResponse {
            code: "dddd".to_string(),
            station_name: "".to_string(),
            distance: 0
        }
    }
}
