
pub fn is_contain(a:&str, b:&str) -> bool {
    a.contains(b)
}


pub fn join_vec_string(v:Vec<&str>, s:&str) -> String {
    v.join(s)
}


pub fn lines_to_string_vec(s:&str) -> Vec<String> {
    let lines = s.lines();
    let l = lines
        .collect::<Vec<_>>();

    let mut string_vec: Vec<String> = Vec::new();
    for s in &l {
        string_vec.push(s.to_string());
    }
    return string_vec;
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_join_vec_string() {
        assert_eq!(join_vec_string(["a","b","c"].to_vec(),""),"abc");
        assert_eq!(join_vec_string(["a","b","c"].to_vec(),"-"),"a-b-c");
        assert_eq!(join_vec_string([].to_vec(),""),"");
        assert_eq!(join_vec_string([].to_vec(),"-"),"");
    }

    #[test]
    fn test_is_contain() {
        assert_eq!(is_contain("",""),true);
        assert_eq!(is_contain("","ac"),false);
        assert_eq!(is_contain("","b"),false);
        assert_eq!(is_contain("abc",""),true);
        assert_eq!(is_contain("abc","ac"),false);
        assert_eq!(is_contain("abc","b"),true);
    }

}
