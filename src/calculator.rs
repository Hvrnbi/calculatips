// Calulation crate
use evalexpr::eval ;
// GUI crate
use iced::{
    alignment::{Horizontal, Vertical}, widget::{button, column, horizontal_rule, horizontal_space, row, text, vertical_rule, vertical_space, Container}, Element, Length
};
use iced_aw::tab_bar::TabLabel;
use crate::{Icon, Message, Tab};

#[derive(Debug, Clone)]
pub enum CalculatorMessage{
    // Numbers messages
    Pressed0,
    Pressed1,
    Pressed2,
    Pressed3,
    Pressed4,
    Pressed5,
    Pressed6,
    Pressed7,
    Pressed8,
    Pressed9,
    // Other messages
    PressedDot,
    PressedEqual,
    PressedDel,
    PressedClear,
    PressedOpeningParanthesis,
    PressedClosingParanthesis,
    PressedAddition,
    PressedSubstraction,
    PressedMultiplication,
    PressedDivision,
}

#[derive(Default)]
pub struct CalculatorTab{
    displayed_calc: String,
    calc: String,
    result: String,
    fontsize: u16,
}

impl CalculatorTab{

    pub fn update(&mut self, _message: CalculatorMessage) {

        // Update the fontsize
        self.fontsize = match self.displayed_calc.len() {
            0 => 24,
            29 => 24,
            30 => 16,
            39 => 16,
            40 => 8,
            49 => 8,
            _ => self.fontsize,
        };

        match _message {
            // When then keyboard buttons are pressed
            CalculatorMessage::Pressed0 => if self.displayed_calc.len()<90 {self.displayed_calc += "0"; if in_float(&self.calc){if &self.displayed_calc[self.displayed_calc.len()-2..self.displayed_calc.len()-1] != "." {self.calc = self.calc[..self.calc.len()-1].to_string()} self.calc += "0"} else {self.calc += "0."}},
            CalculatorMessage::Pressed1 => if self.displayed_calc.len()<90 {self.displayed_calc += "1"; if in_float(&self.calc){if &self.displayed_calc[self.displayed_calc.len()-2..self.displayed_calc.len()-1] != "." {self.calc = self.calc[..self.calc.len()-1].to_string()} self.calc += "1"} else {self.calc += "1."}},
            CalculatorMessage::Pressed2 => if self.displayed_calc.len()<90 {self.displayed_calc += "2"; if in_float(&self.calc){if &self.displayed_calc[self.displayed_calc.len()-2..self.displayed_calc.len()-1] != "." {self.calc = self.calc[..self.calc.len()-1].to_string()} self.calc += "2"} else {self.calc += "2."}},
            CalculatorMessage::Pressed3 => if self.displayed_calc.len()<90 {self.displayed_calc += "3"; if in_float(&self.calc){if &self.displayed_calc[self.displayed_calc.len()-2..self.displayed_calc.len()-1] != "." {self.calc = self.calc[..self.calc.len()-1].to_string()} self.calc += "3"} else {self.calc += "3."}},
            CalculatorMessage::Pressed4 => if self.displayed_calc.len()<90 {self.displayed_calc += "4"; if in_float(&self.calc){if &self.displayed_calc[self.displayed_calc.len()-2..self.displayed_calc.len()-1] != "." {self.calc = self.calc[..self.calc.len()-1].to_string()} self.calc += "4"} else {self.calc += "4."}},
            CalculatorMessage::Pressed5 => if self.displayed_calc.len()<90 {self.displayed_calc += "5"; if in_float(&self.calc){if &self.displayed_calc[self.displayed_calc.len()-2..self.displayed_calc.len()-1] != "." {self.calc = self.calc[..self.calc.len()-1].to_string()} self.calc += "5"} else {self.calc += "5."}},
            CalculatorMessage::Pressed6 => if self.displayed_calc.len()<90 {self.displayed_calc += "6"; if in_float(&self.calc){if &self.displayed_calc[self.displayed_calc.len()-2..self.displayed_calc.len()-1] != "." {self.calc = self.calc[..self.calc.len()-1].to_string()} self.calc += "6"} else {self.calc += "6."}},
            CalculatorMessage::Pressed7 => if self.displayed_calc.len()<90 {self.displayed_calc += "7"; if in_float(&self.calc){if &self.displayed_calc[self.displayed_calc.len()-2..self.displayed_calc.len()-1] != "." {self.calc = self.calc[..self.calc.len()-1].to_string()} self.calc += "7"} else {self.calc += "7."}},
            CalculatorMessage::Pressed8 => if self.displayed_calc.len()<90 {self.displayed_calc += "8"; if in_float(&self.calc){if &self.displayed_calc[self.displayed_calc.len()-2..self.displayed_calc.len()-1] != "." {self.calc = self.calc[..self.calc.len()-1].to_string()} self.calc += "8"} else {self.calc += "8."}},
            CalculatorMessage::Pressed9 => if self.displayed_calc.len()<90 {self.displayed_calc += "9"; if in_float(&self.calc){if &self.displayed_calc[self.displayed_calc.len()-2..self.displayed_calc.len()-1] != "." {self.calc = self.calc[..self.calc.len()-1].to_string()} self.calc += "9"} else {self.calc += "9."}},
            CalculatorMessage::PressedAddition => if self.displayed_calc.len()<90 {self.displayed_calc += "+"; self.calc += "+"},
            CalculatorMessage::PressedSubstraction => if self.displayed_calc.len()<90 {self.displayed_calc += "-"; self.calc += "-"},
            CalculatorMessage::PressedMultiplication => if self.displayed_calc.len()<90 {self.displayed_calc += "x"; self.calc += "*"},
            CalculatorMessage::PressedDivision => if self.displayed_calc.len()<90 {self.displayed_calc += "÷"; self.calc += "/"},
            CalculatorMessage::PressedOpeningParanthesis => if self.displayed_calc.len()<90 {self.displayed_calc += "("; self.calc += "("},
            CalculatorMessage::PressedClosingParanthesis => if self.displayed_calc.len()<90 {self.displayed_calc += ")"; self.calc += ")"},
            CalculatorMessage::PressedDot => if self.displayed_calc.len()<90 && !in_float(&self.displayed_calc) {self.displayed_calc += "."; if &self.calc[self.calc.len()-1..] != "." {self.calc+= "."} } ,
            CalculatorMessage::PressedDel => if self.displayed_calc!="" {self.displayed_calc = self.displayed_calc[..self.displayed_calc.len()-1].to_string(); if &self.calc[self.calc.len()-1..] == "." {self.calc=self.calc[..self.calc.len()-2].to_string()} else {self.calc=self.calc[..self.calc.len()-1].to_string()}},
            CalculatorMessage::PressedClear => {self.displayed_calc.clear(); self.calc.clear(); self.result.clear();},
            CalculatorMessage::PressedEqual => self.result = eval(&self.calc).expect("None").to_string(),
        }
    }


}

