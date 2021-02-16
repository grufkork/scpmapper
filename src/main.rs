use inputbot::{self};
use std::{cmp::max, collections::HashMap, io::{Write, stdout}, thread::sleep};
use std::time::Duration;
use std::fs::read_to_string;


use std::env::current_dir;

enum Turn{
    Forward,
    Right,
    Left
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Direction{
    Up,
    Right,
    Down,
    Left
}

#[derive(Copy, Clone)]
enum Zone{
    Entrance,
    Heavy,
    Light
}


struct Layout{
    map: Vec<Vec<char>>,
    name: String,
    paths: Vec<((usize, usize),Vec<Direction>, bool)>, // .2 = is finished/terminated? (encounted room),
    zone: String
}


struct Map{
    zone: String,
    name: String
}

fn main() {
    println!("scpmapper v0.2.0");
    let dir = current_dir().unwrap();
    let dir = dir.as_path().to_str().unwrap();

    let loop_time = Duration::from_millis(33);

    let mut dirs: Vec<Turn> = vec![];

    let layout_meta = read_to_string("layouts.txt").unwrap();
    let layout_meta = layout_meta.split("\r\n").map(|row| {
        row.split(" ").collect::<Vec<&str>>()
    }).collect::<Vec<Vec<&str>>>();

    let mut layouts = layout_meta.iter().map(|map| {
        let path = format!("scp-sl-layouts/{}/{}.txt", map[1], map[2]);
        //println!("{}",  path);
        Layout{
            map: {
                //println!("{}", path);
                let file = read_to_string(path).unwrap();
                file.split("\r\n").map(|x| x.chars().collect()).collect::<Vec<Vec<char>>>()
            },
            name: map[0].to_string(),
            paths: vec![],
            zone: map[1].to_string()
        }
    }).collect::<Vec<Layout>>();

    // Load char-to-direction file
    let mut char_to_dirs: HashMap<char, Vec<Direction>> = HashMap::new();
    let charmap_file = read_to_string("chars.txt").unwrap();
    for row in charmap_file.split("\r\n") {
        let mut split = row.split(" ");
        char_to_dirs.insert(split.next().unwrap().chars().next().unwrap(), split.map(|dir| match dir{
            "up" => Direction::Up,
            "left" => Direction::Left,
            "down" => Direction::Down,
            "right" => Direction::Right,
            _ => unreachable!()
        }).collect());
    }

    // Pad maps as they are not square, get starting points
    for layout in layouts.iter_mut(){
        let mut longest_row = 0;
        for (y,row) in layout.map.iter().enumerate(){
            longest_row = max(longest_row, row.len());
            for (x, c) in row.iter().enumerate(){
                //println!("{}: {}*", c, *c as i32);
                if c != &' ' && char_to_dirs.get(c).unwrap().len() == 1{
                    layout.paths.push(((x, y), vec![char_to_dirs.get(c).unwrap()[0].clone()], false));
                    /*if char_to_dirs.get(c).unwrap()[0].clone() == Direction::Left{
                        println!("{}", c);
                    }*/
                }
            }
        }
        //println!("count: {}, {}", layout.paths.len(), layout.name);
        for i in 0..layout.map.len(){
            let len = layout.map[i].len();
            layout.map[i].append(&mut vec![' '; longest_row - len]);
        }
    }

    // Find unique paths from each room
    print!("Building Paths...");
    loop{
        let mut all_good = true;
        let mut paths_to_extend: Vec<(usize, usize)> = vec![];
        'check_all: for a in 0..layouts.len(){
            for b in 0..layouts[a].paths.len(){
                
                //let path = &layouts[a].paths[b];
                if layouts[a].paths[b].2 {continue;}
                if layouts[a].paths[b].1.len() > 8{layouts[a].paths[b].2 = true; continue;}
                for x in 0..layouts.len(){
                    if layouts[x].zone != layouts[a].zone {continue;}
                    for y in 0..layouts[x].paths.len(){
                        if x == a && b == y {continue;}
                        //println!("{}", layouts[a].paths[b].1[0]);
                        if layouts[x].paths[y].1.iter().map(|e| direction_to_local(layouts[x].paths[y].1[0], *e)).collect::<Vec<Direction>>().iter().eq(
                            layouts[a].paths[b].1.iter().map(|e| direction_to_local(layouts[a].paths[b].1[0], *e)).collect::<Vec<Direction>>().iter()) {
                            all_good = false;
                            paths_to_extend.push((x, y));
                        }
                    }
                }
                if !all_good{
                    paths_to_extend.push((a, b));
                    //println!("Len: {}", layouts[a].paths[b].1.len());
                    break 'check_all;
                }
                layouts[a].paths[b].2 = true;
            }
        }
        /*println!();
        for l in 0..1{
            for p in 0..layouts[l].paths.len(){
                print!("path {}: {},{}: ", p, layouts[l].paths[p].0.0, layouts[l].paths[p].0.1);
                for dir in layouts[l].paths[p].1.iter(){
                    print!("{}", *dir as u8);
                }
                println!();
            }
        }*/
        if all_good{ break;}else{
            //println!("c: {}", paths_to_extend.len());
            for x in paths_to_extend.iter(){
                //println!("Extending: {}", x.1);
                if layouts[x.0].paths[x.1].2 {continue;}
                let mut pos = layouts[x.0].paths[x.1].0;
                let mut last_dir: Direction = Direction::Up;
                for dir in (&layouts[x.0].paths[x.1].1).iter(){
                    //let d = dir;
                    last_dir = *dir;
                    match dir{
                        Direction::Up => pos.1 -= 1,
                        Direction::Right => pos.0 += 1,
                        Direction::Down => pos.1 += 1,
                        Direction::Left => pos.0 -= 1,
                        _ => unreachable!()
                    }
                }
                if layouts[x.0].map[pos.1][pos.0] == ' '{
                    println!("pos start: {}, {}: {}", layouts[x.0].paths[x.1].0.0, layouts[x.0].paths[x.1].0.1, layouts[x.0].map[layouts[x.0].paths[x.1].0.1][layouts[x.0].paths[x.1].0.0]);
                    println!("AAA: {} {}", layouts[x.0].paths[x.1].0.0, pos.0);
                    println!("Layout: {}, {}, X/Y:{}/{}", x.0, layouts[x.0].map[pos.1][pos.0], pos.0, pos.1);
                    println!("Last dir: {}", last_dir as u8);
                }
                let dirs = char_to_dirs.get(&layouts[x.0].map[pos.1][pos.0]).unwrap();
                if dirs.len() == 1{
                    layouts[x.0].paths[x.1].2 = true;
                    /*if x.0 == 0{
                        println!("xy: {}, {}", layouts[x.0].paths[x.1].0.0, layouts[x.0].paths[x.1].0.1);
                    }*/
                    //println!("damn");
                }else{
                    let mut paths_added = 0;
                    for dir in dirs.iter(){
                        if (*dir as u8) == (last_dir as u8 + 2)%4  {continue;}
                        //print!("A");
                        if paths_added == dirs.len() - 2{
                            layouts[x.0].paths[x.1].1.push(*dir);
                            //println!("m");
                            //println!("{}/{}: {}", x.0, x.1, layouts[x.0].paths[x.1].1.len());
                            /*if layouts[x.0].paths[x.1].1.len() == 10{
                                println!("{}..{}", layouts[x.0].paths[x.1].0.0, layouts[x.0].paths[x.1].0.1);
                                println!("Last dir: {}", last_dir as u8);
                                for d in layouts[x.0].paths[x.1].1.iter(){
                                    println!("{}", *d as u8);
                                }
                            }*/
                        }else{
                            let mut path = layouts[x.0].paths[x.1].clone();
                            path.1.push(*dir);
                            layouts[x.0].paths.push(path);
                        }
                        paths_added += 1;
                    }
                }
            }

            print!(".");
            stdout().flush().unwrap();
        }
    }
    println!();

