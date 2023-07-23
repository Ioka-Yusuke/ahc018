use proconio::input;
use std::convert::TryInto;
use std::process;

fn main() {
    // input
    input! {
        n: usize,
        w: usize,
        k: usize,
        cost: i32,
        ab: [(usize, usize); w],
        cd: [(usize, usize); k],
    }

    // map
    // const STOP_VALUE: usize = 1_000_000_000;
    const STOP_VALUE: usize = 0;
    let mut map = vec![vec![STOP_VALUE; n]; n];

    // 水源 = 1
    for i in 0..w {
        let a = ab[i].0;
        let b = ab[i].1;

        map[(a) as usize][(b) as usize] = 1
    }

    //　 家 = 2
    for i in 0..k {
        let c = cd[i].0;
        let d = cd[i].1;

        map[(c) as usize][(d) as usize] = 2
    }

    // 探索ループ
    let mut k_cnt:usize = 0;
    loop {
        // 家の場所サーチ
        let mut house = vec![];
        for i in 0..n {
            for j in 0..n {
                if map[i][j] == 2 {
                    house.push((i, j));
                }
            }
        }
        
        // 水源と家の距離
        let mut list_dist = vec![];
        for i in 0..house.len() {
            // let c:i32 = cd[i].0.try_into().unwrap();
            let c:i32 = house[i].0.try_into().unwrap();
            let d:i32 = house[i].1.try_into().unwrap();
            
            let mut min_dist:i32 = 1_000_000_000;
            let mut min_a:i32 = 0;
            let mut min_b:i32 = 0;

            for j_x in 0..n {
                for j_y in 0..n {
                    let a:i32 = j_x.try_into().unwrap();
                    let b:i32 = j_y.try_into().unwrap();
                
                    if map[(a) as usize][(b) as usize] == 1 {
                        let diff_ac = a - c;
                        let diff_bd = b - d;
                        let dist = diff_ac.pow(2) + diff_bd.pow(2);
                    
                        if dist < min_dist {
                            min_dist = dist;
                            min_a = a;
                            min_b = b;
                        }
                    }
                }
            }
            list_dist.push([min_dist, min_a, min_b, c, d]);
        }
        list_dist.sort_by(|a, b| a.partial_cmp(b).unwrap());

        // 探索
        let mut now_x:i32 = list_dist[0][1].try_into().unwrap(); 
        let mut now_y:i32 = list_dist[0][2].try_into().unwrap();  
        let g_x:i32 = list_dist[0][3].try_into().unwrap(); 
        let g_y:i32 = list_dist[0][4].try_into().unwrap(); 
        let dx:i32 = g_x - now_x;
        let dy:i32 = g_y - now_y;
        let dx_abs:usize = dx.abs().try_into().unwrap();
        let dy_abs:usize = dy.abs().try_into().unwrap();
        let p:i32 = 4999;
        
        // 確認
        println!("{}", "-------------------");
        for i in 0..list_dist.len() {
            println!("{} {} {} {} {}", list_dist[i][0], list_dist[i][1], list_dist[i][2], list_dist[i][3], list_dist[i][4]);
        }
        for i in 0..n {
            println!("{} {} {} {}", map[i][0], map[i][1], map[i][2], map[i][3]);
        }
        println!("{}", "-------------------");

        // start地点(水源)を掘る
        if k_cnt == 0 {
            loop {
                println!("{} {} {}", now_x, now_y, cost+p);
                input! {
                    flag: usize,
                }

                if flag == 1 {
                    break;
                }
            }
        }
        k_cnt += 1;

        // goal地点(家)まで掘る
        let mut now_dx:usize = 0;
        let mut now_dy:usize = 0;

        let mut direct_key = 0;
        loop {

            if direct_key % 2 == 0 && dx > 0 && now_dx < dx_abs {
                // up
                println!("{} {} {}", now_x+1, now_y, cost+p);

                input! {
                    flag: usize,
                }

                if flag == 1 && (now_x+1 != g_x || now_y != g_y) {
                    map[(now_x+1) as usize][(now_y) as usize] = 1;
                    now_dx += 1;
                    now_x += 1;
                    direct_key += 1;
                    continue;
                } else if flag == 1 && now_x+1 == g_x && now_y == g_y {
                    map[(now_x+1) as usize][(now_y) as usize] = 1;
                    break;
                } else if flag == 2 {
                    process::exit(0);
                }

            } else if direct_key % 2 == 0 && dx < 0 && now_dx < dx_abs {
                // down
                println!("{} {} {}", now_x-1, now_y, cost+p);

                input! {
                    flag: usize,
                }

                if flag == 1 && (now_x-1 != g_x || now_y != g_y) {
                    map[(now_x-1) as usize][(now_y) as usize] = 1;
                    now_dx += 1;
                    now_x -= 1;
                    direct_key += 1;
                    continue;
                } else if flag == 1 && now_x-1 == g_x && now_y == g_y {
                    map[(now_x-1) as usize][(now_y) as usize] = 1;
                    break;
                } else if flag == 2 {
                    process::exit(0);
                }

            } else if direct_key % 2 == 1 && dy > 0 && now_dy < dy_abs {
                // right
                println!("{} {} {}", now_x, now_y+1, cost+p);

                input! {
                    flag: usize,
                }

                if flag == 1 && (now_x != g_x || now_y+1 != g_y) {
                    map[(now_x) as usize][(now_y+1) as usize] = 1;
                    now_dy += 1;
                    now_y += 1;
                    direct_key += 1;
                    continue;
                } else if flag == 1 && now_x == g_x && now_y+1 == g_y {
                    map[(now_x) as usize][(now_y+1) as usize] = 1;
                    break;
                } else if flag == 2 {
                    process::exit(0);
                }

            } else if direct_key % 2 == 1 && dy < 0 && now_dy < dy_abs {
                // left
                println!("{} {} {}", now_x, now_y-1, cost+p);

                input! {
                    flag: usize,
                }

                if flag == 1 && (now_x != g_x || now_y-1 != g_y) {
                    map[(now_x) as usize][(now_y-1) as usize] = 1;
                    now_dy += 1;
                    now_y -= 1;
                    direct_key += 1;
                    continue;
                } else if flag == 1 && now_x == g_x && now_y-1 == g_y {
                    map[(now_x) as usize][(now_y-1) as usize] = 1;
                    break;
                } else if flag == 2 {
                    process::exit(0);
                }

            } else if direct_key % 2 == 0 && (dx == 0 || now_dx == dx_abs) {
                // ex up down 
                direct_key += 1;
                continue;

            } else if direct_key % 2 == 1 && (dy == 0 || now_dy == dy_abs) {
                // ex right left
                direct_key += 1;
                continue;
            }
        }
    }
}
