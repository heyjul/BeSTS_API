use once_cell::sync::Lazy;

static HASHER: Lazy<hashids::HashIds> = Lazy::new(|| {
    hashids::HashIds::new_with_salt("Sel de Guérande".to_owned()).expect("Could not create hashids")
});

pub fn encode_id(id: i64) -> String {
    HASHER.encode(&vec![id])
}

pub fn decode_id(id: String) -> Result<i64, Box<dyn std::error::Error>> {
    Ok(*HASHER.decode(id).first().ok_or("Invalid argument")?)
}
