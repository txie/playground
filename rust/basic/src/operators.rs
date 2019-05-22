pub fn run() {
    println!("===== operations =====");
    let a = 1;
    let b = 2;

    let c = a == b;
    let d = a != b;
    let e = a < b;
    let f = a > b;
    let g = a <= a;
    let h = (a >= a);

    assert_eq!(c, false);
    assert_eq!(d, true);
    assert_eq!(e, true);
    assert_eq!(f, false);
    assert_eq!(g, true);
    assert_eq!(h, true);
    
    let c = a & b;
    let d = a | b;
    println!("c = {}, d = {}", c, d);

}