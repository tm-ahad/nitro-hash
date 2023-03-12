
#[derive(Copy, Clone)]
pub struct HasherConfig<'a> {
    secure: u8,
    hash_len: u8,
    salt: &'a str,
    rihl: usize
}

fn index(s: [char; 71], i: usize) -> char {
    let len = s.len();

    return if i >= len {
        s[i-len]
    } else {
        s[i]
    }
}

impl HasherConfig<'_> {
    fn _hash(self, s: String) -> String {
        let char_v = ['1','2','3','-','4','5','6','7','8','9','0','!','@','#','$','%','^','&','*','q','f','w','e','r','t','y','u','i','o','p','a','s','d','g','h','j','k','l','z','x','c','v','b','n','m','Q','W','E','R','T','Y','U','I','O','P','A','S','D','F','G','H','J','K','L','Z','X','C','V','B','N','M'];
        let mut sa = format!("{}#{}", s.replace(" ", ""), self.salt);
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

                for c in char_v {
                    if a == c {
                        if hmm && b > len {
                            state.push(index(char_v, b-len))
                        } else {
                            state.push(index(char_v, b+len))
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

    pub fn hash(&self, s: &str) -> String {
        let mut _h = s.to_string();

        for _ in 0..self.secure {
            _h = self._hash(_h.to_string())
        }

        format!("${_h}")
    }

    pub fn new<'a>() -> HasherConfig<'a> {
        HasherConfig {
            secure: 2,
            salt: "",
            hash_len: 16,
            rihl: 3
        }
    }
}

fn main() {
    let mut h = HasherConfig::new();

    h.secure = 15;

    println!("{}", h.hash("Tjahmid"));
}