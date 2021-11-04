
pub fn next_spiral(n: i64) -> Vec<i64> {    
    let r: i64 = ((((n+1) as f64).sqrt() - 1.0) / 2.0).floor() as i64 + 1;
    let p: i64 = (8 * (r) * ((r) - 1)) / 2;
    let en: i64 = (r) * 2;
    let a: i64 = (1 + n + (p)) % ((r) * 8);
    
    let mut pos: Vec<i64> = ([0, 0]).to_vec();
    match a / (r * 2) {
        0=> { 
            pos[0] = a - (r as i64);
            pos[1] = -(r);
        }
        1=> { 
            pos[0] = r;
            pos[1] = (a % en) - (r);
        }
        2=> {
            pos[0] =  (r) - (a % en);
            pos[1] = r;
        }
        3=> { 
            pos[0] = -(r);
            pos[1] = (r) - (a % en);
        }
        _=> {}
    }
    return pos
    // println!("{:?}", pos);
}