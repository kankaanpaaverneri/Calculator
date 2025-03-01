use std::f64;

use iced::widget::{button, column, container, row, text};
use iced::Element;
use iced::Length::Fill;

#[derive(Clone, Debug)]
pub enum State {
    None,
    Value(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
    Power,
    Comma,
    Remove,
    Equal,
    ClearAll,
}

pub struct Calculator {
    text_buffer: String,
    value: f64,
    state: State,
}

const PLUS: char = '+';
const MINUS: char = '-';
const MULTIPLY: char = 'x';
const DIVIDE: char = '/';
const EQUAL: char = '=';
const POWER: char = '^';

fn is_number(character: &char) -> bool {
    match character {
        '0'..='9' => true,
        _ => false,
    }
}

fn is_operator(character: &char) -> bool {
    match character {
        &PLUS | &MINUS | &MULTIPLY | &DIVIDE | &EQUAL | &POWER => true,
        _ => false,
    }
}

fn str_is_operator(text: &str) -> bool {
    let mut i = 0;
    let mut operator_found = false;
    for ref character in text.chars() {
        if is_operator(character) {
            operator_found = true;
        }
        i += 1;
    }
    if i == 1 && operator_found {
        return true;
    }
    false
}

fn contains_only_numbers(text: &str) -> bool {
    let mut only_numbers_found = false;
    let mut decimal_point = false;
    let mut decimal_point_count = 0;
    for (i, ch) in text.chars().enumerate() {
        if ch == '.' && decimal_point_count == 0 {
            decimal_point = true;
            decimal_point_count += 1;
        }

        if i == 0 && ch == PLUS || ch == MINUS {
            continue;
        }

        if is_number(&ch) {
            only_numbers_found = true;
        } else if decimal_point {
            decimal_point = false;
        } else {
            return false;
        }
    }
    only_numbers_found
}

#[cfg(test)]
mod tests {
    use crate::{parse_expression, str_is_operator, CalculationError};

    use super::contains_only_numbers;
    #[test]
    fn test_contains_only_numbers() {
        assert_eq!(contains_only_numbers("231"), true);
        assert_eq!(contains_only_numbers("213+213"), false);
        assert_eq!(contains_only_numbers("ee22"), false);
        assert_eq!(contains_only_numbers("2"), true);
        assert_eq!(contains_only_numbers("1200"), true);
        assert_eq!(contains_only_numbers("as"), false);
    }

    #[test]
    fn test_parse_expression() {
        assert_eq!(true, check(parse_expression("30+1.5-5.55+12x5/2")));
        assert_eq!(false, check(parse_expression("30..5+12-5x65")));
        assert_eq!(true, check(parse_expression("35.+12")));
        assert_eq!(true, check(parse_expression(".5x54")));
        assert_eq!(true, check(parse_expression("5+12x5-5/2")));
        assert_eq!(false, check(parse_expression("+534-2")));
    }

    #[test]
    fn test_str_is_operator() {
        assert_eq!(true, str_is_operator("+"));
        assert_eq!(true, str_is_operator("-"));
        assert_eq!(true, str_is_operator("x"));
        assert_eq!(true, str_is_operator("/"));
        assert_eq!(true, str_is_operator("^"));
        assert_eq!(false, str_is_operator(" ^"));
        assert_eq!(false, str_is_operator("^ "));
        assert_eq!(false, str_is_operator(" "));
        assert_eq!(false, str_is_operator("/ "));
    }

