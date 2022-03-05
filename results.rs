fn get_sound(name: &str) -> Result<SoundData , String> {
    if name == "alert" {
        Ok(SoundData::new("alert")),

    }  else {
        Err("Unable to find sound data".to_owned())
    }
}

let sound = get_sound("alert")

match sound {
    Ok(_) => println!(" sound data located"),
    Err(e) => println!("error: {:?}" , e),
}

// **_** the underscore used is for ignoring the all others variables. as  we dont what to print the string "alert"