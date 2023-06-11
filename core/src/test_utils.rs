#![allow(clippy::missing_panics_doc)]

use crate::Search;

//
// utilities
//

pub fn single<'t, 'p, S: Search<'t, 'p>>(t: &'t str, p: &'p str, expected: Option<usize>) {
    println!("searching for pattern [{p}] in text [{t}]");
    let mut s = S::new(t.as_bytes(), p.as_bytes());
    assert_eq!(s.search(), expected);
}

pub fn multiple<'t, 'p, S: Search<'t, 'p>>(t: &'t str, p: &'p str, expected: &[usize]) {
    let len = expected.len();
    print!("searching for pattern [{p}] in text [{t}] ({len} times) (");
    let mut s = S::new(t.as_bytes(), p.as_bytes());
    for i in expected {
        assert_eq!(s.search(), Some(*i));
        print!(".");
    }
    assert_eq!(s.search(), None);
    println!(")");
}

//
// public api
//

pub fn all_tests<'t, 'p, S: Search<'t, 'p>>() {
    println!("@@@ basic_tests @@@");
    basic_tests::<S>();
    println!();

    println!("@@@ multiple_occurrences_tests @@@");
    multiple_occurrences_tests::<S>();
    println!();

    println!("@@@ single_occurrence_tests @@@");
    single_occurrence_tests::<S>();
}

pub fn basic_tests<'t, 'p, S: Search<'t, 'p>>() {
    let cases = &[
        ("abcABCabc", "A", Some(3)),
        ("abcABCabc", "Z", None),
        ("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaab", "aaaab", Some(26)),
    ];
    for (text, pat, expected) in cases {
        single::<S>(text, pat, *expected);
    }
}

pub fn multiple_occurrences_tests<'t, 'p, S: Search<'t, 'p>>() {
    let cases = &[
        ("BABAB", "A", [1, 3].as_slice()),
        ("XXabcYYabcZZabc", "abc", &[2, 7, 12]),
    ];
    for (text, pat, expected) in cases {
        multiple::<S>(text, pat, expected);
    }
}

