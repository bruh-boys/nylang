use std::fs;
use std::io::{self};
use colored::*;

use super::super::lexer;
use super::super::token;
use super::super::eval;
use super::super::parser;

pub fn run_program(file: String) -> Result<(), io::Error> {
    let _lines = match fs::read_to_string(file) {
        Ok(lines) => lines,
        Err(_e) => return Err(io::Error::new(io::ErrorKind::Other, "Could not read file")),
    };

    let blue_color_res: Result<Color, ()> = "magenta".parse();
    println!("\n-! {}\n", "excuting program".color(blue_color_res.unwrap_or(Color::Green)).bold());

    let mut env = eval::eval::Evaluator::new();
    let l = lexer::lexer::Lexer::new(_lines.as_str());
    let mut p = parser::parser::Parser::new(l);
    let program = p.program_parser();

    env.evaluator(program);

    println!("\n-! {}", "-! finished".color(blue_color_res.unwrap_or(Color::Green)).bold());

    Ok(())
}

pub fn ast_program(file: String) -> Result<(), io::Error> {
    let _lines = match fs::read_to_string(file) {
        Ok(lines) => lines,
        Err(_e) => return Err(io::Error::new(io::ErrorKind::Other, "Could not read file")),
    };

    let blue_color_res: Result<Color, ()> = "magenta".parse();
    println!("\n-! {}\n", "builging ast tree...".color(blue_color_res.unwrap_or(Color::Green)).bold());
    
    let l = lexer::lexer::Lexer::new(_lines.as_str());
    let mut p = parser::parser::Parser::new(l);
    let program = p.program_parser();

    println!("{:?}", program);

    println!("\n-! {}", "-! finished".color(blue_color_res.unwrap_or(Color::Green)).bold());

    Ok(())
}

pub fn parse_program(file: String) -> Result<(), io::Error> {
    let _lines = match fs::read_to_string(file) {
        Ok(lines) => lines,
        Err(_e) => return Err(io::Error::new(io::ErrorKind::Other, "Could not read file")),
    };
    let mut l = lexer::lexer::Lexer::new(_lines.as_str());
    let mut tok = l.next_token();

    let blue_color_res: Result<Color, ()> = "magenta".parse();
    println!("\n-! {}\n", "parse started".color(blue_color_res.unwrap_or(Color::Green)).bold());

    loop {
        match &tok {
            token::token::Token::EOF => break,
            _ => println!("{:?}", tok)
        }
            
        tok = l.next_token();
    }

    println!("\n-! {}", "-! finished".color(blue_color_res.unwrap_or(Color::Green)).bold());

    Ok(())
}