impl Tab for CalculatorTab{
    type Message = Message;

    fn tab_label(&self) -> TabLabel {
        TabLabel::IconText(Icon::Calc.into(), String::from("Calculator"))
    }

    fn content(&self) -> Element<'_, Self::Message> {
        let content: Element<'_, CalculatorMessage> = Container::new(
            column![
                row![
                    // Text widget where the user inputs will be displayed
                    text(self.displayed_calc.clone()).width(Length::FillPortion(6)).height(80).align_x(Horizontal::Left).align_y(Vertical::Center).size(8+self.fontsize),
                    vertical_rule(4),
                    // Text widget where the result of the calcul  will be displayed
                    text(self.result.to_string()).width(Length::FillPortion(2)).height(80).align_x(Horizontal::Center).align_y(Vertical::Center).size(32),
                ],
                horizontal_rule(4),
                // Text for Tips
                text("💡 Tips : 2+2 is always equal to 4, you can just memorize it.").shaping(text::Shaping::Advanced).width(Length::Fill).height(80).size(14).align_x(Horizontal::Center).align_y(Vertical::Center),
                horizontal_rule(4),
                vertical_space().height(12),
                // First row : Clear ( ) /
                row![
                    // Space left to the first button
                    horizontal_space().width(Length::FillPortion(2)),
                    // Clear button
                    button("Ø").padding(30).width(70).on_press(CalculatorMessage::PressedClear),
                    // Space between two buttons
                    horizontal_space().width(Length::FillPortion(1)),
                    // ( button
                    button("(").padding(30).width(70).on_press(CalculatorMessage::PressedOpeningParanthesis),
                    // Space between two buttons
                    horizontal_space().width(Length::FillPortion(1)),
                    // ) button
                    button(")").padding(30).width(70).on_press(CalculatorMessage::PressedClosingParanthesis),
                    // Space between two buttons
                    horizontal_space().width(Length::FillPortion(1)),
                    // / button
                    button("÷").padding(30).width(70).on_press(CalculatorMessage::PressedDivision),
                    // Space left to the first button
                    horizontal_space().width(Length::FillPortion(2)),
                ],
                vertical_space().height(12),
                // Second row : 7 8 9 *
                row![
                    // Space left to the first button
                    horizontal_space().width(Length::FillPortion(2)),
                    // 7 button
                    button("7").padding(30).width(70).on_press(CalculatorMessage::Pressed7),
                    // Space between two buttons
                    horizontal_space().width(Length::FillPortion(1)),
                    // 8 button
                    button("8").padding(30).width(70).on_press(CalculatorMessage::Pressed8),
                    // Space between two buttons
                    horizontal_space().width(Length::FillPortion(1)),
                    // 9 button
                    button("9").padding(30).width(70).on_press(CalculatorMessage::Pressed9),
                    // Space between two buttons
                    horizontal_space().width(Length::FillPortion(1)),
                    // * button
                    button("x").padding(30).width(70).on_press(CalculatorMessage::PressedMultiplication),
                    // Space left to the first button
                    horizontal_space().width(Length::FillPortion(2)),
                ],
                vertical_space().height(12),
                // Third row : 4 5 6 -
                row![
                    // Space left to the first button
                    horizontal_space().width(Length::FillPortion(2)),
                    // 4 button
                    button("4").padding(30).width(70).on_press(CalculatorMessage::Pressed4),
                    // Space between two buttons
                    horizontal_space().width(Length::FillPortion(1)),
                    // 5 button
                    button("5").padding(30).width(70).on_press(CalculatorMessage::Pressed5),
                    // Space between two buttons
                    horizontal_space().width(Length::FillPortion(1)),
                    // 6 button
                    button("6").padding(30).width(70).on_press(CalculatorMessage::Pressed6),
                    // Space between two buttons
                    horizontal_space().width(Length::FillPortion(1)),
                    // - button
                    button("-").padding(30).width(70).on_press(CalculatorMessage::PressedSubstraction),
                    // Space left to the first button
                    horizontal_space().width(Length::FillPortion(2)),
                ],
                vertical_space().height(12),
                // Fourth row : 1 2 3 +
                row![
                    // Space left to the first button
                    horizontal_space().width(Length::FillPortion(2)),
                    // 1 button
                    button("1").padding(30).width(70).on_press(CalculatorMessage::Pressed1),
                    // Space between two buttons
                    horizontal_space().width(Length::FillPortion(1)),
                    // 2 button
                    button("2").padding(30).width(70).on_press(CalculatorMessage::Pressed2),
                    // Space between two buttons
                    horizontal_space().width(Length::FillPortion(1)),
                    // 3 button
                    button("3").padding(30).width(70).on_press(CalculatorMessage::Pressed3),
                    // Space between two buttons
                    horizontal_space().width(Length::FillPortion(1)),
                    // + button
                    button("+").padding(30).width(70).on_press(CalculatorMessage::PressedAddition),
                    // Space left to the first button
                    horizontal_space().width(Length::FillPortion(2)),
                ],
                vertical_space().height(12),
                // last row : . 0 = Delete
                row![
                    // Space left to the first button
                    horizontal_space().width(Length::FillPortion(2)),
                    // . button
                    button(".").padding(30).width(70).on_press(CalculatorMessage::PressedDot),
                    // Space between two buttons
                    horizontal_space().width(Length::FillPortion(1)),
                    // 0 button
                    button("0").padding(30).width(70).on_press(CalculatorMessage::Pressed0),
                    // Space between two buttons
                    horizontal_space().width(Length::FillPortion(1)),
                    // = button
                    button("=").padding(30).width(70).on_press(CalculatorMessage::PressedEqual),
                    // Space between two buttons
                    horizontal_space().width(Length::FillPortion(1)),
                    // Delete button
                    button("←").padding(30).width(70).on_press(CalculatorMessage::PressedDel),
                    // Space left to the first button
                    horizontal_space().width(Length::FillPortion(2)),
                ],
                vertical_space().height(12),
            ]
        )
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .into();

    content.map(Message::Calculator)
    }
}

