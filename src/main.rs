use std::fmt::format;

pub struct HasherConfig {
    repeater: i8,
    hash_len: i8,
    salt: String
}

impl HasherConfig {
    pub fn hash(self, s: String) -> String {
        let char_v = ['1','2','3','4','5','6','7','8','9','0','!','@','#','$','%','^','&','*','(',')','q','w','e','r','t','y','u','i','o','p','a','s','d','g','h','j','k','l','z','x','c','v','b','n','m','Q','W','E','R','T','Y','U','I','O','P','A','S','D','F','G','H','J','K','L','Z','X','C','V','B','N','M'];
        let sa = format!("{}#{}", s.replace(" ", ""), self.salt);
        let len = sa.len();
        let offset = self.hash_len % len as i8;
        let r = (self.hash_len - offset)/len as i8;
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
                            state.push(char_v[b-len])
                        } else {
                            state.push(char_v[b+len])
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

        fin = format!("${fin}{}", &jesus[..offset as usize]);

        fin
    }
}

fn main() {
    let h = HasherConfig {
        repeater: 2,
        salt: String::new(),
        hash_len: 13
    };

    println!("{}", h.hash("a".to_string()))
}