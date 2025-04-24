fn main() {
    use device_query::{DeviceEvents, DeviceState};

    let thread: JoinHandle<()> = thread::spawn(move || loop {
        let state = receiver.as_ref().lock().unwrap().recv().unwrap(); //receiver comes from main thread to control the state(bool)
        let device_state: DeviceState = DeviceState::new();
        let guard = device_state.on_mouse_move(move |position| {
            println!("mouse position: {}:{}", position.0, position.1);
        });
        'inner: loop {
            //when state is false, I want to stop guard thread
            if !state {
                //I try these actions but I still can't stop it
                drop(guard); //not working
                break 'inner; //not working
            }
        }
    });
}