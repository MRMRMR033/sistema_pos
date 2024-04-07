use iced::{widget::{button, column, row, text, Button, Text},Element}; 
use iced::Sandbox;

#[derive(Debug, Default)]
struct Counter{
    value:i32,
    texto: String,
}


#[derive(Debug, Clone, Copy)]
pub enum Message {
    ButtonF1Pressed,    //ventana de ventas || cobrar ticket
    ButtonF2Pressed,    //cobrar ticket
    ButtonF3Pressed,    //crear producto
    ButtonF4Pressed,    //inventario
    ButtonF5Pressed,    //cambiar entre tickets
    ButtonF6Pressed,    //Dejar ticket pendiente
    
    IncrementQuantityPressed,   //incrementar cantidad de producto
    DecrementQuantityPressed,   //decrementar cantidad de producto
}

impl Sandbox for Counter {

    type Message = Message;
    
    fn new()-> Self{
        Counter { value: 0, texto: String::new()}

    }
    fn title(&self)-> String{
        String::from("Hola mundo")
    }
    fn view(&self)-> Element<Message>{
        row![
            // The increment button. We tell it to produce an
            // `IncrementPressed` message when pressed
            button("F1 Ventas").on_press(Message::ButtonF1Pressed),

            button("+").on_press(Message::IncrementPressed),

            // We show the value of the counter here
            text(self.value).size(50),
            text(&self.texto).size(50),
            // The decrement button. We tell it to produce a
            // `DecrementPressed` message when pressed
            button("-").on_press(Message::DecrementPressed),
        ].into()
    }
    fn update(&mut self, message: Message){
        match message{
            Message::ButtonF1Pressed =>{
                self.texto = "F1 Presionado".to_string();
            }
            Message::IncrementQuantityPressed =>{
                self.value += 1;
                self.texto = "Aumentando".to_string();
            },
            Message::DecrementQuantitytPressed => {
                self.value -= 1;
                self.texto = "Disminuyendo".to_string();
            }
        }
    }
}

pub fn main()-> iced::Result {
    Counter::run(iced::Settings::default())
}