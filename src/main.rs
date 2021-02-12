use inputbot::{self, handle_input_events};
use std::{collections::HashMap, thread::sleep};
use std::time::Duration;
use std::fs::read_to_string;
use std::collections::hash_map;
use std::process::Command;
use std::env::current_dir;

enum Direction{
    Forward,
    Left,
    Right
}

struct Layout{
    map: Vec<Vec<char>>,
    name: String
}

struct Map{
    zone: String,
    name: String
}

fn main() {
    println!("scpmapper v0.1.1");
    let dir = current_dir().unwrap();
    let dir = dir.as_path().to_str().unwrap();

    let loop_time = Duration::from_millis(33);

    let mut dirs: Vec<Direction> = vec![];

    /*let layout_meta = read_to_string("layouts.txt").unwrap();
    let layout_meta = layout_meta.split("\r\n").map(|row| {
        row.split(" ").collect::<Vec<&str>>()
    }).collect::<Vec<Vec<&str>>>();

    let mut layouts = layout_meta.iter().map(|map| {
        let path = format!("scp-sl-layouts/{}/{}.txt", map[1], map[2]);
        //println!("{}",  path);
        Layout{
            map: {
                let file = read_to_string(path).unwrap();
                file.split("\n").map(|x| x.chars().collect()).collect::<Vec<Vec<char>>>()
            },
            name: map[0].to_string()
        }
    }
    );*/

    let mut codes_to_map: HashMap<&str, Map> = HashMap::new();

    let mut paths_file = read_to_string("paths.txt").unwrap();
    //println!("{}", paths_file.split(" ").next().unwrap());
    for x in paths_file.split("\r\n"){
        //println!("{}", x);
        let data: Vec<&str> = x.split(" ").collect();
        for path in data[2..].iter(){
            //println!("{}", *path);
            codes_to_map.insert(*path, Map{zone: data[0].into(), name: data[1].into()});
        }
    }
    
    /*paths_file.split(" ").map(|x|{
        unreachable!();
        println!("aa: {}", x);
        let data: Vec<&str> = x.split(" ").collect();
        for path in data.iter(){
            println!("{}", *path);
            codes_to_map.insert(*path, Map{zone: data[0].into(), name: data[1].into()});
        }*
});*/

    //for layout in layouts{
        //println!("{}", layout.name);
    //}

    /*inputbot::KeybdKey::Numpad8Key.block_bind( ||{
        dirs.push(Direction::Forward);
    });
    inputbot::KeybdKey::Numpad4Key.bind(||{
        &dirs.push(Direction::Left);
    });
    inputbot::KeybdKey::Numpad6Key.bind(||{
        &dirs.push(Direction::Right);
    });*/


    //handle_input_events();

    let mut pressedLastFrame = false;
    let mut keydown = false;

    let mut dirstring = "".to_string();

    println!("Started!");

    loop{
        if inputbot::KeybdKey::Numpad8Key.is_pressed() || inputbot::KeybdKey::UpKey.is_pressed(){
            if !pressedLastFrame{
                dirs.push(Direction::Forward);
                keydown = true;
                dirstring = [dirstring, "F".into()].concat();
            }
            pressedLastFrame = true;
        }else if inputbot::KeybdKey::Numpad4Key.is_pressed() || inputbot::KeybdKey::LeftKey.is_pressed(){
            if !pressedLastFrame{
                dirs.push(Direction::Forward);
                keydown = true;
                dirstring = [dirstring, "L".into()].concat();
            }
            pressedLastFrame = true;
        }else if inputbot::KeybdKey::Numpad6Key.is_pressed() || inputbot::KeybdKey::RightKey.is_pressed(){
            if !pressedLastFrame{
                dirs.push(Direction::Forward);
                keydown = true;
                dirstring = [dirstring, "R".into()].concat();
            }
            pressedLastFrame = true;
        }else if inputbot::KeybdKey::Numpad5Key.is_pressed() || inputbot::KeybdKey::DownKey.is_pressed(){
            if !pressedLastFrame{
                dirs.push(Direction::Forward);
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

        if keydown{
            println!("{}", dirstring);
            if codes_to_map.contains_key(&*dirstring){
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
            }

            keydown = false;

        }

        sleep(loop_time);
    }
}