// Floating numbers management
fn in_float(calc: &String) -> bool {
    if calc == "" {return false}
    for (_i, &item) in calc.as_bytes().iter().enumerate().rev() {
        if item == b'.' {return true}
        else if item == b'+' || item == b'-' || item == b'*' || item == b'/' /*|| item == b'x' || item == b'\xF7'*/ {return false}
    }
    false
}


// CALCULATIONS

// Calls all the functions needed while the string is not a single number
fn calculate_all(calc: &String) -> String {
    while !is_a_single_number(calc) {

    }
    String::from("0")
}

// Calculates a simple calculation
fn calculate_one_calcul(calc: &String) -> f64 {
    for (i, &item) in calc.as_bytes().iter().enumerate() {
        if i == 0 {
            if &item == &b'(' {
                return calc[1..calc.len()-1].parse().unwrap_or(0.0)
            } else {
                continue
            }
        }
        if &item == &b'+' {
            return calc[..i].parse().unwrap_or(0.0)+calc[i+1..].parse().unwrap_or(0.0)
        } else if &item == &b'-' {
            return calc[..i].parse().unwrap_or(0.0)-calc[i+1..].parse().unwrap_or(0.0)
        } else if &item == &b'*' {
            return calc[..i].parse().unwrap_or(0.0)*calc[i+1..].parse().unwrap_or(0.0)
        } else if &item == &b'/' {
            return calc[..i].parse().unwrap_or(0.0)/calc[i+1..].parse().unwrap_or(0.0)
        }
    }
    0.0
}