#[allow(clippy::too_many_lines)]
pub fn single_occurrence_tests<'t, 'p, S: Search<'t, 'p>>() {
    // Taken from:
    // https://cs.opensource.google/go/go/+/refs/heads/master:src/strings/strings_test.go;l=45;drc=0b3f58c48e3298e49e27f80dc748f0652339d63e
    //
    // (text, pat, expected)
    let cases = &[
        ("", "", Some(0)),
        ("", "a", None),
        ("", "foo", None),
        ("fo", "foo", None),
        ("foo", "foo", Some(0)),
        ("oofofoofooo", "f", Some(2)),
        ("oofofoofooo", "foo", Some(4)),
        ("barfoobarfoo", "foo", Some(3)),
        ("foo", "", Some(0)),
        ("foo", "o", Some(1)),
        ("abcABCabc", "A", Some(3)),
        (
            "jrzm6jjhorimglljrea4w3rlgosts0w2gia17hno2td4qd1jz",
            "jz",
            Some(47),
        ),
        (
            "ekkuk5oft4eq0ocpacknhwouic1uua46unx12l37nioq9wbpnocqks6",
            "ks6",
            Some(52),
        ),
        ("999f2xmimunbuyew5vrkla9cpwhmxan8o98ec", "98ec", Some(33)),
        ("9lpt9r98i04k8bz6c6dsrthb96bhi", "96bhi", Some(24)),
        (
            "55u558eqfaod2r2gu42xxsu631xf0zobs5840vl",
            "5840vl",
            Some(33),
        ),
        // cases with one byte strings - test special case in Index()
        ("", "a", None),
        ("x", "a", None),
        ("x", "x", Some(0)),
        ("abc", "a", Some(0)),
        ("abc", "b", Some(1)),
        ("abc", "c", Some(2)),
        ("abc", "x", None),
        // test special cases in Index() for short strings
        ("", "ab", None),
        ("bc", "ab", None),
        ("ab", "ab", Some(0)),
        ("xab", "ab", Some(1)),
        (&"xab"[..2], "ab", None),
        ("", "abc", None),
        ("xbc", "abc", None),
        ("abc", "abc", Some(0)),
        ("xabc", "abc", Some(1)),
        (&"xabc"[..3], "abc", None),
        ("xabxc", "abc", None),
        ("", "abcd", None),
        ("xbcd", "abcd", None),
        ("abcd", "abcd", Some(0)),
        ("xabcd", "abcd", Some(1)),
        (&"xyabcd"[..5], "abcd", None),
        ("xbcqq", "abcqq", None),
        ("abcqq", "abcqq", Some(0)),
        ("xabcqq", "abcqq", Some(1)),
        (&"xyabcqq"[..6], "abcqq", None),
        ("xabxcqq", "abcqq", None),
        ("xabcqxq", "abcqq", None),
        ("", "01234567", None),
        ("32145678", "01234567", None),
        ("01234567", "01234567", Some(0)),
        ("x01234567", "01234567", Some(1)),
        ("x0123456x01234567", "01234567", Some(9)),
        (&"xx01234567"[..9], "01234567", None),
        ("", "0123456789", None),
        ("3214567844", "0123456789", None),
        ("0123456789", "0123456789", Some(0)),
        ("x0123456789", "0123456789", Some(1)),
        ("x012345678x0123456789", "0123456789", Some(11)),
        (&"xyz0123456789"[..12], "0123456789", None),
        ("x01234567x89", "0123456789", None),
        ("", "0123456789012345", None),
        ("3214567889012345", "0123456789012345", None),
        ("0123456789012345", "0123456789012345", Some(0)),
        ("x0123456789012345", "0123456789012345", Some(1)),
        (
            "x012345678901234x0123456789012345",
            "0123456789012345",
            Some(17),
        ),
        ("", "01234567890123456789", None),
        ("32145678890123456789", "01234567890123456789", None),
        ("01234567890123456789", "01234567890123456789", Some(0)),
        ("x01234567890123456789", "01234567890123456789", Some(1)),
        (
            "x0123456789012345678x01234567890123456789",
            "01234567890123456789",
            Some(21),
        ),
        (
            &"xyz01234567890123456789"[..22],
            "01234567890123456789",
            None,
        ),
        ("", "0123456789012345678901234567890", None),
        (
            "321456788901234567890123456789012345678911",
            "0123456789012345678901234567890",
            None,
        ),
        (
            "0123456789012345678901234567890",
            "0123456789012345678901234567890",
            Some(0),
        ),
        (
            "x0123456789012345678901234567890",
            "0123456789012345678901234567890",
            Some(1),
        ),
        (
            "x012345678901234567890123456789x0123456789012345678901234567890",
            "0123456789012345678901234567890",
            Some(32),
        ),
        (
            &"xyz0123456789012345678901234567890"[..33],
            "0123456789012345678901234567890",
            None,
        ),
        ("", "01234567890123456789012345678901", None),
        (
            "32145678890123456789012345678901234567890211",
            "01234567890123456789012345678901",
            None,
        ),
        (
            "01234567890123456789012345678901",
            "01234567890123456789012345678901",
            Some(0),
        ),
        (
            "x01234567890123456789012345678901",
            "01234567890123456789012345678901",
            Some(1),
        ),
        (
            "x0123456789012345678901234567890x01234567890123456789012345678901",
            "01234567890123456789012345678901",
            Some(33),
        ),
        (
            &"xyz01234567890123456789012345678901"[..34],
            "01234567890123456789012345678901",
            None,
        ),
        (
            "xxxxxx012345678901234567890123456789012345678901234567890123456789012",
            "012345678901234567890123456789012345678901234567890123456789012",
            Some(6),
        ),
        ("", "0123456789012345678901234567890123456789", None),
        (
            "xx012345678901234567890123456789012345678901234567890123456789012",
            "0123456789012345678901234567890123456789",
            Some(2),
        ),
        (
            &"xx012345678901234567890123456789012345678901234567890123456789012"[..41],
            "0123456789012345678901234567890123456789",
            None,
        ),
        (
            "xx012345678901234567890123456789012345678901234567890123456789012",
            "0123456789012345678901234567890123456xxx",
            None,
        ),
        (
            "xx01234567890123456789012345678901234567890123456789012345678\
            90120123456789012345678901234567890123456xxx",
            "0123456789012345678901234567890123456xxx",
            Some(65),
        ),
        // test fallback to Rabin-Karp.
        ("oxoxoxoxoxoxoxoxoxoxoxoy", "oy", Some(22)),
        ("oxoxoxoxoxoxoxoxoxoxoxox", "oy", None),
    ];
    for (text, pat, expected) in cases {
        single::<S>(text, pat, *expected);
    }
}
