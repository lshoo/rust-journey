use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct City {
    pub name: String,
    pub population: usize,
    pub latitude: f64,
    pub longitude: f64,
}

#[test]
fn test_city_serde_should_work() {
    let beijing = City {
        name: "Beijing".to_string(),
        population: 20_000_000,
        latitude: 98.9,
        longitude: 123.2,
    };

    let as_json = serde_json::to_string(&beijing).unwrap();
    let as_bin = bincode::serialize(&beijing).unwrap();
    let as_cbor = serde_cbor::to_vec(&beijing).unwrap();
    println!("as json: {}", &as_json);
    println!("as binary: {:?}", &as_bin);
    println!("as cbor: {:?}", &as_cbor);

    println!(
        "json (as UTF-8): {}",
        String::from_utf8_lossy(as_json.as_bytes())
    );
    println!("binary (as UTF-8): {}", String::from_utf8_lossy(&as_bin));
    println!("cbor (as UTF-8): {}", String::from_utf8_lossy(&as_cbor));

    println!();

    let b2: City = serde_json::from_str(&as_json).unwrap();
    let b3: City = bincode::deserialize(&as_bin).unwrap();
    let b4: City = serde_cbor::from_reader(&as_cbor[..]).unwrap();

    assert_eq!(beijing, b2);
    assert_eq!(beijing, b3);
    assert_eq!(beijing, b4);
}