// Verifiy if a string is a single number (returns true) or a calculation (returns false)
fn is_a_single_number(calc: &String) -> bool {
    for (_i, &item) in calc.as_bytes().iter().enumerate() {
        if &item == &b'+' || &item == &b'-' || &item == &b'*' || &item == &b'/' {
            return false
        }
    }
    true
}

// Find the operation to do first, returns a tuple with the index of
// the first digit of the first number and the last digit of the second number
fn find_priority(calc: &String) -> (usize, usize) {
    let mut operator = b' ';
    let mut op_index: usize = 0;
    for (i, &item) in calc.as_bytes().iter().enumerate() {
        if &item == &b'+' || &item == &b'-' {
            if operator == b' ' || operator == b'(' {
                operator = item;
                op_index = i;
            }
        } else if &item == &b'*' || &item == &b'/' {
            
        }
    }
    (0,0)
}

// Find the index of the start of the previous number
fn find_previous(calc: &String) -> usize {
    for (i, &item) in calc.as_bytes().iter().enumerate().rev() {
        if &item == &b'+' || &item == &b'-' || &item == &b'*' || &item == &b'/' || &item == &b')' || &item == &b'(' {
            return i+1
        }
    }
    0
}

// Find the index of the end of the next number
fn find_next(calc: &String) -> usize {
    for (i, &item) in calc.as_bytes().iter().enumerate() {
        if &item == &b'+' || &item == &b'-' || &item == &b'*' || &item == &b'/' || &item == &b')' || &item == &b'(' {
            return i-1
        }
    }
    calc.len()-1
}



// It's hard so I use a crate, I'll make it myself later if I have the time

// A simplified Python eval() function
//fn eval(calc:&String) -> f64 {
//    for (_i,car) in calc.chars().enumerate() {
//        if car=='(' {
//            // I'll make it later
//        } else if car=='0' || car=='1' || car=='2' || car=='3' || car=='4' || car=='5' || car=='6' || car=='7' || car=='8' || car=='9' {
//            
//        }
//    }
//    0.0
//
//}

//fn find_number(car_chain:&str, index:usize) -> f64{ 
//    if index < car_chain.len()-1 {
//        let mut thing: &str= &car_chain[index..];
//        for i in 0..thing.len() {
//            if thing.as_bytes()[i]==b'+' || thing.as_bytes()[i]==b'-' || thing.as_bytes()[i]==b'x' || thing.as_bytes()[i]==b'/' {
//                let new_thing = &thing[..i];
//                break
//            }
//        }
//        return match new_thing.parse::<f64>() {
//            Ok(num) => num,
//            Err(_) => 0.0
//        }, find_number(thing, new_thing.len())
//    }
//}