use std::{
    fmt::Display,
    io::{BufRead, Write, stderr, stdin},
    thread,
    time::Instant,
};

use rust_countdown::{
    self,
    base_types::{
        expressions::{Operator, Operators},
        numbers::{CountdownNumberBaseType, ModularNumberSystem, NormalNumberSystem},
    },
    generators::expression_tree_generator::find_expressions,
    parsing::{Parsable, token_reader},
    timing::{MyReciever, threaded::channel},
};
fn get_input<S: Display>(question: S) -> Result<String, std::io::Error> {
    let mut input = stdin().lock();
    let mut output = stderr().lock();
    write!(output, "{question}: ")?;
    let mut buf = String::new();
    input.read_line(&mut buf)?;
    Ok(buf)
}
fn ask<T: Parsable + Sized, S: Display>(question: S) -> Result<T, String> {
    match get_input(question) {
        Ok(a) => T::parse(&mut token_reader::read(a)?.into_iter().collect()),
        Err(e) => Err(e.to_string()),
    }
}
enum NumberSystems<T: CountdownNumberBaseType> {
    Normal(NormalNumberSystem),
    Modular(ModularNumberSystem<T>),
}
fn get_number_system<T: CountdownNumberBaseType>(modulus: T) -> NumberSystems<T> {
    if modulus == T::ZERO {
        NumberSystems::Normal(NormalNumberSystem)
    } else {
        NumberSystems::Modular(ModularNumberSystem::new(modulus))
    }
}
fn run<T: CountdownNumberBaseType + Parsable + Sync + Send>(
    source_numbers: Vec<T>,
    target_number: T,
    number_system: NumberSystems<T>,
    operators: Operators,
) -> Result<(), String> {
    let (mut sender, receiver) = channel();
    let start = Instant::now();
    let t = match number_system {
        NumberSystems::Normal(number_system) => thread::spawn(move || {
            // let mut s = sender;
            find_expressions(
                source_numbers,
                &number_system,
                target_number,
                &operators,
                &mut sender,
            )
        }),
        NumberSystems::Modular(number_system) => thread::spawn(move || {
            // let mut s = sender;
            find_expressions(
                source_numbers,
                &number_system,
                target_number,
                &operators,
                &mut sender,
            )
        }),
    };
    let mut receiver = receiver.into_iterator();
    let first = receiver.next();
    let first_time = Instant::now();
    let mut v = vec![];
    let mut time = 1;
    if let Some(first) = first {
        v.push(first);
        eprintln!("First item found in {:?}", (first_time - start));
        v.extend(receiver.enumerate().map(|(i, a)| {
            if time * 10 <= (Instant::now() - start).as_secs() {
                eprintln!(
                    "Found {:?} expressions in {:.2}s",
                    i + 1,
                    (Instant::now() - start).as_secs_f64()
                );
                time += 1;
            }
            a
        }));
    }
    let done = Instant::now();
    eprintln!(
        "{} {} found in {:?}",
        v.len(),
        if v.len() == 1 { "expr" } else { "expressions" },
        done.duration_since(start)
    );
    let len = v.len().min(100);
    eprintln!("First {len} expressions:\n");
    for item in v.iter().take(100) {
        eprintln!("{}\t {} \t {:?}", item.get_value(), item, item);
    }

    t.join().unwrap();
    Ok(())
}

fn _main<T: CountdownNumberBaseType + Parsable + Sync + Send>() -> Result<(), String> {
    let source_numbers = ask::<Vec<T>, _>("Please enter the source numbers")?;
    let target_number = ask::<T, _>("Please enter the target number")?;
    let number_system = get_number_system(ask::<T, _>("Please enter the modulus")?);
    let operators = Operators::from_iter(ask::<Vec<Operator>, _>(
        "Please enter the allowed operators",
    )?);
    run(source_numbers, target_number, number_system, operators)
}

fn main() {
    match _main::<u64>() {
        Ok(()) => {}
        Err(e) => eprintln!("Error: {e}"),
    }
}
