use inputbot::{self};
use std::{cmp::max, collections::HashMap, io::{Write, stdout}, thread::sleep};
use std::time::Duration;
use std::fs::read_to_string;


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

fn main() {
    println!("scpmapper v1.0.0");

    let loop_time = Duration::from_millis(33);


    let layout_meta = read_to_string("layouts.txt").unwrap();
    let layout_meta = layout_meta.split("\r\n").map(|row| {
        row.split(" ").collect::<Vec<&str>>()
    }).collect::<Vec<Vec<&str>>>();

    let mut layouts = layout_meta.iter().map(|map| {
        let path = format!("scp-sl-layouts/{}/{}.txt", map[1], map[2]);
        Layout{
            map: {
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
                if c != &' ' && char_to_dirs.get(c).unwrap().len() == 1{
                    layout.paths.push(((x, y), vec![char_to_dirs.get(c).unwrap()[0].clone()], false));
                }
            }
        }
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
        'check_all: for a in 0..layouts.len(){ // Iterate through all paths
            for b in 0..layouts[a].paths.len(){
                if layouts[a].paths[b].2 {continue;}
                if layouts[a].paths[b].1.len() > 10{layouts[a].paths[b].2 = true; continue;} // Max path length is 10, to stop loops
                for x in 0..layouts.len(){ // And match them against all others
                    if layouts[x].zone != layouts[a].zone {continue;}
                    for y in 0..layouts[x].paths.len(){
                        if x == a && b == y {continue;}
                        if layouts[x].paths[y].1.iter().map(|e| direction_to_local(layouts[x].paths[y].1[0], *e)).collect::<Vec<Direction>>().iter().eq( // Rotate paths so all are pointing the same way
                            layouts[a].paths[b].1.iter().map(|e| direction_to_local(layouts[a].paths[b].1[0], *e)).collect::<Vec<Direction>>().iter()) {
                            all_good = false;
                            paths_to_extend.push((x, y)); // Add equal paths to a list to expand them one step further
                        }
                    }
                }
                if !all_good{
                    paths_to_extend.push((a, b));
                    break 'check_all;
                }
                layouts[a].paths[b].2 = true; // Path is unique and finished, doesn't need to be iterated over again. This line alone sped things up at least 10x
            }
        }
        
        if all_good{ break;}else{ // If all paths are unique, all good, otherwise expand them
            for x in paths_to_extend.iter(){
                if layouts[x.0].paths[x.1].2 {continue;}
                let mut pos = layouts[x.0].paths[x.1].0;
                let mut last_dir: Direction = Direction::Up;
                for dir in (&layouts[x.0].paths[x.1].1).iter(){ // Move to end of path
                    last_dir = *dir;
                    match dir{
                        Direction::Up => pos.1 -= 1,
                        Direction::Right => pos.0 += 1,
                        Direction::Down => pos.1 += 1,
                        Direction::Left => pos.0 -= 1
                    }
                }
                
                let dirs = char_to_dirs.get(&layouts[x.0].map[pos.1][pos.0]).unwrap();
                if dirs.len() == 1{
                    layouts[x.0].paths[x.1].2 = true;
                }else{
                    let mut paths_added = 0;
                    for dir in dirs.iter(){
                        if (*dir as u8) == (last_dir as u8 + 2)%4  {continue;} // Don't go back same way as it came from
                        if paths_added == dirs.len() - 2{
                            layouts[x.0].paths[x.1].1.push(*dir); // Modify old path if no new branches are needed
                        }else{
                            let mut path = layouts[x.0].paths[x.1].clone(); // Add new paths if it branches
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

    let mut pressed_last_frame = false;
    let mut keydown = false;

    let mut dirstring = "".to_string();

    println!("Started!");

    let mut zone = Zone::Entrance;
    let mut state = 0; // 0 = select zone, 1 = awaiting selection, 2 = finding zone

    loop{

        if inputbot::KeybdKey::Numpad8Key.is_pressed() || inputbot::KeybdKey::UpKey.is_pressed(){
            if !pressed_last_frame{
                keydown = true;
                dirstring = [dirstring, "F".into()].concat();
            }
            pressed_last_frame = true;
        }else if inputbot::KeybdKey::Numpad4Key.is_pressed() || inputbot::KeybdKey::LeftKey.is_pressed(){
            if !pressed_last_frame{
                keydown = true;
                dirstring = [dirstring, "L".into()].concat();
            }
            pressed_last_frame = true;
        }else if inputbot::KeybdKey::Numpad6Key.is_pressed() || inputbot::KeybdKey::RightKey.is_pressed(){
            if !pressed_last_frame{
                keydown = true;
                dirstring = [dirstring, "R".into()].concat();
            }
            pressed_last_frame = true;
        }else if inputbot::KeybdKey::Numpad5Key.is_pressed() || inputbot::KeybdKey::DownKey.is_pressed(){
            if !pressed_last_frame{
                keydown = true;
                dirstring = [dirstring, "E".into()].concat();
            }
            pressed_last_frame = true;
        }else if inputbot::KeybdKey::Numpad0Key.is_pressed() || inputbot::KeybdKey::BackspaceKey.is_pressed(){
            if !pressed_last_frame{
                if dirstring.len() > 0{
                    keydown = true;
                    dirstring = dirstring[0..dirstring.len() - 1].into();
                }
            }
            pressed_last_frame = true;
        }else{
            pressed_last_frame = false;
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
        
        
        
                    for x in 0..layouts.len(){
                        if layouts[x].zone != zone_to_string(zone) {continue;}
                        for y in 0..layouts[x].paths.len(){
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
                                Direction::Left => pos.0 -= 1
                            }
                        }
        
                        println!("Map: {}", layouts[matching_layouts[0].0].name);
                        
                        for y in 0..layouts[matching_layouts[0].0].map.len(){ // Draw map
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