    fn check<T>(result: Result<T, CalculationError>) -> bool {
        match result {
            Ok(_) => true,
            Err(CalculationError::InvalidExpression) => false,
            Err(CalculationError::DivideByZero) => false,
        }
    }
}

fn parse_operator(operator: &char) -> Option<State> {
    match operator {
        &PLUS => Some(State::Plus),
        &MINUS => Some(State::Minus),
        &MULTIPLY => Some(State::Multiply),
        &DIVIDE => Some(State::Divide),
        &POWER => Some(State::Power),
        _ => None,
    }
}

fn parse_str_operator(text: &str) -> Option<State> {
    if text.len() == 1 {
        for ref operator in text.chars() {
            return parse_operator(operator);
        }
    }
    None
}

fn calculate_expression(expression: &str) -> Result<f64, CalculationError> {
    let parse_result = parse_expression(expression);
    if let Ok(ref tokens) = parse_result {
        let polish_notation = get_reverse_polish_notation(tokens);
        let result = shunting_yard(&polish_notation);
        match result {
            Ok(mut result) => {
                let value = result.pop();
                match value {
                    Some(value) => return Ok(value),
                    None => {}
                }
            }
            Err(error) => return Err(error),
        }
    }

    if let Err(parse_error) = parse_result {
        return Err(parse_error);
    }

    Err(CalculationError::InvalidExpression)
}

fn shunting_yard(polish_notation: &Vec<&str>) -> Result<Vec<f64>, CalculationError> {
    let mut result_stack: Vec<f64> = Vec::new();
    for token in polish_notation {
        if contains_only_numbers(*token) {
            let result = (*token).parse::<f64>();
            if let Ok(value) = result {
                result_stack.push(value);
            }
        } else if str_is_operator(*token) {
            if let Err(err) = calculate_prev_two_from_stack(&mut result_stack, *token) {
                return Err(err);
            }
        }
    }
    Ok(result_stack)
}

fn calculate_prev_two_from_stack(
    result_stack: &mut Vec<f64>,
    operation: &str,
) -> Result<(), CalculationError> {
    let top = result_stack.pop();
    let prev = result_stack.pop();

    match (prev, top) {
        (Some(a), Some(b)) => {
            if let Some(ref operator) = parse_str_operator(operation) {
                let result = calculate(a, b, operator);
                if let Ok(value) = result {
                    result_stack.push(value);
                }
                if let Err(error) = result {
                    return Err(error);
                }
            }
        }
        _ => {}
    }
    Ok(())
}

fn get_reverse_polish_notation<'a>(tokens: &'a Vec<&str>) -> Vec<&'a str> {
    let mut output: Vec<&str> = Vec::new();
    let mut operator_stack: Vec<&str> = Vec::new();
    for token in tokens {
        if contains_only_numbers(token) {
            output.push(token);
        } else if str_is_operator(token) {
            if let Some(top) = operator_stack.last() {
                if get_precedence(token) <= get_precedence(*top) {
                    let pop_count = flush_operator_stack(&mut output, &operator_stack, *token);
                    for _ in 0..pop_count {
                        operator_stack.pop();
                    }
                }
            }
            operator_stack.push(token);
        }
    }
    //Push remaining operators one by one
    for operator in operator_stack.iter().rev() {
        output.push(*operator);
    }
    output
}

fn flush_operator_stack<'a, 'b>(
    output: &mut Vec<&'a str>,
    operator_stack: &Vec<&'a str>,
    current_token: &'a str,
) -> usize {
    let mut pop_count: usize = 0;
    for operator in operator_stack.iter().rev() {
        if get_precedence(*operator) >= get_precedence(current_token) {
            output.push(*operator);
            pop_count += 1;
        } else {
            break;
        }
    }
    pop_count
}

fn get_character_precedence(operator: &char) -> usize {
    match operator {
        &PLUS => 1,
        &MINUS => 1,
        &MULTIPLY => 2,
        &DIVIDE => 2,
        &POWER => 3,
        _ => 0,
    }
}

fn get_precedence(text: &str) -> usize {
    if text.len() == 1 {
        for ref character in text.chars() {
            return get_character_precedence(character);
        }
    }
    0
}

fn parse_expression(expression: &str) -> Result<Vec<&str>, CalculationError> {
    let mut parsed: Vec<&str> = Vec::new();
    let mut start: usize = 0;
    for (i, character) in expression.chars().enumerate() {
        if is_operator(&character) && i > 0 {
            if contains_only_numbers(&expression[start..i]) {
                parsed.push(&expression[start..i]); // Push the numerical value
                start = i + 1;
            } else {
                return Err(CalculationError::InvalidExpression);
            }
            parsed.push(&expression[i..start]); // Push the operator
        }
    }
    if contains_only_numbers(&expression[start..expression.len()]) {
        parsed.push(&expression[start..expression.len()]); // Push final numerical value
    } else {
        return Err(CalculationError::InvalidExpression);
    }

    Ok(parsed)
}

impl Default for Calculator {
    fn default() -> Self {
        Self {
            text_buffer: String::new(),
            value: 0.0,
            state: State::None,
        }
    }
}
#[derive(Debug)]
enum CalculationError {
    InvalidExpression,
    DivideByZero,
}

fn calculate(
    prev_value: f64,
    next_value: f64,
    calculation_state: &State,
) -> Result<f64, CalculationError> {
    match calculation_state {
        State::Plus => Ok(prev_value + next_value),
        State::Minus => Ok(prev_value - next_value),
        State::Multiply => Ok(prev_value * next_value),
        State::Divide => {
            if next_value <= 0.0 {
                return Err(CalculationError::DivideByZero);
            }
            Ok(prev_value / next_value)
        }
        State::Power => Ok(prev_value.powf(next_value)),
        _ => Ok(prev_value),
    }
}

const BUTTON_HEIGHT: f32 = 100.0;
const BUTTON_WIDTH: f32 = 100.0;
const BUTTON_TEXT_SIZE: f32 = 30.0;
const BUTTON_SPACING: f32 = 5.0;

