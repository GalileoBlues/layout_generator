use itertools::Itertools;

pub static COL_TO_FINGER: [u8; 10] = [0, 1, 2, 3, 3, 4, 4, 5, 6, 7];

#[derive(Copy, Clone)]
pub struct PosPair(pub usize, pub usize);

impl PosPair {
	pub const fn default() -> Self {
		Self(0, 0)
	}

	pub const fn new(x1: usize, x2: usize) -> Self {
		Self(x1, x2)
	}
}

pub static POSSIBLE_SWAPS: [PosPair; 435] = get_possible_swaps();

const fn get_possible_swaps() -> [PosPair; 435] {
	let mut res = [PosPair::default(); 435];
	let mut i = 0;
	let mut pos1 = 0;
	
	while pos1 < 30 {
		let mut pos2 = pos1 + 1;
		while pos2 < 30 {
			res[i] = PosPair(pos1, pos2);
			i += 1;
			pos2 += 1;
		}
		pos1 += 1;
	}
	res
}

pub static EFFORT_MAP: [f64; 30] = [
	3.5, 2.5, 2.1, 1.7, 2.5, 3.4, 2.2, 2.0, 2.4, 3.0,
	1.7, 1.3, 1.1, 1.0, 2.6, 2.6, 1.0, 1.1, 1.3, 1.7,
	3.1, 2.7, 2.4, 1.8, 3.7, 2.1, 1.8, 2.4, 2.7, 3.3
];

pub fn get_sfb_indices() -> [(usize, usize); 48] {
	let mut res: Vec<(usize, usize)> = Vec::new();
	for i in [0, 1, 2, 7, 8, 9] {
		let chars = [i, i+10, i+20];
		for c in chars.into_iter().combinations(2) {
			res.push((c[0], c[1]));
		}
	}
	for i in [0, 2] {
		let chars = [3+i, 13+i, 23+i, 4+i, 14+i, 24+i];
		for c in chars.into_iter().combinations(2) {
			res.push((c[0], c[1]));
		}
	}
	res.try_into().unwrap()
}

pub fn get_scissor_indices() -> [(usize, usize); 14] {
	let mut res: Vec<(usize, usize)> = Vec::new();
	res.push((0, 11));
	res.push((9, 18));
	for i in [0, 1, 2, 6, 7, 8] {
		res.push((i, i+21));
		res.push((i+1, i+20));
	}
	res.try_into().unwrap()
}

pub fn available_chars(language: &str) -> [char; 30] {
	let chars = match language {
		"albanian" =>             "abcdefghijklmnopqrstuvxyzëç.,'",
		"bokmal" | "nynorsk" =>   "abcdefghijklmnopærstuvwøyå',.;",
		"czech" =>                "abcdefghijklmnop*rstuvěxyzá,.í",
		"english_th" =>           "abcdefghijklmnopqrstuvwxyz',.þ",
		"finnish" =>              "abcdefghijklmnopärstuvwxyzö',.",
		"finnish_repeat" =>       "abcdefghijklmnopärstuvw@yzö',.",
		"french" | "french_qu" => "abcdefghijélmnopqrstuvàxy-',.*",
		"german" =>               "abcdefghijklmnoprstuvwxyzüäö.,",
		"spanish" =>              "abcdefghijklmnopqrstuvwxyz',.*",
		_ =>                      "abcdefghijklmnopqrstuvwxyz',.;"
	};
	chars.chars().collect::<Vec<char>>().try_into().unwrap()
}