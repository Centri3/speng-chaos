use {
    chaos_handler::HANDLER,
    chaos_patch::{Opcode, PATCHES},
    rand::Rng,
    std::{io, panic, thread, time::Duration},
};

fn write_opcodes(opcodes: Vec<Opcode>, on: bool) {
    for opcode in opcodes {
        let address = opcode.address;
        let bytes = match on {
            true => opcode.on,
            false => opcode.off,
        };

        HANDLER.write_bytes(HANDLER.base_of(address.0) + address.1, &bytes);
    }
}

fn main() {
    // Panicking instantly closes the offending program. I don't like that.
    panic::set_hook(Box::new(|pi| {
        println!("{pi}");
        println!("press 'enter' to exit.");

        // Is there a better way to do this?
        io::stdin().read_line(&mut String::new()).unwrap();
    }));

    let mut rng = rand::thread_rng();
    let mut time = 5000u64;
    let patches = PATCHES.patches.clone();

    // Print a nice, welcoming message to the user.
    HANDLER.run_script("welcome.se");

    thread::sleep(Duration::from_secs(13u64));

    loop {
        let patch = patches[rng.gen_range(0usize..patches.len())].clone();
        // Rename patch's fields for easier use
        let data = patch.data;
        let persistence = patch.persistence;

        // Execute script if it's specified
        if let Some(script) = data.script {
            HANDLER.run_script(script);
        }

        // Write each opcode into SE's memory
        write_opcodes(data.opcodes.clone().unwrap(), true);

        thread::sleep(Duration::from_millis(rng.gen_range(time..time + 5000u64)));

        // If this shouldn't persist, revert to previous opcodes
        if !rng.gen_bool(persistence.chance) {
            write_opcodes(data.opcodes.unwrap(), false);
        }

        // Make the next patch come ~100ms sooner
        time -= 10u64;
    }
}
