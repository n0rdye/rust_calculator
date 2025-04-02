// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![windows_subsystem = "windows"]
use std::{cell::RefCell, error::Error, rc::Rc};

use slint::{SharedString};

slint::include_modules!();
use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;
    let opers = Rc::new(RefCell::new(vec!["0".to_string()]));
    let num_regx = Regex::new(r"[0-9]").unwrap();

    print_to_slint(ui.as_weak().unwrap(), Rc::clone(&opers).borrow().to_vec());
    
    // number click scope
    ui.on_number_click({
        let weak_ui = ui.as_weak(); 
        let opers = Rc::clone(&opers);
        let num_regx = num_regx.clone();

        move |int|{
            let window = weak_ui.unwrap();
            let mut opers = opers.borrow_mut();
            let len_of_opers = get_len_of_opers(opers.to_vec());

            if num_regx.is_match(&opers[get_len_of_opers(opers.to_vec())]){
                if opers[len_of_opers] == "0"{
                    opers[len_of_opers] = int.to_string();
                }
                else {
                    let cur_val = &opers[get_len_of_opers(opers.to_vec())].to_string();
                    let int_as_string = &int.to_string();
                    let add_val = String::as_str(int_as_string);
                    opers[len_of_opers] = format!("{cur_val}{add_val}");
                }

            }
            #[cfg(debug_assertions)] dbg!(&opers);

            print_to_slint(window, opers.to_vec());
        }
    });

    // operator click scope
    ui.on_operator_click({
            let weak_ui = ui.as_weak();
            let opers = Rc::clone(&opers);
            move |string|{
                let mut opers = opers.borrow_mut();
                let window = weak_ui.unwrap();
                let input = string.clone();

                match String::as_str(&String::from(string)) {
                    "=" => {
                        let mut last_num: Option<i32> = None;
                        let mut m2: Vec<String> = opers.iter().rev().cloned().collect();
                        let mut sum: i32 = 0;
                        #[cfg(debug_assertions)]{dbg!(&m2);}
                        while let Some(num) = m2.pop() {
                            match String::as_str(&num) {
                                "+" => {
                                    if let Some(value) = last_num {
                                        if let Some(next_num) = m2.pop() {
                                            let next_value = get_i32(&next_num);
                                            sum = value + next_value;
                                            last_num = Some(sum); 
                                        }
                                    }
                                    #[cfg(debug_assertions)] dbg!(&m2);
                                }
                                "-" => {
                                    if let Some(value) = last_num {
                                        if let Some(next_num) = m2.pop() {
                                            let next_value = get_i32(&next_num);
                                            sum = value - next_value;
                                            last_num = Some(sum);
                                        }
                                    }
                                    #[cfg(debug_assertions)] dbg!(&m2);
                                }
                                "*" => {
                                    if let Some(value) = last_num {
                                        if let Some(next_num) = m2.pop() {
                                            let next_value = get_i32(&next_num);
                                            sum = value * next_value;
                                            last_num = Some(sum);
                                        }
                                    }
                                    #[cfg(debug_assertions)] dbg!(&m2);
                                }
                                "/" => {
                                    if let Some(value) = last_num {
                                        if let Some(next_num) = m2.pop() {
                                            let next_value = get_i32(&next_num);
                                            if next_value != 0 {
                                                sum = value / next_value;
                                                last_num = Some(sum);
                                            } else {
                                                dbg!("Attempt to divide by zero");
                                            }
                                        }
                                    }
                                    #[cfg(debug_assertions)] dbg!(&m2);
                                }
                                _ => {
                                    let current_value = get_i32(&num);
                                    last_num = Some(current_value); 
                                    #[cfg(debug_assertions)] dbg!(&m2);
                                }
                            }
                            #[cfg(debug_assertions)] dbg!(&opers);
                        }

                        opers.clear();
                        opers.push(sum.to_string());
                        print_to_slint(window, opers.to_vec());
                    }
                    "c" => {
                        opers.clear();
                        opers.push("0".to_string());
                        print_to_slint(window, opers.to_vec());
                    }
                    _=> {
                        opers.push(String::from(input));
                        opers.push("0".to_string());
                        print_to_slint(window, opers.to_vec());
                    }
                }
            #[cfg(debug_assertions)] dbg!(&opers);

            }
        }); 

    ui.run()?;
    Ok(())
}


fn print_to_slint(ui: AppWindow, opers: Vec<String>){
    let weak_ui = ui.as_weak();
    let window = weak_ui.unwrap();
    window.set_result(SharedString::from(opers.join("")));
}

fn get_len_of_opers(opers: Vec<String>) -> usize{
    return (opers.len() as i32 - 1) as usize
}

fn get_i32(num: &str) -> i32{
    num.parse().expect("Not a valid next number")
}