impl Calculator {
    pub fn update(&mut self, state: State) {
        match state {
            State::Value(next_value) => {
                self.text_buffer.push_str(next_value.to_string().as_str());
            }
            State::Plus => {
                self.state = State::Plus;
                self.text_buffer.push('+');
            }
            State::Minus => {
                self.state = State::Minus;
                self.text_buffer.push('-');
            }
            State::Multiply => {
                self.state = State::Multiply;
                self.text_buffer.push('x');
            }
            State::Divide => {
                self.state = State::Divide;
                self.text_buffer.push('/');
            }
            State::Power => {
                self.state = State::Power;
                self.text_buffer.push('^');
            }
            State::Comma => {
                self.text_buffer.push('.');
            }
            State::Remove => {
                self.text_buffer.pop();
            }
            State::Equal => {
                let result = calculate_expression(&self.text_buffer);
                self.text_buffer.clear();
                match result {
                    Ok(value) => {
                        let text_value = format!("{}", value);
                        self.text_buffer.push_str(text_value.as_str());
                        self.value = value;
                    }
                    Err(CalculationError::InvalidExpression) => {
                        self.text_buffer.push_str("Invalid expression");
                    }
                    Err(CalculationError::DivideByZero) => {
                        self.text_buffer.push_str("Error divide by zero");
                    }
                }
            }
            State::ClearAll => self.text_buffer = String::new(),
            State::None => self.state = State::None,
        }
    }

    pub fn view(&self) -> Element<'_, State> {
        container(
            column![
                text(self.text_buffer.as_str()).size(50),
                row![
                    button(text(EQUAL).size(BUTTON_TEXT_SIZE).center()).on_press(State::Equal),
                    button(text("C").size(BUTTON_TEXT_SIZE).center()).on_press(State::ClearAll),
                    button(text("^").size(BUTTON_TEXT_SIZE).center()).on_press(State::Power)
                ]
                .spacing(BUTTON_SPACING),
                row![
                    button(text("7").size(BUTTON_TEXT_SIZE).center())
                        .on_press(State::Value(7.0))
                        .height(BUTTON_HEIGHT)
                        .width(BUTTON_WIDTH),
                    button(text("8").size(BUTTON_TEXT_SIZE).center())
                        .on_press(State::Value(8.0))
                        .height(BUTTON_HEIGHT)
                        .width(BUTTON_WIDTH),
                    button(text("9").size(BUTTON_TEXT_SIZE).center())
                        .on_press(State::Value(9.0))
                        .height(BUTTON_HEIGHT)
                        .width(BUTTON_WIDTH),
                    button(text(MULTIPLY).size(BUTTON_TEXT_SIZE).center())
                        .on_press(State::Multiply)
                        .height(BUTTON_HEIGHT)
                        .width(BUTTON_WIDTH)
                ]
                .spacing(BUTTON_SPACING),
                row![
                    button(text("4").size(BUTTON_TEXT_SIZE).center())
                        .on_press(State::Value(4.0))
                        .height(BUTTON_HEIGHT)
                        .width(BUTTON_WIDTH),
                    button(text("5").size(BUTTON_TEXT_SIZE).center())
                        .on_press(State::Value(5.0))
                        .height(BUTTON_HEIGHT)
                        .width(BUTTON_WIDTH),
                    button(text("6").size(BUTTON_TEXT_SIZE).center())
                        .on_press(State::Value(6.0))
                        .height(BUTTON_HEIGHT)
                        .width(BUTTON_WIDTH),
                    button(text(MINUS).size(BUTTON_TEXT_SIZE).center())
                        .on_press(State::Minus)
                        .height(BUTTON_HEIGHT)
                        .width(BUTTON_WIDTH)
                ]
                .spacing(BUTTON_SPACING),
                row![
                    button(text("1").size(BUTTON_TEXT_SIZE).center())
                        .on_press(State::Value(1.0))
                        .height(BUTTON_HEIGHT)
                        .width(BUTTON_HEIGHT),
                    button(text("2").size(BUTTON_TEXT_SIZE).center())
                        .on_press(State::Value(2.0))
                        .height(BUTTON_HEIGHT)
                        .width(BUTTON_WIDTH),
                    button(text("3").size(BUTTON_TEXT_SIZE).center())
                        .on_press(State::Value(3.0))
                        .height(BUTTON_HEIGHT)
                        .width(BUTTON_WIDTH),
                    button(text(PLUS).size(BUTTON_TEXT_SIZE).center())
                        .on_press(State::Plus)
                        .height(BUTTON_HEIGHT)
                        .width(BUTTON_WIDTH)
                ]
                .spacing(BUTTON_SPACING),
                row![
                    button(text("0").size(BUTTON_TEXT_SIZE).center())
                        .on_press(State::Value(0.0))
                        .height(BUTTON_HEIGHT)
                        .width(BUTTON_WIDTH),
                    button(text("<-").size(BUTTON_TEXT_SIZE).center())
                        .on_press(State::Remove)
                        .height(BUTTON_HEIGHT)
                        .width(BUTTON_WIDTH),
                    button(text(",").size(BUTTON_TEXT_SIZE).center())
                        .on_press(State::Comma)
                        .height(BUTTON_HEIGHT)
                        .width(BUTTON_WIDTH),
                    button(text(DIVIDE).size(BUTTON_TEXT_SIZE).center())
                        .on_press(State::Divide)
                        .height(BUTTON_HEIGHT)
                        .width(BUTTON_WIDTH)
                ]
                .spacing(BUTTON_SPACING)
            ]
            .spacing(BUTTON_SPACING),
        )
        .center(Fill)
        .into()
    }
}