    // Something
    let mut codes_to_map: HashMap<&str, Map> = HashMap::new();

    let mut paths_file = read_to_string("paths.txt").unwrap();
    for x in paths_file.split("\r\n"){
        let data: Vec<&str> = x.split(" ").collect();
        for path in data[2..].iter(){
            codes_to_map.insert(*path, Map{zone: data[0].into(), name: data[1].into()});
        }
    }

    let mut pressedLastFrame = false;
    let mut keydown = false;

    let mut dirstring = "".to_string();

    println!("Started!");

    /*for l in 0..1{
        for p in 0..layouts[l].paths.len(){
            print!("path {},{}: ", layouts[l].paths[p].0.0, layouts[l].paths[p].0.1);
            for dir in layouts[l].paths[p].1.iter(){
                print!("{}", *dir as u8);
            }
            println!();
        }
    }*/

    let mut zone = Zone::Entrance;
    let mut state = 0; // 0 = select zone, 1 = awaiting selection, 2 = finding zone

    loop{

        if inputbot::KeybdKey::Numpad8Key.is_pressed() || inputbot::KeybdKey::UpKey.is_pressed(){
            if !pressedLastFrame{
                dirs.push(Turn::Forward);
                keydown = true;
                dirstring = [dirstring, "F".into()].concat();
            }
            pressedLastFrame = true;
        }else if inputbot::KeybdKey::Numpad4Key.is_pressed() || inputbot::KeybdKey::LeftKey.is_pressed(){
            if !pressedLastFrame{
                dirs.push(Turn::Forward);
                keydown = true;
                dirstring = [dirstring, "L".into()].concat();
            }
            pressedLastFrame = true;
        }else if inputbot::KeybdKey::Numpad6Key.is_pressed() || inputbot::KeybdKey::RightKey.is_pressed(){
            if !pressedLastFrame{
                dirs.push(Turn::Forward);
                keydown = true;
                dirstring = [dirstring, "R".into()].concat();
            }
            pressedLastFrame = true;
        }else if inputbot::KeybdKey::Numpad5Key.is_pressed() || inputbot::KeybdKey::DownKey.is_pressed(){
            if !pressedLastFrame{
                dirs.push(Turn::Forward);
                keydown = true;
                dirstring = [dirstring, "E".into()].concat();
            }
            pressedLastFrame = true;
        }else if inputbot::KeybdKey::Numpad0Key.is_pressed() || inputbot::KeybdKey::BackspaceKey.is_pressed(){
            if !pressedLastFrame{
                if dirstring.len() > 0{
                    keydown = true;
                    dirstring = dirstring[0..dirstring.len() - 1].into();
                }
            }
            pressedLastFrame = true;
        }else{
            pressedLastFrame = false;
        }

        
        match state{
            0=> {
                println!();
                println!("Select zone...");
                println!("← Entrance");
                println!("↑ Heavy Containment");
                println!("→ Light Containment");
                state = 1;
            },
            1 => {
                if dirstring.len() > 0{
                    state = 2;
                    zone = match dirstring.chars().next().unwrap(){
                        'L' => Zone::Entrance,
                        'F' => Zone::Heavy,
                        'R' => Zone::Light,
                        _ => {
                            state = 1;
                            Zone::Entrance
                        }
                    };
                    if state == 2{
                        println!("Selected zone: {}", zone_to_string(zone));
                        println!(); // Make space for cursor movements
                    }
                    dirstring = "".to_string()
                }
            },
            2 => {
                if keydown{
                    print!("\x1b[1A\x1b[99D");
                    println!("{} ", dirstring); // Space to overwrite when erasing
        
                    let mut directions = vec![Direction::Up];
                    let mut angle: u8 = 0;
                    let mut stopped = false;
                    for mov in dirstring.chars(){
                        match mov{
                            'L' => angle = (angle + 3) % 4,
                            'R' => angle = (angle + 1) % 4,
                            'E' => {stopped = true; break;},
                            'F' => (),
                            _ => unreachable!()
                        }
                        directions.push(match angle{
                            0 => Direction::Up,
                            1 => Direction::Right,
                            2 => Direction::Down,
                            3 => Direction::Left,
                            _ => unreachable!()
                        });
                    }
        
                    let mut matching_layouts: Vec<(usize, usize)> = vec![];
        
                    //let mut longer_match_exists = false;
        
        
                    for x in 0..layouts.len(){
                        if layouts[x].zone != zone_to_string(zone) {continue;}
                        for y in 0..layouts[x].paths.len(){
                            if x == 0 && y == 0{
                                /*for d in layouts[x].paths[y].1.iter().map(|e| direction_to_local(layouts[x].paths[y].1[0], *e)){
                                    println!("p {}", d as u8);
                                    
                                }for dir in directions.iter(){
                                    println!("{}", *dir as u8);
                                }*/
                            }
                            if layouts[x].paths[y].1.len() >= directions.len() && layouts[x].paths[y].1[0..directions.len()].iter().map(|e| direction_to_local(layouts[x].paths[y].1[0], *e)).collect::<Vec<Direction>>().iter().eq(&directions) {
                                matching_layouts.push((x, y));
                            }
                        }
                    }
        
                    let mut match_found = false;
                    if matching_layouts.len() == 1{
                        match_found = true;
                    }else if stopped{
                        let mut count = 0;
                        for possible_path in matching_layouts.iter(){
                            if layouts[possible_path.0].paths[possible_path.1].1.len() == directions.len(){
                                count += 1;
                                println!("Possible match: {}", layouts[possible_path.0].name);
                            }
                        }
                        if count == 1{
                            match_found = true;
                        }else{
                            println!("!!! Multiple matches found. Clearing...");
                            dirstring = "".to_string();
                            state = 0;
                        }
                    }
        
                    if match_found{
                        let mut pos = layouts[matching_layouts[0].0].paths[matching_layouts[0].1].0;
                    
                        for dir in (&layouts[matching_layouts[0].0].paths[matching_layouts[0].1].1).iter(){
                            match dir{
                                Direction::Up => pos.1 -= 1,
                                Direction::Right => pos.0 += 1,
                                Direction::Down => pos.1 += 1,
                                Direction::Left => pos.0 -= 1,
                                _ => unreachable!()
                            }
                        }
        
                        println!("Map: {}", layouts[matching_layouts[0].0].name);
                        /*for dir in directions.iter(){
                            println!("{}", *dir as u8);
                        }*/
                        for y in 0..layouts[matching_layouts[0].0].map.len(){
                            for x in 0..layouts[matching_layouts[0].0].map[y].len(){
                                let mut stylepre = "";
                                let mut stylepost = "";
                                if x == layouts[matching_layouts[0].0].paths[matching_layouts[0].1].0.0 && y == layouts[matching_layouts[0].0].paths[matching_layouts[0].1].0.1{
                                    stylepre = "\x1b[0;101m";
                                    stylepost = "\x1b[0m";
                                }else if x == pos.0 && y == pos.1{
                                    stylepre = "\x1b[0;42m";
                                    stylepost = "\x1b[0m";
                                }
                                print!("{}{}{}", stylepre, layouts[matching_layouts[0].0].map[y][x], stylepost);
                            }
                            println!();
                        }
                        println!("Facing: {}", match layouts[matching_layouts[0].0].paths[matching_layouts[0].1].1.last().unwrap(){
                            Direction::Up => "↑",
                            Direction::Right => "→",
                            Direction::Down => "↓",
                            Direction::Left => "←"
                        });
                        println!();
                        println!();
                        dirstring = "".to_string();
                        state = 0;
                    }
        
                    /*if codes_to_map.contains_key(&*dirstring){
                        let map = codes_to_map.get(&*dirstring).unwrap();
                        println!("Found map: {}", map.name);
                        dirstring = "".to_string();
                        //println!("{}", format!("\"C:\\ProgramFiles\\Windows Photo Viewer\\PhotoViewer.dll\", ImageView_Fullscreen {}\\{}\\{}.png", dir, map.zone, map.name));
                        /*Command::new("rundll32")
                        //.arg("\"C:\\ProgramFiles\\Windows Photo Viewer\\PhotoViewer.dll\",")
                        //.arg("ImageView_Fullscreen")
                        //.arg(format!("{}\\{}.png", map.zone, map.name))
                        //.arg("/C")
                        //.arg(format!("C:\\Windows\\System32\\rundll32.exe \"C:\\ProgramFiles\\Windows\\ Photo Viewer\\PhotoViewer.dll\", ImageView_Fullscreen {}\\{}\\{}.png", dir, map.zone, map.name))
                        .arg("\"C:\\Program Files\\Windows Photo Viewer\\PhotoViewer.dll\"")
                        .arg("\"E:\\pics\\pog.png\"")
                        .status().unwrap();*/
                        Command::new("cmd")
                        .arg("/C")
                        //.arg(format!("explorer"))
                        .arg(format!("explorer {}\\mapimgs\\{}\\{}.png", dir, map.zone, map.name))
                        .status().unwrap();
        
                        //println!("{}", format!("explorer {}\\mapimgs\\{}\\{}.png", dir, map.zone, map.name));
                    }*/
        
                    
        
                }
            },
            _ => unreachable!()
        }
        
        keydown = false;
        sleep(loop_time);
    }
}

fn direction_to_local(facing: Direction, direction: Direction) -> Direction{
    match (direction as u8 + 4 - facing as u8) % 4{
        0=> Direction::Up,
        1=> Direction::Right,
        2=> Direction::Down,
        3=> Direction::Left,
        _ => unreachable!()
    }
}

fn zone_to_string(zone: Zone) -> String{
    match zone{
        Zone::Entrance => "entrance".to_string(),
        Zone::Light => "light".to_string(),
        Zone::Heavy => "heavy".to_string()
    }
}