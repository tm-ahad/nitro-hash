//! # nitro_hash

#[derive(Copy, Clone)]
/// A inital set up to hash Strings.
pub struct HasherConfig<'a> {
    pub secure: u8,
    pub hash_len: u8,
    pub salt: &'a str,
    pub rihl: usize
}

fn index(s: Vec<char>, i: usize) -> char {
    let len = s.len();

    return if i >= len {
        s[i-len]
    } else {
        s[i]
    }
}

impl HasherConfig<'_> {
    fn _hash(self, s: String) -> String {
        let mut char_v: Vec<char> = vec!['g', 'h', 'j', 'k', 'l','1', '2', '3', '-', '4', '5', '6', '7', '8', '9', '0', '!', '@', '#', '$', '%', '^', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p', 'a', 's', 'd'];
        let mut sa = format!("{}#{}", s.replace(" ", ""), self.salt);

        for c in self.salt.chars() {
            char_v.push(c)
        }

        char_v.extend(vec!['z', '&', '*', 'q', 'f', 'x', 'c', 'v', 'b', 'n', 'm', 'Q', 'W', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'P', 'A', 'S', 'D', 'F', 'G', 'H', 'J', 'K', 'L', 'Z', 'X', 'C', 'V', 'B', 'N', 'M']);

        let mut len = sa.len();

        if len >= self.hash_len as usize {
            sa = sa[..self.rihl].to_string();
            len = sa.len();
        }
        let offset = self.hash_len % len as u8;
        let r = (self.hash_len - offset)/len as u8;
        let hmm = len % 2 == 0;
        let mut state = String::new();
        let mut fin = String::new();

        let mut jesus = sa;

        for _ in 0..r {

            for a in jesus.chars() {
                let mut founded = true;
                let mut b = 0;

                for c in char_v.clone() {
                    if a == c {
                        if hmm && b > len {
                            state.push(index(char_v.clone(), b-len))
                        } else {
                            state.push(index(char_v.clone(), b+len))
                        }

                        founded = false
                    }

                    b += 1
                }

                if founded {
                    panic!("Char not found {}", a)
                }

            }

            jesus = state.clone();
            fin = format!("{fin}{}", state.clone());
            state = "".to_string()
        }

        fin = format!("{fin}{}", &jesus[..offset as usize]);

        fin
    }

    /// Mainly hashing strings
    /// Here's a example of it
    /// # Examples
    /// ```
    /// use nitro_hash::HasherConfig;
    ///
    /// let mut hasher = HasherConfig::new();
    /// hasher.secure = 3; //Default 1, But security increase cause performance decrease linearly
    /// hasher.salt = "Super-Secret-Salt"; //Setting salt
    /// hasher.hash_len = 32; //Length final hashed string Default, 16
    ///
    /// println!("{}", hasher.hash("Super-secret-password")); //Hash    ///
    pub fn hash(&self, s: &str) -> String {
        let mut _h = s.to_string();

        for _ in 0..self.secure {
            _h = self._hash(_h.to_string())
        }

        format!("${_h}")
    }

    /// Create A new Hasher Config
    /// Check the hash function
    pub fn new<'a>() -> HasherConfig<'a> {
        HasherConfig {
            secure: 2,
            salt: "",
            hash_len: 16,
            rihl: 3
        }
    }
}
