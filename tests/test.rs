use loosen::loose;
use loosen_map::LooseMap;

struct A;
struct B;

#[loose]
fn fa(_a: A, _b: B) {}

#[test]
fn test_map() {
    (A, B).loose_map(fa_loose)
}

#[loose]
fn fb(_a: A, _b: B, _c: A) {}
#[test]
fn test_map_b() {
    (A, B, A).loose_map(fb_loose)
}

#[loose]
fn fc(_a: A, _b: B, _c: A, _d: B) {}
#[test]
fn test_map_c() {
    (A, B, A, B).loose_map(fc_loose)
}
