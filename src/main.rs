use screenshots::Screen; // Importing Screen from the screenshots crate for capturing the screen.
use std::{ collections::HashSet, fs::{ create_dir, read_dir }, sync::{ Arc, Mutex } }; // Importing necessary standard library modules for collections, file operations, and concurrency.
use rdev::{ listen, Event, EventType, Key }; // Importing event handling utilities from the rdev crate.
use chrono::{ Datelike, Local, Timelike }; // Importing time utilities from the chrono crate.

fn main() {
    // Print initial instructions to the user.
    println!( "To take a screenshot, press the left Ctrl key." );
    println!( "Below is a live list of created files." );
    
    // Shared state for tracking pressed keys using Arc and Mutex for thread safety.
    let presed_keys = Arc::new( Mutex::new( HashSet::new() ) );
    let keys_cloned = Arc::clone( &presed_keys ); // Clone the Arc for use in the callback function.
    
    // Start listening for keyboard events, passing the callback function to handle them.
    if let Err( why ) = listen( move |event| callback( event, &keys_cloned ) ) {
        // Handle any errors that occur during the event listening process.
        println!( "An error occurred while listening: {:?}", why );
    }
}

// Callback function to handle keyboard events.
fn callback( event: Event, pressed_key: &Arc<Mutex<HashSet<Key>>> ) {
    // Check if the 'screens' directory exists; if not, attempt to create it.
    if let Err( _ ) = read_dir( "./screens" ) {
        if let Err( why ) = create_dir( "./screens" ) {
            println!( "Error creating directory: {:?}", why );
        }
    }

    // Match the type of event received.
    match event.event_type {
        // If a key is pressed, handle the event.
        EventType::KeyPress( key ) => {
            let mut keys = pressed_key.lock().unwrap(); // Lock the mutex to safely modify the set of pressed keys.
            
            // Insert the key into the set of pressed keys.
            if keys.insert( key ) {
                match key {
                    // If the left Ctrl key is pressed, take a screenshot.
                    Key::ControlLeft => {
                        // Capture the screen starting from the top-left corner (0,0).
                        let screen = Screen::from_point( 0, 0 ).unwrap();
                        let screenshot = screen.capture().unwrap(); // Capture the screen and handle potential errors.
                        
                        // Get the current local time for naming the screenshot file.
                        let d_time = Local::now();
                        let name = format!(
                            "{}{}{}{}{}{}.png",
                            d_time.year(),
                            if d_time.month() < 10 { format!( "0{}", d_time.month() ) } else { format!( "{}", d_time.month() ) },
                            if d_time.day() < 10 { format!( "0{}", d_time.day() ) } else { format!( "{}", d_time.day() ) },
                            if d_time.hour() < 10 { format!( "0{}", d_time.hour() ) } else { format!( "{}", d_time.hour() ) },
                            if d_time.minute() < 10 { format!( "0{}", d_time.minute() ) } else { format!( "{}", d_time.minute() ) },
                            if d_time.second() < 10 { format!( "0{}", d_time.second() ) } else { format!( "{}", d_time.second() ) }
                        ); // Format the timestamp to create a unique filename for the screenshot.
                        
                        // Save the screenshot to the 'screens' directory.
                        screenshot.save( format!( "./screens/{name}" ) ).unwrap();
                        println!( "Created file {name}" ); // Print confirmation of the screenshot creation.
                    },
                    _ => {} // Do nothing for other keys.
                }
            }
        },
        // If a key is released, remove it from the set of pressed keys.
        EventType::KeyRelease( key ) => {
            let mut keys = pressed_key.lock().unwrap(); // Lock the mutex to safely modify the set.
            keys.remove( &key );
        }
        _ => {} // Do nothing for other event types.
    }
}
