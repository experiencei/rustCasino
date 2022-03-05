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


#[derive(Debug)]
enum Menuchoice {
    MainMenu,
    Start,
    Quit
}

fn get_choices(input: &str) -> Result<Menuchoice , String> {
    match input {
        "mainmenu" => Ok(Menuchoice::MainMenu),
        "start" => Ok(Menuchoice::Start),
        "quit" => Ok(Menuchoice::Quit),
        _ => Err("Menu choice not found".to_owned())
    }
}
fn print_choice(choice: &Menuchoice) {
    println!("choice = {:?}", choice);
}

//question mark operator  **()** is called a uni b type 
    // fn pick_choice(input: &str) -> Result<() , String> {
    //     let choice: Menuchoice = get_choices(input)?;
    //     print_choice(&choice);
    //     Ok(())
    // }
fn main() {

    // first approach
    let choice: Result<Menuchoice, _> = get_choices("mainmenu");
    match choice {
        Ok(choice) =>  print_choice(&choice),
        Err(e) => println!("erroe = {:?}", e),
    }
    // println!("choice = {:?}", choice);

    // second approach
  print_choice("start")

   // third approach for error
   let choice = pick_choice("end");
   println!("choice = {:?}", choice);
}