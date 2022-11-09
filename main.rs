mod input;

fn main()
{
    let mut button = String::from("o");
    let mut past = 0;
    let mut led = -1;

    loop
    {  
        input::getstr(&mut button);

        if (button.trim() == "x" && past == 0)
        {
            past = 1;
            led *=-1;
        }

        if (button.trim() != "x" && past == 1)
        {
            past = 0;
        }

        println!("{}", led);
    }
}