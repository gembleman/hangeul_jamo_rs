#[cfg(test)]
mod tests {
    use hangeul_jamo::core::*;
    use hangeul_jamo::hcj::*;
    use hangeul_jamo::jamo::*;

    #[test]
    fn test_is_hangul_syllable() {
        assert!(is_hangul_syllable('한'));
        assert!(is_hangul_syllable('글'));
        assert!(!is_hangul_syllable('a'));
        assert!(!is_hangul_syllable('ㄱ'));
    }

    #[test]
    fn test_decompose_hcj() {
        assert_eq!(decompose_hcj("한글"), "ㅎㅏㄴㄱㅡㄹ");
        assert_eq!(decompose_hcj("안녕하세요"), "ㅇㅏㄴㄴㅕㅇㅎㅏㅅㅔㅇㅛ");
        assert_eq!(decompose_hcj("Hello 한글!"), "Hello ㅎㅏㄴㄱㅡㄹ!");
    }

    #[test]
    fn test_compose_hcj() {
        assert_eq!(compose_hcj("ㅎㅏㄴㄱㅡㄹ"), "한글");
        assert_eq!(compose_hcj("ㅇㅏㄴㄴㅕㅇㅎㅏㅅㅔㅇㅛ"), "안녕하세요");
        assert_eq!(compose_hcj("Hello ㅎㅏㄴㄱㅡㄹ!"), "Hello 한글!");
    }

    #[test]
    fn test_decompose_compound() {
        assert_eq!(decompose_compound('ㄲ'), Some(vec!['ㄱ', 'ㄱ']));
        assert_eq!(decompose_compound('ㅘ'), Some(vec!['ㅗ', 'ㅏ']));
        assert_eq!(decompose_compound('ㄱ'), None);
    }

    #[test]
    fn test_compose_compound() {
        assert_eq!(compose_compound(&['ㄱ', 'ㄱ']), Some('ㄲ'));
        assert_eq!(compose_compound(&['ㅗ', 'ㅏ']), Some('ㅘ'));
    }
}
