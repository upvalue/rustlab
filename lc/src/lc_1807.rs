use std::collections::HashMap;

pub fn lc_1807(s: String, knowledge: Vec<Vec<String>>) -> String {
    let mut kndict = HashMap::new();
    for p in knowledge {
        kndict.insert(p[0].clone(), p[1].clone());
    }

    let mut parts: Vec<String> = vec![];

    let sb = s.as_bytes();

    let qmark = "?".to_string();

    let mut i = 0;
    while i < s.len() {
        println!("{} {:?}", i, parts);
        let c = sb[i];
        let mut j = i + 1;
        if c == b'(' {
            while j < s.len() && sb[j] != b')' {
                j += 1;
            }
            let key = s.get(i + 1..j).unwrap();

            let val = kndict.get(key).unwrap_or(&qmark);
            parts.push(val.to_string());
        } else if c == b')' {
            i += 1;
            continue;
        } else {
            while j < s.len() && sb[j] != b'(' {
                j += 1;
            }
            parts.push(s.get(i..j).unwrap().to_string());
        }

        i = j;
    }

    println!("{:?}", parts);

    return parts.join("");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lc_1087() {
        // case 1
        assert_eq!(lc_1807("asdf".to_string(), vec![]), "asdf");
        assert_eq!(lc_1807("(asdf)".to_string(), vec![]), "?");
        assert_eq!(
            lc_1807(
                "k1(asdf)(gub)".to_string(),
                vec![vec!["asdf".to_string(), "cool".to_string()]]
            ),
            "k1cool?"
        );
    }
}
