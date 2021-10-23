
pub fn next_spiral(n: i32) -> Vec<i32> {    
    let r: i32 = ((((n+1) as f64).sqrt() - 1.0) / 2.0).floor() as i32 + 1;
    let p: i32 = (8 * (r as i32) * ((r as i32) - 1)) / 2;
    let en: i32 = (r as i32) * 2;
    let a: i32 = (1 + n + p) % ((r as i32) * 8);
    
    let mut pos: Vec<i32> = ([0, 0]).to_vec();
    match a as i64 / (r * 2) as i64{
        0=> { 
            pos[0] = a - (r as i32);
            pos[1] = -(r as i32);
        }
        1=> { 
            pos[0] = r as i32;
            pos[1] = (a % en) - (r as i32);
        }
        2=> {
            pos[0] =  (r as i32) - (a % en);
            pos[1] = r as i32;
        }
        3=> { 
            pos[0] = -(r as i32);
            pos[1] = (r as i32) - (a % en);
        }
        _=> {}
    }
    return pos
    // println!("{:?}", pos);
}