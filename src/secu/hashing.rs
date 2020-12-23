use sodiumoxide::crypto::pwhash::argon2id13;

pub fn hash(passwd: &str) -> String {
	sodiumoxide::init().unwrap();
	let hash = argon2id13::pwhash(
		passwd.as_bytes(),
		argon2id13::OPSLIMIT_MODERATE,
		argon2id13::MEMLIMIT_MODERATE,
	)
	.unwrap();
	std::str::from_utf8(&hash.0)
		.unwrap()
		.trim_end_matches('\u{0}') // Trim off the extra null values
		.to_string()
}

pub fn verify(hash: &str, passwd: &str) -> bool {
	// sodium requires a buffer of exactly 128 bytes, so the value must be padded with null values
	let mut hash_buf = [0u8; 128];
	let hash_len = hash.len();
	let hash_bytes = hash.as_bytes();
	for i in 0..hash_len {
		hash_buf[i] = hash_bytes[i]
	}

	sodiumoxide::init().unwrap();
	match argon2id13::HashedPassword::from_slice(&hash_buf) {
		Some(hp) => argon2id13::pwhash_verify(&hp, passwd.as_bytes()),
		_ => false,
	}
}
