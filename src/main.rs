use screenshots::Screen;
use std::{ collections::HashSet, fs::{ create_dir, read_dir }, sync::{Arc, Mutex} };
use rdev::{ listen, Event, EventType, Key };
use chrono::{ Datelike, Local, Timelike };

fn main() {
    println!( "Aby zrobić screen należy wcisnąć lewy Ctrl." );
    println!( "Poniżej znajduje się na bierząco generowana lista utworzonych plików." );
    let presed_keys = Arc::new( Mutex::new( HashSet::new() ) );
    let keys_cloned = Arc::clone( &presed_keys );
    if let Err( why ) = listen( move |event| callback( event, &keys_cloned ) ) {
        println!( "Wystąpił błąd w tarkcie nasłuchiwania: {:?}", why );
    }
}

fn callback( event: Event, pressed_key: &Arc<Mutex<HashSet<Key>>> ) {
    if let Err( _ ) = read_dir( "./screens" ) {
        if let Err( why ) = create_dir( "./screens" ) {
            println!( "Błąd z utworzeniem katalogu: {:?}", why );
        }
    }
    match event.event_type {
        EventType::KeyPress( key ) => {
            let mut keys = pressed_key.lock().unwrap();
            if keys.insert( key ) {
                match key {
                    Key::ControlLeft => {
                        let screen = Screen::from_point( 0, 0 ).unwrap();
                        let screenshot = screen.capture().unwrap();
                        let d_time = Local::now();
                        let name = format!(
                            "{}{}{}{}{}{}.png",
                            d_time.year(),
                            if d_time.month() < 10 { format!( "0{}", d_time.month() ) } else { format!( "{}", d_time.month() ) },
                            if d_time.day() < 10 { format!( "0{}", d_time.day() ) } else { format!( "{}", d_time.day() ) },
                            if d_time.hour() < 10 { format!( "0{}", d_time.hour() ) } else { format!( "{}", d_time.hour() ) },
                            if d_time.minute() < 10 { format!( "0{}", d_time.minute() ) } else { format!( "{}", d_time.minute() ) },
                            if d_time.second() < 10 { format!( "0{}", d_time.second() ) } else { format!( "{}", d_time.second() ) }
                        );
                        screenshot.save( format!(
                            "./screens/{name}"
                        ) ).unwrap();
                        println!( "Utworzono plik {name}" );
                    },
                    _ => {}
                }
            }
        },
        EventType::KeyRelease( key ) => {
            let mut keys = pressed_key.lock().unwrap();
            keys.remove( &key );
        }
        _ => {}
    }
}